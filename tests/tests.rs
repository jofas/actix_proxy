use actix_web::{test, App};
use actix_web::client::Client;

use actix_proxy::util::google_proxy;

#[actix_rt::test]
async fn test_google_proxy() {
  let client = Client::default();

  let mut app = test::init_service(
    App::new()
      .data(client)
      .service(google_proxy)
  )
  .await;

  let req = test::TestRequest::get().uri("/").to_request();

  let resp = test::call_service(&mut app, req).await;

  assert!(resp.status().is_success());
}
