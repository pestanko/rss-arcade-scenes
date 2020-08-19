use super::{
    parser::{ScenarioParser, Token},
    scenario::{GameOption, Scenario, Scene},
};
use slog::{debug, error, o, warn};
use std::{collections::HashMap, fs::File, io::BufRead, path::Path};

pub struct ScenariosLoader {
    log: slog::Logger,
    parser: ScenarioParser,
}

impl ScenariosLoader {
    pub fn new(log: &slog::Logger) -> Self {
        Self {
            log: log.new(o!("l_name" => "scenario_loader")),
            // Too simple to care about DI
            parser: ScenarioParser::new(&log),
        }
    }

    pub fn load(&self, file: &Path) -> std::io::Result<Scenario> {
        let fd = File::open(file)?;
        let buf_reader = std::io::BufReader::new(fd);
        let mut scenes: HashMap<String, Scene> = HashMap::new();

        // basic params to read
        let mut current_scene = Scene::default();

        for l in buf_reader.lines() {
            let line = l?;

            debug!(self.log, "Reading line"; "line" => &line);

            match self.parser.parse_token(&line) {
                Token::Name(value) => {
                    debug!(self.log, "Found name token"; "token" => &value);

                    current_scene.name = value;
                }
                Token::Desc(value) => {
                    debug!(self.log, "Found desc token"; "token" => &value);
                    current_scene.desc = value;
                }
                Token::Option(value) => {
                    debug!(self.log, "Found opt token"; "token" => &value);
                    current_scene
                        .options
                        .push(GameOption::parse(&value).unwrap());
                }
                Token::Quit => {
                    debug!(self.log, "Found quit token");
                    current_scene.quit = true;
                }
                Token::Delim => {
                    // scene separator
                    debug!(self.log, "Found scene delimiter token - adding scene"; "scene_name" => &current_scene.name);
                    let name = current_scene.name.clone();
                    scenes.insert(name, current_scene.clone());
                    current_scene = Scene::default();
                }
                Token::Invalid(val) => {
                    error!(self.log, "Invalid token"; "token" => val)
                }
                Token::Unknown(val) => {
                    warn!(self.log, "Unknown token"; "token" => val)
                }
                Token::Empty => {
                    continue
                }
            }
        }

        if !current_scene.name.is_empty() {
            debug!(self.log, "Adding last scene");
            let name = current_scene.name.clone();
            scenes.insert(name, current_scene);
        }

        Ok(Scenario { scenes })
    }
}

