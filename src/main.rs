use rand::Rng;
use slint::{Color, Model, RgbaColor, VecModel};
use std::rc::Rc;

slint::include_modules!();

pub enum StarType {
    SpectralClassA,
    SpectralClassF,
    SpectralClassG,
    SpectralClassK,
    WhiteDwarf,
    SpectralClassM,
    BrownDwarf,
    Giant,
    Special,
}

struct Star {
    star_type: StarType,
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let ui_handle = ui.as_weak();

    let mut universe: Vec<PotentialGalaxy> = ui.get_galaxy().iter().collect();
    universe.extend(universe.clone());

    let mut rng = rand::thread_rng();

    let row_count = ui.get_row_count();
    let column_count = ui.get_column_count();
    let sz = row_count * column_count;

    for val in 0..sz {
        universe.push(PotentialGalaxy {
            Id: val,
            Populated: rng.gen_bool(1.0/10.0),
            SystemColor: Color::from(RgbaColor {
                red: rng.gen_range(0..255),
                green: rng.gen_range(0..255),
                blue: rng.gen_range(0..255),
                alpha: 255,
            }),
        });
    }
    
    let universe_model = Rc::new(VecModel::from(universe));

    ui.on_generate_universe({
        let universe_model = universe_model.clone();
        move || {
            for i in 0..universe_model.row_count() {
                universe_model.set_row_data(
                    i,
                    PotentialGalaxy {
                        Id: i.try_into().unwrap(),
                        Populated: rng.gen_bool(1.0/10.0),
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
        move |x, y| {
            let idx: usize = (x * column_count + y).try_into().unwrap();
            println!("{}",universe_model.row_data(idx).unwrap().Populated);
    }});

    
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
