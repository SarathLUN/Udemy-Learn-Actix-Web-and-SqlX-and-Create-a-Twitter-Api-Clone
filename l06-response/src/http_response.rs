use actix_web::{HttpResponse, http::header::ContentType};

pub async fn get_profile_name() -> HttpResponse{
    HttpResponse::Ok()
        .insert_header(("custom-header","my header"))
        .content_type(ContentType::plaintext())
        .body("Tony Stark")
}