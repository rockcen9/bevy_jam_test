use std::time::Duration;

use bevy::{app::ScheduleRunnerPlugin, prelude::*};
use bevy_seedling::SeedlingPlugin;
fn main() {
    //
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);
    #[cfg(feature = "native")]
    app.add_plugins(SeedlingPlugin::default());
    #[cfg(feature = "web")]
    app.add_plugins(SeedlingPlugin::new_web_audio());
    app.run();
}
