use reqwest::Client;
use reqwest::header;
use std::time::Duration;
use reqwest::redirect::Policy;
use reqwest::Method;

const API_URL: &str = "https://discord.com/api/v10";

pub struct ApiClient {
    token: String,
    client: Client,
}

impl ApiClient {
    pub fn new(token: String) -> Result<ApiClient, ()> {
        let mut headers = header::HeaderMap::new();

        headers.insert(
            "Authorization",
            to_header_value(&("Bot ".to_owned() + &token))?,
        );

        headers.insert(
            "Accept",
            to_header_value("application/json")?,
        );

        static APP_USER_AGENT: &str = concat!(
            env!("CARGO_PKG_NAME"),
            "/",
            env!("CARGO_PKG_VERSION"),
        );

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .timeout(Duration::from_secs(5))
            .redirect(Policy::none())
            .user_agent(APP_USER_AGENT)
            .build();

        match client {
            Ok(client) => Ok(ApiClient {
                token,
                client,
            }),

            Err(_) => Err(()),
        }
    }

    fn request(self, method: Method, path: &str) {
        let url = API_URL.to_owned() + path;

        self.client.request(method, url).send();
    }
}

fn to_header_value(value: &str) -> Result<header::HeaderValue, ()> {
    match header::HeaderValue::from_str(value) {
        Ok(header) => Ok(header),
        Err(_) => Err(()),
    }
}

#[cfg(test)]
mod tests {

}
