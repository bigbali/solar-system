use bevy::prelude::*;

#[derive(Resource)]
pub struct SimulationParameters {
    pub gravitational_constant: f32,
    pub mass_scale: f32,
    pub unit_scale: f32,
    pub velocity_scale: f32,
    pub time_step: f32,
    pub speed_multiplier: f32,
    pub softening_factor: f32,
}

impl Default for SimulationParameters {
    fn default() -> Self {
        Self {
            gravitational_constant: 6.67430e-11,
            mass_scale: 1.0,
            unit_scale: 1.0,
            velocity_scale: 1.0,
            time_step: 86400.0,
            speed_multiplier: 1.0,
            softening_factor: 1e-10,
        }
    }
}
// impl Default for SimulationParameters {
//     fn default() -> Self {
//         Self {
//             gravitational_constant: 6.67430e-11,
//             mass_scale: 1.988416e+30,
//             unit_scale: 149.60e+6,
//             velocity_scale: 1e3,
//             time_scale: 86400.0,
//             speed_multiplier: 1.0,
//             softening_factor: 1e-10,
//         }
//     }
// }

#[derive(Resource, Default)]
pub struct FollowBody {
    pub entity: Option<Entity>,
    pub is_active: bool,
}

#[derive(Resource, Default)]
pub struct SelectedBody {
    pub entity: Option<Entity>,
}
