pub use crate::{bevy::prelude::*, bevy_ecs, bevy_reflect, bevy_state};
pub use crate::avian::prelude::*;
pub use crate::disabling::Disabled;
pub use crate::initial::Initialisation;
pub use crate::input::{Actionlike, input_map::InputMap, action_state::ActionState};
pub use crate::math::units::*;
pub use crate::physics::{ObjectLayer, ObjectDominance};
pub use crate::players::Player;
pub use crate::progress::{Progress, Done};
pub use crate::setup::SetupMode;
pub use crate::simulation::{SimulationLoaded, SimulationRunning, SimulationUpdate};
pub use crate::simulation::SimulationAppExt;
pub use crate::smallvec::{smallvec, SmallVec};
pub use serde::{Serialize, Deserialize};

pub type InlinedString = smartstring::SmartString<smartstring::LazyCompact>;