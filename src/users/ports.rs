use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InputUser {
	pub first_name: String,
	pub last_name: String,
	pub email: String,
}

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait UsersService {
	async fn get_users(&self) -> bool;
}
