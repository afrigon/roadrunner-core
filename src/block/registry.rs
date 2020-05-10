use crate::block::BlockProperties;

use std::collections::HashMap;

pub struct BlockRegistry {
    data: HashMap<u8, BlockProperties>,
}

impl BlockRegistry {
    pub fn new(data: HashMap<u8, BlockProperties>) -> Self {
        Self { data }
    }

    //pub fn get(&self, name: &str) -> Block {
    // TODO: add another map to get blocks by name
    //}

    pub fn get_properties(&self, id: u8) -> Option<&BlockProperties> {
        self.data.get(&id)
    }

    //pub fn register(id: u8, properties: BlockProperties) {
    // TODO: implement this shit
    //}

    pub fn is_opaque(&self, id: u8) -> bool {
        if let Some(p) = self.get_properties(id) {
            return p.opaque;
        } else {
            return false;
        }
    }
}
