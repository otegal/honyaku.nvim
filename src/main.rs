pub mod translate;

use dotenv;

fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    let url = dotenv::var("TRANSLATE_API_URL")?;
    println!("{}", url);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transrate_api_url() -> Result<(), dotenv::Error> {
        dotenv::from_filename(".env.example").ok();
        let url = dotenv::var("TRANSLATE_API_URL")?;
        assert_eq!(
            "https://script.google.com/macros/s/XXXXXXXXXXXXXXXXXX/exec",
            url
        );

        Ok(())
    }
}
