#![deny(unsafe_op_in_unsafe_fn)]
#![deny(clippy::missing_safety_doc)]
#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

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
