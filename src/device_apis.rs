use reqwest;
use reqwest::Url;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Root {
    devices: Devices,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Devices {
    device: Vec<Device>,
    count: i32,
    page_number: i32,
    page_size: i32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Device {
    #[serde(rename = "maas360DeviceID")]
    maas360_device_id: String,
    device_name: String,
    custom_asset_number: String,
    ownership: String,
    device_owner: String,
    username: String,
    email_address: String,
    platform_name: String,
    #[serde(rename = "sourceID")]
    source_id: i32,
    device_type: String,
    manufacturer: String,
    model: String,
    os_name: String,
    os_service_pack: String,
    imei_esn: Value,
    installed_date: String,
    last_reported: String,
    installed_date_in_epochms: i64,
    last_reported_in_epochms: i64,
    device_status: String,
    maas360_managed_status: String,
    udid: String,
    wifi_mac_address: String,
    mailbox_device_id: String,
    mailbox_last_reported: String,
    mailbox_last_reported_in_epochms: Value,
    mailbox_managed: String,
    is_supervised_device: bool,
    test_device: bool,
    unified_traveler_device_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Value {
    Int(i64),
    Str(String),
}

pub async fn get_all_devices(
    base_url: &str,
    billing_id: &str,
    debug: bool,
    auth_token: &str,
    client: &reqwest::Client,
) -> Option<Root> {
    let device_url = Url::parse(&format!(
        "{}device-apis/devices/1.0/search/{}?deviceStatus=Active",
        base_url, billing_id
    ))
    .unwrap();

    let response: reqwest::Response = client
        .get(device_url)
        .header("content-type", "application/json")
        .header("accept", "application/json")
        .header("authorization", format!("MaaS token=\"{}\"", auth_token))
        .send()
        .await
        .unwrap();

    match response.status() {
        reqwest::StatusCode::OK => {
            let data: String = response.text().await.unwrap();
            println!("Data: {:?}", &data);
            let result: Result<Root, serde_json::Error> = serde_json::from_str(&data);
            match result {
                Ok(devices) => Some(devices),
                Err(err) => panic!("{}", err),
            }
        }
        _other => {
            if debug {
                println!("DEBUG: Http Status = {}", response.status());
                println!("DEBUG: Description = Something went wrong during Authorization");
            }
            let data: String = response.text().await.unwrap();
            println!("Warning: Unable to parse to Device. Response Body = {}", data);
            None
        }
    }
}
