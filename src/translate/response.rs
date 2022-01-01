use serde::Deserialize;

#[derive(Debug, Default, PartialEq, Deserialize)]
pub struct PostResponse {
    pub status: String,
    pub translated: String,
    pub source_text: String,
    pub source_lang: String,
    pub target_lang: String,
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
