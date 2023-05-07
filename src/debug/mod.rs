mod logging;

use bevy::{
    app::PluginGroupBuilder,
    prelude::{App, PluginGroup},
};

pub struct MappiDebugPlugins;

impl PluginGroup for MappiDebugPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
    }
}
