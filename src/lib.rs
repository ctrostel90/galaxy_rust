pub mod mvc;
pub mod ui;

// mod callback;

pub use slint::*;

pub fn main(){
    let main_window = init();

    main_window.run().unwrap();
}

pub fn init() -> ui::MainWindow{
    let view_handle = ui::MainWindow::new().unwrap();

    let overview_galaxy_controller = mvc::UniverseOverviewController::new(mvc::galaxy_repo());
    ui::universe_adapter::connect(&view_handle, overview_galaxy_controller.clone());
   
    view_handle
}