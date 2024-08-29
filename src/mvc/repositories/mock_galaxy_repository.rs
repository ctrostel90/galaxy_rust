// Copyright © SixtyFPS GmbH <info@slint.dev>
// SPDX-License-Identifier: MIT

use std::{cell::RefCell, rc::Rc};

use super::traits;
use crate::mvc;

#[derive(Clone)]
pub struct MockGalaxyRepository {
    galaxies: Rc<RefCell<Vec<mvc::GalaxyModel>>>,
}

impl MockGalaxyRepository {
    pub fn new(galaxies: Vec<mvc::GalaxyModel>) -> Self {
        Self { galaxies: Rc::new(RefCell::new(galaxies)) }
    }
}

impl traits::GalaxyRepository for MockGalaxyRepository {
    fn galaxy_count(&self) -> usize {
        self.galaxies.borrow().len()
    }

    fn get_galaxy(&self, index: usize) -> Option<mvc::GalaxyModel> {
        self.galaxies.borrow().get(index).cloned()
    }

    fn remove_galaxy(&self, index: usize) -> bool {
        if index < self.galaxies.borrow().len() {
            self.galaxies.borrow_mut().remove(index);
            return true;
        }
        false
    }

    fn push_galaxy(&self, galaxy: mvc::GalaxyModel) -> bool {
        self.galaxies.borrow_mut().push(galaxy);
        true
    }
}
