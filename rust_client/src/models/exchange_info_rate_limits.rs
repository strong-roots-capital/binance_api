/* 
 * Binance REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * OpenAPI spec version: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ExchangeInfoRateLimits {
  #[serde(rename = "rateLimitType")]
  rate_limit_type: Option<String>,
  #[serde(rename = "interval")]
  interval: Option<String>,
  #[serde(rename = "limit")]
  limit: Option<f32>
}

impl ExchangeInfoRateLimits {
  pub fn new() -> ExchangeInfoRateLimits {
    ExchangeInfoRateLimits {
      rate_limit_type: None,
      interval: None,
      limit: None
    }
  }

  pub fn set_rate_limit_type(&mut self, rate_limit_type: String) {
    self.rate_limit_type = Some(rate_limit_type);
  }

  pub fn with_rate_limit_type(mut self, rate_limit_type: String) -> ExchangeInfoRateLimits {
    self.rate_limit_type = Some(rate_limit_type);
    self
  }

  pub fn rate_limit_type(&self) -> Option<&String> {
    self.rate_limit_type.as_ref()
  }

  pub fn reset_rate_limit_type(&mut self) {
    self.rate_limit_type = None;
  }

  pub fn set_interval(&mut self, interval: String) {
    self.interval = Some(interval);
  }

  pub fn with_interval(mut self, interval: String) -> ExchangeInfoRateLimits {
    self.interval = Some(interval);
    self
  }

  pub fn interval(&self) -> Option<&String> {
    self.interval.as_ref()
  }

  pub fn reset_interval(&mut self) {
    self.interval = None;
  }

  pub fn set_limit(&mut self, limit: f32) {
    self.limit = Some(limit);
  }

  pub fn with_limit(mut self, limit: f32) -> ExchangeInfoRateLimits {
    self.limit = Some(limit);
    self
  }

  pub fn limit(&self) -> Option<&f32> {
    self.limit.as_ref()
  }

  pub fn reset_limit(&mut self) {
    self.limit = None;
  }

}



