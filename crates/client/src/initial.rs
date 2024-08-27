use shared::bevy::color::palettes::tailwind::{LIME_600, NEUTRAL_400, NEUTRAL_700};
use shared::initial::Initialisation;
use shared::prelude::*;

pub(crate) struct InitialisationUiPlugin;

impl Plugin for InitialisationUiPlugin {
    fn build(&self, app: &mut App) {
        // Start of tracking
        app.add_systems(OnEnter(Initialisation::Loading), (
            spawn_loading_screen,
        ));

        // Tracking update
        app.add_systems(Update, (
            update_loading_screen,
        ).run_if(in_state(Initialisation::Loading)));

        // End of tracking
        app.add_systems(OnExit(Initialisation::Loading), (
            despawn_loading_screen,
        ));
    }
}

#[derive(Component)]
struct InitialLoadingUiElement;

#[derive(Component)]
struct InitialLoadingUiBar;

#[derive(Component)]
struct InitialLoadingInfoText;

fn spawn_loading_screen(
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
            background_color: BackgroundColor::from(NEUTRAL_700),
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
        // Container for the loading data
        parent.spawn(NodeBundle {
            style: Style {
                margin: UiRect::all(Val::Px(6.0)),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        }).with_children(|parent| {
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
                background_color: BackgroundColor::from(NEUTRAL_400),
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
                        background_color: BackgroundColor::from(LIME_600),
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

        // Extra section for question buttons
        parent.spawn(NodeBundle {
            style: Style {
                margin: UiRect::all(Val::Px(6.0)),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        });
    });
}

fn update_loading_screen(
    progress: Res<Progress<Initialisation>>,
    mut bar: Query<&mut Style, With<InitialLoadingUiBar>>,
    mut txt: Query<&mut Text, With<InitialLoadingInfoText>>,
) {
    // Update bars
    let percent = progress.fract() * 100.0;
    for mut bar in bar.iter_mut() {
        bar.width = Val::Percent(percent);
    }

    // Update text
    let (done, required) = progress.work();
    let text = format!("{done} / {required}");
    for mut txt in txt.iter_mut() {
        let t = &mut txt.sections[0].value;
        t.clear(); t.push_str(&text);
    }
}

fn despawn_loading_screen(
    mut commands: Commands,
    roots: Query<Entity, With<InitialLoadingUiElement>>,
) {
    for entity in roots.iter() {
        commands.entity(entity).despawn_recursive();
    }
}