#![allow(unused_variables, unused_mut)]

use shared::input::InputManagerBundle;
use shared::math::transform::TranslateSet;
use shared::physics::{ObjectDominance, ObjectLayer};
use shared::progress::Done;
use shared::schedules::Simulating;
use shared::prelude::*;
use shared::vessel::piloting::controller::VesselController;
use shared::vessel::piloting::VesselMovements;
use crate::initial::InitialLoading;
use crate::settings::ControlSettings;

pub(crate) struct DebugSystemsPlugin;

impl Plugin for DebugSystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PhysicsDebugPlugin::default());

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
    vessel_controls: Res<ControlSettings<VesselMovements>>,
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
        Collider::cuboid(20.0, 0.1, 20.0),
        CollisionLayers::new(
            ObjectLayer::Terrain,
            LayerMask::ALL,
        ),
        Dominance::from(ObjectDominance::Terrain),
    ));

    // Spawn a light
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(0.0, 5.0, 0.0),
        ..default()
    });

    // Debug camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(10.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Character body
    commands.spawn((
        VesselController {
            translate_force: TranslateSet::splat(1.0),
            rotation_force: TranslateSet::splat(1.0),
        },
        TransformBundle::from_transform(
            Transform::from_xyz(
                0.0,
                1.0,
                0.0,
            ),
        ),
        // Camera3dBundle {
        //     transform: Transform::from_xyz(
        //         0.0,
        //         1.0,
        //         0.0,
        //     ),
        //     ..default()
        // },
        VisibilityBundle::default(),
        InputManagerBundle::with_map(vessel_controls.0.clone()),
        RigidBody::Dynamic,
        Collider::capsule_endpoints(
            0.5,
            Vec3 { x: 0.0, y: 0.0, z: 1.0  },
            Vec3 { x: 0.0, y: 0.0, z: -1.0 },
        ),
        CollisionLayers::new(
            ObjectLayer::Character,
            ObjectLayer::Terrain | ObjectLayer::Structure,
        ),
        Dominance::from(ObjectDominance::Terrain),
        LinearDamping(10.0),
        AngularDamping(20.0),
        SweptCcd::NON_LINEAR,
        ExternalImpulse::default(),
        ExternalAngularImpulse::default(),
        GravityScale(0.0),
    ));
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