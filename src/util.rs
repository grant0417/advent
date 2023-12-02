const ADVENT_YEAR: u32 = 2023;
const CACHE_PATH: &str = "target/cache/advent2023";

pub async fn input(day: u32) -> String {
    let file_path = format!("{CACHE_PATH}/day{day}.txt");
    match tokio::fs::read_to_string(&file_path).await {
        Ok(input) => input,
        Err(_) => {
            let url = format!("https://adventofcode.com/{ADVENT_YEAR}/day/{day}/input");
            let client = reqwest::Client::new();
            let res = client
                .get(&url)
                .header("Cookie", include_str!("advent-cookie"))
                .send()
                .await
                .unwrap();
            let text = res.text().await.unwrap();
            tokio::fs::create_dir_all(CACHE_PATH).await.unwrap();
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
        let input = input(1).await;
        println!("{}", input);
        assert!(!input.is_empty());
    }
}
