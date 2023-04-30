use bevy::prelude::*;
use bevy_animation_graph::*;
use std::collections::HashMap;
use std::time::Duration;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(AnimationPlugin)
        .add_startup_system(setup)
        .add_system(accept_player_input)
        .run();
}

fn setup(
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut commands: Commands,
) {
    let texture_handle = asset_server.load("sprite.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(256.0, 256.0), 5, 5, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let idle = AnimationData {
        animation: Animation {
            texture: texture_atlas_handle.clone(),
            first_frame: 0,
            last_frame: 6,
            frame_duration: Duration::from_millis(100),
        },
        has_exit_time: false,
    };
    let forward = AnimationData {
        animation: Animation {
            texture: texture_atlas_handle.clone(),
            first_frame: 11,
            last_frame: 16,
            frame_duration: Duration::from_millis(100),
        },
        has_exit_time: false,
    };
    let mut nodes: HashMap<String, GraphNode<AnimationData>> = HashMap::new();
    nodes.insert(
        "Idle".to_string(),
        GraphNode {
            data: idle,
            transitions: vec![Transition {
                target: "Forward".to_string(),
                conditions: vec![bevy_animation_graph::Condition::Gt(
                    "Movement".to_string(),
                    0.0,
                )],
            }],
        },
    );
    nodes.insert(
        "Forward".to_string(),
        GraphNode {
            data: forward,
            transitions: vec![Transition {
                target: "Idle".to_string(),
                conditions: vec![bevy_animation_graph::Condition::Eq(
                    "Movement".to_string(),
                    0.0,
                )],
            }],
        },
    );
    let state_graph = StateGraph::new("Idle".to_string(), nodes);
    let animator = Animator::new(state_graph);
    let sprite_sheet_bundle = SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        sprite: TextureAtlasSprite::new(0),
        transform: Transform::from_scale(Vec3::new(3.0, 3.0, 3.0)),
        ..default()
    };
    commands.spawn(Camera2dBundle::default());
    commands.spawn(sprite_sheet_bundle).insert(animator);
}

fn accept_player_input(keys: Res<Input<KeyCode>>, mut query: Query<&mut Animator>) {
    if let Ok(mut animator) = query.get_single_mut() {
        if keys.pressed(KeyCode::Space) {
            animator.state_graph.set_float("Movement".to_string(), 1.0);
        } else {
            animator.state_graph.set_float("Movement".to_string(), 0.0);
        }
    }
}
