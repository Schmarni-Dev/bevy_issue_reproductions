use bevy::{pbr::PbrPlugin, prelude::*};
fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins.build().set(PbrPlugin {
            add_default_deferred_lighting_plugin: false,
            ..Default::default()
        }))
        .run()
}
