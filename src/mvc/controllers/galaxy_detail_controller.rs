use std::rc::Rc;

use slint::Model;
use slint::ModelNotify;
use slint::ModelRc;
use slint::ModelTracker;

use crate::mvc::GalaxyModel;
use crate::Callback;

#[derive(Clone)]
pub struct GalaxyDetailController {
    galaxy_model:GalaxyModel,
    select_galaxy_detail_callback: Rc<Callback<(),()>>,    
}

impl GalaxyDetailController{

    pub fn show_galaxy(&self,galaxy:GalaxyModel){
        self.show_galaxy_callback.invoke(&(galaxy));
    }

    pub fn on_show_galaxy(&self,mut callback: impl FnMut() + 'static){
        self.show_galaxy.on(move||{
            callback();
        });
    }
}
