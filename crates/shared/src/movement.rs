use bevy::prelude::*;
use crate::input::prelude::*;

pub(crate) struct MovementInputsPlugin;

impl Plugin for MovementInputsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<GroundedHumanMovements>::default());
        app.add_plugins(InputManagerPlugin::<FloatingHumanMovements>::default());

        #[cfg(feature="multiplayer")] {
            use crate::stardust::prelude::*;

            app.add_channel::<MovementEvent>(ChannelConfiguration {
                reliable: ReliabilityGuarantee::Unreliable,
                ordered: OrderingGuarantee::Sequenced,
                fragmented: false,
                priority: 1,
            });
        }
    }
}

#[derive(Debug, Reflect)]
pub struct MovementEvent;

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum GroundedHumanMovements {
    MoveAxis,
    Forward,
    Backward,
    StrafeLeft,
    StrafeRight,
    LeanLeft,
    LeanRight,
    Jump,
    Vault,
    Crouch,
    Sprint,
    Turn,
}

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum FloatingHumanMovements {
    MoveAxis,
    Forward,
    Backward,
    StrafeLeft,
    StrafeRight,
    Ascend,
    Descend,
    Sprint,
    Turn,
}