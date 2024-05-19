use actix_web::{App, HttpServer};
use actix_web::web::{get, post, scope};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                scope("/v1")
                    .route("/profile", get().to(index))
                    .route("/profile", post().to(insert))
            )
            .service(
                scope("/v2")
                    .route("/profile", get().to(index_v2))
                    .route("/profile", post().to(insert_v2))
            )
    }).bind(("127.0.0.1",8080))?
        .run()
        .await

}

async fn index() -> &'static str {
    "hello world"
}

async fn insert() -> &'static str {
    "inserted"
}

async fn index_v2() -> &'static str {
    "v2: hello world"
}

async fn insert_v2() -> &'static str {
    "v2: inserted"
}