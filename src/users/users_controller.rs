use actix_web::{web, HttpResponse, Responder};
use crate::users::ports::UsersService;

pub fn configure<T: 'static + UsersService>(service: web::Data<T>, cfg: &mut web::ServiceConfig) {
    cfg.app_data(service);
    cfg.route("/users", web::get().to(get_users));
    cfg.route("/users/{id}", web::get().to(get_user_by_id));
    cfg.route("/users", web::post().to(add_user));
    cfg.route("/users/{id}", web::delete().to(delete_user));
}

async fn get_users() -> impl Responder {
    HttpResponse::Ok().body("Hello from get users")
}

// async fn get_user_by_id() -> impl Responder {
//     format!("Hello from get users by Id")
// }
//
// async fn add_user() -> impl Responder {
//     format!("Hello from add user")
// }
//
// async fn delete_user() -> impl Responder {
//     format!("Hello from delete user")
// }
