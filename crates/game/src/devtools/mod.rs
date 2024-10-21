use bevy::{prelude::*, window::PrimaryWindow};
use bevy_egui::{egui, EguiContext};
use egui_dock::{DockArea, DockState, NodeIndex, Style, SurfaceIndex, TabViewer};
use crate::initialisation::Initialisation;

pub struct DevtoolsPlugin;

impl Plugin for DevtoolsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy_egui::EguiPlugin);

        app.insert_resource(DevtoolsDockState {
            state: DockState::new(vec![]),
        });

        app.add_systems(Update, devtools_viewer_system
            .run_if(in_state(Initialisation::Finished)));
    }
}

fn devtools_viewer_system(
    world: &mut World,
) {
    // Try to access the egui context for the primary window
    // This early returns if it's not accessible (no window)
    let mut ctx = match world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .get_single(world) {
            Ok(v) => v.clone(),
            Err(_) => return,
        };

    // Remove the state from the world so it doesn't interfere with our other accesses
    let mut state = world.remove_resource::<DevtoolsDockState>().unwrap();

    // Construct the tab viewer thingy
    let mut viewer = DevtoolsTabViewer {
        world,
        added: Vec::new(),
    };

    // Draw the docks and stuff
    DockArea::new(&mut state.state)
        .style(Style::from_egui(ctx.get_mut().style().as_ref()))
        .show_add_buttons(true)
        .show_add_popup(true)
        .show(ctx.get_mut(), &mut viewer);

    // Drain the added tabs set and add it to the state
    viewer.added.drain(..).for_each(|(surface, node, object)| {
        state.state.set_focused_node_and_surface((surface, node));
        state.state.push_to_focused_leaf(object);
    });

    // Put the state back into the world
    world.insert_resource(state);
}

struct DevtoolsTabViewer<'a> {
    world: &'a mut World,
    added: Vec<(SurfaceIndex, NodeIndex, DevtoolsTab)>,
}

impl<'a> TabViewer for DevtoolsTabViewer<'a> {
    type Tab = DevtoolsTab;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        tab.state.title()
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        tab.state.ui(&mut self.world, ui);
    }
}

#[derive(Resource)]
struct DevtoolsDockState {
    state: DockState<DevtoolsTab>,
}

struct DevtoolsTab {
    state: Box<dyn DevtoolsWidget>,
}

impl<S> From<S> for DevtoolsTab
where
    S: DevtoolsWidget,
{
    fn from(value: S) -> Self {
        Self {
            state: Box::from(value),
        }
    }
}

trait DevtoolsWidget
where
    Self: Send + Sync + 'static,
{
    fn title(
        &mut self,
    ) -> egui::WidgetText;

    fn ui(
        &mut self,
        world: &mut World,
        ui: &mut egui::Ui,
    );
}