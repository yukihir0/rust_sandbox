extern crate floating_duration;
extern crate termion;

use std::env;
use std::io::{self, Read, Write};
use std::fs::File;
use std::time;
use std::time::{Instant};
use floating_duration::TimeAsFloat;
use std::thread;
use termion::{clear, cursor};
use termion::raw;
use termion::raw::IntoRawMode;
use std::cmp;

fn variable_summary<W: Write>(stdout: &mut raw::RawTerminal<W>, vname: &str, data: Vec<f64>) {
    let (avg, dev) = variable_summary_stats(data);
    variable_summary_print(stdout, vname, avg, dev);
}

fn variable_summary_stats(data: Vec<f64>) -> (f64, f64) {
   let len_n = data.len();
   let sum: f64 = data.iter().sum();
   let avg = sum / (len_n as f64);
   let dev = (
       data.clone().into_iter()
       .map(|v| (v - avg).powi(2))
       .fold(0.0, |a, b| a+b)
       / (len_n as f64)
   ).sqrt();
   (avg, dev)
}

fn variable_summary_print<W: Write>(stdout: &mut raw::RawTerminal<W>, vname: &str, avg: f64, dev: f64) {
   write!(stdout, "Average of {:25}{:.6}\r\n", vname, avg);
   write!(stdout, "Standard deviation of {:14}{:.6}\r\n", vname, dev);
   write!(stdout, "\r\n");
}

pub fn run_simulation() {
    let mut location: f64 = 0.0;
    let mut velocity: f64 = 0.0;
    let mut acceleration: f64 = 0.0;

    let mut up_input_voltage: f64 = 0.0;
    let mut down_input_voltage: f64 = 0.0;

    let mut floor_count: u64 = 0;
    let mut floor_height: f64 = 0.0;
    let mut floor_requests: Vec<u64> = Vec::new();

    let buffer = match env::args().nth(1) {
        Some(ref fp) if *fp == "-".to_string() => {
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer)
                .expect("read_to_string failed");
            buffer
        },
        None => {
            let fp = "test1.txt";
            let mut buffer = String::new();
            File::open(fp)
                .expect("File::open failed")
                .read_to_string(&mut buffer)
                .expect("rad_to_string failed");
            buffer
        },
        Some(fp) => {
            let mut buffer = String::new();
            File::open(fp)
                .expect("File::open failed")
                .read_to_string(&mut buffer)
                .expect("rad_to_string failed");
            buffer
        }
    };

    for (li, l) in buffer.lines().enumerate() {
        if li == 0 {
            floor_count = l.parse::<u64>().unwrap();
        } else if li == 1 {
            floor_height = l.parse::<f64>().unwrap();
        } else {
            floor_requests.push(l.parse::<u64>().unwrap());
        }
    }
    
    let mut prev_loop_time = Instant::now();
   
    let termsize = termion::terminal_size().ok();
    let termwidth = termsize.map(|(w,_)| w-2).expect("termwidth");
    let termheight = termsize.map(|(_,h)| h-2).expect("termheight");
    let mut _stdout = io::stdout();
    let mut stdout = _stdout.lock().into_raw_mode().unwrap();

    let mut record_location: Vec<f64> = Vec::new();
    let mut record_velocity: Vec<f64> = Vec::new();
    let mut record_acceleration: Vec<f64> = Vec::new();
    let mut record_voltage: Vec<f64> = Vec::new();

    while floor_requests.len() > 0 {
        let now = Instant::now();
        let dt = now.duration_since(prev_loop_time)
            .as_fractional_secs();
        prev_loop_time = now;

        record_location.push(location);
        record_velocity.push(velocity);
        record_acceleration.push(acceleration);
        record_voltage.push(up_input_voltage-down_input_voltage);

        location = location + velocity * dt;
        velocity = velocity + acceleration * dt;
        acceleration = {
            let force = (up_input_voltage - down_input_voltage) * 8.0;
            let m = 1200000.0;
            -9.8 + force / m
        };

        let next_floor = floor_requests[0];
        if (location - (next_floor as f64) * floor_height).abs() < 0.01 && velocity < 0.01 {
            velocity = 0.0;
            floor_requests.remove(0);
        }

        let t = velocity.abs() / 1.0;
        let d = t * (velocity / 2.0);
        let l = (location - (next_floor as f64) * floor_height).abs();

        let target_acceleration = {
            let going_up = location < (next_floor as f64) * floor_height;

            if velocity.abs() >= 5.0 {
                if (going_up && velocity > 0.0) || (!going_up && velocity < 0.0) {
                    0.0
                } else if going_up {
                    1.0
                } else {
                    -1.0
                }
            } else if l < d && going_up == (velocity > 0.0) {
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
        let target_force = gravity_adjusted_acceleration * 1200000.0;
        let target_voltage = target_force / 8.0;
        if target_voltage > 0.0 {
            up_input_voltage = target_voltage;
            down_input_voltage = 0.0;
        } else {
            up_input_voltage = 0.0;
            down_input_voltage = target_voltage;
        }

        print!("{}{}{}", clear::All, cursor::Goto(1, 1), cursor::Hide);
        let carriage_floor = (location / floor_height).floor();
        let carriage_floor = if carriage_floor < 1.0 { 0 } else { carriage_floor as u64 };
        let carriage_floor = cmp::min(carriage_floor, floor_count-1);
        let mut terminal_buffer = vec![' ' as u8; (termwidth*termheight) as usize];
        for ty in 0..floor_count {
            terminal_buffer[ (ty*(termwidth as u64) + 0) as usize ] = '[' as u8;
            terminal_buffer[ (ty*(termwidth as u64) + 1) as usize ] =
                if   (ty as u64)==((floor_count-1)-carriage_floor) { 'X' as u8 }
                else { ' ' as u8 };
            terminal_buffer[ (ty*(termwidth as u64) + 2) as usize ] = ']' as u8;
            terminal_buffer[ (ty*(termwidth as u64) + (termwidth as u64)-2) as usize ] = '\r' as u8;
            terminal_buffer[ (ty*(termwidth as u64)+ (termwidth as u64)-1) as usize ] = '\n' as u8;
        }
        let stats = vec![
            format!("Carriage at floor {}", carriage_floor+1),
            format!("Location          {:.06}", location),
            format!("Velocity          {:.06}", velocity),
            format!("Acceleration      {:.06}", acceleration),
            format!("Voltage [up-down] {:.06}", up_input_voltage-down_input_voltage),
        ];
        for sy in 0..stats.len() {
            for (sx,sc) in stats[sy].chars().enumerate() {
                terminal_buffer[ sy*(termwidth as usize) + 6 + sx ] = sc as u8;
            }
        }
        write!(stdout, "{}", String::from_utf8(terminal_buffer).unwrap());
        stdout.flush().unwrap();
        
        thread::sleep(time::Duration::from_millis(10));
    }

    write!(stdout, "{}{}{}", clear::All, cursor::Goto(1, 1), cursor::Show).unwrap();
    variable_summary(&mut stdout, "location", record_location);
    variable_summary(&mut stdout, "velocity", record_velocity);
    variable_summary(&mut stdout, "acceleration", record_acceleration);
    variable_summary(&mut stdout, "voltage", record_voltage);
    stdout.flush().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variable_stats() {
        let test_data = vec![
           (vec![1.0, 2.0, 3.0, 4.0, 5.0], 3.0, 1.41),
           (vec![1.0, 3.0, 5.0, 7.0, 9.0], 5.0, 2.83),
           (vec![1.0, 9.0, 1.0, 9.0, 1.0], 4.2, 3.92),
           (vec![1.0, 0.5, 0.7, 0.9, 0.6], 0.74, 0.19),
           (vec![200.0, 3.0, 24.0, 92.0, 111.0], 86.0, 69.84),
        ];
        for (data, avg, dev) in test_data {
           let (ravg, rdev) = variable_summary_stats(data);
           assert!( (avg-ravg).abs() < 0.1 );
           assert!( (dev-rdev).abs() < 0.1 );
        }
    }
}

