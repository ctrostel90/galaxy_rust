use slint::*;
use std::rc::Rc;

use crate::{
    mvc::{UniverseOverviewController, GalaxyModel},
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

    
    // connect_with_controller(view_handle, &controller, {
    //     move |adapter: ui::UniverseOverviewAdapter, controller| {
    //         adapter.on_show_create_task(move || {
    //             controller.show_create_task();
    //         })
    //     }
    // });
}

// maps a GalaxyModel (data) to a SelectionItem (ui)
fn map_galaxy_to_item(galaxy: GalaxyModel) -> ui::GalaxyTileItem {
    ui::GalaxyTileItem {
        name: galaxy.name.into(),
        num_planets: galaxy.number_planets,
        ..Default::default()
        //#sun_color: galaxy.sun_color.into(),
    }
}
