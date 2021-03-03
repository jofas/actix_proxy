use actix_web::{HttpResponse, dev};
use actix_web::client::ClientResponse;

use async_trait::async_trait;

#[async_trait(?Send)]
pub trait IntoHttpResponse {
  async fn into_http_response(mut self) -> HttpResponse;
}

#[async_trait(?Send)]
impl IntoHttpResponse for ClientResponse<dev::Decompress<dev::Payload>> {
  async fn into_http_response(mut self) -> HttpResponse {
    let mut response = HttpResponse::build(self.status());

    self.headers().iter().for_each(|(k, v)| {
      response.set_header(k, v.clone());
    });

    response.streaming(self)
  }
}

pub mod util {
  use actix_web::{get, HttpResponse, web};
  use actix_web::http::StatusCode;
  use actix_web::client::{Client, SendRequestError};
  use actix_web::error::PayloadError;

  use serde::{Serialize};

  use super::IntoHttpResponse;

  pub fn google_config(cfg: &mut web::ServiceConfig) {
    cfg.data(Client::default()).service(google_proxy);
  }

  #[get("/{url:.*}")]
  pub async fn google_proxy(
    web::Path((url,)): web::Path<(String,)>,
    client: web::Data<Client>,
  ) -> actix_web::Result<HttpResponse, Error> {
    let url = format!("https://www.google.com/{}", url);

    Ok(client.get(&url)
      .send()
      .await?
      .into_http_response()
      .await)
  }

  #[derive(Serialize, Debug)]
  pub enum Error {
    PayloadError,
    SendRequestError,
  }

  impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
      write!(f, "{:?}", self)
    }
  }

  impl From<PayloadError> for Error {
    fn from(_: PayloadError) -> Self {
      Self::PayloadError
    }
  }

  impl From<SendRequestError> for Error {
    fn from(_: SendRequestError) -> Self {
      Self::SendRequestError
    }
  }

  impl actix_web::error::ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
      HttpResponse::build(self.status_code())
        .json(self)
    }

    fn status_code(&self) -> StatusCode {
      StatusCode::INTERNAL_SERVER_ERROR
    }
  }
}
