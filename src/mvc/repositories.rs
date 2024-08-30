pub mod traits;

use crate::mvc::models::{GalaxyModel, Coord};

use rand::{rngs::ThreadRng,Rng};
mod mock_galaxy_repository;
pub use mock_galaxy_repository::*;
use slint::format;

pub fn galaxy_repo() -> impl traits::GalaxyRepository + Clone{
    MockGalaxyRepository::new(gen_universe())
}

fn gen_universe() -> Vec<GalaxyModel> {
    let mut universe: Vec<GalaxyModel> = Vec::new();
    let mut rng = rand::thread_rng();

    for x in 0..=8{
        for y in 0..7{
            universe.push(
                GalaxyModel{
                    name:format!("System {}", x * 8 + y).into(),
                    number_planets:rng.gen_range(0..50),
                    position:Coord{x,y},
                    sun_color: format!("#{}{}{}",
                        rng.gen_range(0..255),
                        rng.gen_range(0..255),
                        rng.gen_range(0..255),
                    ).into(),
                }
            )
        }
    }
    universe
}