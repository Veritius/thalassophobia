use shared::bevy::prelude::*;
use super::MainMenuRoot;

pub(super) fn build_main_menu_system(
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    // Spawn root node
    commands.spawn((
        MainMenuRoot, // marker for easy despawning
        NodeBundle {
            background_color: BackgroundColor(
                Color::DARK_GRAY,
            ),
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        },
    )).with_children(|parent| {
        // Camera to view the UI
        parent.spawn(Camera2dBundle::default());

        // UI panel
        parent.spawn(NodeBundle {
            background_color: BackgroundColor(
                Color::GRAY
            ),
            style: Style {
                flex_direction: FlexDirection::Column,
                border: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            ..default()
        }).with_children(|parent| {
            // Game title
            parent.spawn(TextBundle::from_section(
                "Thalassophobia",
                TextStyle {
                    font: assets.load("fonts/FiraSans-Medium.ttf"),
                    font_size: 30.0,
                    ..default()
                }
            ));

            // Buttons container
            parent.spawn(NodeBundle {
                style: Style {
                    border: UiRect {
                        top: Val::Px(8.0),
                        bottom: Val::Px(8.0),
                        ..default()
                    },
                    ..default()
                },
                ..default()
            }).with_children(|parent| {
                
            });

            // Game version
            parent.spawn(TextBundle::from_section(
                format!("version {}", std::env::var("CARGO_PKG_VERSION")
                    .unwrap_or("(unknown)".to_string())),
                TextStyle {
                    font: assets.load("fonts/FiraSans-Medium.ttf"),
                    font_size: 18.0,
                    ..default()
                }
            ));
        });
    });
}