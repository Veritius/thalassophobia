#![allow(unused_variables, unused_mut)]

use shared::character::movement::{CharacterMovements, PlayerController, PlayerControllerHead, PlayerControllerState};
use shared::input::InputManagerBundle;
use shared::physics::{DominancePreset, PHYS_LAYER_CHARACTER, PHYS_LAYER_STRUCTURE, PHYS_LAYER_TERRAIN};
use shared::progress::Done;
use shared::schedules::Simulating;
use shared::prelude::*;
use crate::initial::InitialLoading;
use crate::settings::ControlSettings;

pub(crate) struct DebugSystemsPlugin;

impl Plugin for DebugSystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup_system);
        app.add_systems(Done::<InitialLoading>::new(), loaded_system);
        app.add_systems(Update, update_system);
        app.add_systems(Update, post_update_system
            .after(PhysicsSet::Sync));
    }
}

fn startup_system(
    mut commands: Commands,
) {

}

fn loaded_system(
    mut commands: Commands,
    mut state: ResMut<Simulating>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    character_controls: Res<ControlSettings<CharacterMovements>>,
) {
    // Set game state to simulating
    *state = Simulating::Enabled;

    // Spawn the floor
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(20.0, 0.2, 20.0)),
        material: materials.add(StandardMaterial::default()),
        ..default()
    }).insert((
        RigidBody::Static,
        Collider::cuboid(10.0, 0.1, 10.0),
        CollisionLayers {
            memberships: PHYS_LAYER_TERRAIN,
            filters: LayerMask::ALL,
        },
        Dominance::from(DominancePreset::Terrain),
    ));

    // Spawn a light
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(0.0, 5.0, 0.0),
        ..default()
    });

    // Debug camera
    // commands.spawn(Camera3dBundle {
    //     transform: Transform::from_xyz(10.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    //     ..default()
    // });

    // Character head
    let head = commands.spawn((
        PlayerControllerHead::default(),
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
    )).id();

    // Character body
    commands.spawn((
        PlayerController::new(
            head,
            PlayerControllerState::Grounded,
        ),
        TransformBundle::from_transform(Transform::from_xyz(0.0, 1.0, 0.0)),
        VisibilityBundle::default(),
        InputManagerBundle::with_map(character_controls.0.clone()),
        RigidBody::Dynamic,
        Collider::capsule(0.5, 0.5),
        CollisionLayers {
            memberships: PHYS_LAYER_CHARACTER,
            filters: PHYS_LAYER_TERRAIN | PHYS_LAYER_STRUCTURE,
        },
        Dominance::from(DominancePreset::Terrain),
        LockedAxes::ROTATION_LOCKED,
        LinearDamping(5.0),
        SweptCcd::NON_LINEAR,
        ExternalImpulse::default().with_persistence(false),
    )).add_child(head);
}

fn update_system(
    mut commands: Commands,
    mut gizmos: Gizmos,
) {
    
}

fn post_update_system(
    mut commands: Commands,
    mut gizmos: Gizmos,
) {

}