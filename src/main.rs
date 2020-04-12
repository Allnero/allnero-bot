use serde::{Deserialize, Serialize};
use std::env;
use std::thread;
use std::time::Duration;

mod monero;

#[derive(Debug, Serialize, Deserialize)]
struct Post {
    status: String,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let token = match env::var("BOT_TOKEN") {
        Ok(t) => t,
        Err(e) => {
            panic!("Error get token: {:?}", e);
        }
    };

    let mastodon_domain = match env::var("MASTODON_DOMAIN") {
        Ok(t) => t,
        Err(e) => {
            panic!("Error get mastodon domain: {:?}", e);
        }
    };

    let mastodon_url = format!("https://{}/api/v1/statuses", mastodon_domain);

    let minute = 60;
    let hour = minute * 60;

    let client = reqwest::Client::new();

    loop {
        let status = monero::get_status().await?;

        let new_post = Post { status: status };

        let res = client
            .post(&mastodon_url)
            .bearer_auth(&token)
            .json(&new_post)
            .send()
            .await?;

        let code = res.status().as_u16();
        println!("Status code: {}", code);

        thread::sleep(Duration::from_secs(hour))
    }
}
