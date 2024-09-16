use crate::prelude::*;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Agent {
    size: u8,
}

impl Default for Agent {
    fn default() -> Self {
        Self { size: 1 }
    }
}

impl Agent {
    pub fn with_size(size: u8) -> Self {
        assert!(size > 0);
        Self { size }
    }

    pub fn size(&self) -> u8 {
        self.size
    }
}
