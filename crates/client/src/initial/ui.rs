use shared::bevy::prelude::*;
use shared::bevy_ecs;
use shared::progress::OverallProgress;
use super::InitialLoading;

#[derive(Component)]
pub(super) struct InitialLoadingUiElement;

#[derive(Component)]
pub(super) struct InitialLoadingUiBar;

#[derive(Component)]
pub(super) struct InitialLoadingInfoText;

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
        // Loading text
        parent.spawn(TextBundle::from_section(
            "Loading",
            TextStyle {
                font: assets.load("fonts/FiraSans-Medium.ttf"),
                font_size: 38.0,
                ..default()
            },
        ));

        // Loading bar back
        parent.spawn(NodeBundle {
            background_color: BackgroundColor(Color::GRAY),
            style: Style {
                margin: UiRect {
                    top: Val::Px(12.0),
                    bottom: Val::Px(4.0),
                    ..default()
                },
                height: Val::Px(10.0),
                width: Val::Px(160.0),
                ..default()
            },
            ..default()
        }).with_children(|parent| {
            // Loading bar value
            parent.spawn((
                InitialLoadingUiBar,
                NodeBundle {
                    background_color: BackgroundColor(Color::LIME_GREEN),
                    style: Style {
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    ..default()
                },
            ));
        });

        // Loading info text
        parent.spawn((
            InitialLoadingInfoText,
            TextBundle::from_section(
                "",
                TextStyle {
                    font: assets.load("fonts/FiraSans-Medium.ttf"),
                    font_size: 16.0,
                    ..default()
                },
            ),
        ));
    });
}

pub(super) fn update_loading_screen(
    progress: Res<OverallProgress<InitialLoading>>,
    mut bar: Query<&mut Style, With<InitialLoadingUiBar>>,
    mut txt: Query<&mut Text, With<InitialLoadingInfoText>>,
) {
    let per = (progress.done() as f32 / progress.required() as f32) * 100.0;
    bar.single_mut().width = Val::Percent(per);
    let text = &mut txt.single_mut().sections[0].value;
    text.clear();
    text.push_str(&format!("{} / {}", progress.done(), progress.required()));
}

pub(super) fn despawn_loading_screen(
    mut commands: Commands,
    roots: Query<Entity, With<InitialLoadingUiElement>>,
) {
    for entity in roots.iter() {
        commands.entity(entity).despawn_recursive();
    }
}