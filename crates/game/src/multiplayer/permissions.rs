use std::{borrow::Cow, collections::BTreeSet, fmt::Debug};
use bevy::prelude::*;
use const_fnv1a_hash::fnv1a_hash_str_64;

/// A collection of permissions that can be granted to a player.
#[derive(Debug, Component)]
pub struct Permissions {
    set: BTreeSet<Permission>,
}

impl Permissions {
    /// Adds `permission` if it is in the set.
    pub fn add(&mut self, permission: Permission) {
        self.set.insert(permission);
    }

    /// Removes `permission` if it is in the set.
    pub fn remove(&mut self, permission: &Permission) {
        self.set.remove(permission);
    }

    /// Returns whether or not a peer has had `permission` granted to them.
    pub fn allowed(&self, permission: &Permission) -> bool {
        return self.set.contains(permission);
    }
}

/// An individual permission a player may have.
pub struct Permission {
    hash: u64,
    name: Cow<'static, str>,
}

impl Permission {
    pub const fn new_const(name: &'static str) -> Self {
        Self {
            hash: fnv1a_hash_str_64(name),
            name: Cow::Borrowed(name),
        }
    }

    pub fn new(name: Cow<'static, str>) -> Self {
        Self {
            hash: fnv1a_hash_str_64(name.as_ref()),
            name,
        }
    }
}

impl Debug for Permission {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.name.as_ref().fmt(f)
    }
}

impl PartialEq for Permission {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.hash.eq(&other.hash)
    }
}

impl Eq for Permission {}

impl PartialOrd for Permission {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.hash.partial_cmp(&other.hash)
    }
}

impl Ord for Permission {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.hash.cmp(&other.hash)
    }
}