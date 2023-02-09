# actix-proxy

A rust library for the [`actix-web`](https://actix.rs/) framework. 
Glues together the [`actix-web`] and [`awc`] crates.

This library provides the `IntoHttpResponse` trait which transforms
a [`awc::ClientResponse`] into a [`actix_web::HttpResponse`] and
the `SendRequestError` type bridging the gap between awc's 
[`SendRequestError`] and actix-web, by implementing 
[`actix_web::ResponseError`].

Sometimes you want to implement a gateway or proxy, which makes a 
request to some remote service and forwards the response to the 
client that made the request.
actix-web integrates with the [`awc::Client`] HTTP client. 
Unfortunately, [`awc::ClientResponse`], the response type of the 
client request, does not implement the [`Responder`] trait.
Because of that, you can't return [`awc::ClientResponse`] from an
endpoint of your actix-web server.
This makes it hard to forward the response from the remote location
through an endpoint of the proxy, requiring you to transform the
response into a type that implements [`Responder`].

With the `IntoHttpResponse` trait offered by `actix-proxy`, all you 
need to do is call the `into_http_response` method on your
[`awc::ClientResponse`] to forward the response from the remote 
service through the proxy to the original caller.

## Example

In this example we create a basic proxy for the [duckduckgo] search
engine, simply forwarding the called url's path, query and fragment 
parts to duckduckgo:

```rust
use awc::Client;

use actix_web::{get, web, HttpResponse};

use actix_proxy::{IntoHttpResponse, SendRequestError};

#[get("/{url:.*}")]
async fn proxy(
  path: web::Path<(String,)>,
  client: web::Data<Client>,
) -> Result<HttpResponse, SendRequestError> {
  let (url,) = path.into_inner();

  let url = format!("https://duckduckgo.com/{url}");

  // here we use `IntoHttpResponse` to return the request to 
  // duckduckgo back to the client that called this endpoint
  Ok(client.get(&url).send().await?.into_http_response())
}
```

Alternatively, you can use the `into_wrapped_http_response` method
to avoid having to wrap your result in an `Ok(..)` by hand:

```rust
use awc::Client;

use actix_web::{get, web, HttpResponse};

use actix_proxy::{IntoHttpResponse, SendRequestError};

#[get("/{url:.*}")]
async fn proxy(
  path: web::Path<(String,)>,
  client: web::Data<Client>,
) -> Result<HttpResponse, SendRequestError> {
  let (url,) = path.into_inner();

  let url = format!("https://duckduckgo.com/{url}");

  // here we use `IntoHttpResponse` to return the request to 
  // duckduckgo back to the client that called this endpoint
  client.get(&url).send().await?.into_wrapped_http_response()
}
```

[`actix-web`]: https://docs.rs/actix-web/latest/actix_web/index.html
[`actix_web::HttpResponse`]: https://docs.rs/actix-web/latest/actix_web/struct.HttpResponse.html
[`actix_web::ResponseError`]: https://docs.rs/actix-web/latest/actix_web/trait.ResponseError.html 
[`awc`]: https://docs.rs/awc/latest/awc/
[`awc::Client`]: https://docs.rs/awc/latest/awc/struct.Client.html
[`awc::ClientResponse`]: https://docs.rs/awc/latest/awc/struct.ClientResponse.html
[`SendRequestError`]: https://docs.rs/awc/latest/awc/error/enum.SendRequestError.html
[`Responder`]: https://docs.rs/actix-web/latest/actix_web/trait.Responder.html
[duckduckgo]: https://duckduckgo.com/
