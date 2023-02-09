use actix_web::{body, test, App};

use awc::Client;

use actix_web::{get, web, HttpResponse};

use actix_proxy::{IntoHttpResponse, SendRequestError};

fn config(cfg: &mut web::ServiceConfig) {
  cfg
    .app_data(web::Data::new(Client::default()))
    .service(proxy);
}

#[get("/{url:.*}")]
async fn proxy(
  path: web::Path<(String,)>,
  client: web::Data<Client>,
) -> actix_web::Result<HttpResponse, SendRequestError> {
  let (url,) = path.into_inner();

  let url = format!("https://duckduckgo.com/{url}");

  client.get(&url).send().await?.into_wrapped_http_response()
}

#[actix_web::test]
async fn test_proxy() {
  let mut app =
    test::init_service(App::new().configure(config)).await;

  let req = test::TestRequest::get().uri("/search?q=a").to_request();

  let resp = test::call_service(&app, req).await;

  assert!(resp.status().is_success());

  let bytes = resp.into_body();
  let bytes = body::to_bytes(bytes).await.unwrap();

  assert!(!bytes.is_empty());
}
