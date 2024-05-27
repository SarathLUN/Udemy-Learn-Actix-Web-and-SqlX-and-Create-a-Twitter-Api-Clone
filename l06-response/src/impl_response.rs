use actix_web::{HttpRequest, HttpResponse, Responder, http::header::ContentType};
use actix_web::body::BoxBody;
use serde::Serialize;

#[derive(Serialize)]
pub struct Person{
    name: String
}

impl Responder for Person{
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let result = serde_json::to_string(&self);
        match result {
            Ok(item) => {HttpResponse::Ok().content_type(ContentType::json()).body(item)},
            Err(_) => {HttpResponse::InternalServerError().content_type(ContentType::plaintext()).body("Failure!")}
        }
    }
}

pub async fn get_profile_name()-> Person{
    Person{
        name: "San Jin".to_string()
    }
}