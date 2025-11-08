use bevy::prelude::*;

/// Handles struct holds optional handles for Kenney kits & GLBs you listed.
#[derive(Resource)]
pub struct Handles {
    pub man: Handle<Scene>,
    pub punk: Handle<Scene>,
    pub man_in_suit: Option<Handle<Scene>>,
    pub animated_woman: Option<Handle<Scene>>,
    pub farmer: Option<Handle<Scene>>,
    pub worker_female: Option<Handle<Scene>>,
    pub worker: Option<Handle<Scene>>,
    pub kenney_car: Option<Handle<Scene>>,
    pub kenney_roads: Option<Handle<Scene>>,
    pub kenney_food: Option<Handle<Scene>>,
    pub kenney_furniture: Option<Handle<Scene>>,
    pub kenney_market: Option<Handle<Scene>>,
    pub kenney_skate: Option<Handle<Scene>>,
    pub dirt_icon: Option<Handle<Image>>,
}

pub fn preload_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    // NOTE: Ensure paths exactly match filenames in assets/ — adjust if your GLBs are named differently.
    let handles = Handles {
        man: asset_server.load("Man.glb#Scene0"),
        punk: asset_server.load("Punk.glb#Scene0"),
        man_in_suit: Some(asset_server.load("Man in Suit.glb#Scene0")),
        animated_woman: Some(asset_server.load("Animated Woman.glb#Scene0")),
        farmer: Some(asset_server.load("Farmer.glb#Scene0")),
        worker_female: Some(asset_server.load("Worker Female.glb#Scene0")),
        worker: Some(asset_server.load("Worker.glb#Scene0")),
        kenney_car: asset_server.get_handle("kenney_car-kit/car.glb#Scene0").ok().or_else(|| asset_server.get_handle("kenney_car-kit/Car.glb#Scene0").ok()),
        kenney_roads: asset_server.get_handle("kenney_city-kit-roads/scene.gltf#Scene0").ok(),
        kenney_food: asset_server.get_handle("kenney_food-kit/scene.gltf#Scene0").ok(),
        kenney_furniture: asset_server.get_handle("kenney_furniture-kit/scene.gltf#Scene0").ok(),
        kenney_market: asset_server.get_handle("kenney_mini-market/scene.gltf#Scene0").ok(),
        kenney_skate: asset_server.get_handle("kenney_mini-skate/scene.gltf#Scene0").ok(),
        dirt_icon: asset_server.get_handle("dirt.png").ok(),
    };

    commands.insert_resource(handles);
    info!("Asset handles preloaded (assets_loader).");
}
