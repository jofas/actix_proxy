#![doc = include_str!("../README.md")]

use actix_web::client::ClientResponse;
use actix_web::{dev, HttpResponse};

/// Trait for converting a [`ClientResponse`] into a [`HttpResponse`].
///
pub trait IntoHttpResponse {
  /// Creates a [`HttpResponse`] from `self`.
  ///
  fn into_http_response(self) -> HttpResponse;

  /// Wraps the [`HttpResponse`] created by [`into_http_response`]
  /// in a `Result`.
  ///
  /// # Errors
  ///
  /// Because [`into_http_response`] is infallible, this method is,
  /// too.
  /// So calling this method never fails and never returns an `Err`.
  ///
  /// [`into_http_response`]: Self::into_http_response
  ///
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

    response.streaming(self)
  }
}
