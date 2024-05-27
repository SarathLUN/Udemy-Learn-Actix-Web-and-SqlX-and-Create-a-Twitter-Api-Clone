use actix_web::{App, HttpServer, web};

use crate::string_str::get_profile_name;

mod string_str;
mod http_response;
mod json_response;
mod impl_response;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .service(
                web::resource("/profile")
                    .route(web::get().to(get_profile_name))
            )
            .service(
                web::resource("/profile2")
                    .route(web::get().to(http_response::get_profile_name))
            )
            .service(
                web::resource("/profile3")
                    .route(web::get().to(json_response::get_profile_name))
            )
            .service(
                web::resource("/profile4")
                    .route(web::get().to(impl_response::get_profile_name))
            )

    })
        .bind(("127.0.0.1", 8001))?
        .run()
        .await
}