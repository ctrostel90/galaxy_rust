use std::rc::Rc;

use slint::Model;
use slint::ModelNotify;
use slint::ModelRc;
use slint::ModelTracker;

use crate::mvc;

#[derive(Clone)]
pub struct UniverseOverviewController {
    galaxy_model: GalaxyModel
    
}

impl UniverseOverviewController{
    pub fn new(repo: impl mvc::traits::GalaxyRepository + 'static) -> Self{
        Self{ 
            galaxy_model: GalaxyModel::new(repo),
        }
    }

    pub fn galaxy_model(&self) -> ModelRc<mvc::GalaxyModel> {
        ModelRc::new(self.galaxy_model.clone())
    }

    pub fn select_galaxy(&self, index: usize)->Option<mvc::GalaxyModel>{
        self.galaxy_model.select_galaxy(index)
    }
}

#[derive(Clone)]
struct GalaxyModel{
    repo: Rc<dyn mvc::traits::GalaxyRepository>,
    notify: Rc<ModelNotify>,
}

impl GalaxyModel {
    fn new(repo: impl mvc::traits::GalaxyRepository + 'static) -> Self {
        Self { repo: Rc::new(repo), notify: Rc::new(Default::default())}
    }
    fn select_galaxy(&self, index:usize) -> Option<mvc::GalaxyModel>{
        match self.repo.get_galaxy(index){
            None => {println!("No Galaxy!"); None} ,
            Some(galaxy) => {println!("{}",galaxy.name); Some(galaxy)},
        }
    }
}

impl Model for GalaxyModel{
    type Data = mvc::GalaxyModel;

    fn row_count(&self) -> usize {
        self.repo.galaxy_count()
    }

    fn row_data(&self, row: usize) -> Option<Self::Data> {
        self.repo.get_galaxy(row)
    }

    fn model_tracker(&self) -> &dyn ModelTracker {
        self.notify.as_ref()
    }
}