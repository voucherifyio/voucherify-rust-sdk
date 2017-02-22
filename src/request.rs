use hyper::client::{Client, Response};
use hyper::{Url, Error};
use hyper::net::HttpsConnector;
use hyper::header::Headers;
use hyper_native_tls::NativeTlsClient;

pub struct VoucherifyRequest {
    client: Client,
    headers: Headers,
    payload: String,
    method: Method,
    url: Url,
}

impl VoucherifyRequest {
    pub fn new(api_key: &String, api_user: &String) -> VoucherifyRequest {
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
            payload: String::new(),
            method: Method::Get,
            url: Url::parse("https://api.voucherify.io/v1/vouchers").unwrap(),
        }
    }

    pub fn get(&mut self, url: Url) -> &mut VoucherifyRequest {
        self.method = Method::Get;
        self.url = url;
        self
    }

    pub fn post(&mut self, url: Url) -> &mut VoucherifyRequest {
        self.method = Method::Post;
        self.url = url;
        self
    }

    pub fn payload(&mut self, payload: String) -> &mut VoucherifyRequest {
        self.payload = payload;
        self
    }

    pub fn execute(&self) -> Result<Response, Error> {
        match self.method {
            Method::Get => {
                self.client.get(self.url.clone())
                           .headers(self.headers.clone())
                           .send()
            },
            Method::Post => {
                self.client.post(self.url.clone())
                           .body(self.payload.as_str())
                           .headers(self.headers.clone())
                           .send()
            },
        }
    }
}

enum Method {
    Get,
    Post,
}
