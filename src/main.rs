mod auth_apis;
mod device_apis;
use std::env;

const DEBUG: bool = false;

fn get_base_url(billing_id: &str) -> &str {
    match billing_id.chars().next() {
        Some('1') => "https://services.fiberlink.com/",
        Some('2') => "https://services.m2.maas360.com/",
        Some('3') => "https://services.m3.maas360.com/",
        Some('4') => "https://services.m4.maas360.com/",
        Some('6') => "https://services.m6.maas360.com/",
        Some(_) => panic!("Billing ID is incorrect"),
        None => panic!("No Billing ID entered"),
    }
}

#[tokio::main]
async fn main() {
    let billing: &str = &env::var("MaaS_billing_id").expect("Error: Billing ID environment variable not found");
    let app: &str = &env::var("MaaS_app_id").expect("Error: App ID environment variable not found");
    let access_key: &str = &env::var("MaaS_access_key").expect("Error: Access Key environment variable not found");
    let user: &str = &env::var("MaaS_username").expect("Error: Username environment variable not found");
    let password: &str = &env::var("MaaS_password").expect("Error: Password environment variable not found");
    let auth_params: auth_apis::APIWrapper<'_> = auth_apis::APIWrapper {
        auth_request: auth_apis::AuthRequest {
            maas360_admin_auth: auth_apis::MaaS360AdminAuth {
                billing_id: billing,
                platform_id: "3",
                app_version: "1.0",
                app_id: app,
                app_access_key: access_key,
                user_name: user,
                password: password,
            },
        },
    };

    let billing_id: &str = auth_params.auth_request.maas360_admin_auth.billing_id;
    let base_url: &str = get_base_url(&billing_id);
    let client = reqwest::Client::new();
    let api_token: String = auth_apis::authenticate(base_url, auth_params, DEBUG, &client).await;
    let devices: Result<device_apis::Root, Box<dyn std::error::Error>> =
        Ok(device_apis::get_all_devices(base_url, billing_id, DEBUG, &api_token, &client).await.unwrap());
    println!("All Devices...\n{:?}", devices.expect("Failed to get devices"));
}
