use dotenv;

#[derive(Debug, PartialEq)]
struct PostData {
    text: String,
    source_lang: String,
    target_lang: String,
}

impl Default for PostData {
    fn default() -> Self {
        Self {
            text: "".into(),
            source_lang: "en".into(),
            target_lang: "ja".into(),
        }
    }
}

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

    #[test]
    fn test_post_data_default() {
        let post_data = PostData {
            text: "".into(),
            source_lang: "en".into(),
            target_lang: "ja".into(),
        };
        assert_eq!(
            post_data,
            PostData {
                ..Default::default()
            }
        );
    }
}
