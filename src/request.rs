use hyper::client::{Client, Response};
use hyper::{Url, Error};
use hyper::method::Method;
use hyper::net::HttpsConnector;
use hyper::header::Headers;
use hyper_native_tls::NativeTlsClient;

pub struct VoucherifyRequest {
    client: Client,
    headers: Headers,
    payload: String,
}

impl VoucherifyRequest {
    pub fn new(app_id: &str, app_token: &str) -> VoucherifyRequest {
        let ssl = NativeTlsClient::new().unwrap();
        let connector = HttpsConnector::new(ssl);
        let client = Client::with_connector(connector);

        let mut headers = Headers::new();
        headers.set_raw("X-App-Id",
                        vec![app_id.to_string().bytes().collect::<Vec<_>>()]);
        headers.set_raw("X-App-Token",
                        vec![app_token.to_string().bytes().collect::<Vec<_>>()]);
        headers.set_raw("Content-Type", vec![b"application/json".to_vec()]);

        VoucherifyRequest {
            client: client,
            headers: headers,
            payload: String::new(),
        }
    }

    pub fn payload(&mut self, payload: String) -> &mut VoucherifyRequest {
        self.payload = payload;
        self
    }

    pub fn execute(&self, method: Method, url: Url) -> Result<Response, Error> {
        self.client
            .request(method, url)
            .body(self.payload.as_str())
            .headers(self.headers.clone())
            .send()
    }
}
