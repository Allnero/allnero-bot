use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::thread;
use std::time::Duration;

#[derive(Deserialize)]
struct Monero {
    difficulty: u64,
    height: u64,
    hashrate: f64,
    total_emission: String,
}

fn main() {
    let token = match env::var("BOT_TOKEN") {
        Ok(t) => t,
        Err(e) => {
            panic!("Error get token: {:?}", e);
        }
    };

    let client = reqwest::Client::new();

    loop {
        let res = reqwest::get("https://moneroblocks.info/api/get_stats");

        let json = match res {
            Ok(mut body) => body.json(),
            Err(e) => {
                panic!("Error get request: {:?}", e);
            }
        };

        let monero: Monero = match json {
            Ok(data) => data,
            Err(e) => {
                panic!("Error get request: {:?}", e);
            }
        };

        let (hashrate_num, hashrate_word) = if monero.hashrate < 1000000.0 {
            (monero.hashrate / 1000.0, "kH/s")
        } else {
            (monero.hashrate / 1000000.0, "MH/s")
        };

        let status = format!(
            "Hashrate: {} {}\nHeight: {}\nDifficulty: {}\nTotal emission: {}\n\n#monero",
            hashrate_num, hashrate_word, monero.height, monero.difficulty, monero.total_emission
        );

        let mut map = HashMap::new();
        map.insert("status", status);

        let _res = client
            .post("https://mastodonsocial.ru/api/v1/statuses")
            .bearer_auth(&token)
            .json(&map)
            .send()
            .map_err(|err| println!("request error: {}", err))
            .map(|mut body| {
                let status = body.status();
                if status == 200 {
                    println!("Status code:{:?}", body.status());
                } else {
                    println!("Status code:{:?}", body.status());
                    println!("{:?}", body.text());
                }
            });

        thread::sleep(Duration::from_secs(60))
    }
}
