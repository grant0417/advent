const DATA_PATH: &str = "data/";

fn cookie() -> String {
    std::fs::read_to_string(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("advent-cookie"))
        .unwrap()
}

pub async fn input(year: u32, day: u32) -> String {
    assert!(year >= 2015);
    assert!(day >= 1 && day <= 25);

    let file_path = format!("{DATA_PATH}/{year}/day{day}.txt");
    match tokio::fs::read_to_string(&file_path).await {
        Ok(input) => input,
        Err(_) => {
            let url = format!("https://adventofcode.com/{year}/day/{day}/input");
            let client = reqwest::Client::new();
            let res = client
                .get(&url)
                .header("Cookie", cookie())
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
        let i = input(2022, 1).await;
        println!("{}", i);
        assert!(!i.is_empty());

        let i = input(2023, 1).await;
        println!("{}", i);
        assert!(!i.is_empty());
    }
}
