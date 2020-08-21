use super::{
    entities::Entity,
    scenario::{Scenario, Scene},
    user_actions::{prompt_action, UserAction},
};
use slog::o;

pub struct PlayScenario<'t> {
    log: slog::Logger,
    scenario: &'t Scenario,
    curr_scene: &'t Scene,
    prev_scene_name: Option<String>,
    player: &'t Entity,
    scene_done: bool,
}

impl<'t> PlayScenario<'t> {
    pub fn new(log: &slog::Logger, scenario: &'t Scenario) -> Self {
        let start = scenario
            .start()
            .or(scenario.scenes.first())
            .expect("There is no start scene :-( - please fix!");
        let player = &scenario.player;
        Self {
            log: log.new(o!("l_name" => "play_game")),
            scenario,
            curr_scene: start,
            prev_scene_name: None,
            player,
            scene_done: false
        }
    }

    fn print_scene(&self) {
        println!("{}", self.curr_scene);
    }

    pub fn play(&mut self) {
        self.print_scene();

        loop {
            if self.curr_scene.is_quit() {
                break;
            }
            match prompt_action() {
                Ok(action) => {
                    let act_result = self.process_action(&action);
                    match act_result {
                        ActionResult::Unknown(act) => {
                            eprintln!("Error - Unknown action: {:?}", act);
                            continue;
                        }
                        ActionResult::Quit => {
                            break;
                        }
                        ActionResult::Success => {
                            continue;
                        }
                        ActionResult::Fail(err) => {
                            eprintln!("Error - Fail: {}", err);
                        }
                        ActionResult::Death => {
                            println!("You have died!");
                            break;
                        }
                    }
                }
                Err(err) => {
                    eprintln!("Error: {}", err.to_string());
                }
            }
        println!("Great game - you can try again later!");
        }
    }

    fn process_scene(&mut self, scene: &Scene) -> ActionResult {
        let scene_type = scene.scene_type();
        match scene_type {
            super::scenario::SceneType::Quit => {
                println!("{}", scene);
                return ActionResult::Quit;
            }
            super::scenario::SceneType::Select => {
                println!("{}", scene);
            }
            super::scenario::SceneType::Random(_) => {}
            super::scenario::SceneType::Potion(_) => {}
            super::scenario::SceneType::Weapon(_) => {}
            super::scenario::SceneType::Monster(id) => {
                return self.handle_monster(scene, &id);
            }
            super::scenario::SceneType::Unknown(name) => {
                return ActionResult::Fail(format!("Unknown scene type: {}", name));
            }
        }
        ActionResult::Success
    }

    fn handle_monster(&mut self, scene: & Scene, id: &str) -> ActionResult {
        let monster = self.scenario.get_monster(id).expect("Monster with provided name does not exists!");
        while monster.is_alive() {

        }
        ActionResult::Success
    }

    fn process_action(&mut self, action: & UserAction) -> ActionResult {
        match action {
            UserAction::Select(_selector) => {

            }
            UserAction::Next => {
                // Do nothing
            }
            UserAction::Quit => {}
            UserAction::Inventory => {}
            UserAction::Player => {
                println!("{}", self.player);
            }
            UserAction::Unknown(x) => {
                return ActionResult::Unknown(x.into());
            }
            UserAction::Empty => {
                // Do nothing
            }
            UserAction::Scene => {
                println!("{}", self.curr_scene)
            }
            UserAction::Help => {}
        }
        ActionResult::Success
    }
}
enum ActionResult {
    Unknown(String),
    Quit,
    Success,
    Death,
    Fail(String),
}

pub struct SceneProcessor {
    log: slog::Logger,
}
