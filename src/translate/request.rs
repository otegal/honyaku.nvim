use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
pub struct PostRequest {
    pub text: String,
    pub source_lang: String,
    pub target_lang: String,
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

#[cfg(test)]
mod tests {
    use super::*;

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
}
