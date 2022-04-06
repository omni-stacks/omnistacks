#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

pub mod db;
pub mod db_pool;
pub mod models;
pub mod schema;