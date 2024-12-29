use bevy::{a11y::AccessibilityPlugin, prelude::*};
// Usecase here would be using MinimalPlugins and adding only needed plugins
fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins.build().disable::<AccessibilityPlugin>())
        .run()
}
