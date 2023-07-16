use self::{keys::Keys, users::Users};

pub mod db;
pub mod keys;
pub mod users;

#[derive(Clone)]
pub struct Models {
    pub users: Users,
    pub keys: Keys,
}

impl Models {
    pub async fn new() -> Self {
        Models {
            users: Users::new().await,
            keys: Keys::new().await,
        }
    }
}
