// Copyright © SixtyFPS GmbH <info@slint.dev>
// SPDX-License-Identifier: MIT

use slint::*;

use crate::{
    mvc::{CreateGalaxyController,UniverseOverviewController},
    ui,
};

// a helper function to make adapter and controller connection a little bit easier
fn connect_with_controller(
    view_handle: &ui::MainWindow,
    controller: &CreateGalaxyController,
    connect_adapter_controller: impl FnOnce(ui::CreateGalaxyAdapter, CreateGalaxyController) + 'static,
) {
    connect_adapter_controller(view_handle.global::<ui::CreateGalaxyAdapter>(), controller.clone());
}

// a helper function to make adapter and controller connection a little bit easier
fn connect_with_universe_overview_controller(
    view_handle: &ui::MainWindow,
    controller: &UniverseOverviewController,
    connect_adapter_controller: impl FnOnce(ui::UniverseOverviewAdapter, UniverseOverviewController) + 'static,
) {
    connect_adapter_controller(view_handle.global::<ui::UniverseOverviewAdapter>(), controller.clone());
}