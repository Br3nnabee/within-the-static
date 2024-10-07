use crate::utils::log;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub struct Render {}

impl Plugin for Render {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_render);
        app.add_plugins(WorldInspectorPlugin::new());
    }
}

#[derive(Component)]
struct CameraMarker;

fn setup_render(mut commands: Commands) {
    log("INFO", "Render plugin successfully added.");

    commands.spawn((Camera2dBundle { ..default() }, CameraMarker));
}
