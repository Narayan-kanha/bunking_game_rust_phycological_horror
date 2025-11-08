use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ItemCategory {
    Food,
    Tool,
    Prank,
    Utility,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ItemDef {
    pub id: &'static str,
    pub name: &'static str,
    pub category: ItemCategory,
    pub stack_size: u32,
    pub description: &'static str,
}

// Example item registry (static list)
pub static ITEM_REGISTRY: &[ItemDef] = &[
    ItemDef { id: "food_burger", name: "Burger", category: ItemCategory::Food, stack_size: 8, description: "Hearty burger. Restores hunger." },
    ItemDef { id: "food_apple", name: "Apple", category: ItemCategory::Food, stack_size: 16, description: "Simple fruit. Small hunger restore." },
    ItemDef { id: "tool_shovel", name: "Shovel", category: ItemCategory::Tool, stack_size: 1, description: "Dig ground to collect dirt." },
    ItemDef { id: "tool_hoe", name: "Hoe", category: ItemCategory::Tool, stack_size: 1, description: "Till grass into dirt." },
    ItemDef { id: "tool_torch", name: "Torch", category: ItemCategory::Tool, stack_size: 1, description: "Lights up dark places." },
    ItemDef { id: "prank_stinkbomb", name: "Stink Bomb", category: ItemCategory::Prank, stack_size: 4, description: "Create chaos briefly." },
    ItemDef { id: "prank_paperplane", name: "Paper Plane", category: ItemCategory::Prank, stack_size: 16, description: "Silly diversion." },
    ItemDef { id: "utility_dirt", name: "Dirt", category: ItemCategory::Utility, stack_size: 64, description: "Collected from grass. Used in crafting." },
];
