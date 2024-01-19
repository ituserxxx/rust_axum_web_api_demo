extern crate diesel;
use diesel::prelude::*;

use lazy_static::lazy_static;
use std::sync::Mutex;
use std::env;


lazy_static! {

    static ref DATABASE_URL: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    static ref CONNECTION: Mutex<MysqlConnection> = Mutex::new(MysqlConnection::establish(&DATABASE_URL).expect("Failed to connect to database"));
}