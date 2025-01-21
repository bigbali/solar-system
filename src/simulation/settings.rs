use bevy::prelude::*;

#[derive(Resource)]
pub struct SimulationParameters {
    /// AU^3 / Solar Mass * day^2
    pub gravitational_constant: f32,

    /// Solar Mass
    pub mass_scale: f32,

    /// AU
    pub unit_scale: f32,

    /// Days per second.
    pub time_step: f32,

    /// Arbitrary multiplier for the simulation speed
    pub speed_multiplier: f32,

    /// Prevents bodies from getting too close, causing instability and crazy large forces
    pub softening_factor: f32,
}

impl Default for SimulationParameters {
    fn default() -> Self {
        Self {
            gravitational_constant: 2.96e-4,
            mass_scale: 1.0,
            unit_scale: 1.0,
            time_step: 1.0,
            speed_multiplier: 1.0,
            softening_factor: 1e-8,
        }
    }
}

#[derive(Resource, Default)]
pub struct FollowBody {
    pub entity: Option<Entity>,
    pub is_active: bool,
}

#[derive(Resource, Default)]
pub struct SelectedBody {
    pub entity: Option<Entity>,
}
