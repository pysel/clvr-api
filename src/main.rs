use std::{fs, thread};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::error::Error;
use tokio::net::TcpStream;
use tokio::runtime::Handle;
use tokio::io::AsyncReadExt;
use crossbeam::channel;

use executor::Executor;
use log::error;
use log4rs;

mod clvr;
mod executor;
mod scheduler;
mod trades;

fn cleanup() {
    let log_dir = "log"; // Specify your log directory here
    if let Err(e) = fs::remove_dir_all(log_dir) {
        eprintln!("Error removing log directory: {}", e);
    } else {
        println!("Log directory removed successfully.");
    }
}

fn get_chain_id() -> u64 {
    std::env::var("CHAIN_ID")
        .expect("CHAIN_ID must be set")
        .parse::<u64>()
        .expect("CHAIN_ID must be a valid number")
}

fn main() {
    // load environment variables
    dotenv::dotenv().ok();

    // cleanup existing log files before starting
    cleanup();

    // init logging
    if let Err(e) = log4rs::init_file("log4rs.yml", Default::default()) {
        error!("Error initializing logging: {}", e);
        std::process::exit(1);
    }

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    let executor = rt.block_on(async { 
        let executor = executor::Executor::new().await;
        executor
     });
}
