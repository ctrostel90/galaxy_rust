use rand::{rngs::ThreadRng, Rng};
use slint::{Color, Model, RgbaColor, VecModel};
use std::rc::Rc;

slint::include_modules!();

#[derive(Clone)]
pub struct SolarSystem {
    pub occupied: bool,
    pub star: Option<Star>,
    pub planets: Vec<Planet>,
}
impl SolarSystem {
    pub fn gen_random(mut rng: ThreadRng) -> Self {
        Self {
            occupied: rng.gen_bool(3.0 / 10.0),
            star: Some(Star::new(10)),
            planets: SolarSystem::_create_planets(rng),
        }
    }

    fn _create_planets(mut rng: ThreadRng) -> Vec<Planet>{
        let number_of_planets = rng.gen_range(0..8);
        let mut planets:Vec<Planet> = Vec::new();
        for _ in 0..number_of_planets{
            planets.push(
                Planet{size:5}
            );
        }
        planets
    }
}

#[derive(Clone)]
pub struct Star {
    pub size: u64,
}

impl Star {
    pub fn new(size: u64) -> Self {
        Self { size: size }
    }
}
#[derive(Clone)]
pub struct Planet {
    pub size: u64,
}

impl Planet {
    pub fn new(size: u64) -> Self {
        Self { size: size }
    }
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let ui_handle = ui.as_weak();

    let mut galaxy: Vec<SolarSystem> = Vec::new();
    let mut universe: Vec<PotentialGalaxy> = ui.get_galaxy().iter().collect();
    universe.extend(universe.clone());

    let mut rng = rand::thread_rng();
    
    let row_count = ui.get_row_count();
    let column_count = ui.get_column_count();
    let sz = row_count * column_count;
    for val in 0..sz{
        galaxy.push(SolarSystem::gen_random(rng.clone()));
        universe.push(PotentialGalaxy {
            Id: val,
            Populated: galaxy.last().unwrap().occupied,
            SystemColor: Color::from(RgbaColor {
                red: rng.gen_range(0..255),
                green: rng.gen_range(0..255),
                blue: rng.gen_range(0..255),
                alpha: 255,
            }),
        });
    }
    println!("Size of sz: {} Size of galaxy: {} size of universe {}", sz, galaxy.len(),universe.len());
    let universe_model = Rc::new(VecModel::from(universe));

    ui.on_generate_universe({
        let universe_model = universe_model.clone();
        let mut galaxy = galaxy.clone();
        galaxy.clear();
        move || {
            for i in 0..universe_model.row_count() {
                galaxy.push(SolarSystem::gen_random(rng.clone()));
                universe_model.set_row_data(
                    i,
                    PotentialGalaxy {
                        Id: i.try_into().unwrap(),
                        Populated: galaxy.last().unwrap().occupied,
                        SystemColor: Color::from(RgbaColor {
                            red: rng.gen_range(0..255),
                            green: rng.gen_range(0..255),
                            blue: rng.gen_range(0..255),
                            alpha: 255,
                        }),
                    },
                );
            }
        }
    });

    ui.on_print_xy({
        let universe_model = universe_model.clone();
        let galaxy = galaxy.clone();
        move |x, y| {
            let idx: usize = (x * column_count + y).try_into().unwrap();
            println!("{}", galaxy[idx].planets.len());
            println!("{}", universe_model.row_data(idx).unwrap().Populated);
        }
    });

    ui.set_num_systems(galaxy.iter().filter(|x| x.occupied).count().try_into().unwrap());
    ui.set_galaxy(universe_model.clone().into());
    // ui.on_request_increase_value({

    //     // move || {
    //     //     let ui = ui_handle.unwrap();
    //     //     ui.set_counter(ui.get_counter() + 1);
    //     // }
    // });

    ui.run()
}

//universe has a set size
