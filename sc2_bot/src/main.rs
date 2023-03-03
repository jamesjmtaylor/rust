use rust_sc2::prelude::*;


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
        self.chat("gl hf");
        self.get_minerals();
        Ok(())
    }

    /// Called on every game step
    fn on_step(&mut self, _iteration: usize) -> SC2Result<()> {
        //TODO: Test if one is already being built.
        if self.counter().all().count(UnitTypeId::SupplyDepot) < 1 {
            self.construct_building(UnitTypeId::SupplyDepot);
        } else if self.counter().all().count(UnitTypeId::Barracks) < 1  {
            self.construct_building(UnitTypeId::Barracks);
        } else {
            self.build_marine();
        }
        Ok(())
    }

    // Called once on last step
    // "result" says if your bot won or lost game
    fn on_end(&self, _result: GameResult) -> SC2Result<()> {
        /* code here */
        Ok(())
    }

    /// Called on different events, see more in
    ///`https://github.com/UltraMachine/rust-sc2/blob/master/examples/events.rs`
    fn on_event(&mut self, _event: Event) -> SC2Result<()> {
        /* code here */
        Ok(())
    }
}

impl ResolutionBot {

    fn get_minerals(&mut self) {
        let idle_workers = self.units.my.workers.idle();

        // Splitting workers to closest mineral crystals
		for u in &idle_workers {
            let minerals = &self.units.mineral_fields;
			if let Some(mineral) = minerals.closest(u) {
				u.gather(mineral.tag(), false);
			}
		}
    }


    fn construct_building(&mut self, id: UnitTypeId) { 
        // Building near start, close to map center to not accidentally block mineral line.
        let main_base = self.start_location.towards(self.game_info.map_center, 8.0);
        if self.can_afford(id, false) {
            // Finding a perfect location for a building.
            if let Some(location) = self.find_placement(
                UnitTypeId::Barracks,
                main_base,
                PlacementOptions {  step: 4, ..Default::default() }
            ) { // Finding workers which are not already building.
                if let Some(builder) = self.units.my.workers.iter()
                .filter(|w| !w.is_constructing())
                .closest(location) { // Ordering scv to build  
                    builder.build(id, location, false);
                    self.subtract_resources(UnitTypeId::Barracks, false);
                }
            }
        }
    }

    fn build_marine(&mut self) {
        for barrack in self.units.my.structures.iter()
        .of_type(UnitTypeId::Barracks).ready().idle() {
            if self.can_afford(UnitTypeId::Marine, true) {
                barrack.train(UnitTypeId::Marine, false);
                //TODO: Fix this.
                // self.subtract_resources(UnitTypeId::Marine, true);
            } else { // Can't afford more marines. Stop the iterator.
                break;
            }
        }
    }
}

fn main() -> SC2Result<()> {
    let mut bot = ResolutionBot::default();
     //VeryEasy, Easy, Medium, Hard, VeryHard
    let computer = Computer::new(Race::Random, Difficulty::VeryEasy, None);
    run_vs_computer(&mut bot, computer, "Flat64", Default::default(),)
}