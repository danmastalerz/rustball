use bevy::math::vec3;
use bevy::prelude::*;

// Assets
const PLAYER_RED_SPRITE: &str = "player_red.png";
const PLAYER_BLUE_SPRITE: &str = "player_blue.png";
const BALL_SPRITE: &str = "ball.png";
const PITCH1_SPRITE: &str = "pitch1.png";

// Constants
const SCALE : f32 = 0.5;
const MAX_SPEED: f32 = 3.0;
const PLAYER_RADIUS: f32 = 25.0;
const BALL_RADIUS: f32 = 10.0;
const BALL_MASS: f32 = 1.0;
const PLAYER_MASS: f32 = 2.0;

// Components
#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
struct PlayerRed {
    pub score: i32,
}

#[derive(Component)]
struct PlayerBlue {
    pub score: i32,
}

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Radius(f32);





fn main() {
    App::new()
        // Set background color to green.
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "RustBall".to_string(),
            width: 1024.0,
            height: 768.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(init_game_system)
        .add_startup_system(spawn_players_system)
        .add_startup_system(spawn_ball_system)
        .add_system(player_red_keyboard_system)
        .add_system(player_blue_keyboard_system)
        .add_system(movement_system)
        .add_system(collision_system_red)
        .add_system(collision_system_blue)
        .add_system(players_collision_system)
        .add_system(control_ball_velocity)
        .add_system(edge_collision_system)
        .run();
}

fn init_game_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut windows: ResMut<Windows>,
) {
    // Init camera.
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // // Textures
    // let game_textures = GameTextures {
    //     player_red: asset_server.load(PLAYER_RED_SPRITE),
    //     player_blue: asset_server.load(PLAYER_BLUE_SPRITE),
    //     ball: asset_server.load(BALL_SPRITE),
    // };
    //
    // commands.insert_resource(game_textures);

    // Set background as PITCH1_SPRITE
    let pitch1 = asset_server.load(PITCH1_SPRITE);
    commands.spawn_bundle(SpriteBundle {
        texture: pitch1,
        transform: Transform {
            translation: vec3(0.0, 0.0, 1.0),
            ..Default::default()
        },


        ..Default::default()
    });

}

fn spawn_players_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut windows: ResMut<Windows>,
) {
    // Spawn red circle that'll be representing first player.
    commands.spawn_bundle((SpriteBundle {
        texture: asset_server.load(PLAYER_RED_SPRITE),
        //Move the player to the left side.
        transform: Transform::from_translation(Vec3::new(-200.0, 0.0, 5.0)),

        ..Default::default()
    })).insert(PlayerRed {
        score: 0,

    }).insert(Velocity {
        x: 0.0,
        y: 0.0,
    }).insert(Radius(PLAYER_RADIUS));

    // Spawn blue circle that'll be representing second player.
    commands.spawn_bundle((SpriteBundle {
        texture: asset_server.load(PLAYER_BLUE_SPRITE),
        //Move the player to the right side.
        transform: Transform::from_translation(Vec3::new(200.0, 0.0, 5.0)),
        ..Default::default()
    })).insert(PlayerBlue {
        score: 0,
    }).insert(Velocity {
        x: 0.0,
        y: 0.0,
    }).insert(Radius(PLAYER_RADIUS));
}

fn spawn_ball_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut windows: ResMut<Windows>,
) {
    // Spawn ball.
    commands.spawn_bundle((SpriteBundle {
        texture: asset_server.load(BALL_SPRITE),
        //Move the ball to the center of the screen.
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 5.0)),
        ..Default::default()
    })).insert(Ball).insert(Velocity {
        x: 0.0,
        y: 0.0,
    }).insert(Radius(BALL_RADIUS));
}

fn control_ball_velocity(
    mut query: Query<&mut Velocity, With<Ball>>,
)
{

    // Get ball velocity.
    let mut velocity = query.iter_mut().next().unwrap();

    if velocity.x > 0. {
        velocity.x -= 0.05;
        velocity.x = velocity.x.max(-MAX_SPEED);
    } else if velocity.x < 0. {
        velocity.x += 0.05;
        velocity.x = velocity.x.min(MAX_SPEED);
    };


    if velocity.y > 0. {
        velocity.y -= 0.05;
        velocity.y = velocity.y.max(-MAX_SPEED);
    } else if velocity.y < 0. {
        velocity.y += 0.05;
        velocity.y = velocity.y.min(MAX_SPEED);
    };
}

fn player_red_keyboard_system(
    kb: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<PlayerRed>>,
) {
    for mut velocity in query.iter_mut() {
        if kb.pressed(KeyCode::W) {
            velocity.y += 0.1;
            velocity.y = velocity.y.min(MAX_SPEED);
        } else if kb.pressed(KeyCode::S) {
            velocity.y -= 0.1;
            velocity.y = velocity.y.max(-MAX_SPEED);
        }
        else {
            if velocity.y > 0. {
                velocity.y -= 0.05;
                velocity.y = velocity.y.max(-MAX_SPEED);
            } else if velocity.y < 0. {
                velocity.y += 0.05;
                velocity.y = velocity.y.min(MAX_SPEED);
            }
        }

        if kb.pressed(KeyCode::A) {
            velocity.x -= 0.1;
            velocity.x = velocity.x.max(-MAX_SPEED);
        } else if kb.pressed(KeyCode::D) {
            velocity.x += 0.1;
            velocity.x = velocity.x.min(MAX_SPEED);
        }
        else {
            if velocity.x > 0. {
                velocity.x -= 0.05;
                velocity.x = velocity.x.max(-MAX_SPEED);
            } else if velocity.x < 0. {
                velocity.x += 0.05;
                velocity.x = velocity.x.min(MAX_SPEED);
            }
        }


    }
}

fn player_blue_keyboard_system(
    kb: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<PlayerBlue>>,
) {
    for mut velocity in query.iter_mut() {
        if kb.pressed(KeyCode::Up) {
            velocity.y += 0.1;
            velocity.y = velocity.y.min(MAX_SPEED);
        } else if kb.pressed(KeyCode::Down) {
            velocity.y -= 0.1;
            velocity.y = velocity.y.max(-MAX_SPEED);
        }
        else {
            if velocity.y > 0. {
                velocity.y -= 0.05;
                velocity.y = velocity.y.max(-MAX_SPEED);
            } else if velocity.y < 0. {
                velocity.y += 0.05;
                velocity.y = velocity.y.min(MAX_SPEED);
            }
        }

        if kb.pressed(KeyCode::Left) {
            velocity.x -= 0.1;
            velocity.x = velocity.x.max(-MAX_SPEED);
        } else if kb.pressed(KeyCode::Right) {
            velocity.x += 0.1;
            velocity.x = velocity.x.min(MAX_SPEED);
        }
        else {
            if velocity.x > 0. {
                velocity.x -= 0.05;
                velocity.x = velocity.x.max(-MAX_SPEED);
            } else if velocity.x < 0. {
                velocity.x += 0.05;
                velocity.x = velocity.x.min(MAX_SPEED);
            }
        }
    }
}

fn movement_system(
    mut commands: Commands,
    mut query: Query<(Entity, &Velocity, &mut Transform)>
) {
    for (entity, velocity, mut transform) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += velocity.x;
        translation.y += velocity.y;
    };
}

fn collision_system_red(
    mut query_red: Query<(Entity, &mut Velocity, &Transform, &PlayerRed), Without<Ball>>,
    mut query_ball: Query<(Entity, &mut Velocity, &Transform, &Ball, Without<PlayerRed>)>,
    kb: Res<Input<KeyCode>>,
) {
    let (entity_red, mut velocity_red, transform_red, _) = query_red.iter_mut().next().unwrap();
    let (entity_ball, mut velocity_ball, transform_ball, _, _) = query_ball.iter_mut().next().unwrap();

    // If player red and ball collide
    let player_ball_distance = transform_red.translation.distance(transform_ball.translation);
    if player_ball_distance <= PLAYER_RADIUS + BALL_RADIUS {
        println!("Collision");
        let a = velocity_red.x;
        let b = velocity_ball.x;

        velocity_red.x = (a + 2. * b) / 3.0;
        velocity_ball.x = (4. * a - b) / 3.0;

        let a = velocity_red.y;
        let b = velocity_ball.y;

        velocity_red.y = (a + 2. * b) / 3.0;
        velocity_ball.y = (4. * a - b) / 3.0;

        // If space pressed, shoot the ball
        if kb.pressed(KeyCode::Space) {
            println!("Shoot");
            velocity_ball.x += 5.0;
            velocity_ball.y += 5.0;
        }

    }
}

fn collision_system_blue(
    mut query_blue: Query<(Entity, &mut Velocity, &Transform, &PlayerBlue), Without<Ball>>,
    mut query_ball: Query<(Entity, &mut Velocity, &Transform, &Ball, Without<PlayerBlue>)>,
    kb: Res<Input<KeyCode>>,
) {
    let (entity_blue, mut velocity_blue, transform_blue, _) = query_blue.iter_mut().next().unwrap();
    let (entity_ball, mut velocity_ball, transform_ball, _, _) = query_ball.iter_mut().next().unwrap();

    // If player blue and ball collide
    let player_ball_distance = transform_blue.translation.distance(transform_ball.translation);
    if player_ball_distance <= PLAYER_RADIUS + BALL_RADIUS {
        println!("Collision");
        let a = velocity_blue.x;
        let b = velocity_ball.x;

        velocity_blue.x = (a + 2. * b) / 3.0;
        velocity_ball.x = (4. * a - b) / 3.0;

        let a = velocity_blue.y;
        let b = velocity_ball.y;

        velocity_blue.y = (a + 2. * b) / 3.0;
        velocity_ball.y = (4. * a - b) / 3.0;

        // If right control pressed, shoot the ball
        if kb.pressed(KeyCode::RControl) {
            println!("Shoot");
            velocity_ball.x += 5.0;
            velocity_ball.y += 5.0;
        }

    }
}


fn players_collision_system(
    mut query_red: Query<(Entity, &mut Velocity, &Transform, &PlayerRed), Without<PlayerBlue>>,
    mut query_blue: Query<(Entity, &mut Velocity, &Transform, &PlayerBlue), Without<PlayerRed>>,
) {
    let (entity_red, mut velocity_red, transform_red, _) = query_red.iter_mut().next().unwrap();
    let (entity_blue, mut velocity_blue, transform_blue, _) = query_blue.iter_mut().next().unwrap();

    // If player red and player blue collide
    let players_distance = transform_red.translation.distance(transform_blue.translation);
    if players_distance <= PLAYER_RADIUS * 2.0 {
        //swap velocity
        let temp_velocity = velocity_red.x;
        velocity_red.x = velocity_blue.x;
        velocity_blue.x = temp_velocity;

        let temp_velocity = velocity_red.y;
        velocity_red.y = velocity_blue.y;
        velocity_blue.y = temp_velocity;

    };

}

fn edge_collision_system(
    mut query: Query<(Entity, &mut Velocity, &Transform, &Radius)>
) {

    let (WINDOW_WIDTH, WINDOW_HEIGHT) = (1024., 768.);

    for (entity, mut velocity, transform, radius) in query.iter_mut() {
        let translation = transform.translation.clone();
        let radius = radius.0;

        if translation.x + radius >= WINDOW_WIDTH / 2. {
            velocity.x = -velocity.x;
        } else if translation.x - radius <= -WINDOW_WIDTH / 2. {
            velocity.x = -velocity.x;
        }

        if translation.y + radius >= WINDOW_HEIGHT / 2. {
            velocity.y = -velocity.y;
        } else if translation.y - radius <= -WINDOW_HEIGHT / 2. {
            velocity.y = -velocity.y;
        }
    }
}

fn goal_system(
    mut query_ball: Query<(Entity, &mut Velocity, &Transform, &Ball, &mut ScoreRed, &mut ScoreBlue)>,
) {

}