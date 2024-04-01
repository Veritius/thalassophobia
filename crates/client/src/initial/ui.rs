use shared::bevy::prelude::*;
use shared::bevy_ecs;

#[derive(Component)]
pub(super) struct InitialLoadingUiElement;

pub(super) fn spawn_loading_screen(
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    // Spawn camera
    commands.spawn((
        InitialLoadingUiElement,
        Camera2dBundle::default()
    ));

    // Construct loading screen
    commands
    .spawn((
        InitialLoadingUiElement,
        NodeBundle {
            background_color: BackgroundColor(Color::DARK_GRAY),
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        },
    ))
    .with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            "Loading",
            TextStyle {
                font: assets.load("fonts/FiraSans-Medium.ttf"),
                font_size: 38.0,
                ..default()
            },
        ));
    });
}

pub(super) fn despawn_loading_screen(
    mut commands: Commands,
    roots: Query<Entity, With<InitialLoadingUiElement>>,
) {
    for entity in roots.iter() {
        commands.entity(entity).despawn_recursive();
    }
}