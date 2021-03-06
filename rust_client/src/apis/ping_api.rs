/* 
 * Binance REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * OpenAPI spec version: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

use std::rc::Rc;
use std::borrow::Borrow;

use hyper;
use serde_json;
use futures::Future;

use super::{Error, configuration};
use super::request as __internal_request;

pub struct PingApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> PingApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> PingApiClient<C> {
        PingApiClient {
            configuration: configuration,
        }
    }
}

pub trait PingApi {
    fn ping(&self, ) -> Box<Future<Item = (), Error = Error<serde_json::Value>>>;
}


impl<C: hyper::client::Connect>PingApi for PingApiClient<C> {
    fn ping(&self, ) -> Box<Future<Item = (), Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/ping".to_string())
            .returns_nothing()
            .execute(self.configuration.borrow())
    }

}
