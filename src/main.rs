pub mod adventure;

use slog::{debug, info, o, Drain, Logger};

use adventure::{
    loader::ScenariosLoader,
    scenario::{Scenario, Scene},
};
use dotenv::dotenv;
use std::io::Write;
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
            play(&log, &s);
        }
        Err(err) => {
            eprintln!("Error - unable to load scenario: {:?}", err);
        }
    }
}

fn play(log: &Logger, scenario: &Scenario) {
    let mut next_scene = scenario
        .start()
        .expect("There is no start scene :-( - please fix!");
    loop {
        println!("\n{}", next_scene);
        if next_scene.quit {
            println!("The end!");
            break;
        }
        match prompt_scene(&next_scene) {
            Ok(input) => {
                debug!(log, "User input: "; "input" => &input);
                match next_scene.get_option(&input) {
                    Some(opt) => match scenario.get_scene(&opt.action) {
                        Some(scene) => {
                            next_scene = scene;
                        }
                        None => {
                            eprintln!("Error: No scene with name: {}", &opt.action);
                            break;
                        }
                    },
                    None => {
                        eprintln!("Error: Invalid option: {}\nTry again!", input);
                        continue;
                    }
                }
            }
            Err(err) => {
                eprintln!("Error: Unable to read input: {:?}", err);
                break;
            }
        }
    }
    println!("Great game - you can try again later!");
}

fn prompt_scene(scene: &Scene) -> std::io::Result<String> {
    let opt_string = scene
        .options
        .iter()
        .map(|opt| opt.id.clone())
        .collect::<Vec<String>>()
        .join("/");
    print!("({}) >>> ", opt_string);
    std::io::stdout().flush()?;
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_lowercase())
}

fn configure_log() -> Logger {
    env_logger::init();
    let decorator = slog_term::TermDecorator::new().build();
    let console_drain = slog_term::FullFormat::new(decorator).build().fuse();
    let console_drain = slog_envlogger::new(console_drain);
    let console_drain = slog_async::Async::new(console_drain).build().fuse();
    slog::Logger::root(console_drain, o!("v" => env!("CARGO_PKG_VERSION")))
}
