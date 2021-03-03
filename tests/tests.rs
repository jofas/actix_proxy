use actix_web::{test, App};

use actix_proxy::util::google_config;

use futures::stream::TryStreamExt;

#[actix_rt::test]
async fn test_google_proxy() {
  let mut app = test::init_service(
    App::new().configure(google_config)
  )
  .await;

  let req = test::TestRequest::get().uri("/").to_request();

  let mut resp = test::call_service(&mut app, req).await;

  assert!(resp.status().is_success());

  let bytes = test::load_stream(resp.take_body().into_stream()).await;
  assert!(bytes.unwrap().len() > 0);
}
