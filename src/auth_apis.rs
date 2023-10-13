use reqwest;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct APIResponse {
    auth_response: AuthResponse,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct AuthResponse {
    error_code: u16,
    error_desc: Option<String>,
    auth_token: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MaaS360AdminAuth<'a> {
    #[serde(rename = "billingID")]
    pub billing_id: &'a str,
    #[serde(rename = "platformID")]
    pub platform_id: &'a str,
    pub app_version: &'a str,
    #[serde(rename = "appID")]
    pub app_id: &'a str,
    pub app_access_key: &'a str,
    pub user_name: &'a str,
    pub password: &'a str,
}

#[derive(Serialize)]
pub struct AuthRequest<'a> {
    #[serde(rename = "maaS360AdminAuth")]
    pub maas360_admin_auth: MaaS360AdminAuth<'a>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct APIWrapper<'a> {
    pub auth_request: AuthRequest<'a>,
}

pub async fn authenticate<'a>(
    base_url: &'a str,
    body: APIWrapper<'a>,
    debug: bool,
    client: &'a reqwest::Client,
) -> String {
    let auth_url = &(base_url.to_owned() + "auth-apis/auth/1.0/authenticate/" + &body.auth_request.maas360_admin_auth.billing_id);

    let response = client
        .post(auth_url)
        .header("content-type", "application/json")
        .header("accept", "application/json")
        .body(serde_json::to_string(&body).unwrap())
        .send()
        .await
        .expect("Failed to send request");

    if debug {
        println!("DEBUG: Url = {}", auth_url);
        println!("DEBUG: Body = {}", serde_json::to_string(&body).unwrap());
        println!("DEBUG: Response Headers = \n{:?}", response.headers());
    }

    match response.status() {
        reqwest::StatusCode::OK => {
            match response.json::<APIResponse>().await {
                Ok(data) => {
                    if debug {
                        println!("Error Code: {}", data.auth_response.error_code);
                    }
                    if let Some(error_desc) = data.auth_response.error_desc {
                        println!("Error Description: {:?}", error_desc);
                    }
                    if let Some(auth_token) = data.auth_response.auth_token {
                        println!("Auth Token: {:?}", &auth_token);
                        auth_token
                    } else {
                        String::default()
                    }
                }
                Err(err) => {
                    println!("Response did not match expectation \"AuthResponse\"");
                    println!("{}", err);
                    panic!("Failed to authenticate to the MaaS360 API.")
                }
            }
        }
        _other => {
            if debug {
                println!("DEBUG: Http Status = {}", response.status());
                println!("DEBUG: Description = Something went wrong during Authorization");
            }
            panic!("Failed to authenticate to the MaaS360 API.")
        }
    }
}
