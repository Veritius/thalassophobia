pub mod state;
pub mod stuns;
pub mod vitality;

use bevy::prelude::*;

pub(crate) struct LivingPlugin;

impl Plugin for LivingPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<state::LifeState>();
        app.register_type::<vitality::CalculatedVitality>();
        app.register_type::<vitality::wounds::Wound>();
        app.register_type::<vitality::wounds::WoundFlatDamage>();
        app.register_type::<vitality::wounds::WoundSeverity>();
        app.register_type::<vitality::wounds::WoundSeverityAffectsVitality>();
        app.register_type::<vitality::wounds::WoundSeverityPassiveChange>();
    }
}