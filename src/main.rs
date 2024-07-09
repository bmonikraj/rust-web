
mod inbound;
mod model;
mod outbound;
mod service;

use std::io::Write;
use chrono::Local;
use env_logger::Builder;

fn main() {

    Builder::new()
        .format(|buf, record| {
            writeln!(buf,
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter(None, log::LevelFilter::Debug)
        .init();

    log::info!("Starting rust web server ...")
}
