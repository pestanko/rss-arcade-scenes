use super::scenario::Scenario;
use slog::{error, o};
use std::{fs::File, path::Path};

pub struct ScenariosLoader {
    log: slog::Logger,
}

impl ScenariosLoader {
    pub fn new(log: &slog::Logger) -> Self {
        Self {
            log: log.new(o!("l_name" => "scenario_loader")),
        }
    }

    pub fn load(&self, file: &Path) -> std::io::Result<Scenario> {
        let fd = File::open(file)?;
        let buf_reader = std::io::BufReader::new(fd);
        match serde_json::from_reader(buf_reader) {
            Ok(scenario) => Ok(scenario),
            Err(err) => {
                error!(self.log, "Unable to read scenario!";  "err" => err.to_string());
                Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Unable to parse file!",
                ))
            }
        }
    }
}
