use shared::{bevy::prelude::*, rapier::prelude::*, progress::*, input::prelude::*};
use shared::{state::GameState, physics::*, player::{controller::*, movement::*}};
use crate::initial::InitialLoading;
use crate::settings::ControlSettings;

pub(crate) struct DebugSystemsPlugin;

impl Plugin for DebugSystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup_system);
        app.add_systems(Done::<InitialLoading>::new(), loaded_system);
        app.add_systems(Update, always_system);
    }
}

fn startup_system(
    mut commands: Commands,
) {

}

fn loaded_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    controls: Res<ControlSettings<GroundedHumanMovements>>,
) {
    // Set game state to simulating
    commands.add(|world: &mut World| { world.resource_mut::<NextState<GameState>>().set(GameState::Simulating) });

    // Spawn the floor
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(20.0, 0.2, 20.0)),
        material: materials.add(StandardMaterial::default()),
        ..default()
    }).insert((
        RigidBody::Fixed,
        Collider::cuboid(10.0, 0.1, 10.0),
        CollisionGroups {
            memberships: PHYS_GROUP_STRUCTURE,
            filters: Group::all(),
        },
        PHYS_DOM_TERRAIN,
    ));

    // Spawn a light
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(0.0, 5.0, 0.0),
        ..default()
    });

    // Character head
    let head = commands.spawn((
        PlayerControllerCamera { rotation_pitch: 0.0 },
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
    )).id();

    // Character body
    commands.spawn((
        PlayerController {
            head_entity: Some(head),
            ..default()
        },
        TransformBundle::from_transform(Transform::from_xyz(0.0, 1.0, 0.0)),
        VisibilityBundle::default(),
        InputManagerBundle::with_map(controls.0.clone()),
        RigidBody::Dynamic,
        Collider::capsule_y(0.5, 0.5),
        CollisionGroups {
            memberships: PHYS_GROUP_CHARACTER,
            filters: PHYS_GROUP_TERRAIN | PHYS_GROUP_STRUCTURE,
        },
        PHYS_DOM_CHARACTER,
        LockedAxes::ROTATION_LOCKED,
        Damping { linear_damping: 5.0, angular_damping: 1.0 },
        GravityScale(5.0),
        Ccd::enabled(),
        ExternalImpulse::default(),
    )).add_child(head);
}

fn always_system(
    mut commands: Commands,
) {

}