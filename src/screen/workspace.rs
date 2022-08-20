use std::collections::BTreeMap;

use super::{CResult, Component};

pub struct Workspace {
    // Tree of Components!
    components: BTreeMap<u16, Box<dyn Component>>,
}

impl Workspace {
    pub fn new() -> Self {
        Self {
            components: BTreeMap::new(),
        }
    }
    pub fn add_component(&mut self, id: u16, component: Box<dyn Component>) {
        self.components.insert(id, component);
    }
    pub fn del_component(&mut self, id: u16) {
        self.components.remove(&id);
    }
    pub fn update(&mut self) -> CResult<()> {
        for (_cid, component) in self.components.iter_mut() {
            component.update()?;
        }
        Ok(())
    }
    pub fn draw(&mut self) -> CResult<()> {
        for (_cid, component) in self.components.iter_mut() {
            component.draw()?;
        }
        Ok(())
    }
}
