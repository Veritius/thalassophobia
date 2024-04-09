use shared::{bevy::{prelude::*, window::{CursorGrabMode, PrimaryWindow}}, bevy_ecs, input::prelude::*, player::movement::*};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, States)]
pub enum Escaped {
    Yes,
    No,
}

pub(crate) struct EscapeMenuPlugin;

impl Plugin for EscapeMenuPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(Escaped::No);

        app.add_systems(Update, toggle_escape);
        app.add_systems(OnEnter(Escaped::Yes), on_escape);
        app.add_systems(OnEnter(Escaped::No), on_reenter);

        #[cfg(target_os="windows")]
        app.add_systems(Update, cursor_stealer_system
            .run_if(in_state(Escaped::No)));
    }
}

fn toggle_escape(
    current_state: Res<State<Escaped>>,
    mut next_state: ResMut<NextState<Escaped>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        match current_state.get() {
            Escaped::Yes => next_state.set(Escaped::No),
            Escaped::No => next_state.set(Escaped::Yes),
        }
    }
}

#[derive(Component)]
struct EscapeMenuUiRoot;

fn on_escape(
    mut commands: Commands,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
    assets: Res<AssetServer>,
) {
    // Configure the cursor
    let mut primary_window = window_query.single_mut();
    primary_window.cursor.grab_mode = CursorGrabMode::None;
    primary_window.cursor.visible = true;

    // Disable movement inputs
    commands.insert_resource(ToggleActions::<GroundedHumanMovements>::DISABLED);
    commands.insert_resource(ToggleActions::<FloatingHumanMovements>::DISABLED);

    // Spawn some text saying the game is paused
    let font_handle = assets.load("fonts/FiraSans-Medium.ttf");
    commands.spawn((EscapeMenuUiRoot, NodeBundle {
        background_color: BackgroundColor(Color::DARK_GRAY),
        style: Style {
            justify_self: JustifySelf::Center,
            justify_items: JustifyItems::Center,
            align_self: AlignSelf::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            padding: UiRect::axes(Val::Px(20.0), Val::Px(10.0)),
            ..default()
        },
        ..default()
    })).with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            "Paused",
            TextStyle {
                font: font_handle.clone(),
                font_size: 30.0,
                ..default()
            }),
        );

        parent.spawn(TextBundle::from_section(
            "Press Esc to unpause",
            TextStyle {
                font: font_handle.clone(),
                font_size: 16.0,
                ..default()
            }),
        );
    });
}

fn on_reenter(
    mut commands: Commands,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
    ui_root_query: Query<Entity, With<EscapeMenuUiRoot>>,
) {
    // Configure the cursor
    let mut primary_window = window_query.single_mut();
    primary_window.cursor.grab_mode = CursorGrabMode::Locked;
    primary_window.cursor.visible = false;

    // Enable movement inputs
    commands.insert_resource(ToggleActions::<GroundedHumanMovements>::ENABLED);
    commands.insert_resource(ToggleActions::<FloatingHumanMovements>::ENABLED);

    // Despawn the escape menu ui
    if let Ok(entity) = ui_root_query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}

// A system to reset the position of the cursor every frame.
// This is necessary since Windows doesn't support Locked, and Bevy falls back to Confined.
#[cfg(target_os = "windows")]
fn cursor_stealer_system(
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut primary_window = window_query.single_mut();
    let center = Vec2::new(
        primary_window.width() / 2.0,
        primary_window.height() / 2.0,
    );
    primary_window.set_cursor_position(Some(center));
}