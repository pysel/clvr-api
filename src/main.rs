use std::{fs, sync::{Arc, Mutex}};

use actix_web::{web, App, HttpServer};
use log4rs;
use server::handlers::ScheduledDatabase;
use log::error;

mod clvr;
mod trades;
mod server;
mod executor;
mod pool_fetcher;

const PORT: u16 = 8080;

async fn cleanup() {
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

// async fn expose_api(scheduled_db: ScheduledDatabase) {
//     match HttpServer::new(move || {
//         let app_data = web::Data::new(scheduled_db.clone());
//         App::new()
//             .app_data(app_data)
//             .service(server::handlers::num_trades)
//             .service(server::handlers::submit_trade)
//     })
//     .bind(("127.0.0.1", 8080)).expect("Failed to bind to port")
//     .workers(2)
//     .run()
//     .await {
//         Ok(_) => (),
//         Err(e) => error!("Error running HTTP server: {}", e),
//     }
// }

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    // load environment variables
    dotenv::dotenv().ok();

    // init logging
    cleanup().await; // cleanup existing log files before starting
    if let Err(e) = log4rs::init_file("log4rs.yml", Default::default()) {
        error!("Error initializing logging: {}", e);
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "Error initializing logging"));
    }

    // create the database
    let scheduled_db: ScheduledDatabase = Arc::new(Mutex::new(Vec::new()));

    // start the process that waits and submits trades
    let executor = executor::Executor::new(scheduled_db.clone());
    tokio::spawn(executor.run());

    // expose the api
    HttpServer::new(move || {
        let app_data = web::Data::new(scheduled_db.clone());
        App::new()
            .app_data(app_data)
            .service(server::handlers::num_trades)
            .service(server::handlers::submit_trade)
    })
    .bind(("127.0.0.1", PORT))?
    .workers(2)
    .run()
    .await
}
