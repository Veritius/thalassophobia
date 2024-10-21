mod segment;

pub use segment::BodySegment;

use bevy::prelude::*;

pub(crate) struct BodyPlugin;

impl Plugin for BodyPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<BodySegment>();
    }
}