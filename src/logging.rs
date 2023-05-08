/// Most of this code is copied straight from the official bevy_log plugin
use std::panic;

use bevy::prelude::{App, Plugin};
pub use bevy::utils::tracing::{
    debug, debug_span, error, error_span, info, info_span, trace, trace_span, warn, warn_span,
    Level,
};
use tracing_log::LogTracer;
use tracing_subscriber::{prelude::*, EnvFilter, Registry};

pub struct LogPlugin;

impl Plugin for LogPlugin {
    fn build(&self, _app: &mut App) {
        let old_handler = panic::take_hook();
        panic::set_hook(Box::new(move |infos| {
            println!("{}", tracing_error::SpanTrace::capture());
            old_handler(infos);
        }));

        let default_filter = "info,wgpu=error".to_string();
        let filter_layer = EnvFilter::try_from_env("MAPPI_LOG")
            .or_else(|_| EnvFilter::try_new(&default_filter))
            .unwrap();

        // In practice, there should never be another bevy plugin/thread/whatever writing to env during this step, so this "should" be "safe"
        unsafe {
            time::util::local_offset::set_soundness(time::util::local_offset::Soundness::Unsound);
        }

        #[cfg(not(miri))]
        let now = time::OffsetDateTime::now_local().expect("Failed to get local time offset!");
        #[cfg(miri)]
        let now = {
            // now_local always returns None with miri
            time::OffsetDateTime::now_utc();
        };

        // Safety: Setting to sound is always safe
        unsafe {
            time::util::local_offset::set_soundness(time::util::local_offset::Soundness::Sound);
        }

        let dir = dirs::cache_dir()
            .expect("Failed to find cache dir!")
            .join("mappi/logs");

        std::fs::create_dir_all(dir.clone())
            .unwrap_or_else(|_| panic!("Failed to create path: '{}'", dir.display()));

        let log_file = std::fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(dir.join(format!(
                // "{year}-{month}-{day}-{hour}-{minute}-{second}.log",
                "{year}-{month}-{day}.log",
                year = now.year(),
                month = u8::from(now.month()),
                day = now.day(),
                // hour = now.to_hms().0,
                // minute = now.to_hms().1,
                // second = now.to_hms().2
            )))
            .expect("Failed to open log file!");

        let subscriber = Registry::default()
            .with(filter_layer)
            .with(tracing_error::ErrorLayer::default())
            .with(tracing_subscriber::fmt::Layer::default().with_writer(log_file));

        let logger_already_set = LogTracer::init().is_err();
        let subscriber_already_set =
            bevy::utils::tracing::subscriber::set_global_default(subscriber).is_err();

        match (logger_already_set, subscriber_already_set) {
            (true, true) => warn!(
                "Could not set global logger and tracing subscriber as they are already set. Consider disabling LogPlugin."
            ),
            (true, _) => warn!("Could not set global logger as it is already set. Consider disabling LogPlugin."),
            (_, true) => warn!("Could not set global tracing subscriber as it is already set. Consider disabling LogPlugin."),
            _ => (),
        }
    }
}
