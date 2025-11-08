use bevy::prelude::*;
use crate::states::GameState;
use crate::core::player::PlayerStats;
use crate::systems::inventory::Inventory;
use bevy::ui::Size;


/// UI plugin: Title + HUD (health left, food right)
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_title_ui)
           .add_systems(Update, title_input_system.run_if(in_state(GameState::Title)))
           .add_systems(OnEnter(GameState::Mission1), spawn_hud)
           .add_systems(Update, hud_fill_update_system.run_if(in_state(GameState::Mission1)));
    }
}

fn spawn_title_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        NodeBundle {
            style: Style { size: Size::width(Val::Percent(100.0)), justify_content: JustifyContent::Center, align_items: AlignItems::Center, ..default() },
            ..default()
        },
        Name::new("TitleRoot"),
    ))
    .with_children(|parent| {
        parent.spawn(TextBundle {
            text: Text::from_section("FRESHMAN ROLL\nPress Enter to Start", TextStyle { font: asset_server.load("fonts/FiraSans-Bold.ttf"), font_size: 48.0, color: Color::WHITE }),
            ..default()
        });
    });
}

fn title_input_system(mut next_state: ResMut<NextState<GameState>>, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::Return) {
        next_state.set(GameState::Mission1);
    }
}

// UI marker components for querying/updating
#[derive(Component)] struct HealthHudTag;
#[derive(Component)] struct FoodHudTag;
#[derive(Component)] struct HealthFill;
#[derive(Component)] struct FoodFill;

/// Spawn HUD nodes (health bar left, food bar right)
fn spawn_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Health bar (left)
    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(12.0),
                top: Val::Px(12.0),
                size: Size::new(Val::Px(220.0), Val::Px(36.0)),
                ..default()
            },
            background_color: BackgroundColor(Color::NONE),
            ..default()
        },
        HealthHudTag,
    ))
    .with_children(|parent| {
        // background
        parent.spawn(NodeBundle {
            style: Style { size: Size::new(Val::Percent(100.0), Val::Percent(100.0)), ..default() },
            background_color: BackgroundColor(Color::rgba(0.07, 0.07, 0.07, 0.8)),
            ..default()
        })
        .with_children(|b| {
            // fill (green) - we will update its width dynamically
            b.spawn((
                NodeBundle {
                    style: Style { size: Size::new(Val::Percent(100.0), Val::Percent(100.0)), ..default() },
                    background_color: BackgroundColor(Color::GREEN),
                    ..default()
                },
                HealthFill,
            ));
        });
    });

    // Food bar (right)
    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                right: Val::Px(12.0),
                top: Val::Px(12.0),
                size: Size::new(Val::Px(160.0), Val::Px(24.0)),
                ..default()
            },
            background_color: BackgroundColor(Color::NONE),
            ..default()
        },
        FoodHudTag,
    ))
    .with_children(|parent| {
        parent.spawn(NodeBundle {
            style: Style { size: Size::new(Val::Percent(100.0), Val::Percent(100.0)), ..default() },
            background_color: BackgroundColor(Color::rgba(0.07, 0.07, 0.07, 0.8)),
            ..default()
        })
        .with_children(|b| {
            b.spawn((
                NodeBundle {
                    style: Style { size: Size::new(Val::Percent(100.0), Val::Percent(100.0)), ..default() },
                    background_color: BackgroundColor(Color::rgb(0.95, 0.6, 0.2)),
                    ..default()
                },
                FoodFill,
            ));
        });
    });
}

/// Update the health/food fill widths using the PlayerStats and Inventory
fn hud_fill_update_system(
    stats: Option<Res<PlayerStats>>,
    inv: Option<Res<Inventory>>,
    mut health_query: Query<&mut Style, With<HealthFill>>,
    mut food_query: Query<&mut Style, With<FoodFill>>,
) {
    let (health_pct, food_pct) = if let Some(s) = stats {
        let s = s.clone();
        let hp = if s.max_health > 0.0 { (s.health / s.max_health).clamp(0.0, 1.0) * 100.0 } else { 0.0 };
        let fp = (s.hunger / s.max_hunger).clamp(0.0, 1.0) * 100.0;
        (hp, fp)
    } else {
        (100.0, 100.0)
    };

    for mut style in &mut health_query {
        style.size.width = Val::Percent(health_pct);
    }
    for mut style in &mut food_query {
        style.size.width = Val::Percent(food_pct);
    }
}
