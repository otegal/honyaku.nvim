use crate::translate::request::PostRequest;
use crate::translate::response::PostResponse;

pub struct Client {
    url: String,
    client: reqwest::Client,
}

impl Client {
    pub fn new(url: String) -> Self {
        Self {
            url,
            client: reqwest::Client::new(),
        }
    }

    pub async fn post_request(&self, req: PostRequest) -> anyhow::Result<PostResponse> {
        Ok(self
            .client
            .post(&self.url)
            .json(&req)
            .send()
            .await?
            .json()
            .await?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_post_request() -> anyhow::Result<()> {
        dotenv::from_filename(".env.example").ok();
        let client = Client::new(format!("{}/exec", mockito::server_url()));
        let json = std::fs::read_to_string("test_data/post_response.json").map_err(|e| dbg!(e))?;
        let mock = mockito::mock("POST", "/exec")
            .with_status(200)
            .with_body(json)
            .create();
        let req = PostRequest {
            text: "Hello".into(),
            ..Default::default()
        };
        let res = client.post_request(req).await?;

        mockito::Mock::expect(mock, 1);
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
