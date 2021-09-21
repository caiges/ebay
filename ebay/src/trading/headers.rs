use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::RequestBuilder;

const EBAY_API_COMPATIBILITY_LEVEL: &'static str = "967";
const EBAY_API_SITEID: &'static str = "0";

pub trait AddTradingApiHeaders {
  fn add_trading_api_headers(&mut self, call_name: &str) -> &mut Self;
}

impl AddTradingApiHeaders for RequestBuilder {
  fn add_trading_api_headers(&mut self, call_name: &str) -> &mut Self {
    let mut headers = HeaderMap::new();
    headers.insert(
      "X-EBAY-API-COMPATIBILITY-LEVEL",
      HeaderValue::from_static(EBAY_API_COMPATIBILITY_LEVEL),
    );
    headers.insert(
      "X-EBAY-API-CALL-NAME",
      HeaderValue::from_static(&call_name.to_owned()[..]),
    );
    headers.insert(
      "X-EBAY-API-SITEID",
      HeaderValue::from_static(EBAY_API_SITEID),
    );
    headers.insert("Accept", HeaderValue::from_static("xml"));
    headers.insert("Content-Type", HeaderValue::from_static("application/xml"));
    &mut self.headers(headers)
  }
}
