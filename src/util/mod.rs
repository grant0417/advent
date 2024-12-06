pub mod fast;
pub mod grid;
pub mod math;
pub mod point;

const DATA_PATH: &str = "data/";

async fn cookie() -> String {
    let env_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(".env");
    let env = tokio::fs::read_to_string(env_path).await.unwrap();
    let (_, cookie) = env
        .lines()
        .find(|line| line.starts_with("AOC_COOKIE"))
        .unwrap()
        .split_once('=')
        .unwrap();
    cookie.to_string()
}

pub async fn input(year: u32, day: u32) -> String {
    assert!(year >= 2015);
    assert!((1..=25).contains(&day));

    let file_path = format!("{DATA_PATH}/{year}/day{day}.txt");
    match tokio::fs::read_to_string(&file_path).await {
        Ok(input) => input,
        Err(_) => {
            let url = format!("https://adventofcode.com/{year}/day/{day}/input");
            let client = reqwest::Client::new();
            let res = client
                .get(&url)
                .header("Cookie", cookie().await)
                .header(
                    "User-Agent",
                    "github.com/grant0417/advent by grant@gurvis.net",
                )
                .send()
                .await
                .unwrap();
            let text = res.error_for_status().unwrap().text().await.unwrap();
            tokio::fs::create_dir_all(format!("{DATA_PATH}/{year}"))
                .await
                .unwrap();
            tokio::fs::write(&file_path, &text).await.unwrap();
            text
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_input() {
        let data_2021 = input(2021, 1).await;
        println!("{data_2021}");
        assert!(!data_2021.is_empty());

        let data_2022 = input(2022, 1).await;
        println!("{data_2022}");
        assert!(!data_2022.is_empty());

        let data_2023 = input(2023, 1).await;
        println!("{data_2023}");
        assert!(!data_2023.is_empty());
    }
}
