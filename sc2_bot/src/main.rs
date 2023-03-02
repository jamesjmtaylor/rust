use rust_sc2::prelude::*;
use rust_sc2::action::Action;

#[bot]
#[derive(Default)]
struct ResolutionBot;
impl Player for ResolutionBot {
    /// Must be implemented
    fn get_player_settings(&self) -> PlayerSettings {
        // Race can be Terran, Zerg, Protoss or Random
        PlayerSettings::new(Race::Terran)
    }

    /// Called once on first step
    fn on_start(&mut self) -> SC2Result<()> {
        self.get_minerals();
        Ok(())
    }

    /// Called on every game step
    fn on_step(&mut self, iteration: usize) -> SC2Result<()> {
        /* code for every step of execution here */
        Ok(())
    }

    // Called once on last step
    // "result" says if your bot won or lost game
    fn on_end(&self, result: GameResult) -> SC2Result<()> {
        let last_message = match result {
            GameResult::Victory => "ez",
            _ => "gg"
        };
        // self.chat(last_message);
        Ok(())
    }

    /// Called on different events, see more in `https://github.com/UltraMachine/rust-sc2/blob/master/examples/events.rs`
    fn on_event(&mut self, event: Event) -> SC2Result<()> {
        /* code here */
        Ok(())
    }
}

impl ResolutionBot {
    fn get_minerals(&mut self) {
        let mut idle_workers = self.units.my.workers.idle();

        // Splitting workers to closest mineral crystals
		for u in &idle_workers {
			if let Some(mineral) = self.units.mineral_fields.closest(u) {
				u.gather(mineral.tag(), false);
			}
		}
    }

    // fn chat(&mut self) {
    //     self.Action.chat("test",false);
    //     // self.action.chat("test", false);
    // }
}

fn main() -> SC2Result<()> {
    let mut bot = ResolutionBot::default();
     //VeryEasy, Easy, Medium, Hard, VeryHard
    let computer = Computer::new(Race::Random, Difficulty::VeryHard, None);
    run_vs_computer(&mut bot, computer, "Flat32", Default::default(),)
}