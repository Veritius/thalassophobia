use crate::prelude::*;

/// A segment of a body.
#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
pub struct BodySegment {

}

pub(crate) struct BodyPlugin;

impl Plugin for BodyPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<BodySegment>();
    }
}