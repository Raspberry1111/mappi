use bevy::{app::PluginGroupBuilder, prelude::PluginGroup};
use bevy_ecs_tilemap::TilemapPlugin;
use logging::LogPlugin;

#[cfg(feature = "debug")]
pub mod debug;
pub mod logging;

pub struct MappiPlugins;

impl PluginGroup for MappiPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(LogPlugin)
            .add(TilemapPlugin)
    }
}
