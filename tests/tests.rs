use actix_web::{test, App};

use futures::stream::TryStreamExt;

use actix_web::client::{Client, SendRequestError};
use actix_web::{get, web, HttpResponse};

use actix_proxy::IntoHttpResponse;

fn config(cfg: &mut web::ServiceConfig) {
  cfg.data(Client::default()).service(proxy);
}

#[get("/{url:.*}")]
async fn proxy(
  web::Path((url,)): web::Path<(String,)>,
  client: web::Data<Client>,
) -> actix_web::Result<HttpResponse, SendRequestError> {
  let url = format!("https://duckduckgo.com/{url}");

  client.get(&url).send().await?.into_wrapped_http_response()
}

#[actix_rt::test]
async fn test_proxy() {
  let mut app =
    test::init_service(App::new().configure(config)).await;

  let req = test::TestRequest::get().uri("/search?q=a").to_request();

  let mut resp = test::call_service(&mut app, req).await;

  assert!(resp.status().is_success());

  let bytes = test::load_stream(resp.take_body().into_stream()).await;
  assert!(bytes.unwrap().len() > 0);
}
