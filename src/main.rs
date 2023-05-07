use bevy::{app::PluginGroup, prelude::App, DefaultPlugins};
use mappi::MappiPlugins;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.build().disable::<bevy::log::LogPlugin>())
        .add_plugins(MappiPlugins);

    #[cfg(feature = "debug")]
    app.add_plugins(mappi::debug::MappiDebugPlugins);

    app.run();
}
