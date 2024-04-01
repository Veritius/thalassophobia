use shared::bevy::prelude::*;
use shared::progress::*;

pub(super) fn dummy_progress_tracker(
    time: Res<Time<Real>>,
) -> Progress {
    const UPPER: u32 = 500;
    let ts = (time.elapsed_seconds() * 1000.0) as u32;
    Progress { done: ts.min(UPPER), required: UPPER }
}