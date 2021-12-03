use actix_web::HttpResponse;

#[get("/ping")]
pub(crate) fn ping() -> HttpResponse {
	HttpResponse::Ok().body("pong!".to_string())
}

#[cfg(test)]
mod tests {}
