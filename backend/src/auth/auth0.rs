use reqwest::Error;

#[derive(Debug, Deserialize)]
pub struct UserInfo {
    pub sub: String,
    pub email: String,
}

pub async fn get_user_info(token: &str) -> Result<UserInfo, Error> {
    let authority = std::env::var("AUTHORITY").expect("AUTHORITY must be set");
    let url = format!("{}userinfo", authority);
    let client = reqwest::Client::new();
    let res = client
        .get(url.as_str())
        .bearer_auth(token)
        .send()
        .await
        .unwrap();
    res.json::<UserInfo>().await
}
