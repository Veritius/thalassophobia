use bevy::prelude::*;
use bevy_egui::{egui::{self, Align2}, EguiContexts};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

#[derive(Debug, Resource, Reflect)]
pub struct DeveloperMenu {
    pub visible: bool,

    pub inspector_enabled: bool,
}

pub(super) fn setup_dev_menu(app: &mut App) {
    // Add developer menu resource
    app.insert_resource(DeveloperMenu {
        visible: true, // TODO: Allow changing this

        inspector_enabled: false,
    });

    // Add world inspector plugin
    app.add_plugins(
        WorldInspectorPlugin::new()
        .run_if(IntoSystem::into_system(|menu: Res<DeveloperMenu>| {
            menu.visible && menu.inspector_enabled
        }))
    );

    // Add system
    app.add_systems(Update, developer_menu_system);
}

fn developer_menu_system(
    mut menu: ResMut<DeveloperMenu>,
    mut ctx: EguiContexts,
) {
    if !menu.visible { return }
    let ctx = ctx.ctx_mut();

    egui::Window::new("Developer Menu")
    .anchor(Align2::CENTER_TOP, egui::Vec2::new(0.0, 5.0))
    .title_bar(false)
    .movable(false)
    .resizable(false)
    .show(ctx, |ui| {
        ui.horizontal(|ui| {
            if ui.button("👁 Inspector").clicked() {
                menu.inspector_enabled = !menu.inspector_enabled;
            }

            // Buttons for menus that don't exist yet
            ui.add_enabled_ui(false, |ui| ui.button("✏ Spawner"));
        });
    });
}