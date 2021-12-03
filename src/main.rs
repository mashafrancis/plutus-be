#![allow(unused_must_use)]

#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate log;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate actix_cors;
extern crate actix_rt;
extern crate bcrypt;
extern crate derive_more;
extern crate dotenv;
extern crate env_logger;
extern crate failure;
extern crate futures;
extern crate jsonwebtoken;
extern crate serde;
extern crate uuid;

use actix_cors::Cors;
use actix_service::Service;
use actix_web::{http, App, HttpServer};
use futures::FutureExt;
use std::default::Default;
use std::{env, io};

mod config;
mod controllers;
mod models;
mod schema;
mod users;

#[actix_web::main]
async fn main() -> io::Result<()> {
	dotenv::dotenv().expect("Failed to read .env file");
	env::set_var("RUST_LOG", "actix_web=debug");
	env_logger::init();

	let postgres_connection = Arc::new(config::database::configure().await);
	let redis_client = Arc::new(config::redis::configure().await);

	let port = std::env::var("APP_PORT").expect("APP_PORT not found");
	let address = format!("0.0.0.0:{}", port);
}
