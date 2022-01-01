use dotenv;
use serde::Deserialize;

#[derive(Debug, PartialEq)]
struct PostRequest {
    text: String,
    source_lang: String,
    target_lang: String,
}

impl Default for PostRequest {
    fn default() -> Self {
        Self {
            text: "".into(),
            source_lang: "en".into(),
            target_lang: "ja".into(),
        }
    }
}

#[derive(Debug, Default, PartialEq, Deserialize)]
struct PostResponse {
    status: String,
    translated: String,
    source_text: String,
    source_lang: String,
    target_lang: String,
}

struct Client {
    client: reqwest::Client,
}

impl Client {
    fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    async fn post_request(&self) -> anyhow::Result<reqwest::Response> {
        Ok(self
            .client
            .post(dotenv::var("TRANSLATE_API_URL")?)
            .send()
            .await?)
    }
}

fn transrate_api_url() -> Result<String, dotenv::Error> {
    dotenv::var("TRANSLATE_API_URL")
}

fn main() -> Result<(), dotenv::Error> {
    dotenv::dotenv().ok();
    let url = transrate_api_url()?;
    println!("{}", url);

    Ok(())
}

#[cfg(test)]
mod tests {
    use mockito::Mock;

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

    #[test]
    fn test_post_request_default() {
        let request = PostRequest {
            text: "".into(),
            source_lang: "en".into(),
            target_lang: "ja".into(),
        };
        assert_eq!(
            request,
            PostRequest {
                ..Default::default()
            }
        );
    }

    #[test]
    fn test_post_response_default() {
        let response = PostResponse {
            status: "".into(),
            translated: "".into(),
            source_text: "".into(),
            source_lang: "".into(),
            target_lang: "".into(),
        };
        assert_eq!(
            response,
            PostResponse {
                ..Default::default()
            }
        );
    }

    #[tokio::test]
    async fn test_post_request() -> anyhow::Result<()> {
        let client = Client::new();
        let mock = mockito::mock("POST", dotenv::var("TRANSLATE_API_URL")?.as_str())
            .with_status(200)
            .create();
        let _res = client.post_request().await?;

        Mock::expect(mock, 1);
        Ok(())
    }
}
