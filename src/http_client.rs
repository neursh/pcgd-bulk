use reqwest::{header, blocking::Client};

pub fn create_client_with_headers_preset(cookies: &String) -> Client {
    let mut headers = header::HeaderMap::new();
    headers.insert("Accept", header::HeaderValue::from_static("application/json, text/javascript, */*"));
    headers.insert("Connection", header::HeaderValue::from_static("keep-alive"));
    headers.insert("Accept-Language", header::HeaderValue::from_static("vi-VN,vi;q=0.9,fr-FR;q=0.8,fr;q=0.7,en-US;q=0.6,en;q=0.5"));
    headers.insert("Cookie", header::HeaderValue::from_str(cookies).unwrap());
    headers.insert("Origin", header::HeaderValue::from_static("https://pcgd.moet.gov.vn"));
    headers.insert("Sec-Fetch-Dest", header::HeaderValue::from_static("empty"));
    headers.insert("Sec-Fetch-Mode", header::HeaderValue::from_static("cors"));
    headers.insert("Sec-Fetch-Site", header::HeaderValue::from_static("same-origin"));
    headers.insert("User-Agent", header::HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/130.0.0.0 Safari/537.36"));
    headers.insert("sec-ch-ua", header::HeaderValue::from_static("\"Chromium\";v=\"130\", \"Google Chrome\";v=\"130\", \"Not?A_Brand\";v=\"99\""));
    headers.insert("sec-ch-ua-mobile", header::HeaderValue::from_static("?0"));
    headers.insert("sec-ch-ua-platform", header::HeaderValue::from_static("\"Windows\""));
    
    Client::builder()
        .default_headers(headers)
        .build()
        .unwrap()
}
