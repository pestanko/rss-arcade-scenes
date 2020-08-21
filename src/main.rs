pub mod adventure;

use slog::{debug, info, o, Drain, Logger};

use adventure::{
    loader::ScenariosLoader, play::PlayScenario,
};
use dotenv::dotenv;
use std::path::Path;

fn main() {
    dotenv().ok();
    let log = configure_log();
    let args: Vec<String> = std::env::args().collect();

    let file = args
        .get(1)
        .map(|f| Path::new(f))
        .expect("You need to provide exactly one argument with the location of the scenario!");
    info!(log, "Provided scenario file"; "file" => file.display());

    let scenario_service = ScenariosLoader::new(&log);
    let scenario = scenario_service.load(file);


    match scenario {
        Ok(s) => {
            let mut play = PlayScenario::new(&log, & s);
            play.play();
        }
        Err(err) => {
            eprintln!("Error - unable to load scenario: {:?}", err);
        }
    }
}


fn configure_log() -> Logger {
    env_logger::init();
    let decorator = slog_term::TermDecorator::new().build();
    let console_drain = slog_term::FullFormat::new(decorator).build().fuse();
    let console_drain = slog_envlogger::new(console_drain);
    let console_drain = slog_async::Async::new(console_drain).build().fuse();
    slog::Logger::root(console_drain, o!("v" => env!("CARGO_PKG_VERSION")))
}
