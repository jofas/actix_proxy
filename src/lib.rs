use actix_web::client::ClientResponse;
use actix_web::{dev, HttpResponse};

pub trait IntoHttpResponse {
  fn into_http_response(self) -> HttpResponse;

  fn into_wrapped_http_response<E>(self) -> Result<HttpResponse, E>
  where
    Self: Sized,
  {
    Ok(self.into_http_response())
  }
}

impl IntoHttpResponse
  for ClientResponse<dev::Decompress<dev::Payload>>
{
  fn into_http_response(self) -> HttpResponse {
    let mut response = HttpResponse::build(self.status());

    self.headers().iter().for_each(|(k, v)| {
      response.set_header(k, v.clone());
    });

    // TODO: other stuff than header and status (e.g. extensions or
    // stuff like that)

    response.streaming(self)
  }
}

pub mod util {
  use actix_web::client::{Client, SendRequestError};
  use actix_web::{get, web, HttpResponse};

  use super::IntoHttpResponse;

  pub fn google_config(cfg: &mut web::ServiceConfig) {
    cfg.data(Client::default()).service(google_proxy);
  }

  #[get("/{url:.*}")]
  pub async fn google_proxy(
    web::Path((url,)): web::Path<(String,)>,
    client: web::Data<Client>,
  ) -> actix_web::Result<HttpResponse, SendRequestError> {
    let url = format!("https://www.google.com/{}", url);

    client.get(&url).send().await?.into_wrapped_http_response()
  }
}
