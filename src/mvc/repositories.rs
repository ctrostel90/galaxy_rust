pub mod traits;

use crate::mvc::models::GalaxyModel;

mod mock_galaxy_repository;
pub use mock_galaxy_repository::*;

pub fn galaxy_repo() -> impl traits::GalaxyRepository + Clone{
    MockGalaxyRepository::new( vec![
        GalaxyModel{name:"System 1".into(), number_planets: 10, sun_color: "#ffffff".into()},
        GalaxyModel{name:"System 2".into(), number_planets: 2, sun_color: "#00ff00".into()},
        GalaxyModel{name:"System 3".into(), number_planets: 32, sun_color: "#540024".into()},
        GalaxyModel{name:"System 4".into(), number_planets: 0, sun_color: "#333333".into()},
    ])
}