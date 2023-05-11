#![doc = include_str!("../README.md")]

use actix_web::http::StatusCode;
use actix_web::{dev, HttpResponse, ResponseError};

use awc::error::{ConnectError, SendRequestError as AwcSendRequestError};
use awc::ClientResponse;

use std::fmt;

/// Wrapper around awc's [`SendRequestError`] implementing
/// [`actix_web::ResponseError`].
///
/// [`SendRequestError`]: AwcSendRequestError
/// [`actix_web::ResponseError`]: ResponseError
///
#[derive(Debug)]
pub struct SendRequestError(AwcSendRequestError);

impl fmt::Display for SendRequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<AwcSendRequestError> for SendRequestError {
    fn from(e: AwcSendRequestError) -> Self {
        Self(e)
    }
}

/// Convert `SendRequestError` to a server `Response`
///
impl ResponseError for SendRequestError {
    fn status_code(&self) -> StatusCode {
        match self.0 {
            AwcSendRequestError::Connect(ConnectError::Timeout) => StatusCode::GATEWAY_TIMEOUT,
            AwcSendRequestError::Connect(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

/// Trait for converting a [`ClientResponse`] into a [`HttpResponse`].
///
/// You can implement this trait on your types, of course, but its
/// main goal is to enable [`ClientResponse`] as return value in
/// [`impl Responder`](actix_web::Responder) contexts.
///
/// [`ClientResponse`]: ClientResponse
/// [`HttpResponse`]: HttpResponse
///
pub trait IntoHttpResponse {
    /// Creates a [`HttpResponse`] from `self`.
    ///
    /// [`HttpResponse`]: HttpResponse
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
    /// [`HttpResponse`]: HttpResponse
    /// [`into_http_response`]: Self::into_http_response
    ///
    fn into_wrapped_http_response<E>(self) -> Result<HttpResponse, E>
    where
        Self: Sized,
    {
        Ok(self.into_http_response())
    }
}

impl IntoHttpResponse for ClientResponse<dev::Decompress<dev::Payload>> {
    fn into_http_response(self) -> HttpResponse {
        let mut response = HttpResponse::build(self.status());

        self.headers().into_iter().for_each(|(k, v)| {
            response.insert_header((k, v));
        });

        response.streaming(self)
    }
}
