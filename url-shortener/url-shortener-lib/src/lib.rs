use url::Url as UrlLib;

#[derive(Debug)]
pub enum UrlError {
    InvalidUrl,
}

pub struct Url {
    url: String,
}

impl Url {
    pub fn from(url: &str) -> Result<Url, UrlError> {
        let parsed_url = UrlLib::parse(url);
        match parsed_url {
            Ok(_) => Ok(Url {
                url: url.to_string(),
            }),
            Err(_) => Err(UrlError::InvalidUrl),
        }
    }
}

pub fn shorten_url(url: Url) -> String {
    "a".to_string()
}

#[cfg(test)]
#[test]
fn given_google_url_when_shorten_then_should_return_encoded_url() {
    let result = shorten_url(Url::from("https://google.com").unwrap());
    assert!(result != "https://google.com");
}
