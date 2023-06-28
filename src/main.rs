use bevy::{
    core::FixedTimestep,
    math::{const_vec2, const_vec3},
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};

// Game and canvas
const TIME_STEP: f32 = 1.0 / 60.0;
const CANVAS_WIDTH: f32 = 1000.;
const CANVAS_HEIGHT: f32 = 800.;
const BACKGROUND_COLOR: Color = Color::rgb(34. / 255., 39. / 255., 46. / 255.);

// Paddle
const PADDLE_SIZE: Vec3 = const_vec3!([120.0, 20.0, 0.0]);
const PADDLE_SPEED: f32 = 800.0;
const PADDLE_PADDING: f32 = 20.0;
const PADDLE_COLOR: Color = Color::rgb(173. / 255., 186. / 255., 199. / 255.);

// Ball
const BALL_STARTING_POSITION: Vec3 = const_vec3!([0.0, -50.0, 1.0]);
const BALL_SIZE: Vec3 = const_vec3!([30.0, 30.0, 0.0]);
const BALL_SPEED: f32 = 500.0;
const INITIAL_BALL_DIRECTION: Vec2 = const_vec2!([0., 0.]);
const BALL_COLOR: Color = Color::WHITE;
const CALL_COLLISION_SPEED_INCREASE: f32 = 10.;

// Walls (coordinates originate in the center of the canvas)
const WALL_THICKNESS: f32 = 20.0;
const LEFT_WALL: f32 = -(CANVAS_WIDTH / 2.);
const RIGHT_WALL: f32 = CANVAS_WIDTH / 2.;
const BOTTOM_WALL: f32 = -(CANVAS_HEIGHT / 2.);
const TOP_WALL: f32 = CANVAS_HEIGHT / 2.;
const WALL_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);

// Bricks
const BRICK_SIZE: Vec2 = const_vec2!([70., 25.]);

// Text
const TEXT_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);

#[derive(Component)]
struct Collider;

#[derive(Default)]
struct CollisionEvent;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Paddle;

#[derive(Component)]
struct Wall;

#[derive(Component)]
struct BottomWall;

#[derive(Component)]
struct Brick;

#[derive(Component)]
struct GameOverText;

#[derive(Component)]
struct ScoreText;

enum WallLocation {
    Left,
    Right,
    Bottom,
    Top,
}

fn wall_sprite_bundle(location: WallLocation) -> SpriteBundle {
    let pos = match location {
        WallLocation::Left => Vec2::new(LEFT_WALL, 0.),
        WallLocation::Right => Vec2::new(RIGHT_WALL, 0.),
        WallLocation::Bottom => Vec2::new(0., BOTTOM_WALL),
        WallLocation::Top => Vec2::new(0., TOP_WALL),
    };
    let size = match location {
        WallLocation::Left => Vec2::new(WALL_THICKNESS, CANVAS_HEIGHT + WALL_THICKNESS),
        WallLocation::Right => Vec2::new(WALL_THICKNESS, CANVAS_HEIGHT + WALL_THICKNESS),
        WallLocation::Bottom => Vec2::new(CANVAS_WIDTH + WALL_THICKNESS, WALL_THICKNESS),
        WallLocation::Top => Vec2::new(CANVAS_WIDTH + WALL_THICKNESS, WALL_THICKNESS),
    };
    return SpriteBundle {
        transform: Transform {
            translation: pos.extend(0.0),
            scale: size.extend(1.0),
            ..default()
        },
        sprite: Sprite {
            color: WALL_COLOR,
            ..default()
        },
        ..default()
    };
}

struct GameState {
    score: usize,
    ball_waiting: bool,
    lives: usize,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut windows: ResMut<Windows>) {
    // Cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    // Canvas
    let window = windows.get_primary_mut().unwrap();
    window.set_resolution(CANVAS_WIDTH, CANVAS_HEIGHT);

    // Paddle
    commands
        .spawn()
        .insert(Paddle)
        .insert_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, BOTTOM_WALL + 30., 0.0),
                scale: PADDLE_SIZE,
                ..default()
            },
            sprite: Sprite {
                color: PADDLE_COLOR,
                ..default()
            },
            ..default()
        })
        .insert(Collider);

    // Walls
    commands
        .spawn()
        .insert(Wall)
        .insert_bundle(wall_sprite_bundle(WallLocation::Left))
        .insert(Collider);
    commands
        .spawn()
        .insert(Wall)
        .insert_bundle(wall_sprite_bundle(WallLocation::Right))
        .insert(Collider);
    commands
        .spawn()
        .insert(Wall)
        .insert(BottomWall)
        .insert_bundle(wall_sprite_bundle(WallLocation::Bottom))
        .insert(Collider);
    commands
        .spawn()
        .insert(Wall)
        .insert_bundle(wall_sprite_bundle(WallLocation::Top))
        .insert(Collider);

    // Ball
    commands
        .spawn()
        .insert(Ball)
        .insert_bundle(SpriteBundle {
            transform: Transform {
                scale: BALL_SIZE,
                translation: BALL_STARTING_POSITION,
                ..default()
            },
            sprite: Sprite {
                color: BALL_COLOR,
                ..default()
            },
            ..default()
        })
        .insert(Velocity(INITIAL_BALL_DIRECTION));

    // Bricks
    let colors = vec![
        Color::rgb(236. / 255., 72. / 255., 153. / 255.),
        Color::rgb(239. / 255., 68. / 255., 68. / 255.),
        Color::rgb(249. / 255., 115. / 255., 22. / 255.),
        Color::rgb(234. / 255., 179. / 255., 8. / 255.),
        Color::rgb(34. / 255., 197. / 255., 94. / 255.),
        Color::rgb(6. / 255., 182. / 255., 212. / 255.),
    ];
    for row_x in 0..12 {
        for row_y in 0..6 {
            commands
                .spawn()
                .insert(Brick)
                .insert_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: colors[row_y],
                        ..default()
                    },
                    transform: Transform {
                        translation: Vec3::new(
                            LEFT_WALL + (80. + ((row_x as f32) * 75.)),
                            TOP_WALL - (80. + row_y as f32 * 30.),
                            0.0,
                        ),
                        scale: Vec3::new(BRICK_SIZE.x, BRICK_SIZE.y, 1.0),
                        ..default()
                    },
                    ..default()
                })
                .insert(Collider);
        }
    }

    // Text
    commands
        .spawn_bundle(TextBundle {
            text: Text {
                sections: vec![
                    TextSection {
                        value: "Score:".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                            font_size: 32.,
                            color: TEXT_COLOR,
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                            font_size: 32.,
                            color: TEXT_COLOR,
                        },
                    },
                    TextSection {
                        value: " Lives:".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                            font_size: 32.,
                            color: TEXT_COLOR,
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                            font_size: 32.,
                            color: TEXT_COLOR,
                        },
                    },
                ],
                ..default()
            },
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(10.),
                    left: Val::Px(10.),
                    ..default()
                },
                ..default()
            },
            ..default()
        })
        .insert(ScoreText);

    commands
        .spawn_bundle(TextBundle {
            text: Text {
                sections: vec![TextSection {
                    value: "".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                        font_size: 64.,
                        color: TEXT_COLOR,
                    },
                }],
                ..default()
            },
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(CANVAS_HEIGHT / 2. - 32.),
                    left: Val::Px(CANVAS_WIDTH / 2. - 150.),
                    ..default()
                },
                ..default()
            },
            ..default()
        })
        .insert(GameOverText);
}

fn move_paddle(
    keyboard_input: Res<Input<KeyCode>>,
    mut paddle_query: Query<&mut Transform, With<Paddle>>,
) {
    let mut paddle_transform = paddle_query.single_mut();
    let mut direction = 0.0;

    if keyboard_input.pressed(KeyCode::Left) {
        direction -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        direction += 1.0;
    }

    // Calculate the new horizontal paddle position based on player input
    let new_paddle_position = paddle_transform.translation.x + direction * PADDLE_SPEED * TIME_STEP;

    // Update the paddle position,
    // making sure it doesn't cause the paddle to leave the arena
    let left_bound = LEFT_WALL + WALL_THICKNESS / 2.0 + PADDLE_SIZE.x / 2.0 + PADDLE_PADDING;
    let right_bound = RIGHT_WALL - WALL_THICKNESS / 2.0 - PADDLE_SIZE.x / 2.0 - PADDLE_PADDING;

    paddle_transform.translation.x = new_paddle_position.clamp(left_bound, right_bound);
}

fn move_paddle_by_mouse(
    windows: Res<Windows>,
    mut paddle_query: Query<&mut Transform, With<Paddle>>,
) {
    let window = windows.get_primary().unwrap();
    let pos = window.cursor_position();
    let mut paddle_transform = paddle_query.single_mut();

    if pos.is_some() {
        let new_paddle_position = pos.unwrap().x - CANVAS_WIDTH / 2.;
        // making sure it doesn't cause the paddle to leave the arena
        let left_bound = LEFT_WALL + WALL_THICKNESS / 2.0 + PADDLE_SIZE.x / 2.0 + PADDLE_PADDING;
        let right_bound = RIGHT_WALL - WALL_THICKNESS / 2.0 - PADDLE_SIZE.x / 2.0 - PADDLE_PADDING;

        paddle_transform.translation.x = new_paddle_position.clamp(left_bound, right_bound);
    }
}

fn stick_ball_to_paddle(
    game_state: Res<GameState>,
    paddle_query: Query<&Transform, With<Paddle>>,
    mut ball_query: Query<&mut Transform, (With<Ball>, Without<Paddle>)>,
) {
    let paddle_transform = paddle_query.single();
    if game_state.ball_waiting {
        let mut ball_transform = ball_query.single_mut();
        ball_transform.translation.x = paddle_transform.translation.x;
        ball_transform.translation.y = paddle_transform.translation.y + 25.;
    }
}

fn handle_waiting_click(
    mouse_input: Res<Input<MouseButton>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut game_state: ResMut<GameState>,
    mut ball_query: Query<&mut Velocity, With<Ball>>,
) {
    if game_state.lives > 0
        && game_state.ball_waiting
        && (keyboard_input.pressed(KeyCode::Space) || mouse_input.just_pressed(MouseButton::Left))
    {
        game_state.ball_waiting = false;
        let mut ball_velocity = ball_query.single_mut();
        if keyboard_input.pressed(KeyCode::Left) {
            ball_velocity.x = -0.5 * BALL_SPEED;
            ball_velocity.y = 0.5 * BALL_SPEED;
        } else {
            ball_velocity.x = 0.5 * BALL_SPEED;
            ball_velocity.y = 0.5 * BALL_SPEED;
        }
    }
}

fn check_for_collisions(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    mut ball_query: Query<(&mut Velocity, &Transform), With<Ball>>,
    collider_query: Query<
        (
            Entity,
            &Transform,
            Option<&Brick>,
            Option<&Paddle>,
            Option<&BottomWall>,
        ),
        With<Collider>,
    >,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    let (mut ball_velocity, ball_transform) = ball_query.single_mut();
    let ball_size = ball_transform.scale.truncate();

    for (collider_entity, transform, maybe_brick, maybe_paddle, maybe_bottom_wall) in
        collider_query.iter()
    {
        let collision = collide(
            ball_transform.translation,
            ball_size,
            transform.translation,
            transform.scale.truncate(),
        );
        if let Some(collision) = collision {
            // Sends a collision event so that other systems can react to the collision
            collision_events.send_default();

            // Bricks should be despawned and increment the scoreboard on collision
            if maybe_brick.is_some() {
                game_state.score += 1;
                commands.entity(collider_entity).despawn();

                if ball_velocity.y > 0. {
                    ball_velocity.y += CALL_COLLISION_SPEED_INCREASE;
                } else {
                    ball_velocity.y -= CALL_COLLISION_SPEED_INCREASE;
                }
            }

            // collided with bottom wall
            if maybe_bottom_wall.is_some() {
                ball_velocity.x = 0.;
                ball_velocity.y = 0.;
                game_state.ball_waiting = true;
                game_state.lives -= 1;
            }

            // reflect the ball when it collides
            let mut reflect_x = false;
            let mut reflect_y = false;

            // only reflect if the ball's velocity is going in the opposite direction of the
            // collision
            match collision {
                Collision::Left => reflect_x = ball_velocity.x > 0.0,
                Collision::Right => reflect_x = ball_velocity.x < 0.0,
                Collision::Top => reflect_y = ball_velocity.y < 0.0,
                Collision::Bottom => reflect_y = ball_velocity.y > 0.0,
                Collision::Inside => { /* do nothing */ }
            }

            // reflect velocity on the x-axis if we hit something on the x-axis
            if maybe_paddle.is_some() && (reflect_y || reflect_x) {
                // Always bounce up if paddle collision
                ball_velocity.y = -ball_velocity.y;
                // Decide based on which side of the paddle collides where x velocity goes
                let x_diff = ball_transform.translation.x - transform.translation.x;
                ball_velocity.x = (x_diff / 70.).clamp(-0.8, 0.8) * BALL_SPEED;
                // if (x_diff < 0. && ball_velocity.x > 0.) || (x_diff > 0. && ball_velocity.x < 0.) {
                //     ball_velocity.x = -ball_velocity.x;
                // }
            } else {
                if reflect_x {
                    ball_velocity.x = -ball_velocity.x;
                }

                // reflect velocity on the y-axis if we hit something on the y-axis
                if reflect_y {
                    ball_velocity.y = -ball_velocity.y;
                }
            }
        }
    }
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.x += velocity.x * TIME_STEP;
        transform.translation.y += velocity.y * TIME_STEP;
    }
}

fn update_scoreboard(game_state: Res<GameState>, mut query: Query<&mut Text, With<ScoreText>>) {
    let mut text = query.single_mut();
    text.sections[1].value = format!("{}", game_state.score);
    text.sections[3].value = format!("{}", game_state.lives);
}

fn show_game_over(game_state: Res<GameState>, mut query: Query<&mut Text, With<GameOverText>>) {
    if game_state.lives == 0 {
        let mut text = query.single_mut();
        text.sections[0].value = format!("Game over!");
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(GameState {
            score: 0,
            lives: 3,
            ball_waiting: true,
        })
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_startup_system(setup)
        .add_event::<CollisionEvent>()
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(check_for_collisions)
                .with_system(move_paddle.before(check_for_collisions))
                .with_system(move_paddle_by_mouse.before(check_for_collisions))
                .with_system(stick_ball_to_paddle.before(check_for_collisions))
                .with_system(handle_waiting_click.before(check_for_collisions))
                .with_system(apply_velocity.before(check_for_collisions))
                .with_system(show_game_over.before(check_for_collisions))
                .with_system(update_scoreboard.before(check_for_collisions)),
        )
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}
