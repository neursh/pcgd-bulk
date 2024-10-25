use unidecode::unidecode;
use base64::prelude::*;
use calamine::Data;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ValueFieldHouseOwner {
    pub so_phieu: String,
    pub chuho_hodem: String,
    pub chuho_ten: String,
    pub dia_chi: String,
    pub tinh_trang_cu_tru: String,
    pub dien_thoai: String,
    pub ngay_dieutra: String,
    pub ma_tinh: String,
    pub ma_quanhuyen: String,
    pub ma_phuongxa: String,
    pub ma_thonxom: String,
    pub dien_cu_tru: String,
    pub ma_phieu: String,
    pub ghi_chu: String,
    pub pcgd_csrf_token: String
}

impl ValueFieldHouseOwner {
    pub fn new(col: &[Data], ngay_dieutra: String, ma_tinh: String, ma_quanhuyen: String, ma_phuongxa: String, ma_thonxom: String, pcgd_csrf_token: String) -> Self {
        ValueFieldHouseOwner {
            so_phieu: col[14].to_string(),
            chuho_hodem: col[2].to_string(),
            chuho_ten: col[3].to_string(),
            dia_chi: col[13].to_string(),
            tinh_trang_cu_tru: col[16].to_string(),
            dien_thoai: col[49].to_string(),
            ngay_dieutra,
            ma_tinh,
            ma_quanhuyen,
            ma_phuongxa,
            ma_thonxom,
            dien_cu_tru: col[15].to_string(),
            ma_phieu: "".to_owned(),
            ghi_chu: col[50].to_string(),
            pcgd_csrf_token
        }
    }

    pub fn build(&mut self) -> Self {
        ValueFieldHouseOwner {
            so_phieu: BASE64_STANDARD.encode(&self.so_phieu),
            chuho_hodem: BASE64_STANDARD.encode(&self.chuho_hodem),
            chuho_ten: BASE64_STANDARD.encode(&self.chuho_ten),
            dia_chi: BASE64_STANDARD.encode(&self.dia_chi),
            tinh_trang_cu_tru: BASE64_STANDARD.encode(&self.tinh_trang_cu_tru),
            dien_thoai: BASE64_STANDARD.encode(&self.dien_thoai),
            ngay_dieutra: BASE64_STANDARD.encode(&self.ngay_dieutra),
            ma_tinh: BASE64_STANDARD.encode(&self.ma_tinh),
            ma_quanhuyen: BASE64_STANDARD.encode(&self.ma_quanhuyen),
            ma_phuongxa: BASE64_STANDARD.encode(&self.ma_phuongxa),
            ma_thonxom: BASE64_STANDARD.encode(&self.ma_thonxom),
            dien_cu_tru: BASE64_STANDARD.encode(&self.dien_cu_tru),
            ma_phieu: BASE64_STANDARD.encode(&self.ma_phieu),
            ghi_chu: BASE64_STANDARD.encode(&self.ghi_chu),
            pcgd_csrf_token: self.pcgd_csrf_token.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ValueFieldHouseResident {
    pub ho_ten: String,
    pub ngay_sinh: String,
    pub chi_tiet_hoan_canh_db: String,
    pub qh_chu_ho: String,
    pub ho_ten_cha: String,
    pub dien_uu_tien: String,
    pub dien_thoai: String,
    pub ghi_chu: String,
    pub gioi_tinh: String,
    pub ma_dantoc: String,
    pub ton_giao: String,
    pub hoan_canh_db: String,
    pub ma_phieu: Option<String>,
    pub ma_dot: String,
    pub khuyet_tat_benh: String,
}

impl ValueFieldHouseResident {
    pub fn new(col: &[Data]) -> Self {
        let raw_hoan_canh_db = col[45].to_string().to_lowercase();
        let hoan_canh_db = if raw_hoan_canh_db == "chuyển đến" {
            "1".to_owned()
        } else if raw_hoan_canh_db == "chuyển đi" {
            "2".to_owned()
        } else if raw_hoan_canh_db == "chết" {
            "3".to_owned()
        } else {
            "".to_owned()
        };

        let gioi_tinh = if col[7].to_string().to_lowercase() == "x" {
            "2".to_owned()
        } else {
            "1".to_owned()
        };

        let mut khuyet_tat: Vec<String> = Vec::new();
        for kt in 33..44 {
            if col[kt].to_string().to_lowercase() == "x" {
                khuyet_tat.push((kt - 32).to_string());
            }
        }

        let mut ngay = col[4].to_string();
        let mut thang = col[5].to_string();
        let nam = col[6].to_string();
        if ngay.len() == 1 {
            ngay.insert(0, '0');
        }
        if thang.len() == 1 {
            thang.insert(0, '0');
        }


        ValueFieldHouseResident {
            ho_ten: format!("{} {}", col[2].to_string(), col[3].to_string()),
            ngay_sinh: format!("{}/{}/{}", ngay, thang, nam),
            hoan_canh_db,
            chi_tiet_hoan_canh_db: col[46].to_string(),
            qh_chu_ho: col[47].to_string(),
            ho_ten_cha: col[48].to_string(),
            dien_uu_tien: col[10].to_string(),
            dien_thoai: col[49].to_string(),
            ghi_chu: col[50].to_string(),
            gioi_tinh,
            ma_dantoc: unidecode(&col[8].to_string().to_uppercase().replace(" ", "_").replace("-", "_")),
            ton_giao: unidecode(&col[9].to_string().to_uppercase().replace(" ", "_").replace("-", "_")),
            ma_phieu: None,
            ma_dot: "".to_string(),
            khuyet_tat_benh: khuyet_tat.join(","),
        }
    }

    pub fn update_ma_phieu(&mut self, ma_phieu: String) {
        self.ma_phieu = Some(ma_phieu);
    }

    pub fn build(&self) -> Self {
        ValueFieldHouseResident {
            ho_ten: BASE64_STANDARD.encode(&self.ho_ten),
            ngay_sinh: BASE64_STANDARD.encode(&self.ngay_sinh),
            hoan_canh_db: BASE64_STANDARD.encode(&self.hoan_canh_db),
            chi_tiet_hoan_canh_db: BASE64_STANDARD.encode(&self.chi_tiet_hoan_canh_db),
            qh_chu_ho: BASE64_STANDARD.encode(&self.qh_chu_ho),
            ho_ten_cha: BASE64_STANDARD.encode(&self.ho_ten_cha),
            dien_uu_tien: BASE64_STANDARD.encode(&self.dien_uu_tien),
            dien_thoai: BASE64_STANDARD.encode(&self.dien_thoai),
            ghi_chu: BASE64_STANDARD.encode(&self.ghi_chu),
            gioi_tinh: BASE64_STANDARD.encode(&self.gioi_tinh),
            ma_dantoc: BASE64_STANDARD.encode(&self.ma_dantoc),
            ton_giao: BASE64_STANDARD.encode(&self.ton_giao),
            ma_phieu: Some(BASE64_STANDARD.encode(self.ma_phieu.as_ref().unwrap())),
            ma_dot: BASE64_STANDARD.encode(&self.ma_dot),
            khuyet_tat_benh: BASE64_STANDARD.encode(&self.khuyet_tat_benh),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ValueFieldHouseResidentGeneralEducation {
    pub tn_nam: String,
    pub so_bang_tn: String,
    pub nam_tn_nghe: String,
    pub nam_hx: String,
    pub bohoc_nam: String,
    pub cap_tn: String,
    pub bac_tn_nghe: String,
    pub hoc_xong: String,
    pub bohoc_lop: String,
    pub tai_mu_chu: String,
    pub hoc_xmc_lop: String,
    pub congnhan_xmc: String,
    pub bo_tuc: String,
    pub tnc2_loaitruong: String,
}

impl ValueFieldHouseResidentGeneralEducation {
    pub fn new(col: &[Data]) -> Self {
        ValueFieldHouseResidentGeneralEducation {
            tn_nam: col[24].to_string(),
            so_bang_tn: "".to_owned(),
            nam_tn_nghe: col[26].to_string(),
            nam_hx: "".to_owned(),
            bohoc_nam: col[31].to_string(),
            cap_tn: col[22].to_string(),
            bac_tn_nghe: if col[25] != "" {col[25].to_string()} else {"0".to_owned()},
            hoc_xong: " ".to_owned(),
            bohoc_lop: if col[30] != "" {col[30].to_string()} else {"0".to_owned()},
            tai_mu_chu: if col[34] != "" {col[34].to_string()} else {"0".to_owned()},
            hoc_xmc_lop: if col[32] != "" {col[32].to_string()} else {"0".to_owned()},
            congnhan_xmc: if col[33] != "" {col[33].to_string()} else {"0".to_owned()},
            bo_tuc: "".to_owned(),
            tnc2_loaitruong: "".to_owned(),
        }
    }

    pub fn build(&self) -> Self {
        ValueFieldHouseResidentGeneralEducation {
            tn_nam: BASE64_STANDARD.encode(&self.tn_nam),
            so_bang_tn: BASE64_STANDARD.encode(&self.so_bang_tn),
            nam_tn_nghe: BASE64_STANDARD.encode(&self.nam_tn_nghe),
            nam_hx: BASE64_STANDARD.encode(&self.nam_hx),
            bohoc_nam: BASE64_STANDARD.encode(&self.bohoc_nam),
            cap_tn: BASE64_STANDARD.encode(&self.cap_tn),
            bac_tn_nghe: BASE64_STANDARD.encode(&self.bac_tn_nghe),
            hoc_xong: BASE64_STANDARD.encode(&self.hoc_xong),
            bohoc_lop: BASE64_STANDARD.encode(&self.bohoc_lop),
            tai_mu_chu: BASE64_STANDARD.encode(&self.tai_mu_chu),
            hoc_xmc_lop: BASE64_STANDARD.encode(&self.hoc_xmc_lop),
            congnhan_xmc: BASE64_STANDARD.encode(&self.congnhan_xmc),
            bo_tuc: BASE64_STANDARD.encode(&self.bo_tuc),
            tnc2_loaitruong: BASE64_STANDARD.encode(&self.tnc2_loaitruong),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ValueFieldHouseResident2024Education {
    pub lophoc_2024: String,
    pub ma_tinh: String,
    pub ma_quanhuyen: String,
    pub khoi: String,
    pub ma_truong: String,
    pub ma_hoctap_2024: String,
    pub nam_hoc_re: String,
    pub cb_view_mamnon_2024: String,
    pub luuban_2024: String,
    pub hinhthuchoc_2024: String,
    pub hoc_bo_tuc: String,
}

impl ValueFieldHouseResident2024Education {
    pub fn new(col: &[Data], ma_tinh: String, ma_quanhuyen: String) -> Self {
        let hoc_bo_tuc = if col[23].to_string().to_lowercase() == "x" {
            "1".to_owned()
        } else {
            "".to_owned()
        };

        let raw_khoi = col[17].to_string();
        let khoi = if raw_khoi.ends_with("tuổi") {
            format!("t{}", raw_khoi.chars().next().unwrap())
        } else if raw_khoi.len() == 1 || raw_khoi.len() == 2 {
            format!("k{}", raw_khoi)
        } else {
            raw_khoi
        };

        ValueFieldHouseResident2024Education {
            lophoc_2024: col[18].to_string(),
            ma_tinh,
            ma_quanhuyen,
            khoi,
            ma_truong: col[21].to_string(),
            ma_hoctap_2024: "".to_string(),
            nam_hoc_re: "2024".to_string(),
            cb_view_mamnon_2024: "".to_string(),
            luuban_2024: "".to_string(),
            hinhthuchoc_2024: "".to_string(),
            hoc_bo_tuc,
        }
    }

    pub fn build(&self) -> Self {
        ValueFieldHouseResident2024Education {
            lophoc_2024: BASE64_STANDARD.encode(&self.lophoc_2024),
            ma_tinh: BASE64_STANDARD.encode(&self.ma_tinh),
            ma_quanhuyen: BASE64_STANDARD.encode(&self.ma_quanhuyen),
            khoi: BASE64_STANDARD.encode(&self.khoi),
            ma_truong: BASE64_STANDARD.encode(&self.ma_truong),
            ma_hoctap_2024: BASE64_STANDARD.encode(&self.ma_hoctap_2024),
            nam_hoc_re: BASE64_STANDARD.encode(&self.nam_hoc_re),
            cb_view_mamnon_2024: BASE64_STANDARD.encode(&self.cb_view_mamnon_2024),
            luuban_2024: BASE64_STANDARD.encode(&self.luuban_2024),
            hinhthuchoc_2024: BASE64_STANDARD.encode(&self.hinhthuchoc_2024),
            hoc_bo_tuc: BASE64_STANDARD.encode(&self.hoc_bo_tuc),
        }
    }
}
