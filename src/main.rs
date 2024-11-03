use std::path::Path;
use tokio::time::sleep;
use std::time::Duration;
use reqwest::Client;


mod api;
mod config;

#[tokio::main]
async fn main() {
    let client = Client::new();
    let config_path = Path::new("./config.json");

    match config::Config::init(config_path){
        Ok(config) => {
            match api::User::new(client.clone(), config.token.clone()).await{
                Ok(user) => {
                    println!("Username: {} | Token: {}",user.user.username, user.token);
                    
                    let status_len = config.status_list.len();
                    let mut index = 0;
                    loop {
                        match api::User::change_status(client.clone(), config.token.clone(), config.status_list[index].clone()).await {
                            Ok(status ) => {
                                println!("[Update] Status: {} | Text: {} | Expires at: {} | Emoji Id: {} | Emoji Name: {}", 
                                status.status, 
                                status.custom_status.text.unwrap_or("❌".to_string()), 
                                status.custom_status.expires_at.unwrap_or("❌".to_string()), 
                                status.custom_status.emoji_id.unwrap_or("❌".to_string()), 
                                status.custom_status.emoji_name.unwrap_or("❌".to_string())
                            )
                            
                            },
                            Err(e) => {
                                print!("{}", e);
                                sleep(Duration::from_secs(config.retry_time)).await;
                            }
                        }

                        index = (index + 1) % status_len;

                        sleep(Duration::from_secs(config.duration)).await;
                    }
                },
                Err(e) => println!("{}",e)
            }
        },
        Err(e) => println!("{}",e) 
    }

    
}
