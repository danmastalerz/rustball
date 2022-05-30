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
const CORNER_RADIUS: f32 = 10.0;

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

#[derive(Component)]
pub struct Score {
    pub red: i32,
    pub blue: i32,
}


#[derive(Component)]
struct ScoreText(String);


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
        .add_system(corner_collision_system)
        .add_system(goal_system)
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

    let font = asset_server.load("fonts/FiraSans-Regular.ttf");
    let text_style = TextStyle {
        font,
        font_size: 60.0,
        color: Color::WHITE,
    };
    let text_alignment = TextAlignment {
        vertical: VerticalAlign::Center,
        horizontal: HorizontalAlign::Center,
    };

    // Show score on the screen (on the top left corner)
    let score_text = String::from("Score: 0–0");
    commands
        .spawn_bundle(Text2dBundle {
            text: Text {
                sections: vec![TextSection {
                    value: score_text,
                    style: text_style.clone(),
                    ..Default::default()
                }],
                alignment: text_alignment.clone(),
            },
            transform: Transform::from_translation(vec3(-1024.0 / 2. + 125., -768.2 / 2. + 50., 2.0)),
            global_transform: Default::default(),
            text_2d_size: Default::default(),
            text_2d_bounds: Default::default(),
            visibility: Visibility {
                is_visible: true,
            }
        });

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

    commands.insert_resource(Score {
        red: 0,
        blue: 0,
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

// Inspired with: https://stackoverflow.com/questions/345838/ball-to-ball-collision-detection-and-handling
fn handle_collision(velocity_red : &mut Velocity, velocity_blue : &mut Velocity, transform_red : &mut Transform, transform_blue : &mut Transform, radius1: f32, radius2: f32) {
    let delta = (transform_red.translation - transform_blue.translation).truncate();
    let players_distance = transform_red.translation.distance(transform_blue.translation);
    let d = players_distance.clone();
    let multiplier = (-d + radius1 + radius2) / d;
    let delta_x = delta.x * multiplier;
    let delta_y = delta.y * multiplier;
    let mtd = Vec2::new(delta_x, delta_y);

    let im1 = 1.;
    let im2 = 1.;
    // TODO push-pull them apart
    transform_red.translation.x += mtd[0] * (im1 / (im1 + im2));
    transform_red.translation.y += mtd[1] * (im1 / (im1 + im2));

    transform_blue.translation.x -= mtd[0] * (im2 / (im1 + im2));
    transform_blue.translation.y -= mtd[1] * (im2 / (im1 + im2));

    //impact speed
    let v = Vec2::new(velocity_red.x - velocity_blue.x, velocity_red.y - velocity_blue.y);
    let vn = v.dot(mtd.normalize());



    if vn > 0.0 {
        return;
    }

    // collision impulse
    let i = (-(1.0 + 0.5) * vn) / (im1 + im2);
    let impulse = mtd.normalize() * i;


    //change in momentum
    velocity_red.x += impulse[0] * im1;
    velocity_red.y += impulse[1] * im1;

    velocity_blue.x -= impulse[0] * im2;
    velocity_blue.y -= impulse[1] * im2;


}

fn collision_system_red(
    mut query_red: Query<(Entity, &mut Velocity, &mut Transform, &PlayerRed), Without<Ball>>,
    mut query_ball: Query<(Entity, &mut Velocity, &mut Transform, &Ball, Without<PlayerRed>)>,
    kb: Res<Input<KeyCode>>,
) {
    let (entity_red, mut velocity_red, mut transform_red, _) = query_red.iter_mut().next().unwrap();
    let (entity_ball, mut velocity_ball, mut transform_ball, _, _) = query_ball.iter_mut().next().unwrap();

    // If player red and ball collide
    let player_ball_distance = transform_red.translation.distance(transform_ball.translation);
    if player_ball_distance < PLAYER_RADIUS + BALL_RADIUS {
        // If space pressed, shoot the ball
        if kb.pressed(KeyCode::Space) {
            println!("Shoot");
            let diff_x = transform_red.translation.x - transform_ball.translation.x;
            let diff_y = transform_red.translation.y - transform_ball.translation.y;
            let angle = diff_y.atan2(diff_x);
            println!("{}", angle);
            velocity_ball.y += -5.0 * angle.sin();
            velocity_ball.x += -5.0 * angle.cos();
        }
        handle_collision(&mut velocity_red, &mut velocity_ball, &mut transform_red, &mut transform_ball, PLAYER_RADIUS, BALL_RADIUS);
    }

}

fn collision_system_blue(
    mut query_blue: Query<(Entity, &mut Velocity, &mut Transform, &PlayerBlue), Without<Ball>>,
    mut query_ball: Query<(Entity, &mut Velocity, &mut Transform, &Ball, Without<PlayerBlue>)>,
    kb: Res<Input<KeyCode>>,
) {
    let (entity_blue, mut velocity_blue, mut transform_blue, _) = query_blue.iter_mut().next().unwrap();
    let (entity_ball, mut velocity_ball, mut transform_ball, _, _) = query_ball.iter_mut().next().unwrap();

    // If player blue and ball collide
    let player_ball_distance = transform_blue.translation.distance(transform_ball.translation);
    if player_ball_distance < PLAYER_RADIUS + BALL_RADIUS {
        // If right control pressed, shoot the ball
        if kb.pressed(KeyCode::RControl) {
            let diff_x = transform_blue.translation.x - transform_ball.translation.x;
            let diff_y = transform_blue.translation.y - transform_ball.translation.y;
            let angle = diff_y.atan2(diff_x);
            velocity_ball.y += -5.0 * angle.sin();
            velocity_ball.x += -5.0 * angle.cos();
        }

        handle_collision(&mut velocity_blue, &mut velocity_ball, &mut transform_blue, &mut transform_ball, PLAYER_RADIUS, BALL_RADIUS);

    }
}




fn players_collision_system(
    mut query_red: Query<(Entity, &mut Velocity, &mut Transform, &PlayerRed), Without<PlayerBlue>>,
    mut query_blue: Query<(Entity, &mut Velocity, &mut Transform, &PlayerBlue), Without<PlayerRed>>,
) {
    let (entity_red, mut velocity_red, mut transform_red, _) = query_red.iter_mut().next().unwrap();
    let (entity_blue, mut velocity_blue, mut transform_blue, _) = query_blue.iter_mut().next().unwrap();

    // If player red and player blue collide
    let players_distance = transform_red.translation.distance(transform_blue.translation);
    if players_distance < PLAYER_RADIUS * 2.0 {
        handle_collision(&mut velocity_red, &mut velocity_blue, &mut transform_red, &mut transform_blue, PLAYER_RADIUS, PLAYER_RADIUS);

    };

}

fn corner_collision_system(
    mut query: Query<(Entity, &mut Velocity, &Transform, &Radius)>
) {
    let corner1 = Vec3::new(- 1024. / 2., 100., 5.);
    let corner2 = Vec3::new(1024. / 2., 100., 5.);
    let corner3 = Vec3::new(1024. / 2., -100., 5.);
    let corner4 = Vec3::new(- 1024. / 2., -100., 5.);


    for (entity, mut velocity, transform, radius) in query.iter_mut() {
        let position = &transform.translation;
        let radius = radius.0;

        let d1 = transform.translation.distance(corner1);
        let d2 = transform.translation.distance(corner2);
        let d3 = transform.translation.distance(corner3);
        let d4 = transform.translation.distance(corner4);

        if d1 <= radius + CORNER_RADIUS {
            velocity.x = -velocity.x;
            velocity.y = -velocity.y;
        }
        if d2 <= radius + CORNER_RADIUS {
            velocity.x = -velocity.x;
            velocity.y = -velocity.y;
        }
        if d3 <= radius + CORNER_RADIUS {
            velocity.x = -velocity.x;
            velocity.y = -velocity.y;
        }
        if d4 <= radius + CORNER_RADIUS {
            velocity.x = -velocity.x;
            velocity.y = -velocity.y;
        }
    }
}

fn edge_collision_system(
    mut query: Query<(Entity, &mut Velocity, &Transform, &Radius)>
) {

    let (WINDOW_WIDTH, WINDOW_HEIGHT) = (1024., 768.);

    for (entity, mut velocity, transform, radius) in query.iter_mut() {

        let translation = transform.translation.clone();
        let radius = radius.0;

        if translation.x + radius >= WINDOW_WIDTH / 2. && ((translation.y >= 100.0 || translation.y <= -100.0) || radius == PLAYER_RADIUS) {
            velocity.x = -velocity.x;
        } else if translation.x - radius <= -WINDOW_WIDTH / 2. && ((translation.y >= 100.0 || translation.y <= -100.0) || radius == PLAYER_RADIUS){
            velocity.x = -velocity.x;
        }

        if translation.y + radius >= WINDOW_HEIGHT / 2. && ((translation.y >= 100.0 || translation.y <= -100.0) || radius == PLAYER_RADIUS) {
            velocity.y = -velocity.y;
        } else if translation.y - radius <= -WINDOW_HEIGHT / 2. && ((translation.y >= 100.0 || translation.y <= -100.0) || radius == PLAYER_RADIUS) {
            velocity.y = -velocity.y;
        }
    }
}

fn goal_system(
    mut query_ball: Query<(Entity, &mut Velocity, &mut Transform, &Ball)>,
    mut query_players: Query<(Entity, &mut Velocity, &mut Transform), Without<Ball>>,
    mut score: ResMut<Score>,
    mut score_text: Query<(&mut Text)>,
) {
    // Get tuple from query
    let (entity_ball, mut velocity_ball, mut transform_ball, _) = query_ball.iter_mut().next().unwrap();

    // Get text from score_text
    let mut text = score_text.iter_mut().next().unwrap();



    if transform_ball.translation.x >= 1024. / 2. {
        score.red += 1;
        text.sections[0].value = format!("Score: {}–{}", score.red, score.blue);
        println!("Red score: {}", score.red);
        transform_ball.translation.x = 0.;
        transform_ball.translation.y = 0.;

        velocity_ball.x = 0.;
        velocity_ball.y = 0.;
        let mut i = -200.0;
        for (entity, mut velocity, mut transform) in query_players.iter_mut() {
            velocity.x = 0.;
            velocity.y = 0.;
            transform.translation.x = i.clone();
            i += 400.0;
            transform.translation.y = 0.;
        }
    } else if transform_ball.translation.x <= -1024. / 2. {
        score.blue += 1;
        text.sections[0].value = format!("Score: {}–{}", score.red, score.blue);
        println!("Blue score: {}", score.blue);
        transform_ball.translation.x = 0.;
        transform_ball.translation.y = 0.;

        velocity_ball.x = 0.;
        velocity_ball.y = 0.;

        let mut i = -200.0;
        for (entity, mut velocity, mut transform) in query_players.iter_mut() {
            velocity.x = 0.;
            velocity.y = 0.;
            transform.translation.x = i.clone();
            i += 400.0;
            transform.translation.y = 0.;
        }
    }

    if score.red == 3 {
        score.red = 0;
        score.blue = 0;
        text.sections[0].value = "Red Wins!".to_string();
    }
    else if score.blue == 3 {
        score.red = 0;
        score.blue = 0;
        text.sections[0].value = "Blue Wins!".to_string();
    }


}
