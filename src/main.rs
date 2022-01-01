pub mod translate;

use dotenv;

use crate::translate::request::PostRequest;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    let url = dotenv::var("TRANSLATE_API_URL")?;
    println!("{}", url);

    let client = translate::client::Client::new(url);
    let req = PostRequest {
        text: "translation".into(),
        ..Default::default()
    };
    let res = client.post_request(req).await?;
    println!("{:?}", res);
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
