use crate::{Bullet, DayCycle, LightEngine, WorldMap};


pub struct World {
    pub map: WorldMap,
    pub day_cycle: DayCycle,
    pub bullets: Vec<Bullet>,
}

impl World {
    pub fn new(light_engine: &mut LightEngine) -> World {
        Self {
            map: WorldMap::load_from_file("assets/maps/map0.tmx", 30, 20),
            day_cycle: DayCycle::new(light_engine),
            bullets: vec![],
        }
    }
}