use std::{env, sync::Mutex};

use bevy::prelude::*;

/// Physics update frequency (Hz)
pub const UPDATE_FREQUENCY: i32 = 60;

#[derive(Resource, Reflect)]
pub struct SimulationParameters {
    /// AU^3 / Solar Mass * day^2
    pub gravitational_constant: f32,

    /// Solar Mass
    pub mass_scale: f32,

    /// AU
    pub unit_scale: f32,

    /// Days per second
    pub time_step: f32,

    /// Increasing this will increase the accuracy of the simulation.
    pub updates_per_step: f32,

    /// Prevents bodies from getting too close, causing instability and crazy large forces
    pub softening_factor: f32,

    /// Which integrator to use
    pub integrator: Integrator,
}

impl Default for SimulationParameters {
    fn default() -> Self {
        Self {
            gravitational_constant: 2.96e-4,
            mass_scale: 1.0,
            unit_scale: 1.0,
            time_step: 1.0,
            updates_per_step: 10.0,
            softening_factor: 1e-12,
            integrator: Integrator::default(),
        }
    }
}

#[derive(Default, Reflect, Debug)]
pub enum Integrator {
    Euler,
    #[default]
    Leapfrog,
    RK4,
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

#[derive(Resource, Default)]
pub struct ElapsedTime(pub f32);

pub fn elapsed_time_update_system(
    mut elapsed_time: ResMut<ElapsedTime>,
    parameters: Res<SimulationParameters>,
) {
    elapsed_time.0 += parameters.time_step / UPDATE_FREQUENCY as f32;
}

pub fn params_override_system(mut params: ResMut<SimulationParameters>) {
    let args = env::args().collect::<Vec<_>>();

    if args.len() < 3 {
        return;
    }

    for arg in args[1..args.len()].chunks_exact(2) {
        if arg.len() == 1 {
            error!("Argument {} has no value provided", arg[0]);
            return;
        }

        let field = &arg[0];
        let value = &arg[1];

        if field == "integrator" {
            let parameter_override: &mut Integrator = params.get_field_mut(field).unwrap();
            let override_successful = match value.as_str() {
                "euler" => {
                    let _ = parameter_override.set(Box::new(Integrator::Euler));
                    true
                }
                "leapfrog" => {
                    let _ = parameter_override.set(Box::new(Integrator::Leapfrog));
                    true
                }
                "rk4" => {
                    let _ = parameter_override.set(Box::new(Integrator::RK4));
                    true
                }
                _ => {
                    error!(
                        "Could not parse value \"{}\" for {}: invalid integrator. Defaulting to {:?}",
                        value, field, Integrator::default()
                    );
                    false
                }
            };

            if override_successful {
                info!("Using override value \"{}\" for {}", value, field)
            }

            return;
        }

        let parameter_override: &mut f32 = params.get_field_mut(field).unwrap();
        let parsed_value = value.parse::<f32>();

        match parsed_value {
            Ok(f) => match parameter_override.set(Box::new(f)) {
                Ok(_) => {
                    info!("Using override value \"{}\" for {}", f, field)
                }
                Err(error) => {
                    error!(
                        "Could not set value \"{}\" for {}: {:?}",
                        value, field, error
                    )
                }
            },
            Err(_) => {
                error!("Could not parse value \"{}\" for {}", value, field)
            }
        }
    }
}
