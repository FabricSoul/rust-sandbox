use bevy::prelude::*;

use iyes_perf_ui::prelude::*;
pub fn ui(mut commands: Commands) {
    // create a simple Perf UI with default settings
    // and all entries provided by the crate:
    commands.spawn(PerfUiCompleteBundle::default());
}
