use serde::{Deserialize, Serialize};

const TRANSLATION_SERVICE_URI: &str = "https://api.funtranslations.com";
pub const TRANSLATION_SHAKESPEARE_PATH: &str = "/translate/shakespeare.json";
const TRANSLATION_API_KEY_HEADER: &str = "X-FunTranslations-Api-Secret";

#[derive(Deserialize, Debug)]
struct Translation {
    success: Success,
    contents: Contents,
}

#[derive(Deserialize, Debug)]
struct Success {
    total: u8,
}

#[derive(Deserialize, Debug)]
struct Contents {
    translated: String,
}

#[derive(Deserialize, Serialize)]
struct InputText {
    text: String,
}

#[derive(Clone, Debug)]
pub struct ShakespeareWrapper {
    base_url: String,
    api_key: Option<String>,
}

impl ShakespeareWrapper {
    pub fn new() -> Self {
        Self {
            base_url: TRANSLATION_SERVICE_URI.to_string(),
            api_key: None,
        }
    }

    pub fn with_base_url(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            api_key: None,
        }
    }

    /// Set the shakespeare wrapper's api key.
    pub fn set_api_key(&mut self, api_key: Option<String>) {
        self.api_key = api_key;
    }

    pub async fn get_translation(&self, translation_input: &str) -> Result<String, tide::Error> {
        let tranlation_request_url = format!("{}{}", self.base_url, TRANSLATION_SHAKESPEARE_PATH);
        let translated_text =
            fetch_translation(&tranlation_request_url, translation_input, &self.api_key).await?;
        Ok(translated_text)
    }
}

impl Default for ShakespeareWrapper {
    fn default() -> Self {
        ShakespeareWrapper::new()
    }
}

async fn fetch_translation(
    translation_url: &str,
    translation_input: &str,
    api_key: &Option<String>,
) -> Result<String, tide::Error> {
    let text = InputText {
        text: translation_input.to_string(),
    };

    let mut req = surf::post(translation_url)
        .body(surf::Body::from_json(&text)?)
        .build();

    if let Some(api_key) = api_key {
        req.set_header(TRANSLATION_API_KEY_HEADER, api_key.to_string());
    }

    let client = surf::client();
    let mut res = client.send(req).await?;

    let status: u16 = res.status().into();
    match status {
        200 => {
            let translation: Translation = res.body_json().await.map_err(|e| {
                tide::log::error!("Error: {}, deserializing response to Translation", e);
                tide::Error::from_str(500, "Unexpected Error".to_string())
            })?;

            match translation.success.total {
                1 => Ok(translation.contents.translated),
                _ => {
                    tide::log::error!("Error returned by translation service");
                    Err(tide::Error::from_str(500, "Unexpected Error".to_string()))
                }
            }
        }
        429 => Err(tide::Error::from_str(429, "Too Many Requests".to_string())),
        _ => Err(tide::Error::from_str(500, "Unexpected Error".to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[async_std::test]
    async fn fetch_translation_ok() -> std::result::Result<(), tide::Error> {
        let mock_server = MockServer::start().await;

        const TRASLATION_CONTENT: &str = include_str!("../../samples/shakespeare_translation.json");
        let translation_as_json: serde_json::Value =
            serde_json::from_str(&TRASLATION_CONTENT).unwrap();
        let response = ResponseTemplate::new(200).set_body_json(&translation_as_json);

        Mock::given(method("POST"))
            .and(path(TRANSLATION_SHAKESPEARE_PATH))
            .respond_with(response)
            .mount(&mock_server)
            .await;

        let request_url = format!("{}{}", &mock_server.uri(), TRANSLATION_SHAKESPEARE_PATH);
        let translated_text: String = fetch_translation(
            &request_url,
            "Rust, a language empowering everyone to build reliable and efficient software.",
            &None,
        )
        .await?;

        assert_eq!(
            translated_text,
            "Rust, a language empowering everyone to buildeth reliable and efficient software."
        );

        Ok(())
    }

    #[async_std::test]
    async fn fetch_translation_http_error() -> std::result::Result<(), tide::Error> {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path(TRANSLATION_SHAKESPEARE_PATH))
            .respond_with(ResponseTemplate::new(500))
            .mount(&mock_server)
            .await;

        let request_url = format!("{}{}", &mock_server.uri(), TRANSLATION_SHAKESPEARE_PATH);
        let translation_response = fetch_translation(
            &request_url,
            "Rust, a language empowering everyone to build reliable and efficient software.",
            &None,
        )
        .await;

        assert_eq!(true, translation_response.is_err());

        let status_code = translation_response.err().unwrap().status();
        assert_eq!(tide::http::StatusCode::InternalServerError, status_code);

        Ok(())
    }

    #[async_std::test]
    async fn fetch_translation_parse_error() -> std::result::Result<(), tide::Error> {
        let mock_server = MockServer::start().await;

        const TRASLATION_CONTENT: &str =
            include_str!("../../samples/shakespeare_translation_bad.json");
        let response = ResponseTemplate::new(200).set_body_json(TRASLATION_CONTENT);

        Mock::given(method("POST"))
            .and(path(TRANSLATION_SHAKESPEARE_PATH))
            .respond_with(response)
            .mount(&mock_server)
            .await;

        let request_url = format!("{}{}", &mock_server.uri(), TRANSLATION_SHAKESPEARE_PATH);
        let translation_response = fetch_translation(
            &request_url,
            "Rust, a language empowering everyone to build reliable and efficient software.",
            &None,
        )
        .await;

        assert_eq!(true, translation_response.is_err());

        let status_code = translation_response.err().unwrap().status();
        assert_eq!(tide::http::StatusCode::InternalServerError, status_code);

        Ok(())
    }
}
