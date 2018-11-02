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
pub struct ExchangeInfoSymbols {
  #[serde(rename = "symbol")]
  symbol: Option<String>,
  #[serde(rename = "status")]
  status: Option<String>,
  #[serde(rename = "baseAsset")]
  base_asset: Option<String>,
  #[serde(rename = "baseAssetPrecision")]
  base_asset_precision: Option<f32>,
  #[serde(rename = "quoteAsset")]
  quote_asset: Option<String>,
  #[serde(rename = "quotePrecision")]
  quote_precision: Option<f32>,
  #[serde(rename = "orderTypes")]
  order_types: Option<Vec<String>>,
  #[serde(rename = "icebergAllowed")]
  iceberg_allowed: Option<bool>,
  #[serde(rename = "filters")]
  filters: Option<Vec<::models::ExchangeInfoFilters>>
}

impl ExchangeInfoSymbols {
  pub fn new() -> ExchangeInfoSymbols {
    ExchangeInfoSymbols {
      symbol: None,
      status: None,
      base_asset: None,
      base_asset_precision: None,
      quote_asset: None,
      quote_precision: None,
      order_types: None,
      iceberg_allowed: None,
      filters: None
    }
  }

  pub fn set_symbol(&mut self, symbol: String) {
    self.symbol = Some(symbol);
  }

  pub fn with_symbol(mut self, symbol: String) -> ExchangeInfoSymbols {
    self.symbol = Some(symbol);
    self
  }

  pub fn symbol(&self) -> Option<&String> {
    self.symbol.as_ref()
  }

  pub fn reset_symbol(&mut self) {
    self.symbol = None;
  }

  pub fn set_status(&mut self, status: String) {
    self.status = Some(status);
  }

  pub fn with_status(mut self, status: String) -> ExchangeInfoSymbols {
    self.status = Some(status);
    self
  }

  pub fn status(&self) -> Option<&String> {
    self.status.as_ref()
  }

  pub fn reset_status(&mut self) {
    self.status = None;
  }

  pub fn set_base_asset(&mut self, base_asset: String) {
    self.base_asset = Some(base_asset);
  }

  pub fn with_base_asset(mut self, base_asset: String) -> ExchangeInfoSymbols {
    self.base_asset = Some(base_asset);
    self
  }

  pub fn base_asset(&self) -> Option<&String> {
    self.base_asset.as_ref()
  }

  pub fn reset_base_asset(&mut self) {
    self.base_asset = None;
  }

  pub fn set_base_asset_precision(&mut self, base_asset_precision: f32) {
    self.base_asset_precision = Some(base_asset_precision);
  }

  pub fn with_base_asset_precision(mut self, base_asset_precision: f32) -> ExchangeInfoSymbols {
    self.base_asset_precision = Some(base_asset_precision);
    self
  }

  pub fn base_asset_precision(&self) -> Option<&f32> {
    self.base_asset_precision.as_ref()
  }

  pub fn reset_base_asset_precision(&mut self) {
    self.base_asset_precision = None;
  }

  pub fn set_quote_asset(&mut self, quote_asset: String) {
    self.quote_asset = Some(quote_asset);
  }

  pub fn with_quote_asset(mut self, quote_asset: String) -> ExchangeInfoSymbols {
    self.quote_asset = Some(quote_asset);
    self
  }

  pub fn quote_asset(&self) -> Option<&String> {
    self.quote_asset.as_ref()
  }

  pub fn reset_quote_asset(&mut self) {
    self.quote_asset = None;
  }

  pub fn set_quote_precision(&mut self, quote_precision: f32) {
    self.quote_precision = Some(quote_precision);
  }

  pub fn with_quote_precision(mut self, quote_precision: f32) -> ExchangeInfoSymbols {
    self.quote_precision = Some(quote_precision);
    self
  }

  pub fn quote_precision(&self) -> Option<&f32> {
    self.quote_precision.as_ref()
  }

  pub fn reset_quote_precision(&mut self) {
    self.quote_precision = None;
  }

  pub fn set_order_types(&mut self, order_types: Vec<String>) {
    self.order_types = Some(order_types);
  }

  pub fn with_order_types(mut self, order_types: Vec<String>) -> ExchangeInfoSymbols {
    self.order_types = Some(order_types);
    self
  }

  pub fn order_types(&self) -> Option<&Vec<String>> {
    self.order_types.as_ref()
  }

  pub fn reset_order_types(&mut self) {
    self.order_types = None;
  }

  pub fn set_iceberg_allowed(&mut self, iceberg_allowed: bool) {
    self.iceberg_allowed = Some(iceberg_allowed);
  }

  pub fn with_iceberg_allowed(mut self, iceberg_allowed: bool) -> ExchangeInfoSymbols {
    self.iceberg_allowed = Some(iceberg_allowed);
    self
  }

  pub fn iceberg_allowed(&self) -> Option<&bool> {
    self.iceberg_allowed.as_ref()
  }

  pub fn reset_iceberg_allowed(&mut self) {
    self.iceberg_allowed = None;
  }

  pub fn set_filters(&mut self, filters: Vec<::models::ExchangeInfoFilters>) {
    self.filters = Some(filters);
  }

  pub fn with_filters(mut self, filters: Vec<::models::ExchangeInfoFilters>) -> ExchangeInfoSymbols {
    self.filters = Some(filters);
    self
  }

  pub fn filters(&self) -> Option<&Vec<::models::ExchangeInfoFilters>> {
    self.filters.as_ref()
  }

  pub fn reset_filters(&mut self) {
    self.filters = None;
  }

}



