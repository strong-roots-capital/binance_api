use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient<C: hyper::client::Connect> {
  configuration: Rc<Configuration<C>>,
  default_api: Box<::apis::DefaultApi>,
  ping_api: Box<::apis::PingApi>,
  time_api: Box<::apis::TimeApi>,
}

impl<C: hyper::client::Connect> APIClient<C> {
  pub fn new(configuration: Configuration<C>) -> APIClient<C> {
    let rc = Rc::new(configuration);

    APIClient {
      configuration: rc.clone(),
      default_api: Box::new(::apis::DefaultApiClient::new(rc.clone())),
      ping_api: Box::new(::apis::PingApiClient::new(rc.clone())),
      time_api: Box::new(::apis::TimeApiClient::new(rc.clone())),
    }
  }

  pub fn default_api(&self) -> &::apis::DefaultApi{
    self.default_api.as_ref()
  }

  pub fn ping_api(&self) -> &::apis::PingApi{
    self.ping_api.as_ref()
  }

  pub fn time_api(&self) -> &::apis::TimeApi{
    self.time_api.as_ref()
  }


}
