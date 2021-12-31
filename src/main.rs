use dotenv;

fn transrate_api_url() -> Result<String, dotenv::Error> {
    dotenv::var("TRANSRATE_API_URL")
}

fn main() -> Result<(), dotenv::Error> {
    dotenv::dotenv().ok();
    let url = transrate_api_url()?;
    println!("{}", url);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transrate_api_url() -> Result<(), dotenv::Error> {
        dotenv::from_filename(".env.example").ok();
        let url = transrate_api_url()?;
        assert_eq!(
            "https://script.google.com/macros/s/XXXXXXXXXXXXXXXXXX/exec",
            url
        );

        Ok(())
    }
}
