#[cfg(feature="devstuff")]
mod overlay;

use shared::prelude::*;

pub(crate) struct ElectricityPlugin;

impl Plugin for ElectricityPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature="devstuff")]
        overlay::setup_overlays(app);
    }
}