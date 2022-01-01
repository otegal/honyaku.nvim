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
    url: String,
    client: reqwest::Client,
}

impl Client {
    fn new(url: String) -> Self {
        Self {
            url,
            client: reqwest::Client::new(),
        }
    }

    async fn post_request(&self) -> anyhow::Result<PostResponse> {
        Ok(self.client.post(&self.url).send().await?.json().await?)
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
        dotenv::from_filename(".env.example").ok();
        let client = Client::new(format!("{}/exec", mockito::server_url()));
        let json = std::fs::read_to_string("test_data/post_response.json").map_err(|e| dbg!(e))?;
        let mock = mockito::mock("POST", "/exec")
            .with_status(200)
            .with_body(json)
            .create();
        let res = client.post_request().await?;

        Mock::expect(mock, 1);
        assert_eq!(
            res,
            PostResponse {
                status: "OK".into(),
                translated: "こんにちは".into(),
                source_text: "Hello".into(),
                source_lang: "en".into(),
                target_lang: "ja".into(),
            }
        );
        Ok(())
    }
}
