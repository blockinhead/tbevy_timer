use std::time::Duration;
use bevy::prelude::*;
use bevy_tweening::{Animator, EaseFunction, Tween, TweeningPlugin};
use bevy_tweening::lens::SpriteColorLens;


fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "bevy to wasm".to_string(),
                resolution: (800., 600.).into(),
                canvas: Some("#bevy".to_owned()),
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(TweeningPlugin)
        .init_resource::<Countdown>()
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run();
}

#[derive(Component)]
pub struct SpriteCompo;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(200.0, 200.0)),
                ..default()
            },
            ..default()
        },
        SpriteCompo,
    ));
}


#[derive(Resource)]
pub struct Countdown {
    pub main_timer: Timer,
    pub counter: u32,
    pub default_duration: Duration,
}

impl Countdown {
    pub fn new(duration_sec: u32) -> Self {
        Self {
            main_timer: Timer::from_seconds(duration_sec as f32, TimerMode::Repeating),
            counter: 0,
            default_duration: Duration::from_secs(duration_sec as u64),
        }
    }
}

impl Default for Countdown {
    fn default() -> Self {
        Self::new(2)
    }
}

fn update(
    mut commands: Commands,
    time: Res<Time>,
    mut countdown: ResMut<Countdown>,
    mut sprite: Query<Entity, With<SpriteCompo>>
) {
    countdown.main_timer.tick(time.delta());

    if countdown.main_timer.finished() {
        countdown.counter += 1;
        info!("timer already finished {:?} times", countdown.counter);

        let sprite_entity = sprite.single_mut();

        commands.entity(sprite_entity).remove::<Animator<Sprite>>();
        commands.entity(sprite_entity).insert(
            Animator::new(
                Tween::new(
                    EaseFunction::QuadraticIn,
                    std::time::Duration::from_millis(500),
                    SpriteColorLens{start: Color::BLACK, end: Color::RED},
                )
            )
        );

        if countdown.counter % 10 == 0 {
            info!("increasing speed");
            let cur_duration = countdown.main_timer.duration() / 2;
            countdown.main_timer.set_duration(cur_duration);
        }
        if countdown.counter > 100 {
            countdown.counter = 0;
            let def_dur = countdown.default_duration;
            countdown.main_timer.set_duration(def_dur);
        }
    }
}