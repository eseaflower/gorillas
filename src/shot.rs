use bevy::{ecs::query, prelude::*};
use bevy_rapier2d::{
    dynamics::{AdditionalMassProperties, ExternalForce, ExternalImpulse, RigidBody},
    geometry::{ActiveEvents, Collider, CollidingEntities, Friction, Restitution},
};

pub struct ShotPlugin;

impl Plugin for ShotPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_shot)
            .add_systems(Update, (despawn_shot, handle_collision));
    }
}

#[derive(Debug, Component)]
pub struct Shot;

fn spawn_shot(mut commands: Commands) {
    let foo = commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(30.0)),
                    color: Color::rgb(0.8, 0.0, 0.0),
                    ..Default::default()
                },
                transform: Transform::from_xyz(0.0, -30.0, 0.0),
                ..Default::default()
            },
            RigidBody::Dynamic,
            // Collider::ball(15.0),
            Collider::cuboid(15.0, 15.0),
            Shot,
        ))
        .insert(ExternalImpulse {
            impulse: Vec2::new(5.0, 0.0),
            ..Default::default()
        })
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(CollidingEntities::default())
        .id();
    dbg!(foo);
}

fn despawn_shot(
    mut commands: Commands,
    shot_quuery: Query<(Entity, &Transform), With<Shot>>,
    camera_query: Query<&Transform, With<Camera>>,
) {
    for (e, transform) in shot_quuery.iter() {
        // Shot is 100 units away from all cameras.
        let do_despawn = camera_query.iter().all(|c| {
            let distance = c.translation - transform.translation;
            distance.length() > 1000.0
        });
        if do_despawn {
            eprintln!("Despawning shot due to distance");
            commands.entity(e).despawn_recursive();
        }
    }
}

fn handle_collision(query: Query<(&Shot, &CollidingEntities)>) {
    for (shot, colliding_entities) in query.iter() {
        for entity in colliding_entities.iter() {
            eprintln!("Shot collided with {:?}", entity);
        }
    }
}
