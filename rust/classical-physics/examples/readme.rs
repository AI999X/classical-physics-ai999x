//! Classical Physics: 15 calculations through the Rust library (the same engine as the desktop app, the
//! command line and the browser). Run from the rust/ folder:
//!     cargo run --release --example readme -p classical-physics-ai999x
//! Inputs are text, read like a form field ("2 m", "0.5", "x^3 - 3x"); a JSON list stays a list.

use serde_json::{json, Value};

/// One calculation: the request in, its outcome and every value with its unit out.
fn show(request: Value) {
    let answer: Value = serde_json::from_str(&classical_physics_ai999x::solve_json(&request.to_string())).unwrap();
    println!("{}", answer["outcome"].as_str().unwrap_or(""));
    for v in answer["values"].as_array().into_iter().flatten() {
        let unit = v[2].as_str().unwrap_or("");
        let unit = if unit.is_empty() || unit == "1" { String::new() } else { format!(" {unit}") };
        println!("{} = {}{}", v[0].as_str().unwrap_or(""), v[1], unit);
    }
}

fn main() {
    show(json!({"key": "capacitive_reactance", "inputs": {"f": "60 Hz", "C": "1e-06 F"}}));
    println!();
    show(json!({"key": "parallel_plate_capacitor", "inputs": {"A": "0.01 m^2", "d": "0.001 m", "eps0": "8.854187813e-12 C^2/(N*m^2)"}}));
    println!();
    show(json!({"key": "ohm_power_current", "inputs": {"I": "2 A", "R": "5 ohm"}}));
    println!();
    show(json!({"key": "angular_frequency", "inputs": {"f": "0.5 Hz"}}));
    println!();
    show(json!({"key": "elastic_collision_first_final", "inputs": {"m1": "2 kg", "m2": "1 kg", "v1": "3 m/s", "v2": "0 m/s"}}));
    println!();
    show(json!({"key": "terminal_voltage_internal_resistance", "inputs": {"emf": "12 V", "r": "0.5 ohm", "R": "5.5 ohm"}}));
    println!();
    show(json!({"key": "acceleration_frictionless_incline", "inputs": {"g": "9.8 m/s^2", "theta": "30 deg"}}));
    println!();
    show(json!({"key": "bulk_modulus_compression", "inputs": {"dP": "20 MPa", "V0": "1 m^3", "B": "2.2 GPa"}}));
    println!();
    show(json!({"key": "charge_current", "inputs": {"I": "2 A", "t": "30 s"}}));
    println!();
    show(json!({"key": "faraday_induced_emf", "inputs": {"N": "200", "A1": "100 cm^2", "B1": "0 T", "B2": "0.5 T", "dt": "0.1 s"}}));
    println!();
    show(json!({"key": "coulomb_force", "inputs": {"q1": "2 uC", "q2": "3 uC", "r": "10 cm"}}));
    println!();
    show(json!({"key": "power_energy", "inputs": {"E": "600 J", "t": "20 s"}}));
    println!();
    show(json!({"key": "absolute_pressure_depth", "inputs": {"P0": "101325 Pa", "rho": "1000 kg/m^3", "g": "9.80665 m/s^2", "h": "10 m"}}));
    println!();
    show(json!({"key": "static_friction_incline", "inputs": {"mu_s": "0.5 1", "theta": "20 deg"}}));
    println!();
    show(json!({"key": "critical_angle_total_internal_reflection", "inputs": {"n1": "1.5 1", "n2": "1 1"}}));
}
