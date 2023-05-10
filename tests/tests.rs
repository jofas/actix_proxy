use actix_web::{body, body::MessageBody, test, App};
use actix_web::{get, web, HttpResponse};

use awc::Client;

use futures_util::{future, pin_mut};

use actix_proxy::{IntoHttpResponse, SendRequestError};

const STREAMING_URL: &str = "https://adswizz.podigee-cdn.net/version/1679607802/media/podcast_59711_die_nervigen_episode_1054673_38_kasefuss_kuss_mit_zitterlippe.mp3?awCollectionId=svo_4b3edb&awEpisodeId=1054673&v=1679607802&listeningSessionID=0CD_382_129__ccb094e7a0cefd8a0582e6dda25d97602cfd337c";

fn config(cfg: &mut web::ServiceConfig) {
  cfg
    .app_data(web::Data::new(Client::default()))
    .service(proxy)
    .service(streaming_proxy);
}

#[get("/proxy/{url:.*}")]
async fn proxy(
  path: web::Path<(String,)>,
  client: web::Data<Client>,
) -> actix_web::Result<HttpResponse, SendRequestError> {
  let (url,) = path.into_inner();

  let url = format!("https://duckduckgo.com/{url}");

  client.get(&url).send().await?.into_wrapped_http_response()
}

#[get("/streaming")]
async fn streaming_proxy(
  client: web::Data<Client>,
) -> actix_web::Result<HttpResponse, SendRequestError> {
  client
    .get(STREAMING_URL)
    .send()
    .await?
    .into_wrapped_http_response()
}

#[actix_web::test]
async fn test_proxy() {
  let app = test::init_service(App::new().configure(config)).await;

  let req = test::TestRequest::get()
    .uri("/proxy/search?q=a")
    .to_request();

  let resp = test::call_service(&app, req).await;

  assert!(resp.status().is_success());

  let bytes = resp.into_body();
  let bytes = body::to_bytes(bytes).await.unwrap();

  assert!(!bytes.is_empty());
}

#[actix_web::test]
async fn test_streaming_proxy() {
  let app = test::init_service(App::new().configure(config)).await;

  let req = test::TestRequest::get().uri("/streaming").to_request();

  let resp = test::call_service(&app, req).await;

  assert!(resp.status().is_success());

  let body = resp.into_body();
  pin_mut!(body);

  // pull a chunk of data
  let bytes = future::poll_fn(|cx| body.as_mut().poll_next(cx)).await;
  assert!(!bytes.unwrap().unwrap().is_empty());

  // pull a second chunk of data
  let bytes = future::poll_fn(|cx| body.as_mut().poll_next(cx)).await;
  assert!(!bytes.unwrap().unwrap().is_empty());
}
