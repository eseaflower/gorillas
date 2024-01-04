use bevy::{ecs::query, prelude::*, transform::commands, window::PrimaryWindow};
use bevy_rapier2d::{
    dynamics::{
        AdditionalMassProperties, CoefficientCombineRule, ExternalForce, ExternalImpulse, RigidBody,
    },
    geometry::{
        ActiveEvents, Collider, ColliderMassProperties, CollidingEntities, Friction, Restitution,
    },
};

pub struct ShotPlugin;

impl Plugin for ShotPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Gun>()
            .add_systems(Update, (handle_collision, shoot, aim_system, debug_aim));
    }
}

#[derive(Debug, Component)]
pub struct Shot;

fn spawn_shot(commands: &mut Commands, impulse: Vec2) {
    let foo = commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(8.0)),
                    color: Color::rgb(0.8, 0.0, 0.0),
                    ..Default::default()
                },
                transform: Transform::from_xyz(-100.0, 0.0, 0.0),
                ..Default::default()
            },
            Shot,
        ))
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(4.0, 4.0))
        // .insert(Collider::cuboid(8.0, 8.0))
        .insert(ExternalImpulse {
            impulse,
            ..Default::default()
        })
        .insert(ActiveEvents::COLLISION_EVENTS)
        // .insert(CollidingEntities::default())
        .id();
    dbg!(foo);
}

fn handle_collision(query: Query<(&Shot, &CollidingEntities)>) {
    for (shot, colliding_entities) in query.iter() {
        for entity in colliding_entities.iter() {
            eprintln!("Shot collided with {:?}", entity);
        }
    }
}

#[derive(Debug, Resource, Default)]
struct Gun {
    force: f32,
    direction: Vec2,
}

fn shoot(mut commands: Commands, keyboard: Res<Input<KeyCode>>, mut gun: ResMut<Gun>) {
    if keyboard.just_released(KeyCode::Space) {
        eprintln!("Shooting");
        let impulse = gun.direction.normalize() * gun.force;
        eprintln!("Impulse: {:?}", impulse);
        spawn_shot(&mut commands, impulse);

        // Reset gun
        gun.force = 0.0;
    }
    if keyboard.pressed(KeyCode::Space) {
        gun.force += 1.0;
    }
}

fn aim_system(
    mut gun: ResMut<Gun>,
    // query to get the window (so we can read the current cursor position)
    q_window: Query<&Window, With<PrimaryWindow>>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform)>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so Query::single() is OK
    let (camera, camera_transform) = q_camera.single();

    // There is only one primary window, so we can similarly get it from the query:
    let window = q_window.single();

    // check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates, and truncate to discard Z
    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        gun.direction = world_position - Vec2::new(-100.0, 0.0);
        // eprintln!("World coords: {}/{}", world_position.x, world_position.y);
    }
}

fn debug_aim(gun: Res<Gun>, mut gizmo: Gizmos) {
    gizmo.line_2d(
        Vec2::new(-100.0, 0.0),
        Vec2::new(-100.0, 0.0) + gun.direction,
        Color::rgb(1.0, 0.0, 0.0),
    );
}
