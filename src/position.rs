use bevy::prelude::*;

pub struct PositionPlugin;

#[derive(Debug, Resource)]
struct PrintTimer(Timer);

impl Plugin for PositionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_positions, spawn_sprite))
            .add_systems(Update, move_sprite)
            .insert_resource(PrintTimer(Timer::from_seconds(1.0, TimerMode::Repeating)));
    }
}

#[derive(Debug, Component)]
struct Position {
    x: f32,
    y: f32,
}

fn print_position(time: Res<Time>, mut timer: ResMut<PrintTimer>, query: Query<&Position>) {
    // Update the timer
    timer.0.tick(time.delta());

    if timer.0.just_finished() {
        println!("Positions:");
        for position in query.iter() {
            println!("Position: {:?}", position);
        }
    }
}

fn spawn_positions(mut commands: Commands) {
    commands.spawn((Position { x: 1.0, y: 2.0 },));
    commands.spawn((Position { x: 3.0, y: 4.0 },));
}

fn move_sprite(time: Res<Time>, mut query: Query<&mut Transform, Without<Camera>>) {
    for mut transform in query.iter_mut() {
        let delta_x = time.elapsed_seconds() * 100.0;
        println!("delta_x: {}", delta_x);
        let sign = delta_x.sin().signum();
        println!("total_delta_x: {}", delta_x * sign);

        transform.translation.x += delta_x * sign;
    }
}

fn spawn_sprite(mut commands: Commands) {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(100.0, 100.0)),
            color: Color::rgb(0.8, 0.0, 0.0),
            ..Default::default()
        },
        ..Default::default()
    });
}
