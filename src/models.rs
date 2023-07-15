use axum::Extension;

use self::{db::Database, users::Users};

pub mod db;
pub mod keys;
pub mod users;

#[derive(Clone)]
pub struct Models {
    pub users: Users,
}

impl Models {
    pub fn new(Extension(db): Extension<Database>) -> Self {
        Models {
            users: Users::new(db),
        }
    }
}
