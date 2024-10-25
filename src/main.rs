mod household_info;
mod http_client;

use base64::prelude::*;
use household_info::{ValueFieldHouseOwner, ValueFieldHouseResident, ValueFieldHouseResident2024Education, ValueFieldHouseResidentGeneralEducation};
use serde_json::Value;
use std::{collections::HashMap, fs, path::PathBuf};
use calamine::{open_workbook, Error, Reader, Xls};
use colored::Colorize;
use inquire::{Confirm, Text};
use regex::Regex;
use rfd::FileDialog;

fn workbook_reader(file: &PathBuf, ngay_dieutra: &String, preflix_so_phieu: &String, pcgd_csrf_token: &String, cookies: &String) -> Result<(), Error> {
    let mut workbook: Xls<_> = open_workbook(file)?;
    let http_client = http_client::create_client_with_headers_preset(cookies);

    let preflix_so_phieu = preflix_so_phieu.split("_").collect::<Vec<&str>>();
    let ma_tinh = preflix_so_phieu[0];
    let ma_quanhuyen = format!("{}_{}", ma_tinh, preflix_so_phieu[1]);
    let ma_phuongxa = format!("{}_{}", ma_quanhuyen, preflix_so_phieu[2]);
    let ma_thonxom = format!("{}_{}", ma_phuongxa, preflix_so_phieu[3]);

    let mut so_chu_ho = 0;
    let mut so_thanh_vien = 0;
    let mut so_chu_ho_uploaded = 0;
    let mut so_thanh_vien_uploaded = 0;

    if let Ok(range) = workbook.worksheet_range("MauNhapLieu") {
        let mut houses_owners: HashMap<String, Vec<ValueFieldHouseOwner>> = HashMap::new();
        let mut houses_residents: HashMap<
            String,
            Vec<(ValueFieldHouseResident, ValueFieldHouseResidentGeneralEducation, ValueFieldHouseResident2024Education)>
        > = HashMap::new();

        let rows = range.rows();
        let rows_data = rows.skip(4);

        println!("{} Đang thiết lập mẫu dữ liệu...", ">".green().bold());

        for col in rows_data {
            if col[47].to_string().to_lowercase() == "chủ hộ" {
                let household_owner = ValueFieldHouseOwner::new(
                    col,
                    ngay_dieutra.to_string(),
                    ma_tinh.to_string(),
                    ma_quanhuyen.to_string(),
                    ma_phuongxa.to_string(),
                    ma_thonxom.to_string(),
                    pcgd_csrf_token.to_string()
                );

                houses_owners.entry(col[14].to_string())
                    .or_insert_with(Vec::new)
                    .push(household_owner);

                so_chu_ho += 1;
            }
            
            let household_resident = ValueFieldHouseResident::new(col);
            let resident_education = ValueFieldHouseResidentGeneralEducation::new(col);
            let resident_2024_education = ValueFieldHouseResident2024Education::new(
                col,
                ma_tinh.to_string(),
                ma_quanhuyen.to_string()
            );

            houses_residents.entry(col[14].to_string())
            .or_insert_with(Vec::new)
            .push((
                household_resident,
                resident_education,
                resident_2024_education
            ));

            so_thanh_vien += 1;
        }

        println!("{} Đã dựng được {} chủ hộ và {} thành viên.", ">".green().bold(), so_chu_ho, so_thanh_vien);

        let mut mismatches: Vec<&String> = vec![];

        for token in houses_residents.keys() {
            if !houses_owners.contains_key(token) {
                mismatches.push(token);
            }
        }

        if mismatches.len() > 0 {
            println!("{} Đã phát hiện {} phiếu sau đây không có chủ hộ:", ">".green().bold(), mismatches.len());
            for mismatch in mismatches.iter().enumerate() {
                println!("{}. {}", mismatch.0 + 1, mismatch.1);
            }
        }

        match Confirm::new("Tiếp tục công việc?").with_default(false).prompt() {
            Ok(true) => println!("{} Đang thêm...", ">".green().bold()),
            Ok(false) => {
                println!("{}", "> Đã dừng công việc.".red().bold());
                return Ok(());
            },
            Err(_) => {
                println!("{}", "> Đã dừng công việc.".red().bold());
                return Ok(());
            },
        }

        let houses_owners_vec: Vec<(String, Vec<ValueFieldHouseOwner>)> =  houses_owners.into_iter().collect();

        for mut owner in houses_owners_vec {
            let json_owner = serde_json::to_string(&owner.1[0].build()).unwrap();
            let owner_params = [("data", &json_owner), ("pcgd-csrf-token", pcgd_csrf_token)];

            let response = http_client.post("https://pcgd.moet.gov.vn/doing/phieudieutra/update")
                .form(&owner_params)
                .send()
                .unwrap();

            let text = response.text().unwrap();
            let creation_response: Value = serde_json::from_str(&text).unwrap();

            println!("{:#?}", owner_params);
            println!("{}", creation_response);

            let mut ma_phieu = String::new();

            if creation_response["result"] == "success" {
                println!("{}", format!("{} \"{} {}\" {}", "> Tải lên thành công hộ gia đình", owner.1[0].chuho_hodem, owner.1[0].chuho_ten, owner.1[0].so_phieu).green().bold());
                ma_phieu = creation_response["ma_phieu"].to_string();
                
                ma_phieu.pop();
                ma_phieu.remove(0);

                so_chu_ho_uploaded += 1;
                println!("{}", ma_phieu);
            } else if creation_response["errors"]["so_phieu"].as_str().unwrap().ends_with(" đã tồn tại.") {
                println!("{}", format!("{} \"{} {}\" {} {}", "> Hộ gia đình", owner.1[0].chuho_hodem, owner.1[0].chuho_ten, owner.1[0].so_phieu, "đã tồn tại, đang sửa lại dữ liệu...").yellow().bold());

                let search_response = http_client.post("https://pcgd.moet.gov.vn/doing/phieudieutra/lay_phieu")
                    .form(
                        &[
                            ("tinh", ma_tinh),
                            ("quanhuyen", &ma_quanhuyen),
                            ("phuongxa", &ma_phuongxa),
                            ("tukhoa", &owner.1[0].so_phieu),
                            ("_search", "false"),
                            ("rows", "5"), 
                            ("page", "1"),
                            ("sidx", "so_phieu"),
                            ("pcgd-csrf-token", pcgd_csrf_token),
                        ])
                    .send()
                    .unwrap();

                let re = Regex::new("id\":\"([^\"]+)").unwrap();
                ma_phieu = re.captures(&search_response.text().unwrap()).unwrap().get(0).unwrap().as_str().split("id\":\"").last().unwrap().to_string();

                let doituong_response = http_client.post(format!("https://pcgd.moet.gov.vn/doing/doituong/lay_doituong?phieu={}", ma_phieu))
                    .form(
                        &[
                            ("_search", "false"),
                            ("rows", "10"), 
                            ("page", "1"),
                            ("sord", "asc"),
                            ("sidx", "ngay_sinh"),
                            ("pcgd-csrf-token", pcgd_csrf_token),
                        ])
                    .send()
                    .unwrap();

                let mut json_doituong: Value = serde_json::from_str(&doituong_response.text().unwrap()).unwrap();

                while json_doituong["records"] != "0" {
                    let mut delete_params: Vec<(&str, String)> = vec![];

                    let list_doituong = json_doituong["rows"].as_array_mut().unwrap();

                    for doituong in 0..list_doituong.len() {
                        let mut id = list_doituong[doituong]["id"].to_string();
                        id.pop();
                        id.remove(0);
                        println!("{}", format!("> Thiết lập {}", id).green().bold());
                        delete_params.push(("id[]", id));
                    }

                    delete_params.push(("pcgd-csrf-token", pcgd_csrf_token.to_string()));

                    let delete_response = http_client.post(format!("https://pcgd.moet.gov.vn/doing/doituong/delete"))
                        .form(&delete_params)
                        .send()
                        .unwrap();

                    let json_delete: Value = serde_json::from_str(&delete_response.text().unwrap()).unwrap();

                    if json_delete["result"] == "success" {
                        println!("{}", format!("> Đã lọc {} thành viên", list_doituong.len()).green().bold());
                    } else {
                        println!("{}", json_delete);
                    }

                    let doituong_response = http_client.post(format!("https://pcgd.moet.gov.vn/doing/doituong/lay_doituong?phieu={}", ma_phieu))
                        .form(
                            &[
                                ("_search", "false"),
                                ("rows", "10"), 
                                ("page", "1"),
                                ("sord", "asc"),
                                ("sidx", "ngay_sinh"),
                                ("pcgd-csrf-token", pcgd_csrf_token),
                            ])
                        .send()
                        .unwrap();

                    json_doituong = serde_json::from_str(&doituong_response.text().unwrap()).unwrap();
                }

                so_chu_ho_uploaded += 1;
                println!("{}", format!("> Hoàn thành lọc thành viên, đang thêm vào...").green().bold());
            } else {
                println!("{}", format!("{} \"{} {}\" {}", "> Có lỗi khi tải lên hộ gia đình\n\nThông tin debug:", owner.1[0].chuho_hodem, owner.1[0].chuho_ten, owner.1[0].so_phieu).red().bold());
                
                println!("{:#?}", owner_params);
                println!("{}", creation_response);

                println!("{}", "> Kết thúc thông tin debug.".red().bold());

                match Confirm::new("Bạn có muốn tiếp tục?").with_default(false).prompt() {
                    Ok(true) => println!("{} Đang tiếp tục...", ">".green().bold()),
                    Ok(false) => {
                        println!("{}", "> Đã dừng công việc.".red().bold());
                        return Ok(());
                    },
                    Err(_) => {
                        println!("{}", "> Đã dừng công việc.".red().bold());
                        return Ok(());
                    },
                }
            }

            for resident in houses_residents.get_mut(&owner.0).unwrap() {
                resident.0.update_ma_phieu(ma_phieu.clone());

                let data1_json = serde_json::to_string(&resident.0.build()).unwrap();
                let data2_json = serde_json::to_string(&resident.1.build()).unwrap();
                let data_dtht_json = serde_json::to_string(&resident.2.build()).unwrap();

                let resident_params = [
                    ("data1", &data1_json),
                    ("data2", &data2_json),
                    ("data3", &format!("{{\"pcgd-csrf-token\" : \"{}\"}}", pcgd_csrf_token)),
                    ("data_dtht", &format!("{{\"2024\": \"{}\"}}", BASE64_STANDARD.encode(data_dtht_json))),
                    ("pcgd-csrf-token", pcgd_csrf_token),
                ];

                let response = http_client.post("https://pcgd.moet.gov.vn/doing/doituong/add")
                    .form(&resident_params)
                    .send()
                    .unwrap();

                let resident_response: Value = serde_json::from_str(&response.text().unwrap()).unwrap();

                if resident_response["result"] == "success" {
                    so_thanh_vien_uploaded += 1;
                    println!("{}", format!("> Đã thêm \"{}\" vào hộ {}", resident.0.ho_ten, owner.0).green().bold());
                } else {
                    println!("{}", format!("> Có lỗi khi thêm \"{}\" vào hộ {}\n\n Thông tin debug:\n", resident.0.ho_ten, owner.0).red().bold());
                    
                    println!("{:#?}", resident.0);
                    println!("{:#?}", resident.1);
                    println!("{:#?}", resident.2);
                    println!("{:#?}", resident_params);
                    println!("{}", resident_response);

                    println!("{}", "> Kết thúc thông tin debug.".red().bold());

                    match Confirm::new("Bạn có muốn tiếp tục?").with_default(false).prompt() {
                        Ok(true) => println!("{} Đang tiếp tục...", ">".green().bold()),
                        Ok(false) => {
                            println!("{}", "> Đã dừng công việc.".red().bold());
                            return Ok(());
                        },
                        Err(_) => {
                            println!("{}", "> Đã dừng công việc.".red().bold());
                            return Ok(());
                        },
                    }
                }
            }
        }
    }

    println!("{}", format!("> Đã thêm {}/{} hộ và {}/{} thành viên và các hộ.", so_chu_ho_uploaded, so_chu_ho, so_thanh_vien_uploaded, so_thanh_vien).green().bold());

    Ok(())
}

fn main() {
    println!("{} Chọn file XLS", ">".green().bold());

    let excel_file = match FileDialog::new()
    .add_filter("Excel spreadsheet", &["xls"])
    .set_directory("/")
    .pick_file() {
        Some(file) => file,
        None => {
            println!("{}", "> Không nhận được file!".red().bold());
            return;
        },
    };

    println!("{} Chọn file có chứa lệnh cURL", ">".green().bold());

    let curl_file = match FileDialog::new()
    .set_directory("/")
    .pick_file() {
        Some(file) => file,
        None => {
            println!("{}", "> Không nhận được file!".red().bold());
            return;
        },
    };

    let curl_content = fs::read_to_string(curl_file).unwrap();

    let token_extactor = curl_content.split("pcgd-csrf-token=");
    let mut pcgd_csrf_token = token_extactor.last().unwrap().to_string();
    let _ = pcgd_csrf_token.pop();

    let re = Regex::new(r"'Cookie:\s([^']*)").unwrap();
    let cookies = re.captures(&curl_content).unwrap().get(0).unwrap().as_str().split("'Cookie: ").last().unwrap().to_string();

    let ngay_dieutra = match Text::new("Nhập ngày điều tra:").prompt() {
        Ok(ngay_dieutra) => ngay_dieutra,
        Err(_) => {
            println!("{}", "> Ngày điều tra không được để trống.".red().bold());
            return;
        },
    };

    let preflix_so_phieu = match Text::new("Nhập phần đầu của mã số phiếu (VD: XX_YYYY_ZZZZZ_N_):").prompt() {
        Ok(ngay_dieutra) => ngay_dieutra,
        Err(_) => {
            println!("{}", "> Đầu số phiếu không được để trống.".red().bold());
            return;
        },
    };

    let _ = workbook_reader(&excel_file, &ngay_dieutra, &preflix_so_phieu, &pcgd_csrf_token, &cookies);
}
