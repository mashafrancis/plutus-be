use crate::{
	config::database::Pool,
	constants,
	models::{filters::PersonFilter, person::PersonDTO, response::ResponseBody},
	services::people_service,
};
use actix_web::{web, HttpResponse, Result};

// GET api/people
pub async fn find_all(pool: web::Data<Pool>) -> Result<HttpResponse> {
	match people_service::find_all(&pool) {
		Ok(people) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, people))),
		Err(err) => Ok(err.response()),
	}
}

// GET api/people/id/{id}
pub async fn find_by_id(id: web::Path<i32>, pool: web::Data<Pool>) -> Result<HttpResponse> {
	match people_service::find_by_id(id.into_inner(), &pool) {
		Ok(person) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, person))),
		Err(err) => Ok(err.response()),
	}
}

// GET api/people/filter
pub async fn filter(
	web::Query(filter): web::Query<PersonFilter>,
	pool: web::Data<Pool>,
) -> Result<HttpResponse> {
	match people_service::filter(filter, &pool) {
		Ok(page) => Ok(HttpResponse::Ok().json(page)),
		Err(err) => Ok(err.response()),
	}
}

// POST api/people
pub async fn insert(
	new_person: web::Json<PersonDTO>,
	pool: web::Data<Pool>,
) -> Result<HttpResponse> {
	match people_service::insert(new_person.0, &pool) {
		Ok(()) => Ok(HttpResponse::Created()
			.json(ResponseBody::new(constants::MESSAGE_OK, constants::EMPTY))),
		Err(err) => Ok(err.response()),
	}
}

// PUT api/people/id/{id}
pub async fn update(
	id: web::Path<i32>,
	updated_person: web::Json<PersonDTO>,
	pool: web::Data<Pool>,
) -> Result<HttpResponse> {
	match people_service::update(id.into_inner(), updated_person.0, &pool) {
		Ok(()) => {
			Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, constants::EMPTY)))
		}
		Err(err) => Ok(err.response()),
	}
}

// DELETE api/people/id/{id}
pub async fn delete(id: web::Path<i32>, pool: web::Data<Pool>) -> Result<HttpResponse> {
	match people_service::delete(id.into_inner(), &pool) {
		Ok(()) => {
			Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, constants::EMPTY)))
		}
		Err(err) => Ok(err.response()),
	}
}
