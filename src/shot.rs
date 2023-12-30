use bevy::prelude::*;
use bevy_rapier2d::{
    dynamics::{AdditionalMassProperties, ExternalForce, ExternalImpulse, RigidBody},
    geometry::{ActiveEvents, Collider, Friction, Restitution},
};

pub struct ShotPlugin;

impl Plugin for ShotPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_shot)
            .add_systems(Update, move_shot);
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
            AdditionalMassProperties::Mass(1.0),
            Collider::cuboid(15.0, 15.0),
            Shot,
        ))
        .insert(ExternalImpulse {
            impulse: Vec2::new(5.0, 0.0),
            ..Default::default()
        })
        .insert(ActiveEvents::COLLISION_EVENTS)
        .id();
    dbg!(foo);
}

fn move_shot() {}
