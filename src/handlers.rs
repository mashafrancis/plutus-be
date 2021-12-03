use actix_web::{HttpResponse, Responder};

pub async fn get_users() -> impl Responder {
    HttpResponse::Ok().body("Hello from get users")
}

pub async fn get_user_by_id() -> impl Responder {
    format!("Hello from get users by Id")
}

pub async fn add_user() -> impl Responder {
    format!("Hello from add user")
}

pub async fn delete_user() -> impl Responder {
    format!("Hello from delete user")
}
