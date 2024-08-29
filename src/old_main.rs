use rand::{rngs::ThreadRng, Rng};
use slint::{Color, Model, ModelRc, RgbaColor, VecModel};
use std::{rc::Rc};

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
            star: Some(Star::new(rng.gen_range(10.0..50.0))),
            planets: SolarSystem::_create_planets(rng),
        }
    }

    fn _create_planets(mut rng: ThreadRng) -> Vec<Planet>{
        let number_of_planets = rng.gen_range(0..8);
        let mut planets:Vec<Planet> = Vec::new();
        for _ in 0..number_of_planets{
            planets.push(
                Planet{size:5.0}
            );
        }
        planets
    }
    pub fn get_planets_model(self) -> VecModel<PlanetModel>{
        let planets_model = VecModel::new();
        self.planets.iter().map(|x| {
            planets_model.push(PlanetModel{ Size: x.size});
        });
        return planets_model;
    }
}

#[derive(Clone)]
pub struct Star {
    pub size: f32,
}

impl Star {
    pub fn new(size: f32) -> Self {
        Self { size: size }
    }
}
#[derive(Clone)]
pub struct Planet {
    pub size: f32,
}

impl Planet {
    pub fn new(size: f32) -> Self {
        Self { size: size }
    }
}

fn main() {
    let state = init();
    let main_window = state.main_window.clone_strong();

    main_window.run().unwrap();
}

fn init() -> State {
    let main_window = MainWindow::new().unwrap();
    let mut universe_model = Rc::new(VecModel::<GalaxyModel>::from(Vec::new()));

    let mut galaxy: Vec<SolarSystem> = Vec::new();
    let mut rng = rand::thread_rng();

    let row_count = main_window.get_row_count();
    let column_count = main_window.get_column_count();
    let sz = row_count * column_count;

    for val in 0..sz{
        galaxy.push(SolarSystem::gen_random(rng.clone()));
        universe_model.push(GalaxyModel {
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

    main_window.on_generate_universe({
        let weak_window = main_window.as_weak();
        let universe_model = universe_model.clone();
        let mut galaxy = galaxy.clone();
        galaxy.clear();
        move || {
            let window = weak_window.unwrap();
            for i in 0..universe_model.row_count() {
                galaxy.push(SolarSystem::gen_random(rng.clone()));
                universe_model.set_row_data(
                    i,
                    GalaxyModel {
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
            window.set_galaxy(universe_model.clone().into());
        }
    });

    main_window.on_select_system({
        let weak_window = main_window.as_weak();
        let universe_model = universe_model.clone();
        let galaxy = galaxy.clone();
        
        move |x, y| {
            let window = weak_window.unwrap();
            let idx: usize = (x * column_count + y).try_into().unwrap();
            
            window.set_selected_system(SolarSystemModel{
                Star:StarModel { Size: galaxy[idx].star.as_ref().unwrap().size },
                Planets: VecModel::<PlanetModel>::from(galaxy[idx].planets),
            });
        }
    });

    main_window.set_num_systems(galaxy.iter().filter(|x| x.occupied).count().try_into().unwrap());
    main_window.set_galaxy(universe_model.clone().into());

    State { main_window, universe_model }
}

pub struct State {
    pub main_window: MainWindow,
    pub universe_model: Rc<VecModel<GalaxyModel>>,
}

#[cfg(sandbox_code)]
fn main_old() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let mut galaxy: Vec<SolarSystem> = Vec::new();
    let mut universe: Vec<GalaxyModel> = ui.get_galaxy().iter().collect();
    universe.extend(universe.clone());

    let mut rng = rand::thread_rng();
    
    let row_count = ui.get_row_count();
    let column_count = ui.get_column_count();
    let sz = row_count * column_count;
    for val in 0..sz{
        galaxy.push(SolarSystem::gen_random(rng.clone()));
        universe.push(GalaxyModel {
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
                    GalaxyModel {
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
        let ui_weak = ui.as_weak();
        move |x, y| {
            let idx: usize = (x * column_count + y).try_into().unwrap();
            println!("{}", galaxy[idx].planets.len());
            println!("{}", universe_model.row_data(idx).unwrap().Populated);
            
            ui.set_selected_system(SolarSystemModel{
                Star:StarModel { Size: galaxy[idx].star.as_ref().unwrap().size },
            });
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
