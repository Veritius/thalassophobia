use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::render::RapierDebugRenderPlugin;

/// Various debugging features that can be turned off and on at runtime.
#[derive(Debug, Resource, Reflect)]
pub struct DeveloperMenu {
    pub visible: bool,

    pub inspector_enabled: bool,
    pub network_stats_enabled: bool,
}

pub(super) fn setup_dev_menu(app: &mut App) {
    // Add developer menu resource
    app.insert_resource(DeveloperMenu {
        visible: true, // TODO: Allow changing this

        inspector_enabled: false,
        network_stats_enabled: false,
    });

    // Add world inspector plugin
    app.add_plugins(
        WorldInspectorPlugin::new()
        .run_if(IntoSystem::into_system(|menu: Res<DeveloperMenu>| {
            menu.visible && menu.inspector_enabled
        }))
    );

    // Add debug physics view plugin
    app.add_plugins(
        RapierDebugRenderPlugin::default()
        .disabled()
    );

    // Add system
    app.add_systems(Update, developer_menu_system);
}

fn developer_menu_system(
    mut menu: ResMut<DeveloperMenu>,
    mut ctx: EguiContexts,
    mut phys_ctx: ResMut<bevy_rapier3d::render::DebugRenderContext>,
) {
    if !menu.visible { return }
    let ctx = ctx.ctx_mut();

    egui::TopBottomPanel::top("dev_menu")
    .show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("Debugging", |ui| {
                ui.checkbox(&mut menu.inspector_enabled, "World inspector");
                ui.checkbox(&mut menu.network_stats_enabled, "Network stats");
                ui.checkbox(&mut phys_ctx.enabled, "Physics view");
            });
        })
    });
}