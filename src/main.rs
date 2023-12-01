use run::run;

mod config;
mod db;
mod event;
mod page;
mod run;

#[tokio::main]
async fn main() {
    run().await.unwrap();
}
