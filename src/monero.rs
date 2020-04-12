use serde::Deserialize;

#[derive(Deserialize)]
struct Monero {
    difficulty: u64,
    height: u64,
    hashrate: f64,
    total_emission: String,
}

pub async fn get_status() -> Result<String, reqwest::Error> {
    let res = reqwest::get("https://moneroblocks.info/api/get_stats").await?;

    let monero = res.json::<Monero>().await?;

    let (hashrate_num, hashrate_word) = if monero.hashrate < 1000000.0 {
        (monero.hashrate / 1000.0, "kH/s")
    } else {
        (monero.hashrate / 1000000.0, "MH/s")
    };

    let result = format!(
        "Hashrate: {} {}\nHeight: {}\nDifficulty: {}\nTotal emission: {}\n\n#monero",
        hashrate_num, hashrate_word, monero.height, monero.difficulty, monero.total_emission
    );

    Ok(result)
}
