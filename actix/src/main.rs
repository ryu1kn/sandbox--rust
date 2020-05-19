use actix_web::{
    App, HttpResponse, HttpServer, middleware, web
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Health<'a> {
    status: &'a str
}

async fn health() -> HttpResponse {
    HttpResponse::Ok().json(Health { status: "healthy" })
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/health").route(web::get().to(health)))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

#[cfg(test)]
mod tests {
    use actix_web::{App, http, test, web, Error};
    use actix_web::dev::Service;

    use super::*;

    #[actix_rt::test]
    async fn test_index() -> Result<(), Error> {
        let mut app = test::init_service(
            App::new().service(web::resource("/health").route(web::get().to(health))),
        )
            .await;

        let req = test::TestRequest::get()
            .uri("/health")
            .to_request();
        let resp = app.call(req).await.unwrap();

        assert_eq!(resp.status(), http::StatusCode::OK);

        let response_body = match resp.response().body().as_ref() {
            Some(actix_web::body::Body::Bytes(bytes)) => bytes,
            _ => panic!("Response error"),
        };

        assert_eq!(response_body, r##"{"status":"healthy"}"##);

        Ok(())
    }
}
