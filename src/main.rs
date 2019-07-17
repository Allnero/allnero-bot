use serde::Deserialize;
use std::env;
use std::thread;
use std::time::Duration;
use std::collections::HashMap;

#[derive(Deserialize)]
struct Monero {
    difficulty: u64,
    height: u64,
    hashrate: f64,
    total_emission: String,
}

fn main() {
    let mut token: String = String::new();

	match env::var("BOT_TOKEN") {
		Ok(t) => token=t,
		Err(e) => println!("Error={:?}", e),
	}

    let client = reqwest::Client::new();

    loop {
        let res = reqwest::get("https://moneroblocks.info/api/get_stats");
        
        match res {
            Ok(mut body) => {
                let json = body.json();
                
                match json {
                    Ok(data) => {
                        let monero: Monero = data;

                        let mut hashrate_word = "H/s";
                        let mut hashrate_num = 0.0;

                        if monero.hashrate > 1000000.0 {
                            hashrate_num = monero.hashrate / 1000000.0;
                            hashrate_word = "MH/s"
                        } else if monero.hashrate > 1000.0 {
                            hashrate_num = monero.hashrate / 1000.0;
                            hashrate_word = "kH/s"
                        }

                        let status = format!("Hashrate: {} {}\nHeight: {}\nDifficulty: {}\nTotal emission: {}", 
                                        hashrate_num,
                                        hashrate_word,
                                        monero.height,
                                        monero.difficulty,
                                        monero.total_emission
                                    );

                        let mut map = HashMap::new();
                        map.insert("status", status);
                        
                        let _res = client.post("https://mastodonsocial.ru/api/v1/statuses")
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
                    }
                    Err(error) => {
                        panic!("Error get request: {:?}", error);
                    }
                }
            }
            Err(error) => {
                panic!("Error get request: {:?}", error);
            }
        }
        thread::sleep(Duration::from_secs(60))
    }
}
