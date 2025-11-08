use bevy::prelude::*;
use rand::prelude::*;
use crate::config::SIMULATION_RADIUS;
use crate::core::player::Ethan;

/// An NPC category
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NpcCategory {
    Civilian,
    Trader,
    Student,
}

/// The NPC component
#[derive(Component)]
pub struct Npc {
    pub category: NpcCategory,
    pub state: NpcState,
    pub timer: f32, // action timer for state changes
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NpcState {
    Idle,
    Walking,
    Interacting,
}

pub struct NpcAiPlugin;

impl Plugin for NpcAiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, npc_ai_system.run_if(in_state(crate::states::GameState::OpenWorld)));
    }
}

/// Simple wandering AI:
/// - Idle: wait a bit then decide to walk or interact
/// - Walking: move toward a random nearby target (uses simple position translation)
/// - Interacting: wait (simulate trading/chatting), then go Idle
fn npc_ai_system(
    time: Res<Time>,
    mut rng: Local<SmallRng>,
    player_q: Query<&Transform, With<Ethan>>,
    mut npc_q: Query<(Entity, &mut Npc, &mut Transform)>,
) {
    // If no player present (Mission1 pre-open-world), skip
    let Ok(player_tf) = player_q.get_single() else { return };
    let player_pos = player_tf.translation;

    for (entity, mut npc, mut transform) in &mut npc_q {
        let d = transform.translation.distance(player_pos);
        if d > SIMULATION_RADIUS {
            // skip far-away NPCs for performance
            continue;
        }

        npc.timer -= time.delta_seconds();
        match npc.state {
            NpcState::Idle => {
                if npc.timer <= 0.0 {
                    // choose next action
                    let roll: f32 = rng.gen();
                    if roll < 0.6 {
                        // start walking to a nearby spot
                        npc.state = NpcState::Walking;
                        npc.timer = 2.0 + rng.gen::<f32>() * 4.0;
                        // choose a small random displacement target by storing in translation's rotation (cheap packing)
                        let dx = (rng.gen::<f32>() - 0.5) * 8.0;
                        let dz = (rng.gen::<f32>() - 0.5) * 8.0;
                        transform.translation += Vec3::new(dx, 0.0, dz);
                    } else {
                        // interact briefly (e.g., trade or chat)
                        npc.state = NpcState::Interacting;
                        npc.timer = 1.2 + rng.gen::<f32>() * 2.4;
                    }
                }
            }
            NpcState::Walking => {
                // move forward slowly toward transform.translation's set position (we used transform as target in Idle)
                // For simplicity we move toward the player+offset (we already added offset to translation),
                // so walking is implicit during the idle->walk step above and timer counts down.
                if npc.timer <= 0.0 {
                    npc.state = NpcState::Idle;
                    npc.timer = 0.5 + rng.gen::<f32>() * 2.0;
                } else {
                    // slight bobbing: animate via small vertical sin offset (visual only)
                    let offset = (time.elapsed_seconds() * 6.0).sin() * 0.01;
                    transform.translation.y = offset;
                }
            }
            NpcState::Interacting => {
                if npc.timer <= 0.0 {
                    npc.state = NpcState::Idle;
                    npc.timer = 0.2 + rng.gen::<f32>() * 1.0;
                }
            }
        }
    }
}
