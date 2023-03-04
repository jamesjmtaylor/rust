use rust_sc2::prelude::*;


#[bot]
#[derive(Default)]
struct ResolutionBot {
    attacking_index : usize
}
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
        self.defend_base();
        self.get_minerals();

        if self.counter().all().count(UnitTypeId::SupplyDepot) < 1
         || (self.supply_left < 3 && self.supply_cap < 200) {
            self.construct_building(UnitTypeId::SupplyDepot);
        } else if self.counter().all().count(UnitTypeId::Barracks) < 4 {
            self.construct_building(UnitTypeId::Barracks);
        } 

        if self.counter().all().count(UnitTypeId::Marine) < 20 {
            self.build_marine();
        } else {
            self.attack_enemy_base();
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

const MARINE_RANGE : f32 = 5.0;
impl ResolutionBot {
    fn attack_targets(targets: &Units, marine: &Unit) {
        if let Some(target) = targets
        .in_range_of(marine, MARINE_RANGE).iter()
        .min_by_key(|t| t.hits())
        .or_else(|| targets.closest(marine)) {
            marine.attack(Target::Pos(target.position()), false)
        }
    }

    fn defend_base(&mut self) {
        if let Some(townhall) = self.units.my.townhalls.first() {
            let defend_location = townhall.position();
            let enemy = &self.units.enemy.all;
            let targets = enemy.filter(|e| e.is_closer(20.0, defend_location));
            for marine in &self.units.my.units.of_type(UnitTypeId::Marine) {
                ResolutionBot::attack_targets(&targets, marine);
            }
        } 
    }

    fn attack_enemy_base(&mut self) {
        let targets = &self.units.enemy.all;
        let mut index = self.attacking_index;
        for marine in &self.units.my.units.of_type(UnitTypeId::Marine) {
            if targets.is_empty() {
                if self.attacking_index == 0 
                && marine.position().is_further(MARINE_RANGE, self.enemy_start) {
                        marine.move_to(Target::Pos(self.enemy_start), false)
                } else if let Some(expansion) = self.enemy_expansions().nth(self.attacking_index) {
                    if marine.position().is_further(MARINE_RANGE, expansion.center) {
                        marine.move_to(Target::Pos(expansion.center), false)
                    } else {
                        index = index + 1;
                    }
                }
            } else {
                ResolutionBot::attack_targets(&targets, marine);
            }
        }
        self.attacking_index = index;
    }

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
        let main_base = self.start_location.towards(self.game_info.map_center, 8.0);
        if self.can_afford(id, false) { 
            if let Some(location) = self.find_placement(
                id, 
                main_base,
                PlacementOptions {  step: 2, ..Default::default() }
            ) { 
                if let Some(builder) = self.units.my.workers.iter()
                .filter(|w| !w.is_constructing())
                .closest(location) { 
                    builder.build(id, location, false);
                    self.subtract_resources(id, false);
                }
            }
        }
    }

    fn build_marine(&mut self) {
        let barracks =  &self.units.my.structures.of_type(UnitTypeId::Barracks);
        for barrack in barracks.almost_idle() {
            if self.can_afford(UnitTypeId::Marine, true) {
                barrack.train(UnitTypeId::Marine, false);
                self.subtract_resources(UnitTypeId::Marine, true);
            } else { // Can't afford more marines. Stop the iterator.
                break;
            }
        }
    }
}

fn main() -> SC2Result<()> {
    let mut bot = ResolutionBot::default();
     //VeryEasy, Easy, Medium, Hard, VeryHard
    let computer = Computer::new(Race::Random, Difficulty::VeryHard, None);
    run_vs_computer(&mut bot, computer, "Flat64", Default::default(),)
}