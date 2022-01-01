use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
pub struct PostRequest {
    pub text: String,
    pub source_lang: String,
    pub target_lang: String,
}

impl PostRequest {
    pub fn build_params_en_to_ja(text: String) -> Self {
        Self {
            text: text.into(),
            source_lang: "en".into(),
            target_lang: "ja".into(),
        }
    }
    pub fn build_params_ja_to_en(text: String) -> Self {
        Self {
            text: text.into(),
            source_lang: "ja".into(),
            target_lang: "en".into(),
        }
    }
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
        let req = PostRequest {
            text: "".into(),
            source_lang: "en".into(),
            target_lang: "ja".into(),
        };
        assert_eq!(
            req,
            PostRequest {
                ..Default::default()
            }
        );
    }

    #[test]
    fn test_build_params_en_to_ja() {
        let req = PostRequest {
            text: "".into(),
            source_lang: "en".into(),
            target_lang: "ja".into(),
        };
        assert_eq!(req, PostRequest::build_params_en_to_ja("".into()))
    }

    #[test]
    fn test_build_params_ja_to_en() {
        let req = PostRequest {
            text: "".into(),
            source_lang: "ja".into(),
            target_lang: "en".into(),
        };
        assert_eq!(req, PostRequest::build_params_ja_to_en("".into()))
    }
}
