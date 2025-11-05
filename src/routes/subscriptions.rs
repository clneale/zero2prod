use actix_web::{HttpResponse, web};

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

pub async fn subscribe(form: web::Form<FormData>) -> HttpResponse {
    print!("{} : {}", form.name, form.email);
    HttpResponse::Ok().finish()
}
