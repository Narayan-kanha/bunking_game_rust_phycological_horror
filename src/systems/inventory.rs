use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum ItemType {
    Torch,
    Shovel,
    Hoe,
    Dirt,
    Food,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemStack {
    pub item: ItemType,
    pub count: u32,
    pub durability: Option<i32>,
}

#[derive(Resource, Serialize, Deserialize, Default)]
pub struct Inventory {
    pub slots: HashMap<ItemType, ItemStack>,
    pub equipped: Option<ItemType>,
    pub mission_progress: u8,
}

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Inventory::default())
           .add_systems(OnEnter(crate::states::GameState::Mission1), give_starter_items)
           .add_systems(Update, pickup_food_system.run_if(in_state(crate::states::GameState::Mission1)));
    }
}

fn give_starter_items(mut inv: ResMut<Inventory>) {
    if inv.slots.is_empty() {
        inv.slots.insert(ItemType::Torch, ItemStack { item: ItemType::Torch, count: 1, durability: Some(100) });
        inv.slots.insert(ItemType::Shovel, ItemStack { item: ItemType::Shovel, count: 1, durability: Some(40) });
        inv.slots.insert(ItemType::Hoe, ItemStack { item: ItemType::Hoe, count: 1, durability: Some(40) });
    }
}

#[derive(Component)]
pub struct FoodPickup;

fn pickup_food_system(
    mut commands: Commands,
    player_q: Query<&Transform, With<crate::core::player::Ethan>>,
    pickup_q: Query<(Entity, &Transform), With<FoodPickup>>,
    mut inv: ResMut<Inventory>,
) {
    let Ok(player_tf) = player_q.get_single() else { return };

    for (e, tf) in &pickup_q {
        if tf.translation.distance(player_tf.translation) < 1.2 {
            inv.slots.entry(ItemType::Food)
                .and_modify(|s| s.count += 1)
                .or_insert(ItemStack { item: ItemType::Food, count: 1, durability: None });
            commands.entity(e).despawn_recursive();
            info!("Picked up food.");
        }
    }
}
