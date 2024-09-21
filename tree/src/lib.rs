#![allow(dead_code, unused)]

mod database;

mod auth_utils;


fn authenticate(cred: auth_utils::models::Credentials) {
    if let database::Status::Connected = database::connect_to_database() {
        auth_utils::login(cred);
    }
}

