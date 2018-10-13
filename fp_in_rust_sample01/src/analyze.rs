mod physics;
mod motor;

use physics::{ElevatorSpecification, ElevatorState, MotorInput, simulate_elevator, DataRecorder, MotorController, MotorVoltage};

#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate floating_duration;
use std::time::Instant;
use std::env;
use std::fs::File;
use std::io::{self, Read, Write, BufRead, BufReader};
use std::io::prelude::*;

#[derive(Clone)]
struct Trip {
    dst: u64,
    up: f64,
    down: f64
}

const MAX_JERK: f64 = 0.2;
const MAX_ACCELERATION: f64 = 2.0;
const MAX_VELOCITY: f64 = 5.0;

fn main() {
    let simlog = File::open("simulation.log").expect("read simulation log");
    let mut simlog = BufReader::new(&simlog);
    let mut jerk = 0.0;
    let mut prev_est: Option<ElevatorState> = None;
    let mut dst_timing: Vec<Trip> = Vec::new();
    let mut start_location = 0.0;

    let mut first_line = String::new();
    let len = simlog.read_line(&mut first_line).unwrap();
    let esp: ElevatorSpecification = serde_json::from_str(&first_line).unwrap();

    for line in simlog.lines() {
       let l = line.unwrap();
       let (est, dst): (ElevatorState,u64) = serde_json::from_str(&l).unwrap();
       let dl = dst_timing.len();
       if dst_timing.len()==0 || dst_timing[dl-1].dst != dst {
           dst_timing.push(Trip { dst:dst, up:0.0, down:0.0 });
       }

       if let Some(prev_est) = prev_est {
           let dt = est.timestamp - prev_est.timestamp;
           if est.velocity > 0.0 {
               dst_timing[dl-1].up += dt;
           } else {
               dst_timing[dl-1].down += dt;
           }
           
           let da = (est.acceleration - prev_est.acceleration).abs();
           jerk = (jerk * (1.0 - dt)) + (da * dt);
           if jerk.abs() > 0.22 {
               panic!("jerk is outside of acceptable limits: {} {:?}", jerk, est)
           }
       } else {
           start_location = est.location;
       }
      
       if est.acceleration.abs() > 2.2 {
           panic!("acceleration is outside of acceptable limits: {:?}", est)
       }
       
       if est.velocity.abs() > 5.5 {
           panic!("velocity is outside of acceptable limits: {:?}", est)
       }
      
       prev_est = Some(est);
    }

    let mut total_time = 0.0;
    let mut total_direct = 0.0;
    for trip in dst_timing.clone() {
        total_time += (trip.up + trip.down);
        if trip.up > trip.down {
            total_direct += trip.up;
        } else {
            total_direct += trip.down;
        }
    }
    
    if (total_direct / total_time) < 0.9 {
        panic!("elevator back up is too common: {}", total_direct / total_time)
    }
   
    let mut trip_start_location = start_location;
    let mut theoretical_time = 0.0;
    let floor_height = esp.floor_height;
    for trip in dst_timing.clone() {
        let next_floor = (trip.dst as f64) * floor_height;
        let d = (trip_start_location - next_floor).abs();
        theoretical_time += (
            2.0*(MAX_ACCELERATION / MAX_JERK) +
            2.0*(MAX_JERK / MAX_ACCELERATION) +
            d / MAX_VELOCITY
        );
        trip_start_location = next_floor;
     }
   
    if total_time > (theoretical_time * 1.2) {
        panic!("elevator moves to slow {} {}", total_time, theoretical_time * 1.2)
    }
    
    println!("All simulation checks passing.");
}
