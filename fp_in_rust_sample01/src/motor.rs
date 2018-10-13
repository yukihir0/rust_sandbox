extern crate floating_duration;
use std::time::Instant;
use floating_duration::{TimeAsFloat, TimeFormat};
use physics::{ElevatorSpecification, ElevatorState, MotorInput, MotorController};

pub struct SimpleMotorController {
    pub esp: ElevatorSpecification
}

impl MotorController for SimpleMotorController {
    fn init(&mut self, esp: ElevatorSpecification, est: ElevatorState) {
        self.esp = esp;
    }

    fn poll(&mut self, est: ElevatorState, dst: u64) -> MotorInput {
        let t = est.velocity.abs() / 1.0;
      
        let d = t * (est.velocity/2.0);

        let dst_height = (dst as f64) * self.esp.floor_height;
      
        let l = (est.location - dst_height).abs();
      
        let target_acceleration = {
            let going_up = est.location < dst_height;
         
            if est.velocity.abs() >= 5.0 {
                if (going_up && est.velocity>0.0) || (!going_up && est.velocity<0.0) {
                    0.0
                } else if going_up {
                    1.0
                } else {
                    -1.0
                }
         
            } else if l < d && ((going_up && est.velocity>0.0) || (!going_up && est.velocity<0.0)) {
                if going_up {
                    -1.0
                } else {
                    1.0
                }
         
            } else {
                if going_up {
                    1.0
                } else {
                    -1.0
                }
            }
        };

        let gravity_adjusted_acceleration = target_acceleration + 9.8;
        let target_force = gravity_adjusted_acceleration * self.esp.carriage_weight;
        let target_voltage = target_force / 8.0;
        if target_voltage > 0.0 {
            MotorInput::Up { voltage: target_voltage }
        } else {
            MotorInput::Down { voltage: target_voltage.abs() }
        }
    }
}

const MAX_JERK: f64 = 0.2;
const MAX_ACCELERATION: f64 = 2.0;
const MAX_VELOCITY: f64 = 5.0;

pub struct SmoothMotorController {
    pub esp: ElevatorSpecification,
    pub timestamp: f64
}

impl MotorController for SmoothMotorController {
    fn init(&mut self, esp: ElevatorSpecification, est: ElevatorState) {
        self.esp = esp;
        self.timestamp = est.timestamp;
    }

    fn poll(&mut self, est: ElevatorState, dst: u64) -> MotorInput {
        let t_accel = MAX_ACCELERATION / MAX_JERK;
        let t_veloc = MAX_VELOCITY / MAX_ACCELERATION;
      
        let decel_t = if (est.velocity>0.0) == (est.acceleration>0.0) {
            (est.acceleration.abs() / MAX_JERK) +
            (est.velocity.abs() / (MAX_ACCELERATION / 2.0)) +
            2.0 * (MAX_ACCELERATION / MAX_JERK)
        } else {
            est.velocity.abs() / (MAX_JERK + est.acceleration.abs())
        };
      
        let d = est.velocity.abs() * decel_t;

        let l = (est.location - (dst as f64)*self.esp.floor_height).abs();

        let target_acceleration = {
            let going_up = est.location < (dst as f64)*self.esp.floor_height;

            let dt = est.timestamp - self.timestamp;
            self.timestamp = est.timestamp;

            if est.acceleration.abs() >= MAX_ACCELERATION {
                if est.acceleration > 0.0 {
                    est.acceleration - (dt * MAX_JERK)
                } else {
                    est.acceleration + (dt * MAX_JERK)
                }
            } else if est.velocity.abs() >= MAX_VELOCITY || (est.velocity + est.acceleration * (est.acceleration.abs() / MAX_JERK)).abs() >= MAX_VELOCITY {
                if est.velocity > 0.0 {
                    est.acceleration - (dt * MAX_JERK)
                } else {
                    est.acceleration + (dt * MAX_JERK)
                }
            } else if l < d && (est.velocity>0.0) == going_up {
                if going_up {
                    est.acceleration - (dt * MAX_JERK)
                } else {
                    est.acceleration + (dt * MAX_JERK)
                }
            } else {
                if going_up {
                    est.acceleration + (dt * MAX_JERK)
                } else {
                    est.acceleration - (dt * MAX_JERK)
                }
            }
        };

        let gravity_adjusted_acceleration = target_acceleration + 9.8;
        let target_force = gravity_adjusted_acceleration * self.esp.carriage_weight;
        let target_voltage = target_force / 8.0;
        if !target_voltage.is_finite() {
            MotorInput::Up { voltage: 0.0 }
        } else if target_voltage > 0.0 {
            MotorInput::Up { voltage: target_voltage }
        } else {
            MotorInput::Down { voltage: target_voltage.abs() }
        }
    }
}
