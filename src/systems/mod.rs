pub mod inventory;
pub mod savegame;
pub mod performance;
pub mod settings;
pub mod controls;
pub mod ui;
pub mod cars;

pub use inventory::InventoryPlugin;
pub use savegame::SaveGamePlugin;
pub use performance::PerformancePlugin;
pub use settings::SettingsPlugin;
pub use controls::ControlsPlugin;
pub use ui::UiPlugin;
pub use cars::CarsPlugin;
