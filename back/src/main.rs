// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use log::{info, LevelFilter};
use simple_logger::SimpleLogger;

mod app;
mod database;
mod store;

fn main() {
    SimpleLogger::new()
        .with_level(LevelFilter::Debug)
        .env()
        .init()
        .unwrap();
    info!("Starting beevolution");

    // create store file if file does not exits
    if !store::init_store_file() {
        panic!("Unable to access or create store file");
    }

    let app = app::init_app();
    app.run(|_, _| {})
}
