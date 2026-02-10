use bevy::prelude::*;
use bevy_seedling::SeedlingPlugin;
fn main() {
    //
    App::new()
        .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(
            // Run 60 times per second.
            Duration::from_secs_f64(1.0 / 60.0),
        )))
        .add_plugins(SeedlingPlugin)
        .run();
}
