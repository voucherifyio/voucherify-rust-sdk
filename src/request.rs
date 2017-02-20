use hyper::client::{Client, Response};
use hyper::{Url, Error};
use hyper::net::HttpsConnector;
use hyper::header::Headers;
use hyper_native_tls::NativeTlsClient;

pub struct VoucherifyRequest {
    client: Client,
    headers: Headers,
}

impl VoucherifyRequest {
    pub fn new(api_key: &str, api_user: &str) -> VoucherifyRequest {
        let ssl = NativeTlsClient::new().unwrap();
        let connector = HttpsConnector::new(ssl);
        let client = Client::with_connector(connector);

        let mut headers = Headers::new();
        headers.set_raw("X-App-Id", vec![api_key.to_string().bytes().collect::<Vec<_>>()]);
        headers.set_raw("X-App-Token", vec![api_user.to_string().bytes().collect::<Vec<_>>()]);
        headers.set_raw("Content-Type", vec![b"application/json".to_vec()]);

        VoucherifyRequest {
            client: client,
            headers: headers,
        }
    }

    pub fn execute(&self, url: Url) -> Result<Response, Error> {
        self.client.get(url).headers(self.headers.clone()).send()
    }
}
