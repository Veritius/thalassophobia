use bevy::diagnostic::DiagnosticsStore;
use super::*;

#[inline(always)]
pub(super) fn setup(app: &mut App) {
    app.observe(observer_system);
}

fn observer_system(
    mut trigger: Trigger<DevtoolLayout>,
    diagnostics: Res<DiagnosticsStore>,
) {
    let ui = &mut trigger.event_mut().ui;

    ui.collapsing("Diagnostics", |ui| {
        egui::Grid::new("devtool_diagnostics")
        .striped(true)
        .show(ui, |ui| {
            let count = diagnostics.iter().count();
            let mut vec = Vec::with_capacity(count);

            for diagnostic in diagnostics.iter() {
                vec.push((diagnostic.path(), diagnostic.value()));
            }

            // Make the set ordered instead of weird and unstable
            vec.sort_unstable_by(|(a,_),(b,_)| a.as_str().cmp(b.as_str()));

            for (name, value) in vec.drain(..) {
                ui.label(name.as_str());
                if let Some(value) = value {
                    const EPSILON_A: f64 = 0.001;
                    const EPSILON_B: f64 = 0.999;

                    let frac = value % 1.0;

                    if frac < EPSILON_A || frac > EPSILON_B {
                        ui.label(format!("{value:.0}"));
                    } else {
                        ui.label(format!("{value:.3}"));
                    }
                }
                ui.end_row();
            }
        });
    });
}