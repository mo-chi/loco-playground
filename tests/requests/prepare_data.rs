use axum::http::{HeaderName, HeaderValue};
use loco_playground::views::auth::LoginResponse;
use loco_rs::TestServer;

const USER_EMAIL: &str = "user1@example.com";
const USER_PASSWORD: &str = "12341234";

pub struct LoggedInUser {
    pub token: String,
}

pub async fn user_login(request: &TestServer) -> LoggedInUser {
    let response = request
        .post("/api/auth/login")
        .json(&serde_json::json!({
            "email": USER_EMAIL,
            "password": USER_PASSWORD,
        }))
        .await;

    let login_response: LoginResponse = serde_json::from_str(&response.text()).unwrap();

    LoggedInUser {
        token: login_response.token,
    }
}

pub fn auth_header(token: &str) -> (HeaderName, HeaderValue) {
    let auth_header_name = HeaderName::from_static("authorization");
    let auth_header_value = HeaderValue::from_str(&format!("Bearer {}", &token)).unwrap();

    (auth_header_name, auth_header_value)
}
