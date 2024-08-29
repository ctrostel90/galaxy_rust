use crate::mvc;

pub trait GalaxyRepository {
    fn galaxy_count(&self) -> usize;
    fn get_galaxy(&self, index: usize) -> Option<mvc::GalaxyModel>;
    fn remove_galaxy(&self, index: usize) -> bool;
    fn push_galaxy(&self, task: mvc::GalaxyModel) -> bool;
}