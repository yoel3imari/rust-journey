use crate::database;
pub mod models;

pub fn login(cred: models::Credentials) {
    // do staff
    database::get_user();
}

pub fn logout() {}

