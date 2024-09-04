use slint::*;
use std::rc::Rc;

use crate::{
    mvc::{GalaxyDetailController, GalaxyModel, UniverseOverviewController},
    ui,
};

// a helper function to make adapter and controller connection a little bit easier
pub fn connect_with_controller(
    view_handle: &ui::MainWindow,
    controller: &UniverseOverviewController,
    connect_adapter_controller: impl FnOnce(ui::UniverseOverviewAdapter, UniverseOverviewController) + 'static,
) {
    connect_adapter_controller(view_handle.global::<ui::UniverseOverviewAdapter>(), controller.clone());
}

// one place to implement connection between adapter (view) and controller
pub fn connect(view_handle: &ui::MainWindow, controller: UniverseOverviewController) {
    // sets a mapped list of the task items to the ui
    view_handle
        .global::<ui::UniverseOverviewAdapter>()
        .set_galaxies(Rc::new(MapModel::new(controller.galaxy_model(), map_galaxy_to_item)).into());

    connect_with_controller(view_handle, &controller, {
        move |adapter: ui::UniverseOverviewAdapter, controller| {
            adapter.on_select_galaxy(move |index| {
                
            })
        }
    });
}

fn connect_with_universe_overview_controller(view_handle:&ui::MainWindow,
    controller:&GalaxyDetailController,
    connect_adapter_controller: impl FnOnce(ui::UniverseOverviewAdapter, GalaxyDetailController) + 'static,){
        connect_adapter_controller(view_handle.global::<ui::UniverseOverviewAdapter>(),controller.clone());
}

pub fn connect_universe_overview_controller(view_handle:&ui::MainWindow, controller:&GalaxyDetailController, universe_controller: &UniverseOverviewController){
    
    connect_with_universe_overview_controller(view_handle, &controller,{
        move|adapter,controller|{
            adapter.on_select_galaxy(move|index:i32|{
                let galaxy:GalaxyModel = universe_controller.select_galaxy(index.try_into().unwrap()).unwrap();
                controller.show_galaxy(galaxy);
            })
        }
    })
}
// maps a GalaxyModel (data) to a SelectionItem (ui)
fn map_galaxy_to_item(galaxy: GalaxyModel) -> ui::GalaxyTileItem {
    ui::GalaxyTileItem {
        name: galaxy.name.into(),
        num_planets: galaxy.number_planets,
        sun_color: color_from_str(galaxy.sun_color),
        occupied: if galaxy.number_planets>3 { true } else { false },
    }
}

fn color_from_str(input: String) -> Color{
    Color::from_rgb_u8(
        u8::from_str_radix(&input[1..3], 16).unwrap(),
        u8::from_str_radix(&input[3..5], 16).unwrap(),
        u8::from_str_radix(&input[5..7], 16).unwrap(),
    )
}
