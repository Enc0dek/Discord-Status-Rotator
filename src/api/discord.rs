
use reqwest::{header::{HeaderMap, HeaderValue}, Client, Error};
use serde::{Serialize, Deserialize};

const URL_SETTINGS:&str = "https://discord.com/api/v10/users/@me/settings";
const URL_USER:&str = "https://discord.com/api/v10/users/@me";

#[derive(Serialize, Deserialize, Debug)]
pub struct UserInfo{
    pub id: String,
    pub username: String,
    pub global_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub bio: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User{
    pub token: String,
    pub user: UserInfo,
    pub status: Status
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CustomStatus{
    pub text: Option<String>,
    pub expires_at: Option<String>,
    pub emoji_id: Option<String>,
    pub emoji_name: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Status {
    pub status: String,
    pub custom_status: CustomStatus,
}

impl User{
    pub async fn new(client: Client, token: String) -> Result<User, Error>{

            let user_info = User::get_user_info(client.clone(), &token).await?;
            let status_info = User::get_status(client, &token).await?;

            Ok(User{
                token: token,
                user: UserInfo{
                    id: user_info.id,
                    username: user_info.username,
                    global_name: user_info.global_name,
                    email: user_info.email,
                    phone: user_info.phone,
                    bio: user_info.bio,
                },
                status: status_info
            })
        }

        async fn get_user_info(client: Client, token: &str) -> Result<UserInfo, Error>{
            let mut headers = HeaderMap::new();

            headers.insert("authorization", HeaderValue::from_str(&token).unwrap());

            let user_info_req = client
                .get(URL_USER)
                .headers(headers.clone())
                .send()
                .await?;

            let user_info_json:UserInfo = user_info_req.json().await?;

            Ok(user_info_json)
        } 

        pub async fn get_status(client: Client, token: &str) -> Result<Status, Error>{
            let mut headers = HeaderMap::new();

            headers.insert("authorization", HeaderValue::from_str(&token).unwrap());

            let user_settings_req = client
                .get(URL_SETTINGS)
                .headers(headers)
                .send()
                .await?;

            let user_settings_json:Status = user_settings_req.json().await?;
            Ok(user_settings_json)
        }

        pub async fn change_status(client: Client, token: String, new_status: Status) -> Result<Status, Error>{
            let mut headers = HeaderMap::new();

            headers.insert("authorization", HeaderValue::from_str(&token).unwrap());
            headers.insert("Content-Type", HeaderValue::from_static("application/json"));

            client
                .patch(URL_SETTINGS)
                .json(&new_status)
                .headers(headers)
                .send()
                .await?;

            Ok(User::get_status(client, &token).await?)
        }
}

// impl CustomStatus {
//     pub fn new(text: Option<String>, expires_at: Option<String>, emoji_id: Option<String>, emoji_name: Option<String>) -> CustomStatus{
//         CustomStatus{
//             text,
//             expires_at,
//             emoji_id,
//             emoji_name
//         }
//     }
// }

impl Clone for CustomStatus {
    fn clone(&self) -> Self {
        CustomStatus{
            text: self.text.clone(),
            expires_at: self.expires_at.clone(),
            emoji_id: self.emoji_id.clone(),
            emoji_name: self.emoji_name.clone()
        }
    }
}

// impl Status {
//     pub fn new(status: String, custom_status: CustomStatus) -> Status{
//         Status{
//             status,
//             custom_status
//         }
//     }
// }

impl Clone for Status {
    fn clone(&self) -> Self {
        Status{
            status: self.status.clone(),
            custom_status: self.custom_status.clone()
        }
    }
}