use bevy::prelude::*;
#[derive(Component, Reflect, Default, Clone)]
#[reflect(Component)]
pub struct Blinker {
    pub timer: Timer,
    pub speed: f32,
    pub duration: f32,
    pub number_of_blinks: usize,
    pub duration_const: f32,
    pub direction: f32,
}

impl Blinker {
    pub fn new(speed: f32, duration: f32, number_of_blinks: usize) -> Self {
        Self {
            timer: Timer::from_seconds(0.01, TimerMode::Repeating),
            speed: speed,
            duration: duration,
            number_of_blinks: number_of_blinks,
            duration_const: duration,
            direction: -1.,
        }
    }
}

pub fn blink_system(
    mut commands: Commands,
    mut blinkers: Query<(Entity, &mut Handle<StandardMaterial>, &mut Blinker), With<Blinker>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    time: Res<Time>,
) {
    for (entity, material, mut blinker) in &mut blinkers {
        if let Some(material) = materials.get_mut(&material) {
            blinker.timer.tick(time.delta());
            if blinker.timer.just_finished() {
                if blinker.number_of_blinks <= 0 {
                    commands.entity(entity).despawn_recursive();
                    continue;
                }

                if blinker.duration < 0. {
                    blinker.duration = 0.;
                    blinker.direction = 1.;
                    blinker.number_of_blinks -= 1;
                }

                if blinker.duration > blinker.duration_const {
                    blinker.duration = blinker.duration_const;
                    blinker.direction = -1.;
                }

                blinker.duration += (blinker.speed + time.delta_seconds()) * blinker.direction;

                let alpha =
                    map_value_to_range(blinker.duration, 0., blinker.duration_const, 0., 1.);

                material.base_color.set_a(alpha);
                material.emissive.set_a(alpha);
            }
        }
    }
}

fn map_value_to_range(value: f32, in_min: f32, in_max: f32, out_min: f32, out_max: f32) -> f32 {
    return (value - in_min) * (out_max - out_min) / (in_max - in_min) + out_min;
}
