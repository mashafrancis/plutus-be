use crate::controllers::*;
use actix_web::web;

pub fn config_routes(cfg: &mut web::ServiceConfig) {
	info!("Configuring routes.");
	cfg.service(
		web::scope("/api")
			.service(ping_controller::ping)
			.service(
				web::scope("/auth")
					.service(
						web::resource("/signup").route(web::post().to(account_controller::signup)),
					)
					.service(
						web::resource("/login").route(web::post().to(account_controller::login)),
					)
					.service(
						web::resource("/logout").route(web::post().to(account_controller::logout)),
					),
			)
			.service(
				web::scope("/people")
					.service(
						web::resource("")
							.route(web::get().to(people_controller::find_all))
							.route(web::post().to(people_controller::insert)),
					)
					.service(
						web::resource("/id/{id}")
							.route(web::get().to(people_controller::find_by_id))
							.route(web::put().to(people_controller::update))
							.route(web::delete().to(people_controller::delete)),
					)
					.service(
						web::resource("/filter").route(web::get().to(people_controller::filter)),
					),
			),
	);
}
