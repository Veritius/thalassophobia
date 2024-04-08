use crate::rapier::{geometry::Group, dynamics::Dominance};

pub const PHYS_GROUP_TERRAIN:   Group = Group::GROUP_1;
pub const PHYS_GROUP_STRUCTURE: Group = Group::GROUP_2;
pub const PHYS_GROUP_CHARACTER: Group = Group::GROUP_3;

pub const PHYS_DOM_TERRAIN: Dominance = Dominance { groups: 127 };
pub const PHYS_DOM_STRUCTURE: Dominance = Dominance { groups: 111 };
pub const PHYS_DOM_CHARACTER: Dominance = Dominance { groups: 95 };