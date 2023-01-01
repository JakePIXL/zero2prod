use actix_web::{web, HttpResponse};

#[derive(serde::Deserialize)]
pub struct FormData {
    pub email: String,
    pub name: String,
}

pub async fn subscribe(form: web::Form<FormData>) -> HttpResponse {
    _ = form.email;
    _ = form.name;
    HttpResponse::Ok().finish()
}