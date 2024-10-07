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

#[derive(Component)]
pub struct OnGameScreen;

fn setup_render(mut commands: Commands) {
    info!("Render setup started.");

    commands.spawn((Camera2dBundle { ..default() }, CameraMarker));
}

pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
