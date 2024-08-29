use std::rc::Rc;

use slint::Model;
use slint::ModelNotify;
use slint::ModelRc;
use slint::ModelTracker;

#[derive(Clone)]
pub struct GalaxyController {
    galaxy_model:GalaxyModel,
    
}

#[derive(Clone)]
struct GalaxyModel{

}