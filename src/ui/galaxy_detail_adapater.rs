use slint::*;
use crate::{
    mvc::{
        {GalaxyDetailController, UniverseOverviewController},
    },
    ui,
};

fn connect_with_controller(
    view_handle:&ui::MainWindow,
    controller:&GalaxyDetailController,
    connect_adapter_controller: impl FnOnce(ui::GalaxyDetailAdapter, GalaxyDetailController) + 'static,){
        connect_adapter_controller(view_handle.global::<ui::GalaxyDetailAdapter>(),controller.clone());
}

pub fn connect(view_handle:&ui::MainWindow, controller:&GalaxyDetailController){
    
}

fn connect_with_universe_overview_controller(view_handle:&ui::MainWindow,
    controller:&UniverseOverviewController,
    connect_adapter_controller: impl FnOnce(ui::GalaxyDetailAdapter, UniverseOverviewController) + 'static,){
        connect_adapter_controller(view_handle.global::<ui::GalaxyDetailAdapter>(),controller.clone());
}
pub fn connect_universe_overview_controller(){}
