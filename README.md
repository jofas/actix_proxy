# actix-proxy

A rust library for the `actix-web` framework. 
This library provides the `IntoHttpResponse` trait which transforms
a `ClientResponse` into a `HttpResponse`.

Sometimes you want to implement a gateway or proxy which makes a 
request to some remote service and simply forwards the response to the 
client that made the request to the proxy.
`actix-web` provides the `actix_web::client::Client` HTTP client. 
Unfortunately, `actix_web::client::ClientResponse`, the response
type of the client request, does not implement the `Responder` trait.
This makes it hard to forward the response from the remote location
through an endpoint of the proxy.

With the `IntoHttpResponse` trait offered by `actix-proxy`, all you 
need to do is call `.into_http_response()` on your `ClientResponse` to 
forward the response from the remote service back to the client.

## Example

In this example we create a basic proxy for the [duckduckgo] search
engine, simply forwarding the called url's path, query and fragment 
parts to duckduckgo:

```
use actix_web::client::{Client, SendRequestError};
use actix_web::{get, web, HttpResponse};

use actix_proxy::IntoHttpResponse;

#[get("/{url:.*}")]
async fn proxy(
  web::Path((url,)): web::Path<(String,)>,
  client: web::Data<Client>,
) -> actix_web::Result<HttpResponse, SendRequestError> {
  let url = format!("https://duckduckgo.com/{url}");

  // here we use `IntoHttpResponse` to return the request to 
  // duckduckgo back to the client that called this endpoint
  client.get(&url).send().await?.into_wrapped_http_response()
}
```

[duckduckgo]: https://duckduckgo.com/
