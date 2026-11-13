//! classical_physics: procedures translated from the package's Python by tools/py2rust.py (do not edit; regenerate).
//! Source: models/classical_physics/capabilities/procedures.py. Each keeps Python's number rules through
//! ax_engine::py and is checked against the package by tools/rust_accuracy.py before the build is listed.
#![allow(clippy::all, non_snake_case, unused_parens, unused_braces, unused_mut, unused_variables, unreachable_code)]

use ax_engine::py::{self, Py, PyErr, R};
use ax_engine::Val;
use serde_json::Value;
use std::collections::HashMap;

/// The procedures this crate implements (tools/export_rust_pack.py reads this list).
pub const NATIVE: &[&str] = &[
    "angular_momentum_conservation",
    "beam_support_reactions",
    "bulk_modulus_compression",
    "centre_of_mass_point_masses",
    "coulomb_force",
    "critical_angle_total_internal_reflection",
    "diffraction_grating_order",
    "double_slit_bright_fringe",
    "faraday_induced_emf",
    "gauss_law_symmetric_field",
    "gravitational_force_two_masses",
    "ideal_transformer_secondary",
    "lens_mirror_magnification",
    "malus_law_polarizer",
    "rotational_second_law",
    "series_rlc_ac_circuit",
    "shear_modulus_deformation",
    "single_slit_minimum",
    "snell_refraction_angle",
    "sound_intensity_level",
    "sound_level_change",
    "standing_wave_harmonics",
    "static_friction_incline",
    "static_friction_limit",
    "string_wave_speed",
    "terminal_voltage_internal_resistance",
    "vector_components_resultant",
    "youngs_modulus_deformation",
];

type Out = Result<(Vec<(String, Value)>, f64), String>;

/// The run's result as the engine's answer: a dict of outputs and the residual of the package's own check; a
/// ValueError is the refusal's reason; any other error is an internal error (None).
fn finish(r: R, residual: impl Fn(&Py, &Py) -> f64, v: &HashMap<String, Val>) -> Option<Out> {
    let o = match r {
        Ok(o) => o,
        Err(PyErr::Value(m)) => return Some(Err(m)),
        Err(e) => {
            if std::env::var_os("AX_DEBUG").is_some() {        // the error behind an internal error, for diagnosis
                eprintln!("internal error: {e:?}");
            }
            return None;
        }
    };
    let Py::Dict(d) = &o else { return None };
    let outs: Vec<(String, Value)> = d.borrow().iter().map(|(k, x)| (py::to_str(k), py::to_json(x))).collect();
    let i = Py::dict(v.iter().map(|(k, x)| (Py::str(k), py::from_val(x))).collect());
    let res = residual(&i, &o);
    if std::env::var_os("AX_DEBUG").is_some() && !(res <= 1e-9) {
        eprintln!("check failed: residual {res}");
    }
    Some(Ok((outs, res)))
}

/// The engine's entry point (ax_engine::pack::NativeFn): None for a key that is not one of these procedures.
pub fn native(key: &str, v: &HashMap<String, Val>) -> Option<Out> {
    match key {
        "angular_momentum_conservation" => {
            let r = f_angular_momentum_conservation(match v.get("I1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("omega1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("I2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f_verify_spin(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "beam_support_reactions" => {
            let r = f_beam_reactions(match v.get("loads") { Some(x) => py::from_val(x), None => { return None } }, match v.get("positions") { Some(x) => py::from_val(x), None => { return None } }, match v.get("xA") { Some(x) => py::from_val(x), None => { return None } }, match v.get("xB") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f_verify_beam(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bulk_modulus_compression" => {
            let r = f_bulk_compression(match v.get("dP") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("B") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f_verify_bulk(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "centre_of_mass_point_masses" => {
            let r = f_centre_of_mass(match v.get("masses") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f_verify_com(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "coulomb_force" => {
            let r = f_coulomb_force(match v.get("q1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("q2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f_verify_coulomb(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "critical_angle_total_internal_reflection" => {
            let r = f_critical_angle(match v.get("n1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f_verify_critical(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "diffraction_grating_order" => {
            let r = f_diffraction_grating(match v.get("lam") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("N") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f_verify_grating(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "double_slit_bright_fringe" => {
            let r = f_double_slit(match v.get("d") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lam") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f_verify_double_slit(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "faraday_induced_emf" => {
            let r = f_faraday_emf(match v.get("N") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("B1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("B2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dt") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A2") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("theta") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f_verify_faraday(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "gauss_law_symmetric_field" => {
            let r = f_gauss_field(match v.get("geometry") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("L") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("A") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("R") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f_verify_gauss(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "gravitational_force_two_masses" => {
            let r = f_gravitational_force(match v.get("m1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f_verify_gravity(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ideal_transformer_secondary" => {
            let r = f_ideal_transformer(match v.get("Np") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ns") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vp") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ip") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f_verify_transformer(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "lens_mirror_magnification" => {
            let r = f_lens_mirror_magnification(match v.get("d_o") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h_o") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d_i") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("f") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f_verify_magnification(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "malus_law_polarizer" => {
            let r = f_malus_law(match v.get("I0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("theta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("light") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("polarized")) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f_verify_malus(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "rotational_second_law" => {
            let r = f_rotational_second_law(match v.get("tau") { Some(x) => py::from_val(x), None => { return None } }, match v.get("I") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f_verify_rotation(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "series_rlc_ac_circuit" => {
            let r = f_series_rlc_ac(match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f_verify_rlc(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "shear_modulus_deformation" => {
            let r = f_shear_deformation(match v.get("F") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f_verify_shear(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "single_slit_minimum" => {
            let r = f_single_slit(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lam") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f_verify_single_slit(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "snell_refraction_angle" => {
            let r = f_snell_refraction(match v.get("n1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("theta1") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f_verify_snell(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "sound_intensity_level" => {
            let r = f_sound_intensity_level(match v.get("I") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("beta") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f_verify_level(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "sound_level_change" => {
            let r = f_sound_level_change(match v.get("beta1") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("beta2") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("ratio") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f_verify_level_change(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "standing_wave_harmonics" => {
            let r = f_standing_wave_harmonic(match v.get("system") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f_verify_standing(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "static_friction_incline" => {
            let r = f_static_friction_incline(match v.get("mu_s") { Some(x) => py::from_val(x), None => { return None } }, match v.get("theta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f_verify_incline(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "static_friction_limit" => {
            let r = f_static_friction_limit(match v.get("mu_s") { Some(x) => py::from_val(x), None => { return None } }, match v.get("N") { Some(x) => py::from_val(x), None => { return None } }, match v.get("F") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f_verify_static_limit(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "string_wave_speed" => {
            let r = f_string_wave_speed(match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f_verify_string(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "terminal_voltage_internal_resistance" => {
            let r = f_terminal_voltage(match v.get("emf") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f_verify_terminal(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "vector_components_resultant" => {
            let r = f_vector_resultant(match v.get("magnitudes") { Some(x) => py::from_val(x), None => { return None } }, match v.get("directions") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f_verify_vectors(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "youngs_modulus_deformation" => {
            let r = f_young_deformation(match v.get("F") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("E") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f_verify_young(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        _ => None,
    }
}

#[allow(non_snake_case)]
fn c_G0() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(9.80665)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_G_NEWTON() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(6.6743e-11)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_I0_HEAR() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(1e-12)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_KE() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(8987551792.3)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__GAUSS() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::tuple(vec![Py::str("line"), Py::str("plane"), Py::str("sphere"), Py::str("shell")])) })());
    V.with(|v| v.clone())
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__cos_sin_deg(mut v_d: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_t: Py = Py::None;
    v_a = py::modulo(&v_d.clone(), &Py::Float(360.0))?;
    if ({ let t654 = v_a.clone(); Py::Bool({ let t655 = Py::Int(0i64); if py::eq(&t654, &t655) { true } else { false } }) }).truthy() {
        return Ok(Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)]));
    }
    if ({ let t656 = v_a.clone(); Py::Bool({ let t657 = Py::Int(90i64); if py::eq(&t656, &t657) { true } else { false } }) }).truthy() {
        return Ok(Py::tuple(vec![Py::Float(0.0), Py::Float(1.0)]));
    }
    if ({ let t658 = v_a.clone(); Py::Bool({ let t659 = Py::Int(180i64); if py::eq(&t658, &t659) { true } else { false } }) }).truthy() {
        return Ok(Py::tuple(vec![py::neg(&Py::Float(1.0))?, Py::Float(0.0)]));
    }
    if ({ let t660 = v_a.clone(); Py::Bool({ let t661 = Py::Int(270i64); if py::eq(&t660, &t661) { true } else { false } }) }).truthy() {
        return Ok(Py::tuple(vec![Py::Float(0.0), py::neg(&Py::Float(1.0))?]));
    }
    v_t = py::math1("radians", &v_a.clone())?;
    return Ok(Py::tuple(vec![py::math1("cos", &v_t.clone())?, py::math1("sin", &v_t.clone())?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__cos_snap(mut v_theta: Py) -> R {
    let mut v_c: Py = Py::None;
    v_c = py::math1("cos", &v_theta.clone())?;
    if ({ let t369 = py::abs(&v_c.clone())?; Py::Bool({ let t370 = Py::Float(1e-15); if py::lt(&t369, &t370)? { true } else { false } }) }).truthy() {
        v_c = Py::Float(0.0);
    }
    return Ok(v_c.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__index_ok(mut v_n: Py, mut v_name: Py) -> R {
    if (py::not(&{ let t454 = py::math1("isfinite", &v_n.clone())?; if !t454.truthy() { t454 } else { { let t452 = v_n.clone(); Py::Bool({ let t453 = Py::Int(1i64); if py::ge(&t452, &t453)? { true } else { false } }) } } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must be a refractive index of at least 1".to_string()].concat()))));
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__pipe_length_factor(mut v_system: Py) -> R {
    return Ok((if ({ let t547 = v_system.clone(); Py::Bool({ let t548 = Py::str("closed pipe"); if py::eq(&t547, &t548) { true } else { false } }) }).truthy() { Py::Float(4.0) } else { Py::Float(2.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__positive(mut v_x: Py, mut v_what: Py) -> R {
    if (py::not(&{ let t700 = py::math1("isfinite", &v_x.clone())?; if !t700.truthy() { t700 } else { { let t698 = v_x.clone(); Py::Bool({ let t699 = Py::Int(0i64); if py::gt(&t698, &t699)? { true } else { false } }) } } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_what.clone(), "")?, " must be positive".to_string()].concat()))));
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__r(mut v_x: Py) -> R {
    if ({ let t665 = { let t663 = v_x.clone(); Py::Bool({ let t664 = Py::Int(0i64); if py::eq(&t663, &t664) { true } else { false } }) }; if t665.truthy() { t665 } else { py::not(&py::math1("isfinite", &v_x.clone())?) } }).truthy() {
        return Ok((if ({ let t666 = v_x.clone(); Py::Bool({ let t667 = Py::Int(0i64); if py::eq(&t666, &t667) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { v_x.clone() }));
    }
    return Ok(py::add(&py::float(&Py::str(&[py::format(&v_x.clone(), ".12g")?].concat()))?, &Py::Float(0.0))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__rel(mut v_a: Py, mut v_b: Py) -> R {
    let mut v_d: Py = Py::None;
    if (py::not(&{ let t708 = py::math1("isfinite", &v_a.clone())?; if !t708.truthy() { t708 } else { py::math1("isfinite", &v_b.clone())? } })).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_d = py::abs(&py::sub(&v_a.clone(), &v_b.clone())?)?;
    if ({ let t709 = v_d.clone(); Py::Bool({ let t710 = Py::Int(0i64); if py::eq(&t709, &t710) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    return Ok(py::truediv(&v_d.clone(), &py::minmax(&[py::abs(&v_a.clone())?, py::abs(&v_b.clone())?], true)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__slips_incline(mut v_mu_s: Py, mut v_theta: Py) -> R {
    return Ok({ let t566 = py::math1("sin", &v_theta.clone())?; Py::Bool({ let t567 = py::mul(&py::mul(&v_mu_s.clone(), &py::math1("cos", &v_theta.clone())?)?, &py::add(&Py::Int(1i64), &Py::Float(1e-12))?)?; if py::gt(&t566, &t567)? { true } else { false } }) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__slips_level(mut v_mu_s: Py, mut v_N: Py, mut v_F: Py) -> R {
    return Ok({ let t606 = v_F.clone(); Py::Bool({ let t607 = py::mul(&py::mul(&v_mu_s.clone(), &v_N.clone())?, &py::add(&Py::Int(1i64), &Py::Float(1e-12))?)?; if py::gt(&t606, &t607)? { true } else { false } }) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_angular_momentum_conservation(mut v_I1: Py, mut v_omega1: Py, mut v_I2: Py) -> R {
    let mut v_K1: Py = Py::None;
    let mut v_K2: Py = Py::None;
    let mut v_w2: Py = Py::None;
    let _ = f__positive(v_I1.clone(), Py::str("the initial moment of inertia I1"))?;
    let _ = f__positive(v_I2.clone(), Py::str("the final moment of inertia I2"))?;
    v_w2 = py::truediv(&py::mul(&v_I1.clone(), &v_omega1.clone())?, &v_I2.clone())?;
    v_K1 = py::truediv(&py::mul(&py::mul(&v_I1.clone(), &v_omega1.clone())?, &v_omega1.clone())?, &Py::Int(2i64))?;
    v_K2 = py::truediv(&py::mul(&py::mul(&v_I2.clone(), &v_w2.clone())?, &v_w2.clone())?, &Py::Int(2i64))?;
    return Ok(Py::dict(vec![(Py::str("omega2"), py::add(&v_w2.clone(), &Py::Float(0.0))?), (Py::str("L"), py::add(&py::mul(&v_I1.clone(), &v_omega1.clone())?, &Py::Float(0.0))?), (Py::str("K1"), v_K1.clone()), (Py::str("K2"), v_K2.clone()), (Py::str("dK"), py::sub(&v_K2.clone(), &v_K1.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_beam_reactions(mut v_loads: Py, mut v_positions: Py, mut v_xA: Py, mut v_xB: Py) -> R {
    let mut v_RA: Py = Py::None;
    let mut v_RB: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_moment: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_scale: Py = Py::None;
    let mut v_total: Py = Py::None;
    v_n = py::len(&v_loads.clone())?;
    if ({ let t13 = { let t9 = v_n.clone(); Py::Bool({ let t10 = Py::Int(1i64); if py::lt(&t9, &t10)? { true } else { false } }) }; if t13.truthy() { t13 } else { { let t11 = py::len(&v_positions.clone())?; Py::Bool({ let t12 = v_n.clone(); if py::ne(&t11, &t12) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give one position for every load"))));
    }
    if ({ let t14 = v_xA.clone(); Py::Bool({ let t15 = v_xB.clone(); if py::eq(&t14, &t15) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the two supports must be at different points"))));
    }
    let t16 = (Py::tuple(vec![Py::Float(0.0), Py::Float(0.0), Py::Float(0.0)])).items()?;
    if t16.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_total = t16[0].clone();
    v_moment = t16[1].clone();
    v_scale = t16[2].clone();
    for t17 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_k = t17;
        v_total = py::iadd(&v_total, &py::getitem(&v_loads.clone(), &v_k.clone())?)?;
        v_moment = py::iadd(&v_moment, &py::mul(&py::getitem(&v_loads.clone(), &v_k.clone())?, &py::sub(&py::getitem(&v_positions.clone(), &v_k.clone())?, &v_xA.clone())?)?)?;
        v_scale = py::iadd(&v_scale, &py::abs(&py::getitem(&v_loads.clone(), &v_k.clone())?)?)?;
    }
    v_RB = py::truediv(&v_moment.clone(), &py::sub(&v_xB.clone(), &v_xA.clone())?)?;
    v_RA = py::sub(&v_total.clone(), &v_RB.clone())?;
    if ({ let t18 = py::abs(&v_RA.clone())?; Py::Bool({ let t19 = py::mul(&Py::Float(1e-12), &v_scale.clone())?; if py::le(&t18, &t19)? { true } else { false } }) }).truthy() {
        v_RA = Py::Float(0.0);
    }
    if ({ let t20 = py::abs(&v_RB.clone())?; Py::Bool({ let t21 = py::mul(&Py::Float(1e-12), &v_scale.clone())?; if py::le(&t20, &t21)? { true } else { false } }) }).truthy() {
        v_RB = Py::Float(0.0);
    }
    return Ok(Py::dict(vec![(Py::str("RA"), py::add(&v_RA.clone(), &Py::Float(0.0))?), (Py::str("RB"), py::add(&v_RB.clone(), &Py::Float(0.0))?), (Py::str("total"), v_total.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bulk_compression(mut v_dP: Py, mut v_V0: Py, mut v_B: Py) -> R {
    let mut v_dV: Py = Py::None;
    let mut v_strain: Py = Py::None;
    let _ = f__positive(v_V0.clone(), Py::str("the original volume V0"))?;
    let _ = f__positive(v_B.clone(), Py::str("the bulk modulus B"))?;
    if (py::not(&py::math1("isfinite", &v_dP.clone())?)).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the pressure change must be a finite number"))));
    }
    v_strain = py::truediv(&py::neg(&v_dP.clone())?, &v_B.clone())?;
    if (py::not(&{ let t33 = py::abs(&v_strain.clone())?; Py::Bool({ let t34 = Py::Int(1i64); if py::lt(&t33, &t34)? { true } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a volume change as large as the volume itself is outside linear elasticity"))));
    }
    v_dV = py::mul(&v_strain.clone(), &v_V0.clone())?;
    return Ok(Py::dict(vec![(Py::str("dV"), v_dV.clone()), (Py::str("strain"), v_strain.clone()), (Py::str("V"), py::add(&v_V0.clone(), &v_dV.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_centre_of_mass(mut v_masses: Py, mut v_x: Py, mut v_y: Py) -> R {
    let mut v_M: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_sx: Py = Py::None;
    let mut v_sy: Py = Py::None;
    v_n = py::len(&v_masses.clone())?;
    if ({ let t51 = { let t41 = v_n.clone(); Py::Bool({ let t42 = Py::Int(1i64); if py::lt(&t41, &t42)? { true } else { false } }) }; if t51.truthy() { t51 } else { { let t50 = { let t43 = py::len(&v_x.clone())?; Py::Bool({ let t44 = v_n.clone(); if py::ne(&t43, &t44) { true } else { false } }) }; if t50.truthy() { t50 } else { { let t49 = { let t45 = v_y.clone(); Py::Bool({ let t46 = Py::None; if !py::is(&t45, &t46) { true } else { false } }) }; if !t49.truthy() { t49 } else { { let t47 = py::len(&v_y.clone())?; Py::Bool({ let t48 = v_n.clone(); if py::ne(&t47, &t48) { true } else { false } }) } } } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give one coordinate for every mass"))));
    }
    let t52 = (Py::tuple(vec![Py::Float(0.0), Py::Float(0.0), Py::Float(0.0)])).items()?;
    if t52.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_M = t52[0].clone();
    v_sx = t52[1].clone();
    v_sy = t52[2].clone();
    for t53 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_k = t53;
        if (py::not(&{ let t56 = py::math1("isfinite", &py::getitem(&v_masses.clone(), &v_k.clone())?)?; if !t56.truthy() { t56 } else { { let t54 = py::getitem(&v_masses.clone(), &v_k.clone())?; Py::Bool({ let t55 = Py::Int(0i64); if py::gt(&t54, &t55)? { true } else { false } }) } } })).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("every mass must be positive"))));
        }
        v_M = py::iadd(&v_M, &py::getitem(&v_masses.clone(), &v_k.clone())?)?;
        v_sx = py::iadd(&v_sx, &py::mul(&py::getitem(&v_masses.clone(), &v_k.clone())?, &py::getitem(&v_x.clone(), &v_k.clone())?)?)?;
        if ({ let t57 = v_y.clone(); Py::Bool({ let t58 = Py::None; if !py::is(&t57, &t58) { true } else { false } }) }).truthy() {
            v_sy = py::iadd(&v_sy, &py::mul(&py::getitem(&v_masses.clone(), &v_k.clone())?, &py::getitem(&v_y.clone(), &v_k.clone())?)?)?;
        }
    }
    v_out = Py::dict(vec![(Py::str("M"), v_M.clone()), (Py::str("x_cm"), py::add(&py::truediv(&v_sx.clone(), &v_M.clone())?, &Py::Float(0.0))?)]);
    if ({ let t59 = v_y.clone(); Py::Bool({ let t60 = Py::None; if !py::is(&t59, &t60) { true } else { false } }) }).truthy() {
        py::setitem(&v_out.clone(), &Py::str("y_cm"), py::add(&py::truediv(&v_sy.clone(), &v_M.clone())?, &Py::Float(0.0))?)?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_coulomb_force(mut v_q1: Py, mut v_q2: Py, mut v_r: Py) -> R {
    let mut v_Fr: Py = Py::None;
    if (py::not(&{ let t77 = py::math1("isfinite", &v_r.clone())?; if !t77.truthy() { t77 } else { { let t75 = v_r.clone(); Py::Bool({ let t76 = Py::Int(0i64); if py::gt(&t75, &t76)? { true } else { false } }) } } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the separation r must be positive"))));
    }
    v_Fr = py::truediv(&py::mul(&py::mul(&c_KE()?, &v_q1.clone())?, &v_q2.clone())?, &py::mul(&v_r.clone(), &v_r.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("F"), py::abs(&v_Fr.clone())?), (Py::str("F_radial"), py::add(&v_Fr.clone(), &Py::Float(0.0))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_critical_angle(mut v_n1: Py, mut v_n2: Py) -> R {
    let mut v_t: Py = Py::None;
    let _ = f__index_ok(v_n1.clone(), Py::str("n1"))?;
    let _ = f__index_ok(v_n2.clone(), Py::str("n2"))?;
    if (py::not(&{ let t88 = v_n1.clone(); Py::Bool({ let t89 = v_n2.clone(); if py::gt(&t88, &t89)? { true } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("total internal reflection needs light going from a denser medium (n1) into a less dense one (n2 < n1)"))));
    }
    v_t = py::math1("asin", &py::truediv(&v_n2.clone(), &v_n1.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("theta_c"), f__r(v_t.clone())?), (Py::str("theta_c_deg"), f__r(py::math1("degrees", &v_t.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_diffraction_grating(mut v_lam: Py, mut v_m: Py, mut v_d: Py, mut v_N: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_ratio: Py = Py::None;
    let mut v_th: Py = Py::None;
    if ({ let t103 = { let t105 = v_d.clone(); Py::Bool({ let t106 = Py::None; if py::is(&t105, &t106) { true } else { false } }) }; Py::Bool({ let t104 = { let t107 = v_N.clone(); Py::Bool({ let t108 = Py::None; if py::is(&t107, &t108) { true } else { false } }) }; if py::eq(&t103, &t104) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give exactly one of the line spacing d or the number of lines per millimetre N"))));
    }
    if ({ let t109 = v_d.clone(); Py::Bool({ let t110 = Py::None; if py::is(&t109, &t110) { true } else { false } }) }).truthy() {
        let _ = f__positive(v_N.clone(), Py::str("the number of lines per millimetre N"))?;
        v_d = py::truediv(&Py::Float(0.001), &v_N.clone())?;
    }
    let _ = f__positive(v_d.clone(), Py::str("the line spacing d"))?;
    let _ = f__positive(v_lam.clone(), Py::str("the wavelength"))?;
    if ({ let t114 = v_m.clone(); Py::Bool({ let t115 = Py::Int(1i64); if py::lt(&t114, &t115)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the order m is a whole number 1, 2, 3, ..."))));
    }
    v_ratio = py::truediv(&v_d.clone(), &v_lam.clone())?;
    v_k = py::math1("floor", &v_ratio.clone())?;
    if ({ let t116 = py::abs(&py::sub(&v_ratio.clone(), &py::round(&v_ratio.clone(), None)?)?)?; Py::Bool({ let t117 = py::mul(&Py::Float(1e-09), &v_ratio.clone())?; if py::le(&t116, &t117)? { true } else { false } }) }).truthy() {
        v_k = py::sub(&py::round(&v_ratio.clone(), None)?, &Py::Int(1i64))?;
    }
    if ({ let t118 = v_k.clone(); Py::Bool({ let t119 = Py::Int(1i64); if py::lt(&t118, &t119)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the line spacing is not larger than the wavelength: no order beyond the central maximum"))));
    }
    if ({ let t120 = v_m.clone(); Py::Bool({ let t121 = v_k.clone(); if py::gt(&t120, &t121)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["order ".to_string(), py::format(&v_m.clone(), "")?, " is not seen: the highest order is ".to_string(), py::format(&v_k.clone(), "")?].concat()))));
    }
    v_th = py::math1("asin", &py::truediv(&py::mul(&v_m.clone(), &v_lam.clone())?, &v_d.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("theta"), f__r(v_th.clone())?), (Py::str("theta_deg"), f__r(py::math1("degrees", &v_th.clone())?)?), (Py::str("m_max"), py::int(&v_k.clone())?), (Py::str("d"), v_d.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_double_slit(mut v_d: Py, mut v_lam: Py, mut v_m: Py, mut v_L: Py) -> R {
    let mut v_s: Py = Py::None;
    let mut v_th: Py = Py::None;
    let _ = f__positive(v_d.clone(), Py::str("the slit separation d"))?;
    let _ = f__positive(v_lam.clone(), Py::str("the wavelength"))?;
    let _ = f__positive(v_L.clone(), Py::str("the screen distance L"))?;
    if ({ let t156 = v_m.clone(); Py::Bool({ let t157 = Py::Int(0i64); if py::lt(&t156, &t157)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the order m is a whole number 0, 1, 2, ..."))));
    }
    v_s = py::truediv(&py::mul(&v_m.clone(), &v_lam.clone())?, &v_d.clone())?;
    if ({ let t158 = v_s.clone(); Py::Bool({ let t159 = Py::Int(1i64); if py::ge(&t158, &t159)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["order ".to_string(), py::format(&v_m.clone(), "")?, " does not exist: m lambda must be smaller than the slit separation d".to_string()].concat()))));
    }
    v_th = py::math1("asin", &v_s.clone())?;
    return Ok(Py::dict(vec![(Py::str("theta"), f__r(v_th.clone())?), (Py::str("theta_deg"), f__r(py::math1("degrees", &v_th.clone())?)?), (Py::str("y"), f__r(py::mul(&v_L.clone(), &py::math1("tan", &v_th.clone())?)?)?), (Py::str("dy"), py::truediv(&py::mul(&v_lam.clone(), &v_L.clone())?, &v_d.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_faraday_emf(mut v_N: Py, mut v_A1: Py, mut v_B1: Py, mut v_B2: Py, mut v_dt: Py, mut v_A2: Py, mut v_theta: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_dphi: Py = Py::None;
    let mut v_emf_s: Py = Py::None;
    let mut v_phi1: Py = Py::None;
    let mut v_phi2: Py = Py::None;
    if ({ let t178 = v_N.clone(); Py::Bool({ let t179 = Py::Int(1i64); if py::lt(&t178, &t179)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the number of turns N must be at least 1"))));
    }
    let _ = f__positive(v_dt.clone(), Py::str("the time interval dt"))?;
    if ({ let t183 = v_A2.clone(); Py::Bool({ let t184 = Py::None; if py::is(&t183, &t184) { true } else { false } }) }).truthy() {
        v_A2 = v_A1.clone();
    }
    if (py::not(&{ let t189 = { let t185 = v_A1.clone(); Py::Bool({ let t186 = Py::Int(0i64); if py::ge(&t185, &t186)? { true } else { false } }) }; if !t189.truthy() { t189 } else { { let t187 = v_A2.clone(); Py::Bool({ let t188 = Py::Int(0i64); if py::ge(&t187, &t188)? { true } else { false } }) } } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("loop areas cannot be negative"))));
    }
    v_c = f__cos_snap(v_theta.clone())?;
    v_phi1 = py::mul(&py::mul(&v_B1.clone(), &v_A1.clone())?, &v_c.clone())?;
    v_phi2 = py::mul(&py::mul(&v_B2.clone(), &v_A2.clone())?, &v_c.clone())?;
    v_dphi = py::sub(&v_phi2.clone(), &v_phi1.clone())?;
    v_emf_s = py::truediv(&py::mul(&py::neg(&v_N.clone())?, &v_dphi.clone())?, &v_dt.clone())?;
    return Ok(Py::dict(vec![(Py::str("phi1"), f__r(v_phi1.clone())?), (Py::str("phi2"), f__r(v_phi2.clone())?), (Py::str("dphi"), f__r(v_dphi.clone())?), (Py::str("emf"), f__r(py::abs(&v_emf_s.clone())?)?), (Py::str("emf_signed"), f__r(v_emf_s.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_gauss_field(mut v_geometry: Py, mut v_Q: Py, mut v_r: Py, mut v_L: Py, mut v_A: Py, mut v_R: Py) -> R {
    let mut v_Er: Py = Py::None;
    let mut v_dens: Py = Py::None;
    let mut v_qenc: Py = Py::None;
    if ({ let t204 = v_geometry.clone(); Py::Bool({ let t205 = c__GAUSS()?; if !py::contains(&t205, &t204)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("geometry must be line, plane, sphere or shell"))));
    }
    if (py::not(&py::math1("isfinite", &v_Q.clone())?)).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the charge Q must be a finite number"))));
    }
    if ({ let t206 = v_r.clone(); Py::Bool({ let t207 = Py::None; if !py::is(&t206, &t207) { true } else { false } }) }).truthy() {
        let _ = f__positive(v_r.clone(), Py::str("the distance r"))?;
    }
    if ({ let t211 = v_geometry.clone(); Py::Bool({ let t212 = Py::str("line"); if py::eq(&t211, &t212) { true } else { false } }) }).truthy() {
        if ({ let t217 = { let t213 = v_L.clone(); Py::Bool({ let t214 = Py::None; if py::is(&t213, &t214) { true } else { false } }) }; if t217.truthy() { t217 } else { { let t215 = v_r.clone(); Py::Bool({ let t216 = Py::None; if py::is(&t215, &t216) { true } else { false } }) } } }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("a line of charge needs the length L that carries Q and the distance r from the line"))));
        }
        if ({ let t222 = { let t218 = v_A.clone(); Py::Bool({ let t219 = Py::None; if !py::is(&t218, &t219) { true } else { false } }) }; if t222.truthy() { t222 } else { { let t220 = v_R.clone(); Py::Bool({ let t221 = Py::None; if !py::is(&t220, &t221) { true } else { false } }) } } }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("a line of charge takes no area A or radius R"))));
        }
        let _ = f__positive(v_L.clone(), Py::str("the length L"))?;
        v_dens = py::truediv(&v_Q.clone(), &v_L.clone())?;
        v_Er = py::truediv(&py::mul(&py::mul(&Py::Float(2.0), &c_KE()?)?, &v_dens.clone())?, &v_r.clone())?;
        v_qenc = v_Q.clone();
    } else {
        if ({ let t223 = v_geometry.clone(); Py::Bool({ let t224 = Py::str("plane"); if py::eq(&t223, &t224) { true } else { false } }) }).truthy() {
            if ({ let t225 = v_A.clone(); Py::Bool({ let t226 = Py::None; if py::is(&t225, &t226) { true } else { false } }) }).truthy() {
                return Err(py::value_error(py::to_str(&Py::str("a plane of charge needs the area A that carries Q"))));
            }
            if ({ let t231 = { let t227 = v_L.clone(); Py::Bool({ let t228 = Py::None; if !py::is(&t227, &t228) { true } else { false } }) }; if t231.truthy() { t231 } else { { let t229 = v_R.clone(); Py::Bool({ let t230 = Py::None; if !py::is(&t229, &t230) { true } else { false } }) } } }).truthy() {
                return Err(py::value_error(py::to_str(&Py::str("a plane of charge takes no length L or radius R"))));
            }
            let _ = f__positive(v_A.clone(), Py::str("the area A"))?;
            v_dens = py::truediv(&v_Q.clone(), &v_A.clone())?;
            v_Er = py::mul(&py::mul(&py::mul(&Py::Float(2.0), &Py::Float(std::f64::consts::PI))?, &c_KE()?)?, &v_dens.clone())?;
            v_qenc = v_Q.clone();
        } else {
            if ({ let t236 = { let t232 = v_R.clone(); Py::Bool({ let t233 = Py::None; if py::is(&t232, &t233) { true } else { false } }) }; if t236.truthy() { t236 } else { { let t234 = v_r.clone(); Py::Bool({ let t235 = Py::None; if py::is(&t234, &t235) { true } else { false } }) } } }).truthy() {
                return Err(py::value_error(py::to_str(&Py::str("a charged sphere or shell needs its radius R and the distance r from its centre"))));
            }
            if ({ let t241 = { let t237 = v_L.clone(); Py::Bool({ let t238 = Py::None; if !py::is(&t237, &t238) { true } else { false } }) }; if t241.truthy() { t241 } else { { let t239 = v_A.clone(); Py::Bool({ let t240 = Py::None; if !py::is(&t239, &t240) { true } else { false } }) } } }).truthy() {
                return Err(py::value_error(py::to_str(&Py::str("a sphere or shell takes no length L or area A"))));
            }
            let _ = f__positive(v_R.clone(), Py::str("the radius R"))?;
            if ({ let t246 = { let t242 = v_geometry.clone(); Py::Bool({ let t243 = Py::str("shell"); if py::eq(&t242, &t243) { true } else { false } }) }; if !t246.truthy() { t246 } else { { let t244 = v_r.clone(); Py::Bool({ let t245 = v_R.clone(); if py::eq(&t244, &t245) { true } else { false } }) } } }).truthy() {
                return Err(py::value_error(py::to_str(&Py::str("on the shell itself (r = R) the field is not defined: it jumps from 0 to kQ/R^2"))));
            }
            if ({ let t247 = v_geometry.clone(); Py::Bool({ let t248 = Py::str("sphere"); if py::eq(&t247, &t248) { true } else { false } }) }).truthy() {
                v_dens = py::truediv(&v_Q.clone(), &py::truediv(&py::mul(&py::mul(&Py::Float(4.0), &Py::Float(std::f64::consts::PI))?, &py::pow(&v_R.clone(), &Py::Int(3i64))?)?, &Py::Float(3.0))?)?;
            } else {
                v_dens = py::truediv(&v_Q.clone(), &py::mul(&py::mul(&py::mul(&Py::Float(4.0), &Py::Float(std::f64::consts::PI))?, &v_R.clone())?, &v_R.clone())?)?;
            }
            if ({ let t249 = v_r.clone(); Py::Bool({ let t250 = v_R.clone(); if py::ge(&t249, &t250)? { true } else { false } }) }).truthy() {
                v_qenc = v_Q.clone();
            } else {
                if ({ let t251 = v_geometry.clone(); Py::Bool({ let t252 = Py::str("sphere"); if py::eq(&t251, &t252) { true } else { false } }) }).truthy() {
                    v_qenc = py::mul(&v_Q.clone(), &py::pow(&py::truediv(&v_r.clone(), &v_R.clone())?, &Py::Int(3i64))?)?;
                } else {
                    v_qenc = Py::Float(0.0);
                }
            }
            v_Er = py::truediv(&py::mul(&c_KE()?, &v_qenc.clone())?, &py::mul(&v_r.clone(), &v_r.clone())?)?;
        }
    }
    return Ok(Py::dict(vec![(Py::str("E"), f__r(py::abs(&v_Er.clone())?)?), (Py::str("E_radial"), f__r(v_Er.clone())?), (Py::str("Q_enc"), f__r(v_qenc.clone())?), (Py::str("flux"), f__r(py::mul(&py::mul(&py::mul(&Py::Float(4.0), &Py::Float(std::f64::consts::PI))?, &c_KE()?)?, &v_qenc.clone())?)?), (Py::str("density"), f__r(v_dens.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_gravitational_force(mut v_m1: Py, mut v_m2: Py, mut v_r: Py) -> R {
    let _ = f__positive(v_m1.clone(), Py::str("the mass m1"))?;
    let _ = f__positive(v_m2.clone(), Py::str("the mass m2"))?;
    if (py::not(&{ let t285 = py::math1("isfinite", &v_r.clone())?; if !t285.truthy() { t285 } else { { let t283 = v_r.clone(); Py::Bool({ let t284 = Py::Int(0i64); if py::gt(&t283, &t284)? { true } else { false } }) } } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the separation r must be positive"))));
    }
    return Ok(Py::dict(vec![(Py::str("F"), py::truediv(&py::mul(&py::mul(&c_G_NEWTON()?, &v_m1.clone())?, &v_m2.clone())?, &py::mul(&v_r.clone(), &v_r.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ideal_transformer(mut v_Np: Py, mut v_Ns: Py, mut v_Vp: Py, mut v_Ip: Py) -> R {
    let mut v_kind: Py = Py::None;
    let mut v_out: Py = Py::None;
    if ({ let t293 = { let t289 = v_Np.clone(); Py::Bool({ let t290 = Py::Int(1i64); if py::lt(&t289, &t290)? { true } else { false } }) }; if t293.truthy() { t293 } else { { let t291 = v_Ns.clone(); Py::Bool({ let t292 = Py::Int(1i64); if py::lt(&t291, &t292)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("both windings need at least one turn"))));
    }
    let _ = f__positive(v_Vp.clone(), Py::str("the primary voltage Vp"))?;
    if ({ let t302 = { let t297 = v_Ip.clone(); Py::Bool({ let t298 = Py::None; if !py::is(&t297, &t298) { true } else { false } }) }; if !t302.truthy() { t302 } else { py::not(&{ let t301 = py::math1("isfinite", &v_Ip.clone())?; if !t301.truthy() { t301 } else { { let t299 = v_Ip.clone(); Py::Bool({ let t300 = Py::Int(0i64); if py::ge(&t299, &t300)? { true } else { false } }) } } }) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the primary current Ip cannot be negative"))));
    }
    v_kind = (if ({ let t303 = v_Ns.clone(); Py::Bool({ let t304 = v_Np.clone(); if py::gt(&t303, &t304)? { true } else { false } }) }).truthy() { Py::str("step-up") } else { (if ({ let t305 = v_Ns.clone(); Py::Bool({ let t306 = v_Np.clone(); if py::lt(&t305, &t306)? { true } else { false } }) }).truthy() { Py::str("step-down") } else { Py::str("isolation (1:1)") }) });
    v_out = Py::dict(vec![(Py::str("Vs"), py::truediv(&py::mul(&v_Vp.clone(), &v_Ns.clone())?, &v_Np.clone())?), (Py::str("ratio"), py::truediv(&v_Ns.clone(), &v_Np.clone())?), (Py::str("type"), v_kind.clone())]);
    if ({ let t307 = v_Ip.clone(); Py::Bool({ let t308 = Py::None; if !py::is(&t307, &t308) { true } else { false } }) }).truthy() {
        py::setitem(&v_out.clone(), &Py::str("Is"), py::truediv(&py::mul(&v_Ip.clone(), &v_Np.clone())?, &v_Ns.clone())?)?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_lens_mirror_magnification(mut v_d_o: Py, mut v_h_o: Py, mut v_d_i: Py, mut v_f: Py) -> R {
    let mut v_image: Py = Py::None;
    let mut v_m: Py = Py::None;
    if (py::not(&{ let t325 = py::math1("isfinite", &v_d_o.clone())?; if !t325.truthy() { t325 } else { { let t323 = v_d_o.clone(); Py::Bool({ let t324 = Py::Int(0i64); if py::gt(&t323, &t324)? { true } else { false } }) } } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the object distance d_o must be positive (a real object)"))));
    }
    if (py::not(&{ let t328 = py::math1("isfinite", &v_h_o.clone())?; if !t328.truthy() { t328 } else { { let t326 = v_h_o.clone(); Py::Bool({ let t327 = Py::Int(0i64); if py::gt(&t326, &t327)? { true } else { false } }) } } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the object height h_o must be positive"))));
    }
    if ({ let t329 = { let t331 = v_d_i.clone(); Py::Bool({ let t332 = Py::None; if py::is(&t331, &t332) { true } else { false } }) }; Py::Bool({ let t330 = { let t333 = v_f.clone(); Py::Bool({ let t334 = Py::None; if py::is(&t333, &t334) { true } else { false } }) }; if py::eq(&t329, &t330) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give exactly one of the image distance d_i or the focal length f"))));
    }
    if ({ let t335 = v_d_i.clone(); Py::Bool({ let t336 = Py::None; if py::is(&t335, &t336) { true } else { false } }) }).truthy() {
        if ({ let t337 = v_f.clone(); Py::Bool({ let t338 = Py::Int(0i64); if py::eq(&t337, &t338) { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("the focal length must not be zero"))));
        }
        if ({ let t339 = v_d_o.clone(); Py::Bool({ let t340 = v_f.clone(); if py::eq(&t339, &t340) { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("the object is at the focal point: the image is at infinity"))));
        }
        v_d_i = py::truediv(&py::mul(&v_d_o.clone(), &v_f.clone())?, &py::sub(&v_d_o.clone(), &v_f.clone())?)?;
    } else {
        if ({ let t341 = v_d_i.clone(); Py::Bool({ let t342 = Py::Int(0i64); if py::eq(&t341, &t342) { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("the image distance must not be zero"))));
        }
    }
    v_m = py::truediv(&py::neg(&v_d_i.clone())?, &v_d_o.clone())?;
    v_image = py::add(&(if ({ let t343 = v_d_i.clone(); Py::Bool({ let t344 = Py::Int(0i64); if py::gt(&t343, &t344)? { true } else { false } }) }).truthy() { Py::str("real") } else { Py::str("virtual") }), &(if ({ let t345 = v_m.clone(); Py::Bool({ let t346 = Py::Int(0i64); if py::lt(&t345, &t346)? { true } else { false } }) }).truthy() { Py::str(", inverted") } else { Py::str(", upright") }))?;
    return Ok(Py::dict(vec![(Py::str("m"), v_m.clone()), (Py::str("h_i"), py::mul(&v_m.clone(), &v_h_o.clone())?), (Py::str("d_i"), v_d_i.clone()), (Py::str("image"), v_image.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_malus_law(mut v_I0: Py, mut v_theta: Py, mut v_light: Py) -> R {
    let mut v_I: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_k: Py = Py::None;
    let _ = f__positive(v_I0.clone(), Py::str("the incident intensity I0"))?;
    if (py::not(&py::math1("isfinite", &v_theta.clone())?)).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the angle must be a finite number"))));
    }
    v_c = f__cos_snap(v_theta.clone())?;
    v_k = (if ({ let t371 = v_light.clone(); Py::Bool({ let t372 = Py::str("polarized"); if py::eq(&t371, &t372) { true } else { false } }) }).truthy() { Py::Float(1.0) } else { Py::Float(0.5) });
    v_I = py::mul(&py::mul(&py::mul(&v_k.clone(), &v_I0.clone())?, &v_c.clone())?, &v_c.clone())?;
    return Ok(Py::dict(vec![(Py::str("I"), f__r(v_I.clone())?), (Py::str("fraction"), f__r(py::truediv(&v_I.clone(), &v_I0.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_rotational_second_law(mut v_tau: Py, mut v_I: Py) -> R {
    if (py::not(&{ let t384 = py::math1("isfinite", &v_I.clone())?; if !t384.truthy() { t384 } else { { let t382 = v_I.clone(); Py::Bool({ let t383 = Py::Int(0i64); if py::gt(&t382, &t383)? { true } else { false } }) } } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the moment of inertia must be positive"))));
    }
    return Ok(Py::dict(vec![(Py::str("alpha"), py::add(&py::truediv(&v_tau.clone(), &v_I.clone())?, &Py::Float(0.0))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_series_rlc_ac(mut v_R: Py, mut v_L: Py, mut v_C: Py, mut v_f: Py, mut v_V: Py) -> R {
    let mut v_I: Py = Py::None;
    let mut v_X: Py = Py::None;
    let mut v_XC: Py = Py::None;
    let mut v_XL: Py = Py::None;
    let mut v_Z: Py = Py::None;
    let mut v_phi: Py = Py::None;
    let mut v_w: Py = Py::None;
    let _ = f__positive(v_R.clone(), Py::str("the resistance R"))?;
    let _ = f__positive(v_L.clone(), Py::str("the inductance L"))?;
    let _ = f__positive(v_C.clone(), Py::str("the capacitance C"))?;
    let _ = f__positive(v_f.clone(), Py::str("the frequency f"))?;
    let _ = f__positive(v_V.clone(), Py::str("the rms voltage V"))?;
    v_w = py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &v_f.clone())?;
    v_XL = py::mul(&v_w.clone(), &v_L.clone())?;
    v_XC = py::truediv(&Py::Float(1.0), &py::mul(&v_w.clone(), &v_C.clone())?)?;
    v_X = py::sub(&v_XL.clone(), &v_XC.clone())?;
    if ({ let t391 = py::abs(&v_X.clone())?; Py::Bool({ let t392 = py::mul(&Py::Float(1e-12), &py::minmax(&[v_XL.clone(), v_XC.clone()], true)?)?; if py::le(&t391, &t392)? { true } else { false } }) }).truthy() {
        v_X = Py::Float(0.0);
    }
    v_Z = py::hypot(&[v_R.clone(), v_X.clone()])?;
    v_I = py::truediv(&v_V.clone(), &v_Z.clone())?;
    v_phi = py::atan2(&v_X.clone(), &v_R.clone())?;
    return Ok(Py::dict(vec![(Py::str("Z"), f__r(v_Z.clone())?), (Py::str("I"), f__r(v_I.clone())?), (Py::str("phi"), f__r(v_phi.clone())?), (Py::str("phi_deg"), f__r(py::math1("degrees", &v_phi.clone())?)?), (Py::str("pf"), f__r(py::truediv(&v_R.clone(), &v_Z.clone())?)?), (Py::str("P"), f__r(py::mul(&py::mul(&v_I.clone(), &v_I.clone())?, &v_R.clone())?)?), (Py::str("XL"), v_XL.clone()), (Py::str("XC"), v_XC.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_shear_deformation(mut v_F: Py, mut v_A: Py, mut v_h: Py, mut v_S: Py) -> R {
    let mut v_strain: Py = Py::None;
    let mut v_stress: Py = Py::None;
    let _ = f__positive(v_A.clone(), Py::str("the area A of the sheared face"))?;
    let _ = f__positive(v_h.clone(), Py::str("the height h between the sheared faces"))?;
    let _ = f__positive(v_S.clone(), Py::str("the shear modulus S"))?;
    v_stress = py::truediv(&v_F.clone(), &v_A.clone())?;
    v_strain = py::truediv(&v_stress.clone(), &v_S.clone())?;
    if (py::not(&{ let t411 = py::abs(&v_strain.clone())?; Py::Bool({ let t412 = Py::Int(1i64); if py::lt(&t411, &t412)? { true } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a shear displacement as large as the height is outside linear elasticity"))));
    }
    return Ok(Py::dict(vec![(Py::str("stress"), v_stress.clone()), (Py::str("strain"), v_strain.clone()), (Py::str("dx"), py::mul(&v_strain.clone(), &v_h.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_single_slit(mut v_a: Py, mut v_lam: Py, mut v_m: Py, mut v_L: Py) -> R {
    let mut v_th: Py = Py::None;
    let mut v_th1: Py = Py::None;
    let _ = f__positive(v_a.clone(), Py::str("the slit width a"))?;
    let _ = f__positive(v_lam.clone(), Py::str("the wavelength"))?;
    let _ = f__positive(v_L.clone(), Py::str("the screen distance L"))?;
    if ({ let t424 = v_m.clone(); Py::Bool({ let t425 = Py::Int(1i64); if py::lt(&t424, &t425)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the order m of a dark fringe is a whole number 1, 2, 3, ..."))));
    }
    if (py::not(&{ let t426 = v_lam.clone(); Py::Bool({ let t427 = v_a.clone(); if py::lt(&t426, &t427)? { true } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the slit must be wider than the wavelength for a dark fringe to exist"))));
    }
    if ({ let t428 = py::mul(&v_m.clone(), &v_lam.clone())?; Py::Bool({ let t429 = v_a.clone(); if py::ge(&t428, &t429)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["dark fringe ".to_string(), py::format(&v_m.clone(), "")?, " does not exist: m lambda must be smaller than the slit width a".to_string()].concat()))));
    }
    v_th = py::math1("asin", &py::truediv(&py::mul(&v_m.clone(), &v_lam.clone())?, &v_a.clone())?)?;
    v_th1 = py::math1("asin", &py::truediv(&v_lam.clone(), &v_a.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("theta"), f__r(v_th.clone())?), (Py::str("theta_deg"), f__r(py::math1("degrees", &v_th.clone())?)?), (Py::str("y"), f__r(py::mul(&v_L.clone(), &py::math1("tan", &v_th.clone())?)?)?), (Py::str("W"), f__r(py::mul(&py::mul(&Py::Int(2i64), &v_L.clone())?, &py::math1("tan", &v_th1.clone())?)?)?), (Py::str("W_angle"), f__r(py::mul(&Py::Int(2i64), &v_th1.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_snell_refraction(mut v_n1: Py, mut v_n2: Py, mut v_theta1: Py) -> R {
    let mut v_s: Py = Py::None;
    let mut v_t2: Py = Py::None;
    let _ = f__index_ok(v_n1.clone(), Py::str("n1"))?;
    let _ = f__index_ok(v_n2.clone(), Py::str("n2"))?;
    if (py::not(&{ let t455 = Py::Int(0i64); Py::Bool({ let t456 = v_theta1.clone(); if py::le(&t455, &t456)? { { let t457 = py::add(&py::truediv(&Py::Float(std::f64::consts::PI), &Py::Int(2i64))?, &Py::Float(1e-12))?; if py::le(&t456, &t457)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the angle of incidence must lie between 0 and 90 degrees (measured from the normal)"))));
    }
    v_s = py::truediv(&py::mul(&v_n1.clone(), &py::math1("sin", &py::minmax(&[v_theta1.clone(), py::truediv(&Py::Float(std::f64::consts::PI), &Py::Int(2i64))?], false)?)?)?, &v_n2.clone())?;
    if ({ let t458 = v_s.clone(); Py::Bool({ let t459 = Py::Int(1i64); if py::gt(&t458, &t459)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("total internal reflection: n1 sin(theta1) exceeds n2, so there is no refracted ray"))));
    }
    v_t2 = py::math1("asin", &v_s.clone())?;
    return Ok(Py::dict(vec![(Py::str("theta2"), f__r(v_t2.clone())?), (Py::str("theta2_deg"), f__r(py::math1("degrees", &v_t2.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_sound_intensity_level(mut v_I: Py, mut v_beta: Py) -> R {
    if ({ let t473 = { let t475 = v_I.clone(); Py::Bool({ let t476 = Py::None; if py::is(&t475, &t476) { true } else { false } }) }; Py::Bool({ let t474 = { let t477 = v_beta.clone(); Py::Bool({ let t478 = Py::None; if py::is(&t477, &t478) { true } else { false } }) }; if py::eq(&t473, &t474) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give exactly one of the intensity I or the level beta"))));
    }
    if ({ let t479 = v_I.clone(); Py::Bool({ let t480 = Py::None; if !py::is(&t479, &t480) { true } else { false } }) }).truthy() {
        let _ = f__positive(v_I.clone(), Py::str("the intensity I"))?;
        return Ok(Py::dict(vec![(Py::str("beta"), f__r(py::mul(&Py::Int(10i64), &py::math1("log10", &py::truediv(&v_I.clone(), &c_I0_HEAR()?)?)?)?)?), (Py::str("I"), v_I.clone())]));
    }
    if (py::not(&py::math1("isfinite", &v_beta.clone())?)).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the level must be a finite number of decibels"))));
    }
    return Ok(Py::dict(vec![(Py::str("beta"), v_beta.clone()), (Py::str("I"), f__r(py::mul(&c_I0_HEAR()?, &py::pow(&Py::Int(10i64), &py::truediv(&v_beta.clone(), &Py::Int(10i64))?)?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_sound_level_change(mut v_beta1: Py, mut v_beta2: Py, mut v_ratio: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_out: Py = Py::None;
    if ({ let t501 = v_ratio.clone(); Py::Bool({ let t502 = Py::None; if !py::is(&t501, &t502) { true } else { false } }) }).truthy() {
        if ({ let t503 = v_beta2.clone(); Py::Bool({ let t504 = Py::None; if !py::is(&t503, &t504) { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("give the intensity ratio or the second level, not both"))));
        }
        if (py::not(&{ let t507 = py::math1("isfinite", &v_ratio.clone())?; if !t507.truthy() { t507 } else { { let t505 = v_ratio.clone(); Py::Bool({ let t506 = Py::Int(0i64); if py::gt(&t505, &t506)? { true } else { false } }) } } })).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("the intensity ratio I2/I1 must be positive"))));
        }
        v_d = py::mul(&Py::Int(10i64), &py::math1("log10", &v_ratio.clone())?)?;
        v_out = Py::dict(vec![(Py::str("d_beta"), f__r(v_d.clone())?), (Py::str("ratio"), v_ratio.clone())]);
        if ({ let t513 = v_beta1.clone(); Py::Bool({ let t514 = Py::None; if !py::is(&t513, &t514) { true } else { false } }) }).truthy() {
            py::setitem(&v_out.clone(), &Py::str("beta2"), f__r(py::add(&v_beta1.clone(), &v_d.clone())?)?)?;
        }
        return Ok(v_out.clone());
    }
    if ({ let t519 = { let t515 = v_beta1.clone(); Py::Bool({ let t516 = Py::None; if py::is(&t515, &t516) { true } else { false } }) }; if t519.truthy() { t519 } else { { let t517 = v_beta2.clone(); Py::Bool({ let t518 = Py::None; if py::is(&t517, &t518) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give both levels beta1 and beta2, or the intensity ratio"))));
    }
    v_d = py::sub(&v_beta2.clone(), &v_beta1.clone())?;
    return Ok(Py::dict(vec![(Py::str("d_beta"), f__r(v_d.clone())?), (Py::str("ratio"), f__r(py::pow(&Py::Int(10i64), &py::truediv(&v_d.clone(), &Py::Int(10i64))?)?)?), (Py::str("beta2"), v_beta2.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_standing_wave_harmonic(mut v_system: Py, mut v_L: Py, mut v_v: Py, mut v_n: Py) -> R {
    let mut v_f1: Py = Py::None;
    let mut v_k: Py = Py::None;
    let _ = f__positive(v_L.clone(), Py::str("the length L"))?;
    let _ = f__positive(v_v.clone(), Py::str("the wave speed v"))?;
    if ({ let t540 = v_n.clone(); Py::Bool({ let t541 = Py::Int(1i64); if py::lt(&t540, &t541)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the harmonic number n is a whole number 1, 2, 3, ..."))));
    }
    if ({ let t546 = { let t542 = v_system.clone(); Py::Bool({ let t543 = Py::str("closed pipe"); if py::eq(&t542, &t543) { true } else { false } }) }; if !t546.truthy() { t546 } else { { let t544 = py::modulo(&v_n.clone(), &Py::Int(2i64))?; Py::Bool({ let t545 = Py::Int(0i64); if py::eq(&t544, &t545) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a pipe closed at one end has only odd harmonics (n = 1, 3, 5, ...)"))));
    }
    v_k = f__pipe_length_factor(v_system.clone())?;
    v_f1 = py::truediv(&v_v.clone(), &py::mul(&v_k.clone(), &v_L.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("f_n"), py::mul(&v_n.clone(), &v_f1.clone())?), (Py::str("lam_n"), py::truediv(&py::mul(&v_k.clone(), &v_L.clone())?, &v_n.clone())?), (Py::str("f1"), v_f1.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_static_friction_incline(mut v_mu_s: Py, mut v_theta: Py, mut v_m: Py) -> R {
    let mut v_Nf: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_tc: Py = Py::None;
    if (py::not(&{ let t557 = py::math1("isfinite", &v_mu_s.clone())?; if !t557.truthy() { t557 } else { { let t555 = v_mu_s.clone(); Py::Bool({ let t556 = Py::Int(0i64); if py::ge(&t555, &t556)? { true } else { false } }) } } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the coefficient of static friction cannot be negative"))));
    }
    if (py::not(&{ let t558 = Py::Int(0i64); Py::Bool({ let t559 = v_theta.clone(); if py::le(&t558, &t559)? { { let t560 = py::truediv(&Py::Float(std::f64::consts::PI), &Py::Int(2i64))?; if py::lt(&t559, &t560)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the incline angle must lie between 0 and 90 degrees (90 excluded)"))));
    }
    v_tc = py::math1("atan", &v_mu_s.clone())?;
    v_out = Py::dict(vec![(Py::str("theta_c"), f__r(v_tc.clone())?), (Py::str("theta_c_deg"), f__r(py::math1("degrees", &v_tc.clone())?)?), (Py::str("slips"), f__slips_incline(v_mu_s.clone(), v_theta.clone())?)]);
    if ({ let t568 = v_m.clone(); Py::Bool({ let t569 = Py::None; if !py::is(&t568, &t569) { true } else { false } }) }).truthy() {
        let _ = f__positive(v_m.clone(), Py::str("the mass m"))?;
        v_Nf = py::mul(&py::mul(&v_m.clone(), &c_G0()?)?, &py::math1("cos", &v_theta.clone())?)?;
        py::setitem(&v_out.clone(), &Py::str("N"), f__r(v_Nf.clone())?)?;
        py::setitem(&v_out.clone(), &Py::str("f_needed"), f__r(py::mul(&py::mul(&v_m.clone(), &c_G0()?)?, &py::math1("sin", &v_theta.clone())?)?)?)?;
        py::setitem(&v_out.clone(), &Py::str("f_max"), f__r(py::mul(&v_mu_s.clone(), &v_Nf.clone())?)?)?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_static_friction_limit(mut v_mu_s: Py, mut v_N: Py, mut v_F: Py) -> R {
    let mut v_fm: Py = Py::None;
    if (py::not(&{ let t599 = py::math1("isfinite", &v_mu_s.clone())?; if !t599.truthy() { t599 } else { { let t597 = v_mu_s.clone(); Py::Bool({ let t598 = Py::Int(0i64); if py::ge(&t597, &t598)? { true } else { false } }) } } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the coefficient of static friction cannot be negative"))));
    }
    let _ = f__positive(v_N.clone(), Py::str("the normal force N"))?;
    if (py::not(&{ let t605 = py::math1("isfinite", &v_F.clone())?; if !t605.truthy() { t605 } else { { let t603 = v_F.clone(); Py::Bool({ let t604 = Py::Int(0i64); if py::ge(&t603, &t604)? { true } else { false } }) } } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the applied force F is a magnitude (zero or positive)"))));
    }
    v_fm = py::mul(&v_mu_s.clone(), &v_N.clone())?;
    return Ok(Py::dict(vec![(Py::str("f_max"), v_fm.clone()), (Py::str("slips"), f__slips_level(v_mu_s.clone(), v_N.clone(), v_F.clone())?), (Py::str("margin"), py::sub(&v_fm.clone(), &v_F.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_string_wave_speed(mut v_T: Py, mut v_mu: Py) -> R {
    let _ = f__positive(v_T.clone(), Py::str("the tension T"))?;
    let _ = f__positive(v_mu.clone(), Py::str("the linear mass density mu"))?;
    return Ok(Py::dict(vec![(Py::str("v"), py::math1("sqrt", &py::truediv(&v_T.clone(), &v_mu.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_terminal_voltage(mut v_emf: Py, mut v_r: Py, mut v_R: Py) -> R {
    let mut v_I: Py = Py::None;
    let _ = f__positive(v_emf.clone(), Py::str("the emf"))?;
    if (py::not(&{ let t633 = py::math1("isfinite", &v_r.clone())?; if !t633.truthy() { t633 } else { { let t632 = { let t627 = v_r.clone(); Py::Bool({ let t628 = Py::Int(0i64); if py::ge(&t627, &t628)? { true } else { false } }) }; if !t632.truthy() { t632 } else { { let t631 = py::math1("isfinite", &v_R.clone())?; if !t631.truthy() { t631 } else { { let t629 = v_R.clone(); Py::Bool({ let t630 = Py::Int(0i64); if py::ge(&t629, &t630)? { true } else { false } }) } } } } } } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("resistances cannot be negative"))));
    }
    if ({ let t634 = py::add(&v_R.clone(), &v_r.clone())?; Py::Bool({ let t635 = Py::Int(0i64); if py::eq(&t634, &t635) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a short circuit with no internal resistance would draw an unlimited current"))));
    }
    v_I = py::truediv(&v_emf.clone(), &py::add(&v_R.clone(), &v_r.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("I"), v_I.clone()), (Py::str("V_t"), py::mul(&v_I.clone(), &v_R.clone())?), (Py::str("V_lost"), py::mul(&v_I.clone(), &v_r.clone())?), (Py::str("P_load"), py::mul(&py::mul(&v_I.clone(), &v_I.clone())?, &v_R.clone())?), (Py::str("efficiency"), py::truediv(&v_R.clone(), &py::add(&v_R.clone(), &v_r.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_vector_resultant(mut v_magnitudes: Py, mut v_directions: Py) -> R {
    let mut v_R: Py = Py::None;
    let mut v_Rx: Py = Py::None;
    let mut v_Ry: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_direction: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_mag: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_scale: Py = Py::None;
    let mut v_xs: Py = Py::None;
    let mut v_ys: Py = Py::None;
    v_n = py::len(&v_magnitudes.clone())?;
    if ({ let t647 = { let t643 = v_n.clone(); Py::Bool({ let t644 = Py::Int(1i64); if py::lt(&t643, &t644)? { true } else { false } }) }; if t647.truthy() { t647 } else { { let t645 = py::len(&v_directions.clone())?; Py::Bool({ let t646 = v_n.clone(); if py::ne(&t645, &t646) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give one direction for every magnitude"))));
    }
    let t648 = (Py::tuple(vec![Py::list(vec![]), Py::list(vec![])])).items()?;
    if t648.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_xs = t648[0].clone();
    v_ys = t648[1].clone();
    let t649 = (Py::tuple(vec![Py::Float(0.0), Py::Float(0.0), Py::Float(0.0)])).items()?;
    if t649.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_Rx = t649[0].clone();
    v_Ry = t649[1].clone();
    v_scale = t649[2].clone();
    for t650 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_k = t650;
        v_mag = py::getitem(&v_magnitudes.clone(), &v_k.clone())?;
        if (py::not(&{ let t653 = py::math1("isfinite", &v_mag.clone())?; if !t653.truthy() { t653 } else { { let t651 = v_mag.clone(); Py::Bool({ let t652 = Py::Int(0i64); if py::ge(&t651, &t652)? { true } else { false } }) } } })).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("magnitudes cannot be negative (reverse the direction instead)"))));
        }
        let t662 = (f__cos_sin_deg(py::getitem(&v_directions.clone(), &v_k.clone())?)?).items()?;
        if t662.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_c = t662[0].clone();
        v_s = t662[1].clone();
        let _ = py::method(&v_xs.clone(), "append", &[f__r(py::mul(&v_mag.clone(), &v_c.clone())?)?])?;
        let _ = py::method(&v_ys.clone(), "append", &[f__r(py::mul(&v_mag.clone(), &v_s.clone())?)?])?;
        v_Rx = py::iadd(&v_Rx, &py::mul(&v_mag.clone(), &v_c.clone())?)?;
        v_Ry = py::iadd(&v_Ry, &py::mul(&v_mag.clone(), &v_s.clone())?)?;
        v_scale = py::iadd(&v_scale, &v_mag.clone())?;
    }
    if ({ let t668 = py::abs(&v_Rx.clone())?; Py::Bool({ let t669 = py::mul(&Py::Float(1e-12), &v_scale.clone())?; if py::le(&t668, &t669)? { true } else { false } }) }).truthy() {
        v_Rx = Py::Float(0.0);
    }
    if ({ let t670 = py::abs(&v_Ry.clone())?; Py::Bool({ let t671 = py::mul(&Py::Float(1e-12), &v_scale.clone())?; if py::le(&t670, &t671)? { true } else { false } }) }).truthy() {
        v_Ry = Py::Float(0.0);
    }
    v_R = py::hypot(&[v_Rx.clone(), v_Ry.clone()])?;
    v_direction = Py::Float(0.0);
    if ({ let t672 = v_R.clone(); Py::Bool({ let t673 = Py::Int(0i64); if py::gt(&t672, &t673)? { true } else { false } }) }).truthy() {
        v_direction = py::modulo(&py::math1("degrees", &py::atan2(&v_Ry.clone(), &v_Rx.clone())?)?, &Py::Float(360.0))?;
    }
    return Ok(Py::dict(vec![(Py::str("x_components"), v_xs.clone()), (Py::str("y_components"), v_ys.clone()), (Py::str("Rx"), f__r(v_Rx.clone())?), (Py::str("Ry"), f__r(v_Ry.clone())?), (Py::str("R"), f__r(v_R.clone())?), (Py::str("direction_deg"), f__r(v_direction.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_verify_beam(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_RA: Py = Py::None;
    let mut v_RB: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_loads: Py = Py::None;
    let mut v_mom: Py = Py::None;
    let mut v_res: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_span: Py = Py::None;
    let mut v_tot: Py = Py::None;
    let mut v_xA: Py = Py::None;
    let mut v_xB: Py = Py::None;
    let mut v_xs: Py = Py::None;
    let t22 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("loads"))?, py::getitem(&v_i.clone(), &Py::str("positions"))?, py::getitem(&v_i.clone(), &Py::str("xA"))?, py::getitem(&v_i.clone(), &Py::str("xB"))?])).items()?;
    if t22.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_loads = t22[0].clone();
    v_xs = t22[1].clone();
    v_xA = t22[2].clone();
    v_xB = t22[3].clone();
    let t23 = (Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("RA"))?, py::getitem(&v_o.clone(), &Py::str("RB"))?])).items()?;
    if t23.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_RA = t23[0].clone();
    v_RB = t23[1].clone();
    if (py::not(&{ let t25 = py::math1("isfinite", &v_RA.clone())?; if !t25.truthy() { t25 } else { { let t24 = py::math1("isfinite", &v_RB.clone())?; if !t24.truthy() { t24 } else { py::math1("isfinite", &py::getitem(&v_o.clone(), &Py::str("total"))?)? } } } })).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    let t26 = (Py::tuple(vec![Py::Float(0.0), Py::Float(0.0), Py::Float(0.0), py::minmax(&[py::abs(&v_xA.clone())?, py::abs(&v_xB.clone())?], true)?])).items()?;
    if t26.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_s = t26[0].clone();
    v_tot = t26[1].clone();
    v_mom = t26[2].clone();
    v_span = t26[3].clone();
    for t27 in py::range_iter(&[(py::len(&v_loads.clone())?).i()?]) {
        v_k = t27;
        v_s = py::iadd(&v_s, &py::abs(&py::getitem(&v_loads.clone(), &v_k.clone())?)?)?;
        v_tot = py::iadd(&v_tot, &py::getitem(&v_loads.clone(), &v_k.clone())?)?;
        v_mom = py::iadd(&v_mom, &py::mul(&py::getitem(&v_loads.clone(), &v_k.clone())?, &py::getitem(&v_xs.clone(), &v_k.clone())?)?)?;
        v_span = py::minmax(&[v_span.clone(), py::abs(&py::getitem(&v_xs.clone(), &v_k.clone())?)?], true)?;
    }
    if ({ let t28 = v_s.clone(); Py::Bool({ let t29 = Py::Int(0i64); if py::eq(&t28, &t29) { true } else { false } }) }).truthy() {
        return Ok(py::add(&py::add(&py::abs(&v_RA.clone())?, &py::abs(&v_RB.clone())?)?, &py::abs(&py::getitem(&v_o.clone(), &Py::str("total"))?)?)?);
    }
    v_res = py::add(&py::truediv(&py::abs(&py::sub(&py::add(&v_RA.clone(), &v_RB.clone())?, &v_tot.clone())?)?, &v_s.clone())?, &py::truediv(&py::abs(&py::sub(&py::add(&py::mul(&v_RA.clone(), &v_xA.clone())?, &py::mul(&v_RB.clone(), &v_xB.clone())?)?, &v_mom.clone())?)?, &py::mul(&v_s.clone(), &py::minmax(&[v_span.clone(), Py::Float(1e-300)], true)?)?)?)?;
    return Ok(py::add(&v_res.clone(), &py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("total"))?, &v_tot.clone())?)?, &v_s.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_verify_bulk(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_V: Py = Py::None;
    let mut v_dV: Py = Py::None;
    let mut v_sn: Py = Py::None;
    let t35 = (Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("dV"))?, py::getitem(&v_o.clone(), &Py::str("strain"))?, py::getitem(&v_o.clone(), &Py::str("V"))?])).items()?;
    if t35.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_dV = t35[0].clone();
    v_sn = t35[1].clone();
    v_V = t35[2].clone();
    if (py::not(&{ let t37 = py::math1("isfinite", &v_dV.clone())?; if !t37.truthy() { t37 } else { { let t36 = py::math1("isfinite", &v_sn.clone())?; if !t36.truthy() { t36 } else { py::math1("isfinite", &v_V.clone())? } } } })).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    return Ok(py::add(&py::add(&f__rel(py::mul(&v_sn.clone(), &py::getitem(&v_i.clone(), &Py::str("B"))?)?, py::neg(&py::getitem(&v_i.clone(), &Py::str("dP"))?)?)?, &f__rel(v_dV.clone(), py::mul(&v_sn.clone(), &py::getitem(&v_i.clone(), &Py::str("V0"))?)?)?)?, &f__rel(v_V.clone(), py::add(&py::getitem(&v_i.clone(), &Py::str("V0"))?, &v_dV.clone())?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_verify_com(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_M: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_ms: Py = Py::None;
    let mut v_res: Py = Py::None;
    let mut v_sx: Py = Py::None;
    let mut v_sy: Py = Py::None;
    let mut v_tx: Py = Py::None;
    let mut v_ty: Py = Py::None;
    let mut v_xs: Py = Py::None;
    let mut v_ys: Py = Py::None;
    let t61 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("masses"))?, py::getitem(&v_i.clone(), &Py::str("x"))?, py::method(&v_i.clone(), "get", &[Py::str("y")])?])).items()?;
    if t61.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_ms = t61[0].clone();
    v_xs = t61[1].clone();
    v_ys = t61[2].clone();
    if (py::not(&{ let t62 = py::math1("isfinite", &py::getitem(&v_o.clone(), &Py::str("M"))?)?; if !t62.truthy() { t62 } else { py::math1("isfinite", &py::getitem(&v_o.clone(), &Py::str("x_cm"))?)? } })).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    let t63 = (Py::tuple(vec![Py::Float(0.0), Py::Float(0.0), Py::Float(0.0)])).items()?;
    if t63.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_M = t63[0].clone();
    v_tx = t63[1].clone();
    v_sx = t63[2].clone();
    for t64 in py::range_iter(&[(py::len(&v_ms.clone())?).i()?]) {
        v_k = t64;
        v_M = py::iadd(&v_M, &py::getitem(&v_ms.clone(), &v_k.clone())?)?;
        v_tx = py::iadd(&v_tx, &py::mul(&py::getitem(&v_ms.clone(), &v_k.clone())?, &py::sub(&py::getitem(&v_xs.clone(), &v_k.clone())?, &py::getitem(&v_o.clone(), &Py::str("x_cm"))?)?)?)?;
        v_sx = py::iadd(&v_sx, &py::mul(&py::getitem(&v_ms.clone(), &v_k.clone())?, &py::abs(&py::getitem(&v_xs.clone(), &v_k.clone())?)?)?)?;
    }
    v_res = py::add(&f__rel(py::getitem(&v_o.clone(), &Py::str("M"))?, v_M.clone())?, &py::truediv(&py::abs(&v_tx.clone())?, &py::minmax(&[v_sx.clone(), py::mul(&v_M.clone(), &py::abs(&py::getitem(&v_o.clone(), &Py::str("x_cm"))?)?)?, Py::Float(1e-300)], true)?)?)?;
    if ({ let t68 = v_ys.clone(); Py::Bool({ let t69 = Py::None; if !py::is(&t68, &t69) { true } else { false } }) }).truthy() {
        if ({ let t72 = { let t70 = Py::str("y_cm"); Py::Bool({ let t71 = v_o.clone(); if !py::contains(&t71, &t70)? { true } else { false } }) }; if t72.truthy() { t72 } else { py::not(&py::math1("isfinite", &py::getitem(&v_o.clone(), &Py::str("y_cm"))?)?) } }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        let t73 = (Py::tuple(vec![Py::Float(0.0), Py::Float(0.0)])).items()?;
        if t73.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_ty = t73[0].clone();
        v_sy = t73[1].clone();
        for t74 in py::range_iter(&[(py::len(&v_ms.clone())?).i()?]) {
            v_k = t74;
            v_ty = py::iadd(&v_ty, &py::mul(&py::getitem(&v_ms.clone(), &v_k.clone())?, &py::sub(&py::getitem(&v_ys.clone(), &v_k.clone())?, &py::getitem(&v_o.clone(), &Py::str("y_cm"))?)?)?)?;
            v_sy = py::iadd(&v_sy, &py::mul(&py::getitem(&v_ms.clone(), &v_k.clone())?, &py::abs(&py::getitem(&v_ys.clone(), &v_k.clone())?)?)?)?;
        }
        v_res = py::iadd(&v_res, &py::truediv(&py::abs(&v_ty.clone())?, &py::minmax(&[v_sy.clone(), py::mul(&v_M.clone(), &py::abs(&py::getitem(&v_o.clone(), &Py::str("y_cm"))?)?)?, Py::Float(1e-300)], true)?)?)?;
    }
    return Ok(v_res.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_verify_coulomb(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_F: Py = Py::None;
    let mut v_Fr: Py = Py::None;
    let mut v_s: Py = Py::None;
    let t78 = (Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("F"))?, py::getitem(&v_o.clone(), &Py::str("F_radial"))?])).items()?;
    if t78.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_F = t78[0].clone();
    v_Fr = t78[1].clone();
    if (py::not(&{ let t79 = py::math1("isfinite", &v_F.clone())?; if !t79.truthy() { t79 } else { py::math1("isfinite", &v_Fr.clone())? } })).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_s = py::abs(&py::mul(&py::mul(&c_KE()?, &py::getitem(&v_i.clone(), &Py::str("q1"))?)?, &py::getitem(&v_i.clone(), &Py::str("q2"))?)?)?;
    if ({ let t80 = v_s.clone(); Py::Bool({ let t81 = Py::Int(0i64); if py::eq(&t80, &t81) { true } else { false } }) }).truthy() {
        return Ok(py::add(&py::abs(&v_F.clone())?, &py::abs(&v_Fr.clone())?)?);
    }
    return Ok(py::add(&py::truediv(&py::abs(&py::sub(&py::mul(&py::mul(&v_Fr.clone(), &py::getitem(&v_i.clone(), &Py::str("r"))?)?, &py::getitem(&v_i.clone(), &Py::str("r"))?)?, &py::mul(&py::mul(&c_KE()?, &py::getitem(&v_i.clone(), &Py::str("q1"))?)?, &py::getitem(&v_i.clone(), &Py::str("q2"))?)?)?)?, &v_s.clone())?, &f__rel(v_F.clone(), py::abs(&v_Fr.clone())?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_verify_critical(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_t: Py = Py::None;
    let t95 = (Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("theta_c"))?, py::getitem(&v_o.clone(), &Py::str("theta_c_deg"))?])).items()?;
    if t95.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_t = t95[0].clone();
    v_d = t95[1].clone();
    if ({ let t102 = py::not(&{ let t96 = py::math1("isfinite", &v_t.clone())?; if !t96.truthy() { t96 } else { py::math1("isfinite", &v_d.clone())? } }); if t102.truthy() { t102 } else { { let t101 = { let t97 = v_t.clone(); Py::Bool({ let t98 = Py::Int(0i64); if py::le(&t97, &t98)? { true } else { false } }) }; if t101.truthy() { t101 } else { { let t99 = v_t.clone(); Py::Bool({ let t100 = py::truediv(&Py::Float(std::f64::consts::PI), &Py::Int(2i64))?; if py::gt(&t99, &t100)? { true } else { false } }) } } } } }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    return Ok(py::add(&py::truediv(&py::abs(&py::sub(&py::mul(&py::getitem(&v_i.clone(), &Py::str("n1"))?, &py::math1("sin", &v_t.clone())?)?, &py::getitem(&v_i.clone(), &Py::str("n2"))?)?)?, &py::getitem(&v_i.clone(), &Py::str("n2"))?)?, &py::truediv(&py::abs(&py::sub(&v_d.clone(), &py::math1("degrees", &v_t.clone())?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&v_d.clone())?], true)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_verify_double_slit(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_L: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_dd: Py = Py::None;
    let mut v_dy: Py = Py::None;
    let mut v_lam: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_res: Py = Py::None;
    let mut v_th: Py = Py::None;
    let mut v_y: Py = Py::None;
    let t165 = (Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("theta"))?, py::getitem(&v_o.clone(), &Py::str("y"))?, py::getitem(&v_o.clone(), &Py::str("dy"))?, py::getitem(&v_o.clone(), &Py::str("theta_deg"))?])).items()?;
    if t165.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_th = t165[0].clone();
    v_y = t165[1].clone();
    v_dy = t165[2].clone();
    v_dd = t165[3].clone();
    if (py::not(&{ let t168 = py::math1("isfinite", &v_th.clone())?; if !t168.truthy() { t168 } else { { let t167 = py::math1("isfinite", &v_y.clone())?; if !t167.truthy() { t167 } else { { let t166 = py::math1("isfinite", &v_dy.clone())?; if !t166.truthy() { t166 } else { py::math1("isfinite", &v_dd.clone())? } } } } } })).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    if ({ let t173 = { let t169 = v_th.clone(); Py::Bool({ let t170 = Py::Int(0i64); if py::lt(&t169, &t170)? { true } else { false } }) }; if t173.truthy() { t173 } else { { let t171 = v_th.clone(); Py::Bool({ let t172 = py::truediv(&Py::Float(std::f64::consts::PI), &Py::Int(2i64))?; if py::ge(&t171, &t172)? { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    let t174 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("d"))?, py::getitem(&v_i.clone(), &Py::str("lam"))?, py::getitem(&v_i.clone(), &Py::str("m"))?, py::getitem(&v_i.clone(), &Py::str("L"))?])).items()?;
    if t174.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_d = t174[0].clone();
    v_lam = t174[1].clone();
    v_m = t174[2].clone();
    v_L = t174[3].clone();
    v_res = py::truediv(&py::abs(&py::sub(&py::mul(&v_d.clone(), &py::math1("sin", &v_th.clone())?)?, &py::mul(&v_m.clone(), &v_lam.clone())?)?)?, &py::minmax(&[py::mul(&v_m.clone(), &v_lam.clone())?, v_lam.clone()], true)?)?;
    v_res = py::iadd(&v_res, &py::truediv(&py::abs(&py::sub(&v_y.clone(), &py::mul(&v_L.clone(), &py::math1("tan", &v_th.clone())?)?)?)?, &py::minmax(&[v_L.clone(), py::abs(&v_y.clone())?], true)?)?)?;
    v_res = py::iadd(&v_res, &f__rel(py::mul(&v_dy.clone(), &v_d.clone())?, py::mul(&v_lam.clone(), &v_L.clone())?)?)?;
    return Ok(py::add(&v_res.clone(), &py::truediv(&py::abs(&py::sub(&v_dd.clone(), &py::math1("degrees", &v_th.clone())?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&v_dd.clone())?], true)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_verify_faraday(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_A1: Py = Py::None;
    let mut v_A2: Py = Py::None;
    let mut v_N: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_dt: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_p1: Py = Py::None;
    let mut v_p2: Py = Py::None;
    let mut v_res: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_scale: Py = Py::None;
    let mut v_th: Py = Py::None;
    for t197 in py::iter(&(Py::tuple(vec![Py::str("phi1"), Py::str("phi2"), Py::str("dphi"), Py::str("emf"), Py::str("emf_signed")])))? {
        v_k = t197;
        if (py::not(&py::math1("isfinite", &py::getitem(&v_o.clone(), &v_k.clone())?)?)).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
    }
    v_A1 = py::getitem(&v_i.clone(), &Py::str("A1"))?;
    v_A2 = (if ({ let t198 = py::method(&v_i.clone(), "get", &[Py::str("A2")])?; Py::Bool({ let t199 = Py::None; if !py::is(&t198, &t199) { true } else { false } }) }).truthy() { py::getitem(&v_i.clone(), &Py::str("A2"))? } else { v_A1.clone() });
    v_th = py::method(&v_i.clone(), "get", &[Py::str("theta")])?;
    if ({ let t200 = v_th.clone(); Py::Bool({ let t201 = Py::None; if py::is(&t200, &t201) { true } else { false } }) }).truthy() {
        v_th = Py::Float(0.0);
    }
    v_c = f__cos_snap(v_th.clone())?;
    let t202 = (Py::tuple(vec![py::mul(&py::mul(&py::getitem(&v_i.clone(), &Py::str("B1"))?, &v_A1.clone())?, &v_c.clone())?, py::mul(&py::mul(&py::getitem(&v_i.clone(), &Py::str("B2"))?, &v_A2.clone())?, &v_c.clone())?])).items()?;
    if t202.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_p1 = t202[0].clone();
    v_p2 = t202[1].clone();
    v_s = py::minmax(&[py::abs(&v_p1.clone())?, py::abs(&v_p2.clone())?, Py::Float(1e-300)], true)?;
    v_res = py::add(&py::add(&py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("phi1"))?, &v_p1.clone())?)?, &v_s.clone())?, &py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("phi2"))?, &v_p2.clone())?)?, &v_s.clone())?)?, &py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("dphi"))?, &py::sub(&py::getitem(&v_o.clone(), &Py::str("phi2"))?, &py::getitem(&v_o.clone(), &Py::str("phi1"))?)?)?)?, &v_s.clone())?)?;
    let t203 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("N"))?, py::getitem(&v_i.clone(), &Py::str("dt"))?])).items()?;
    if t203.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_N = t203[0].clone();
    v_dt = t203[1].clone();
    v_e = py::truediv(&py::mul(&v_N.clone(), &py::abs(&py::getitem(&v_o.clone(), &Py::str("dphi"))?)?)?, &v_dt.clone())?;
    v_scale = py::minmax(&[v_e.clone(), py::truediv(&py::mul(&py::mul(&v_N.clone(), &v_s.clone())?, &Py::Float(1e-12))?, &v_dt.clone())?, Py::Float(1e-300)], true)?;
    return Ok(py::add(&py::add(&v_res.clone(), &py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("emf"))?, &v_e.clone())?)?, &v_scale.clone())?)?, &py::truediv(&py::abs(&py::add(&py::getitem(&v_o.clone(), &Py::str("emf_signed"))?, &py::truediv(&py::mul(&v_N.clone(), &py::getitem(&v_o.clone(), &Py::str("dphi"))?)?, &v_dt.clone())?)?)?, &v_scale.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_verify_gauss(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_Q: Py = Py::None;
    let mut v_R: Py = Py::None;
    let mut v_area: Py = Py::None;
    let mut v_dens: Py = Py::None;
    let mut v_fk: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_qenc: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_s: Py = Py::None;
    for t258 in py::iter(&(Py::tuple(vec![Py::str("E"), Py::str("E_radial"), Py::str("Q_enc"), Py::str("flux"), Py::str("density")])))? {
        v_k = t258;
        if ({ let t261 = { let t259 = v_k.clone(); Py::Bool({ let t260 = v_o.clone(); if !py::contains(&t260, &t259)? { true } else { false } }) }; if t261.truthy() { t261 } else { py::not(&py::math1("isfinite", &py::getitem(&v_o.clone(), &v_k.clone())?)?) } }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
    }
    let t262 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("geometry"))?, py::getitem(&v_i.clone(), &Py::str("Q"))?, py::method(&v_i.clone(), "get", &[Py::str("r")])?])).items()?;
    if t262.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_g = t262[0].clone();
    v_Q = t262[1].clone();
    v_r = t262[2].clone();
    v_fk = py::mul(&py::mul(&Py::Float(4.0), &Py::Float(std::f64::consts::PI))?, &c_KE()?)?;
    if ({ let t263 = v_g.clone(); Py::Bool({ let t264 = Py::str("line"); if py::eq(&t263, &t264) { true } else { false } }) }).truthy() {
        let t265 = (Py::tuple(vec![py::truediv(&v_Q.clone(), &py::getitem(&v_i.clone(), &Py::str("L"))?)?, v_Q.clone(), py::mul(&py::mul(&py::mul(&Py::Float(2.0), &Py::Float(std::f64::consts::PI))?, &v_r.clone())?, &py::getitem(&v_i.clone(), &Py::str("L"))?)?])).items()?;
        if t265.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_dens = t265[0].clone();
        v_qenc = t265[1].clone();
        v_area = t265[2].clone();
    } else {
        if ({ let t266 = v_g.clone(); Py::Bool({ let t267 = Py::str("plane"); if py::eq(&t266, &t267) { true } else { false } }) }).truthy() {
            let t268 = (Py::tuple(vec![py::truediv(&v_Q.clone(), &py::getitem(&v_i.clone(), &Py::str("A"))?)?, v_Q.clone(), py::mul(&Py::Float(2.0), &py::getitem(&v_i.clone(), &Py::str("A"))?)?])).items()?;
            if t268.len() != 3 { return Err(PyErr::Value("unpack".into())); }
            v_dens = t268[0].clone();
            v_qenc = t268[1].clone();
            v_area = t268[2].clone();
        } else {
            v_R = py::getitem(&v_i.clone(), &Py::str("R"))?;
            v_dens = (if ({ let t269 = v_g.clone(); Py::Bool({ let t270 = Py::str("sphere"); if py::eq(&t269, &t270) { true } else { false } }) }).truthy() { py::truediv(&v_Q.clone(), &py::truediv(&py::mul(&py::mul(&Py::Float(4.0), &Py::Float(std::f64::consts::PI))?, &py::pow(&v_R.clone(), &Py::Int(3i64))?)?, &Py::Float(3.0))?)? } else { py::truediv(&v_Q.clone(), &py::mul(&py::mul(&py::mul(&Py::Float(4.0), &Py::Float(std::f64::consts::PI))?, &v_R.clone())?, &v_R.clone())?)? });
            if ({ let t271 = v_r.clone(); Py::Bool({ let t272 = v_R.clone(); if py::ge(&t271, &t272)? { true } else { false } }) }).truthy() {
                v_qenc = v_Q.clone();
            } else {
                if ({ let t273 = v_g.clone(); Py::Bool({ let t274 = Py::str("sphere"); if py::eq(&t273, &t274) { true } else { false } }) }).truthy() {
                    v_qenc = py::mul(&v_Q.clone(), &py::pow(&py::truediv(&v_r.clone(), &v_R.clone())?, &Py::Int(3i64))?)?;
                } else {
                    v_qenc = Py::Float(0.0);
                }
            }
            v_area = py::mul(&py::mul(&py::mul(&Py::Float(4.0), &Py::Float(std::f64::consts::PI))?, &v_r.clone())?, &v_r.clone())?;
        }
    }
    v_s = py::abs(&py::mul(&v_fk.clone(), &v_Q.clone())?)?;
    if ({ let t275 = v_s.clone(); Py::Bool({ let t276 = Py::Int(0i64); if py::eq(&t275, &t276) { true } else { false } }) }).truthy() {
        return Ok(py::add(&py::add(&py::add(&py::abs(&py::getitem(&v_o.clone(), &Py::str("E"))?)?, &py::abs(&py::getitem(&v_o.clone(), &Py::str("E_radial"))?)?)?, &py::abs(&py::getitem(&v_o.clone(), &Py::str("Q_enc"))?)?)?, &py::abs(&py::getitem(&v_o.clone(), &Py::str("flux"))?)?)?);
    }
    return Ok(py::add(&py::add(&py::add(&py::add(&py::truediv(&py::abs(&py::sub(&py::mul(&py::getitem(&v_o.clone(), &Py::str("E_radial"))?, &v_area.clone())?, &py::mul(&v_fk.clone(), &py::getitem(&v_o.clone(), &Py::str("Q_enc"))?)?)?)?, &v_s.clone())?, &f__rel(py::getitem(&v_o.clone(), &Py::str("Q_enc"))?, v_qenc.clone())?)?, &f__rel(py::getitem(&v_o.clone(), &Py::str("flux"))?, py::mul(&v_fk.clone(), &v_qenc.clone())?)?)?, &f__rel(py::getitem(&v_o.clone(), &Py::str("E"))?, py::abs(&py::getitem(&v_o.clone(), &Py::str("E_radial"))?)?)?)?, &f__rel(py::getitem(&v_o.clone(), &Py::str("density"))?, v_dens.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_verify_grating(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_d_in: Py = Py::None;
    let mut v_dd: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_lam: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_res: Py = Py::None;
    let mut v_th: Py = Py::None;
    let t127 = (Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("theta"))?, py::getitem(&v_o.clone(), &Py::str("d"))?, py::getitem(&v_o.clone(), &Py::str("m_max"))?, py::getitem(&v_o.clone(), &Py::str("theta_deg"))?])).items()?;
    if t127.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_th = t127[0].clone();
    v_d = t127[1].clone();
    v_k = t127[2].clone();
    v_dd = t127[3].clone();
    if ({ let t135 = py::not(&{ let t129 = py::math1("isfinite", &v_th.clone())?; if !t129.truthy() { t129 } else { { let t128 = py::math1("isfinite", &v_d.clone())?; if !t128.truthy() { t128 } else { py::math1("isfinite", &v_dd.clone())? } } } }); if t135.truthy() { t135 } else { { let t134 = { let t130 = v_th.clone(); Py::Bool({ let t131 = Py::Int(0i64); if py::le(&t130, &t131)? { true } else { false } }) }; if t134.truthy() { t134 } else { { let t132 = v_th.clone(); Py::Bool({ let t133 = py::truediv(&Py::Float(std::f64::consts::PI), &Py::Int(2i64))?; if py::ge(&t132, &t133)? { true } else { false } }) } } } } }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_d_in = (if ({ let t136 = py::method(&v_i.clone(), "get", &[Py::str("d")])?; Py::Bool({ let t137 = Py::None; if !py::is(&t136, &t137) { true } else { false } }) }).truthy() { py::getitem(&v_i.clone(), &Py::str("d"))? } else { py::truediv(&Py::Float(0.001), &py::getitem(&v_i.clone(), &Py::str("N"))?)? });
    let t138 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("lam"))?, py::getitem(&v_i.clone(), &Py::str("m"))?])).items()?;
    if t138.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lam = t138[0].clone();
    v_m = t138[1].clone();
    v_res = py::add(&py::add(&f__rel(v_d.clone(), v_d_in.clone())?, &py::truediv(&py::abs(&py::sub(&py::mul(&v_d.clone(), &py::math1("sin", &v_th.clone())?)?, &py::mul(&v_m.clone(), &v_lam.clone())?)?)?, &py::mul(&v_m.clone(), &v_lam.clone())?)?)?, &py::truediv(&py::abs(&py::sub(&v_dd.clone(), &py::math1("degrees", &v_th.clone())?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&v_dd.clone())?], true)?)?)?;
    if ({ let t152 = { let t142 = v_k.clone(); Py::Bool({ let t143 = py::int(&v_k.clone())?; if py::ne(&t142, &t143) { true } else { false } }) }; if t152.truthy() { t152 } else { { let t151 = { let t144 = v_k.clone(); Py::Bool({ let t145 = Py::Int(1i64); if py::lt(&t144, &t145)? { true } else { false } }) }; if t151.truthy() { t151 } else { { let t150 = py::not(&{ let t146 = py::mul(&v_k.clone(), &v_lam.clone())?; Py::Bool({ let t147 = py::mul(&v_d.clone(), &py::sub(&Py::Int(1i64), &Py::Float(1e-09))?)?; if py::lt(&t146, &t147)? { true } else { false } }) }); if t150.truthy() { t150 } else { py::not(&{ let t148 = py::mul(&py::add(&v_k.clone(), &Py::Int(1i64))?, &v_lam.clone())?; Py::Bool({ let t149 = py::mul(&v_d.clone(), &py::sub(&Py::Int(1i64), &Py::Float(1e-09))?)?; if py::ge(&t148, &t149)? { true } else { false } }) }) } } } } } }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    return Ok(v_res.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_verify_gravity(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::mul(&py::mul(&py::getitem(&v_o.clone(), &Py::str("F"))?, &py::getitem(&v_i.clone(), &Py::str("r"))?)?, &py::getitem(&v_i.clone(), &Py::str("r"))?)?, py::mul(&py::mul(&c_G_NEWTON()?, &py::getitem(&v_i.clone(), &Py::str("m1"))?)?, &py::getitem(&v_i.clone(), &Py::str("m2"))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_verify_incline(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_Nf: Py = Py::None;
    let mut v_dd: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_mu: Py = Py::None;
    let mut v_res: Py = Py::None;
    let mut v_tc: Py = Py::None;
    let mut v_th: Py = Py::None;
    let t573 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("mu_s"))?, py::getitem(&v_i.clone(), &Py::str("theta"))?])).items()?;
    if t573.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_mu = t573[0].clone();
    v_th = t573[1].clone();
    let t574 = (Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("theta_c"))?, py::getitem(&v_o.clone(), &Py::str("theta_c_deg"))?])).items()?;
    if t574.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_tc = t574[0].clone();
    v_dd = t574[1].clone();
    if ({ let t581 = py::not(&{ let t575 = py::math1("isfinite", &v_tc.clone())?; if !t575.truthy() { t575 } else { py::math1("isfinite", &v_dd.clone())? } }); if t581.truthy() { t581 } else { { let t580 = { let t576 = v_tc.clone(); Py::Bool({ let t577 = Py::Int(0i64); if py::lt(&t576, &t577)? { true } else { false } }) }; if t580.truthy() { t580 } else { { let t578 = v_tc.clone(); Py::Bool({ let t579 = py::truediv(&Py::Float(std::f64::consts::PI), &Py::Int(2i64))?; if py::ge(&t578, &t579)? { true } else { false } }) } } } } }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    if ({ let t582 = py::getitem(&v_o.clone(), &Py::str("slips"))?; Py::Bool({ let t583 = f__slips_incline(v_mu.clone(), v_th.clone())?; if py::ne(&t582, &t583) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_res = py::add(&py::truediv(&py::abs(&py::sub(&py::math1("tan", &v_tc.clone())?, &v_mu.clone())?)?, &py::minmax(&[v_mu.clone(), Py::Float(1e-300)], true)?)?, &py::truediv(&py::abs(&py::sub(&v_dd.clone(), &py::math1("degrees", &v_tc.clone())?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&v_dd.clone())?], true)?)?)?;
    v_m = py::method(&v_i.clone(), "get", &[Py::str("m")])?;
    if ({ let t584 = v_m.clone(); Py::Bool({ let t585 = Py::None; if !py::is(&t584, &t585) { true } else { false } }) }).truthy() {
        if ({ let t593 = { let t586 = Py::str("N"); Py::Bool({ let t587 = v_o.clone(); if !py::contains(&t587, &t586)? { true } else { false } }) }; if t593.truthy() { t593 } else { { let t592 = { let t588 = Py::str("f_needed"); Py::Bool({ let t589 = v_o.clone(); if !py::contains(&t589, &t588)? { true } else { false } }) }; if t592.truthy() { t592 } else { { let t590 = Py::str("f_max"); Py::Bool({ let t591 = v_o.clone(); if !py::contains(&t591, &t590)? { true } else { false } }) } } } } }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_Nf = py::mul(&py::mul(&v_m.clone(), &c_G0()?)?, &py::math1("cos", &v_th.clone())?)?;
        v_res = py::iadd(&v_res, &py::add(&py::add(&f__rel(py::getitem(&v_o.clone(), &Py::str("N"))?, v_Nf.clone())?, &f__rel(py::getitem(&v_o.clone(), &Py::str("f_needed"))?, py::mul(&py::mul(&v_m.clone(), &c_G0()?)?, &py::math1("sin", &v_th.clone())?)?)?)?, &f__rel(py::getitem(&v_o.clone(), &Py::str("f_max"))?, py::mul(&v_mu.clone(), &v_Nf.clone())?)?)?)?;
    }
    return Ok(v_res.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_verify_level(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_I: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_res: Py = Py::None;
    let t489 = (Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("beta"))?, py::getitem(&v_o.clone(), &Py::str("I"))?])).items()?;
    if t489.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_b = t489[0].clone();
    v_I = t489[1].clone();
    if ({ let t493 = py::not(&{ let t490 = py::math1("isfinite", &v_b.clone())?; if !t490.truthy() { t490 } else { py::math1("isfinite", &v_I.clone())? } }); if t493.truthy() { t493 } else { { let t491 = v_I.clone(); Py::Bool({ let t492 = Py::Int(0i64); if py::le(&t491, &t492)? { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_res = py::truediv(&py::abs(&py::sub(&v_b.clone(), &py::mul(&Py::Int(10i64), &py::math1("log10", &py::truediv(&v_I.clone(), &c_I0_HEAR()?)?)?)?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&v_b.clone())?], true)?)?;
    if ({ let t494 = py::method(&v_i.clone(), "get", &[Py::str("I")])?; Py::Bool({ let t495 = Py::None; if !py::is(&t494, &t495) { true } else { false } }) }).truthy() {
        v_res = py::iadd(&v_res, &f__rel(v_I.clone(), py::getitem(&v_i.clone(), &Py::str("I"))?)?)?;
    }
    if ({ let t499 = py::method(&v_i.clone(), "get", &[Py::str("beta")])?; Py::Bool({ let t500 = Py::None; if !py::is(&t499, &t500) { true } else { false } }) }).truthy() {
        v_res = py::iadd(&v_res, &py::truediv(&py::abs(&py::sub(&v_b.clone(), &py::getitem(&v_i.clone(), &Py::str("beta"))?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&v_b.clone())?], true)?)?)?;
    }
    return Ok(v_res.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_verify_level_change(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_ratio: Py = Py::None;
    let mut v_res: Py = Py::None;
    let t520 = (Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("d_beta"))?, py::getitem(&v_o.clone(), &Py::str("ratio"))?])).items()?;
    if t520.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_d = t520[0].clone();
    v_ratio = t520[1].clone();
    if ({ let t524 = py::not(&{ let t521 = py::math1("isfinite", &v_d.clone())?; if !t521.truthy() { t521 } else { py::math1("isfinite", &v_ratio.clone())? } }); if t524.truthy() { t524 } else { { let t522 = v_ratio.clone(); Py::Bool({ let t523 = Py::Int(0i64); if py::le(&t522, &t523)? { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_res = py::truediv(&py::abs(&py::sub(&py::mul(&Py::Int(10i64), &py::math1("log10", &v_ratio.clone())?)?, &v_d.clone())?)?, &py::minmax(&[Py::Float(1.0), py::abs(&v_d.clone())?], true)?)?;
    if ({ let t525 = py::method(&v_i.clone(), "get", &[Py::str("ratio")])?; Py::Bool({ let t526 = Py::None; if !py::is(&t525, &t526) { true } else { false } }) }).truthy() {
        v_res = py::iadd(&v_res, &f__rel(v_ratio.clone(), py::getitem(&v_i.clone(), &Py::str("ratio"))?)?)?;
    }
    if ({ let t530 = py::method(&v_i.clone(), "get", &[Py::str("beta1")])?; Py::Bool({ let t531 = Py::None; if !py::is(&t530, &t531) { true } else { false } }) }).truthy() {
        if ({ let t534 = { let t532 = Py::str("beta2"); Py::Bool({ let t533 = v_o.clone(); if !py::contains(&t533, &t532)? { true } else { false } }) }; if t534.truthy() { t534 } else { py::not(&py::math1("isfinite", &py::getitem(&v_o.clone(), &Py::str("beta2"))?)?) } }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_res = py::iadd(&v_res, &py::truediv(&py::abs(&py::sub(&py::sub(&py::getitem(&v_o.clone(), &Py::str("beta2"))?, &py::getitem(&v_i.clone(), &Py::str("beta1"))?)?, &v_d.clone())?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::getitem(&v_o.clone(), &Py::str("beta2"))?)?], true)?)?)?;
    }
    if ({ let t535 = py::method(&v_i.clone(), "get", &[Py::str("beta2")])?; Py::Bool({ let t536 = Py::None; if !py::is(&t535, &t536) { true } else { false } }) }).truthy() {
        v_res = py::iadd(&v_res, &py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("beta2"))?, &py::getitem(&v_i.clone(), &Py::str("beta2"))?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::getitem(&v_i.clone(), &Py::str("beta2"))?)?], true)?)?)?;
    }
    return Ok(v_res.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_verify_magnification(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_d_i: Py = Py::None;
    let mut v_d_o: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_h_i: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_res: Py = Py::None;
    let mut v_want: Py = Py::None;
    let t347 = (Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("m"))?, py::getitem(&v_o.clone(), &Py::str("h_i"))?, py::getitem(&v_o.clone(), &Py::str("d_i"))?])).items()?;
    if t347.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_m = t347[0].clone();
    v_h_i = t347[1].clone();
    v_d_i = t347[2].clone();
    if ({ let t352 = py::not(&{ let t349 = py::math1("isfinite", &v_m.clone())?; if !t349.truthy() { t349 } else { { let t348 = py::math1("isfinite", &v_h_i.clone())?; if !t348.truthy() { t348 } else { py::math1("isfinite", &v_d_i.clone())? } } } }); if t352.truthy() { t352 } else { { let t350 = v_d_i.clone(); Py::Bool({ let t351 = Py::Int(0i64); if py::eq(&t350, &t351) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_d_o = py::getitem(&v_i.clone(), &Py::str("d_o"))?;
    v_res = py::add(&py::truediv(&py::abs(&py::add(&py::mul(&v_m.clone(), &v_d_o.clone())?, &v_d_i.clone())?)?, &py::abs(&v_d_i.clone())?)?, &f__rel(v_h_i.clone(), py::mul(&v_m.clone(), &py::getitem(&v_i.clone(), &Py::str("h_o"))?)?)?)?;
    if ({ let t356 = py::method(&v_i.clone(), "get", &[Py::str("d_i")])?; Py::Bool({ let t357 = Py::None; if !py::is(&t356, &t357) { true } else { false } }) }).truthy() {
        v_res = py::iadd(&v_res, &f__rel(v_d_i.clone(), py::getitem(&v_i.clone(), &Py::str("d_i"))?)?)?;
    }
    if ({ let t358 = py::method(&v_i.clone(), "get", &[Py::str("f")])?; Py::Bool({ let t359 = Py::None; if !py::is(&t358, &t359) { true } else { false } }) }).truthy() {
        v_f = py::getitem(&v_i.clone(), &Py::str("f"))?;
        v_res = py::iadd(&v_res, &py::truediv(&py::abs(&py::sub(&py::add(&py::truediv(&Py::Int(1i64), &v_d_o.clone())?, &py::truediv(&Py::Int(1i64), &v_d_i.clone())?)?, &py::truediv(&Py::Int(1i64), &v_f.clone())?)?)?, &py::minmax(&[py::truediv(&Py::Int(1i64), &v_d_o.clone())?, py::truediv(&Py::Int(1i64), &py::abs(&v_d_i.clone())?)?, py::truediv(&Py::Int(1i64), &py::abs(&v_f.clone())?)?], true)?)?)?;
    }
    v_want = py::add(&(if ({ let t360 = v_d_i.clone(); Py::Bool({ let t361 = Py::Int(0i64); if py::gt(&t360, &t361)? { true } else { false } }) }).truthy() { Py::str("real") } else { Py::str("virtual") }), &(if ({ let t362 = v_m.clone(); Py::Bool({ let t363 = Py::Int(0i64); if py::lt(&t362, &t363)? { true } else { false } }) }).truthy() { Py::str(", inverted") } else { Py::str(", upright") }))?;
    if ({ let t364 = py::getitem(&v_o.clone(), &Py::str("image"))?; Py::Bool({ let t365 = v_want.clone(); if py::ne(&t364, &t365) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    return Ok(v_res.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_verify_malus(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_I: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_fr: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_light: Py = Py::None;
    let t378 = (Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("I"))?, py::getitem(&v_o.clone(), &Py::str("fraction"))?])).items()?;
    if t378.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_I = t378[0].clone();
    v_fr = t378[1].clone();
    if (py::not(&{ let t379 = py::math1("isfinite", &v_I.clone())?; if !t379.truthy() { t379 } else { py::math1("isfinite", &v_fr.clone())? } })).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_c = f__cos_snap(py::getitem(&v_i.clone(), &Py::str("theta"))?)?;
    v_light = py::method(&v_i.clone(), "get", &[Py::str("light")])?;
    v_k = (if ({ let t380 = v_light.clone(); Py::Bool({ let t381 = Py::str("unpolarized"); if py::eq(&t380, &t381) { true } else { false } }) }).truthy() { Py::Float(0.5) } else { Py::Float(1.0) });
    return Ok(py::add(&py::truediv(&py::abs(&py::sub(&v_I.clone(), &py::mul(&py::mul(&py::mul(&v_k.clone(), &py::getitem(&v_i.clone(), &Py::str("I0"))?)?, &v_c.clone())?, &v_c.clone())?)?)?, &py::getitem(&v_i.clone(), &Py::str("I0"))?)?, &py::abs(&py::sub(&v_fr.clone(), &py::truediv(&v_I.clone(), &py::getitem(&v_i.clone(), &Py::str("I0"))?)?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_verify_rlc(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_C: Py = Py::None;
    let mut v_L: Py = Py::None;
    let mut v_R: Py = Py::None;
    let mut v_V: Py = Py::None;
    let mut v_X: Py = Py::None;
    let mut v_XC: Py = Py::None;
    let mut v_XL: Py = Py::None;
    let mut v_Z: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_res: Py = Py::None;
    let mut v_w: Py = Py::None;
    for t398 in py::iter(&(Py::tuple(vec![Py::str("Z"), Py::str("I"), Py::str("phi"), Py::str("phi_deg"), Py::str("pf"), Py::str("P"), Py::str("XL"), Py::str("XC")])))? {
        v_k = t398;
        if (py::not(&py::math1("isfinite", &py::getitem(&v_o.clone(), &v_k.clone())?)?)).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
    }
    let t399 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("R"))?, py::getitem(&v_i.clone(), &Py::str("L"))?, py::getitem(&v_i.clone(), &Py::str("C"))?, py::getitem(&v_i.clone(), &Py::str("f"))?, py::getitem(&v_i.clone(), &Py::str("V"))?])).items()?;
    if t399.len() != 5 { return Err(PyErr::Value("unpack".into())); }
    v_R = t399[0].clone();
    v_L = t399[1].clone();
    v_C = t399[2].clone();
    v_f = t399[3].clone();
    v_V = t399[4].clone();
    v_w = py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &v_f.clone())?;
    let t400 = (Py::tuple(vec![py::mul(&v_w.clone(), &v_L.clone())?, py::truediv(&Py::Float(1.0), &py::mul(&v_w.clone(), &v_C.clone())?)?])).items()?;
    if t400.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_XL = t400[0].clone();
    v_XC = t400[1].clone();
    v_X = py::sub(&v_XL.clone(), &v_XC.clone())?;
    if ({ let t401 = py::abs(&v_X.clone())?; Py::Bool({ let t402 = py::mul(&Py::Float(1e-12), &py::minmax(&[v_XL.clone(), v_XC.clone()], true)?)?; if py::le(&t401, &t402)? { true } else { false } }) }).truthy() {
        v_X = Py::Float(0.0);
    }
    v_Z = py::getitem(&v_o.clone(), &Py::str("Z"))?;
    if ({ let t403 = v_Z.clone(); Py::Bool({ let t404 = Py::Int(0i64); if py::le(&t403, &t404)? { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_res = py::add(&py::add(&f__rel(py::getitem(&v_o.clone(), &Py::str("XL"))?, v_XL.clone())?, &f__rel(py::getitem(&v_o.clone(), &Py::str("XC"))?, v_XC.clone())?)?, &py::truediv(&py::abs(&py::sub(&py::mul(&v_Z.clone(), &v_Z.clone())?, &py::add(&py::mul(&v_R.clone(), &v_R.clone())?, &py::mul(&v_X.clone(), &v_X.clone())?)?)?)?, &py::add(&py::mul(&v_R.clone(), &v_R.clone())?, &py::mul(&v_X.clone(), &v_X.clone())?)?)?)?;
    v_res = py::iadd(&v_res, &f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("I"))?, &v_Z.clone())?, v_V.clone())?)?;
    v_res = py::iadd(&v_res, &py::add(&py::truediv(&py::abs(&py::sub(&py::mul(&v_Z.clone(), &py::math1("cos", &py::getitem(&v_o.clone(), &Py::str("phi"))?)?)?, &v_R.clone())?)?, &v_Z.clone())?, &py::truediv(&py::abs(&py::sub(&py::mul(&v_Z.clone(), &py::math1("sin", &py::getitem(&v_o.clone(), &Py::str("phi"))?)?)?, &v_X.clone())?)?, &v_Z.clone())?)?)?;
    v_res = py::iadd(&v_res, &py::add(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("pf"))?, &py::truediv(&v_R.clone(), &v_Z.clone())?)?)?, &f__rel(py::getitem(&v_o.clone(), &Py::str("P"))?, py::mul(&py::mul(&v_V.clone(), &py::getitem(&v_o.clone(), &Py::str("I"))?)?, &py::getitem(&v_o.clone(), &Py::str("pf"))?)?)?)?)?;
    return Ok(py::add(&v_res.clone(), &py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("phi_deg"))?, &py::math1("degrees", &py::getitem(&v_o.clone(), &Py::str("phi"))?)?)?)?, &Py::Int(180i64))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_verify_rotation(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("alpha"))?, &py::getitem(&v_i.clone(), &Py::str("I"))?)?, py::getitem(&v_i.clone(), &Py::str("tau"))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_verify_shear(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_dx: Py = Py::None;
    let mut v_sn: Py = Py::None;
    let mut v_st: Py = Py::None;
    let t413 = (Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("stress"))?, py::getitem(&v_o.clone(), &Py::str("strain"))?, py::getitem(&v_o.clone(), &Py::str("dx"))?])).items()?;
    if t413.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_st = t413[0].clone();
    v_sn = t413[1].clone();
    v_dx = t413[2].clone();
    if (py::not(&{ let t415 = py::math1("isfinite", &v_st.clone())?; if !t415.truthy() { t415 } else { { let t414 = py::math1("isfinite", &v_sn.clone())?; if !t414.truthy() { t414 } else { py::math1("isfinite", &v_dx.clone())? } } } })).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    if ({ let t416 = py::getitem(&v_i.clone(), &Py::str("F"))?; Py::Bool({ let t417 = Py::Int(0i64); if py::eq(&t416, &t417) { true } else { false } }) }).truthy() {
        return Ok(py::add(&py::add(&py::abs(&v_st.clone())?, &py::abs(&v_sn.clone())?)?, &py::abs(&v_dx.clone())?)?);
    }
    return Ok(py::add(&py::add(&f__rel(py::mul(&v_st.clone(), &py::getitem(&v_i.clone(), &Py::str("A"))?)?, py::getitem(&v_i.clone(), &Py::str("F"))?)?, &f__rel(py::mul(&v_sn.clone(), &py::getitem(&v_i.clone(), &Py::str("S"))?)?, v_st.clone())?)?, &f__rel(v_dx.clone(), py::mul(&v_sn.clone(), &py::getitem(&v_i.clone(), &Py::str("h"))?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_verify_single_slit(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_L: Py = Py::None;
    let mut v_W: Py = Py::None;
    let mut v_Wa: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_dd: Py = Py::None;
    let mut v_lam: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_res: Py = Py::None;
    let mut v_th: Py = Py::None;
    let mut v_y: Py = Py::None;
    let t435 = (Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("theta"))?, py::getitem(&v_o.clone(), &Py::str("y"))?, py::getitem(&v_o.clone(), &Py::str("W"))?, py::getitem(&v_o.clone(), &Py::str("W_angle"))?, py::getitem(&v_o.clone(), &Py::str("theta_deg"))?])).items()?;
    if t435.len() != 5 { return Err(PyErr::Value("unpack".into())); }
    v_th = t435[0].clone();
    v_y = t435[1].clone();
    v_W = t435[2].clone();
    v_Wa = t435[3].clone();
    v_dd = t435[4].clone();
    if (py::not(&{ let t439 = py::math1("isfinite", &v_th.clone())?; if !t439.truthy() { t439 } else { { let t438 = py::math1("isfinite", &v_y.clone())?; if !t438.truthy() { t438 } else { { let t437 = py::math1("isfinite", &v_W.clone())?; if !t437.truthy() { t437 } else { { let t436 = py::math1("isfinite", &v_Wa.clone())?; if !t436.truthy() { t436 } else { py::math1("isfinite", &v_dd.clone())? } } } } } } } })).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    if ({ let t450 = { let t440 = v_th.clone(); Py::Bool({ let t441 = Py::Int(0i64); if py::le(&t440, &t441)? { true } else { false } }) }; if t450.truthy() { t450 } else { { let t449 = { let t442 = v_th.clone(); Py::Bool({ let t443 = py::truediv(&Py::Float(std::f64::consts::PI), &Py::Int(2i64))?; if py::ge(&t442, &t443)? { true } else { false } }) }; if t449.truthy() { t449 } else { { let t448 = { let t444 = v_Wa.clone(); Py::Bool({ let t445 = Py::Int(0i64); if py::le(&t444, &t445)? { true } else { false } }) }; if t448.truthy() { t448 } else { { let t446 = v_Wa.clone(); Py::Bool({ let t447 = Py::Float(std::f64::consts::PI); if py::ge(&t446, &t447)? { true } else { false } }) } } } } } } }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    let t451 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("lam"))?, py::getitem(&v_i.clone(), &Py::str("m"))?, py::getitem(&v_i.clone(), &Py::str("L"))?])).items()?;
    if t451.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_a = t451[0].clone();
    v_lam = t451[1].clone();
    v_m = t451[2].clone();
    v_L = t451[3].clone();
    v_res = py::add(&py::truediv(&py::abs(&py::sub(&py::mul(&v_a.clone(), &py::math1("sin", &v_th.clone())?)?, &py::mul(&v_m.clone(), &v_lam.clone())?)?)?, &py::mul(&v_m.clone(), &v_lam.clone())?)?, &py::truediv(&py::abs(&py::sub(&v_y.clone(), &py::mul(&v_L.clone(), &py::math1("tan", &v_th.clone())?)?)?)?, &py::minmax(&[v_L.clone(), py::abs(&v_y.clone())?], true)?)?)?;
    v_res = py::iadd(&v_res, &py::truediv(&py::abs(&py::sub(&py::mul(&v_a.clone(), &py::math1("sin", &py::truediv(&v_Wa.clone(), &Py::Int(2i64))?)?)?, &v_lam.clone())?)?, &v_lam.clone())?)?;
    v_res = py::iadd(&v_res, &py::truediv(&py::abs(&py::sub(&v_W.clone(), &py::mul(&py::mul(&Py::Int(2i64), &v_L.clone())?, &py::math1("tan", &py::truediv(&v_Wa.clone(), &Py::Int(2i64))?)?)?)?)?, &py::minmax(&[v_L.clone(), py::abs(&v_W.clone())?], true)?)?)?;
    return Ok(py::add(&v_res.clone(), &py::truediv(&py::abs(&py::sub(&v_dd.clone(), &py::math1("degrees", &v_th.clone())?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&v_dd.clone())?], true)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_verify_snell(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_d2: Py = Py::None;
    let mut v_t2: Py = Py::None;
    let t465 = (Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("theta2"))?, py::getitem(&v_o.clone(), &Py::str("theta2_deg"))?])).items()?;
    if t465.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_t2 = t465[0].clone();
    v_d2 = t465[1].clone();
    if ({ let t472 = py::not(&{ let t466 = py::math1("isfinite", &v_t2.clone())?; if !t466.truthy() { t466 } else { py::math1("isfinite", &v_d2.clone())? } }); if t472.truthy() { t472 } else { { let t471 = { let t467 = v_t2.clone(); Py::Bool({ let t468 = Py::Int(0i64); if py::lt(&t467, &t468)? { true } else { false } }) }; if t471.truthy() { t471 } else { { let t469 = v_t2.clone(); Py::Bool({ let t470 = py::add(&py::truediv(&Py::Float(std::f64::consts::PI), &Py::Int(2i64))?, &Py::Float(1e-12))?; if py::gt(&t469, &t470)? { true } else { false } }) } } } } }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_a = py::mul(&py::getitem(&v_i.clone(), &Py::str("n1"))?, &py::math1("sin", &py::minmax(&[py::getitem(&v_i.clone(), &Py::str("theta1"))?, py::truediv(&Py::Float(std::f64::consts::PI), &Py::Int(2i64))?], false)?)?)?;
    v_b = py::mul(&py::getitem(&v_i.clone(), &Py::str("n2"))?, &py::math1("sin", &v_t2.clone())?)?;
    return Ok(py::add(&py::truediv(&py::abs(&py::sub(&v_a.clone(), &v_b.clone())?)?, &py::minmax(&[py::abs(&v_a.clone())?, py::abs(&v_b.clone())?, Py::Float(1e-300)], true)?)?, &py::truediv(&py::abs(&py::sub(&v_d2.clone(), &py::math1("degrees", &v_t2.clone())?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&v_d2.clone())?], true)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_verify_spin(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_I1: Py = Py::None;
    let mut v_I2: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_res: Py = Py::None;
    let mut v_w1: Py = Py::None;
    let mut v_w2: Py = Py::None;
    for t4 in py::iter(&(Py::tuple(vec![Py::str("omega2"), Py::str("L"), Py::str("K1"), Py::str("K2"), Py::str("dK")])))? {
        v_k = t4;
        if (py::not(&py::math1("isfinite", &py::getitem(&v_o.clone(), &v_k.clone())?)?)).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
    }
    let t5 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("I1"))?, py::getitem(&v_i.clone(), &Py::str("omega1"))?, py::getitem(&v_i.clone(), &Py::str("I2"))?])).items()?;
    if t5.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_I1 = t5[0].clone();
    v_w1 = t5[1].clone();
    v_I2 = t5[2].clone();
    v_w2 = py::getitem(&v_o.clone(), &Py::str("omega2"))?;
    v_res = py::add(&f__rel(py::mul(&v_I2.clone(), &v_w2.clone())?, py::mul(&v_I1.clone(), &v_w1.clone())?)?, &f__rel(py::getitem(&v_o.clone(), &Py::str("L"))?, py::mul(&v_I1.clone(), &v_w1.clone())?)?)?;
    v_res = py::iadd(&v_res, &py::add(&f__rel(py::getitem(&v_o.clone(), &Py::str("K1"))?, py::truediv(&py::mul(&py::mul(&v_I1.clone(), &v_w1.clone())?, &v_w1.clone())?, &Py::Int(2i64))?)?, &f__rel(py::getitem(&v_o.clone(), &Py::str("K2"))?, py::truediv(&py::mul(&py::mul(&v_I2.clone(), &v_w2.clone())?, &v_w2.clone())?, &Py::Int(2i64))?)?)?)?;
    return Ok(py::add(&v_res.clone(), &py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("dK"))?, &py::sub(&py::getitem(&v_o.clone(), &Py::str("K2"))?, &py::getitem(&v_o.clone(), &Py::str("K1"))?)?)?)?, &py::minmax(&[py::getitem(&v_o.clone(), &Py::str("K1"))?, py::getitem(&v_o.clone(), &Py::str("K2"))?, Py::Float(1e-300)], true)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_verify_standing(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_f1: Py = Py::None;
    let mut v_fn: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_lam: Py = Py::None;
    let t549 = (Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("f_n"))?, py::getitem(&v_o.clone(), &Py::str("lam_n"))?, py::getitem(&v_o.clone(), &Py::str("f1"))?])).items()?;
    if t549.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_fn = t549[0].clone();
    v_lam = t549[1].clone();
    v_f1 = t549[2].clone();
    if (py::not(&{ let t551 = py::math1("isfinite", &v_fn.clone())?; if !t551.truthy() { t551 } else { { let t550 = py::math1("isfinite", &v_lam.clone())?; if !t550.truthy() { t550 } else { py::math1("isfinite", &v_f1.clone())? } } } })).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_k = f__pipe_length_factor(py::getitem(&v_i.clone(), &Py::str("system"))?)?;
    return Ok(py::add(&py::add(&f__rel(py::mul(&v_fn.clone(), &v_lam.clone())?, py::getitem(&v_i.clone(), &Py::str("v"))?)?, &f__rel(py::mul(&v_lam.clone(), &py::getitem(&v_i.clone(), &Py::str("n"))?)?, py::mul(&v_k.clone(), &py::getitem(&v_i.clone(), &Py::str("L"))?)?)?)?, &f__rel(py::mul(&py::mul(&v_f1.clone(), &v_k.clone())?, &py::getitem(&v_i.clone(), &Py::str("L"))?)?, py::getitem(&v_i.clone(), &Py::str("v"))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_verify_static_limit(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_F: Py = Py::None;
    let mut v_N: Py = Py::None;
    let mut v_fm: Py = Py::None;
    let mut v_mu: Py = Py::None;
    let t608 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("mu_s"))?, py::getitem(&v_i.clone(), &Py::str("N"))?, py::getitem(&v_i.clone(), &Py::str("F"))?])).items()?;
    if t608.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_mu = t608[0].clone();
    v_N = t608[1].clone();
    v_F = t608[2].clone();
    v_fm = py::getitem(&v_o.clone(), &Py::str("f_max"))?;
    if (py::not(&{ let t609 = py::math1("isfinite", &v_fm.clone())?; if !t609.truthy() { t609 } else { py::math1("isfinite", &py::getitem(&v_o.clone(), &Py::str("margin"))?)? } })).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    if ({ let t610 = py::getitem(&v_o.clone(), &Py::str("slips"))?; Py::Bool({ let t611 = f__slips_level(v_mu.clone(), v_N.clone(), v_F.clone())?; if py::ne(&t610, &t611) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    return Ok(py::add(&f__rel(v_fm.clone(), py::mul(&v_mu.clone(), &v_N.clone())?)?, &py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("margin"))?, &py::sub(&v_fm.clone(), &v_F.clone())?)?)?, &py::minmax(&[py::abs(&v_fm.clone())?, v_F.clone(), Py::Float(1e-300)], true)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_verify_string(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_v: Py = Py::None;
    v_v = py::getitem(&v_o.clone(), &Py::str("v"))?;
    if ({ let t620 = py::not(&py::math1("isfinite", &v_v.clone())?); if t620.truthy() { t620 } else { { let t618 = v_v.clone(); Py::Bool({ let t619 = Py::Int(0i64); if py::le(&t618, &t619)? { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    return Ok(f__rel(py::mul(&py::mul(&py::getitem(&v_i.clone(), &Py::str("mu"))?, &v_v.clone())?, &v_v.clone())?, py::getitem(&v_i.clone(), &Py::str("T"))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_verify_terminal(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_I: Py = Py::None;
    let mut v_R: Py = Py::None;
    let mut v_emf: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_res: Py = Py::None;
    for t636 in py::iter(&(Py::tuple(vec![Py::str("I"), Py::str("V_t"), Py::str("V_lost"), Py::str("P_load"), Py::str("efficiency")])))? {
        v_k = t636;
        if (py::not(&py::math1("isfinite", &py::getitem(&v_o.clone(), &v_k.clone())?)?)).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
    }
    let t637 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("emf"))?, py::getitem(&v_i.clone(), &Py::str("r"))?, py::getitem(&v_i.clone(), &Py::str("R"))?])).items()?;
    if t637.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_emf = t637[0].clone();
    v_r = t637[1].clone();
    v_R = t637[2].clone();
    v_I = py::getitem(&v_o.clone(), &Py::str("I"))?;
    if ({ let t638 = v_I.clone(); Py::Bool({ let t639 = Py::Int(0i64); if py::le(&t638, &t639)? { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_res = f__rel(py::mul(&v_I.clone(), &py::add(&v_R.clone(), &v_r.clone())?)?, v_emf.clone())?;
    v_res = py::iadd(&v_res, &py::add(&py::add(&py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("V_t"))?, &py::mul(&v_I.clone(), &v_R.clone())?)?)?, &v_emf.clone())?, &py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("V_lost"))?, &py::mul(&v_I.clone(), &v_r.clone())?)?)?, &v_emf.clone())?)?, &py::truediv(&py::abs(&py::sub(&py::add(&py::getitem(&v_o.clone(), &Py::str("V_t"))?, &py::getitem(&v_o.clone(), &Py::str("V_lost"))?)?, &v_emf.clone())?)?, &v_emf.clone())?)?)?;
    v_res = py::iadd(&v_res, &py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("P_load"))?, &py::mul(&py::getitem(&v_o.clone(), &Py::str("V_t"))?, &v_I.clone())?)?)?, &py::mul(&v_emf.clone(), &v_I.clone())?)?)?;
    return Ok(py::add(&v_res.clone(), &py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("efficiency"))?, &py::truediv(&py::getitem(&v_o.clone(), &Py::str("V_t"))?, &v_emf.clone())?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_verify_transformer(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_Np: Py = Py::None;
    let mut v_Ns: Py = Py::None;
    let mut v_Vp: Py = Py::None;
    let mut v_res: Py = Py::None;
    let mut v_want: Py = Py::None;
    let t309 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("Np"))?, py::getitem(&v_i.clone(), &Py::str("Ns"))?, py::getitem(&v_i.clone(), &Py::str("Vp"))?])).items()?;
    if t309.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_Np = t309[0].clone();
    v_Ns = t309[1].clone();
    v_Vp = t309[2].clone();
    v_res = py::add(&f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("Vs"))?, &v_Np.clone())?, py::mul(&v_Vp.clone(), &v_Ns.clone())?)?, &f__rel(py::getitem(&v_o.clone(), &Py::str("ratio"))?, py::truediv(&v_Ns.clone(), &v_Np.clone())?)?)?;
    v_want = (if ({ let t313 = v_Ns.clone(); Py::Bool({ let t314 = v_Np.clone(); if py::gt(&t313, &t314)? { true } else { false } }) }).truthy() { Py::str("step-up") } else { (if ({ let t315 = v_Ns.clone(); Py::Bool({ let t316 = v_Np.clone(); if py::lt(&t315, &t316)? { true } else { false } }) }).truthy() { Py::str("step-down") } else { Py::str("isolation (1:1)") }) });
    if ({ let t317 = py::getitem(&v_o.clone(), &Py::str("type"))?; Py::Bool({ let t318 = v_want.clone(); if py::ne(&t317, &t318) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    if ({ let t319 = py::method(&v_i.clone(), "get", &[Py::str("Ip")])?; Py::Bool({ let t320 = Py::None; if !py::is(&t319, &t320) { true } else { false } }) }).truthy() {
        if ({ let t321 = Py::str("Is"); Py::Bool({ let t322 = v_o.clone(); if !py::contains(&t322, &t321)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_res = py::iadd(&v_res, &f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("Is"))?, &v_Ns.clone())?, py::mul(&py::getitem(&v_i.clone(), &Py::str("Ip"))?, &v_Np.clone())?)?)?;
    }
    return Ok(v_res.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_verify_vectors(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_R: Py = Py::None;
    let mut v_Rx: Py = Py::None;
    let mut v_Ry: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_dirs: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_mags: Py = Py::None;
    let mut v_res: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_scale: Py = Py::None;
    let mut v_sx: Py = Py::None;
    let mut v_sy: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_xs: Py = Py::None;
    let mut v_ys: Py = Py::None;
    let t674 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("magnitudes"))?, py::getitem(&v_i.clone(), &Py::str("directions"))?])).items()?;
    if t674.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_mags = t674[0].clone();
    v_dirs = t674[1].clone();
    let t675 = (Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("x_components"))?, py::getitem(&v_o.clone(), &Py::str("y_components"))?])).items()?;
    if t675.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_xs = t675[0].clone();
    v_ys = t675[1].clone();
    if ({ let t680 = { let t676 = py::len(&v_xs.clone())?; Py::Bool({ let t677 = py::len(&v_mags.clone())?; if py::ne(&t676, &t677) { true } else { false } }) }; if t680.truthy() { t680 } else { { let t678 = py::len(&v_ys.clone())?; Py::Bool({ let t679 = py::len(&v_mags.clone())?; if py::ne(&t678, &t679) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    let t681 = (Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("Rx"))?, py::getitem(&v_o.clone(), &Py::str("Ry"))?, py::getitem(&v_o.clone(), &Py::str("R"))?, py::getitem(&v_o.clone(), &Py::str("direction_deg"))?])).items()?;
    if t681.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_Rx = t681[0].clone();
    v_Ry = t681[1].clone();
    v_R = t681[2].clone();
    v_d = t681[3].clone();
    if (py::not(&{ let t684 = py::math1("isfinite", &v_Rx.clone())?; if !t684.truthy() { t684 } else { { let t683 = py::math1("isfinite", &v_Ry.clone())?; if !t683.truthy() { t683 } else { { let t682 = py::math1("isfinite", &v_R.clone())?; if !t682.truthy() { t682 } else { py::math1("isfinite", &v_d.clone())? } } } } } })).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_scale = Py::Float(0.0);
    for t685 in py::range_iter(&[(py::len(&v_mags.clone())?).i()?]) {
        v_k = t685;
        v_scale = py::iadd(&v_scale, &py::getitem(&v_mags.clone(), &v_k.clone())?)?;
    }
    if ({ let t686 = v_scale.clone(); Py::Bool({ let t687 = Py::Int(0i64); if py::eq(&t686, &t687) { true } else { false } }) }).truthy() {
        v_scale = Py::Float(1.0);
    }
    let t688 = (Py::tuple(vec![Py::Float(0.0), Py::Float(0.0), Py::Float(0.0)])).items()?;
    if t688.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_res = t688[0].clone();
    v_sx = t688[1].clone();
    v_sy = t688[2].clone();
    for t689 in py::range_iter(&[(py::len(&v_mags.clone())?).i()?]) {
        v_k = t689;
        let t690 = (f__cos_sin_deg(py::getitem(&v_dirs.clone(), &v_k.clone())?)?).items()?;
        if t690.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_c = t690[0].clone();
        v_s = t690[1].clone();
        v_res = py::iadd(&v_res, &py::add(&py::truediv(&py::abs(&py::sub(&py::getitem(&v_xs.clone(), &v_k.clone())?, &py::mul(&py::getitem(&v_mags.clone(), &v_k.clone())?, &v_c.clone())?)?)?, &v_scale.clone())?, &py::truediv(&py::abs(&py::sub(&py::getitem(&v_ys.clone(), &v_k.clone())?, &py::mul(&py::getitem(&v_mags.clone(), &v_k.clone())?, &v_s.clone())?)?)?, &v_scale.clone())?)?)?;
        v_sx = py::iadd(&v_sx, &py::mul(&py::getitem(&v_mags.clone(), &v_k.clone())?, &v_c.clone())?)?;
        v_sy = py::iadd(&v_sy, &py::mul(&py::getitem(&v_mags.clone(), &v_k.clone())?, &v_s.clone())?)?;
    }
    v_res = py::iadd(&v_res, &py::add(&py::add(&py::truediv(&py::abs(&py::sub(&v_Rx.clone(), &v_sx.clone())?)?, &v_scale.clone())?, &py::truediv(&py::abs(&py::sub(&v_Ry.clone(), &v_sy.clone())?)?, &v_scale.clone())?)?, &py::truediv(&py::abs(&py::sub(&v_R.clone(), &py::hypot(&[v_Rx.clone(), v_Ry.clone()])?)?)?, &v_scale.clone())?)?)?;
    if ({ let t691 = v_R.clone(); Py::Bool({ let t692 = Py::Int(0i64); if py::gt(&t691, &t692)? { true } else { false } }) }).truthy() {
        if ({ let t697 = { let t693 = v_d.clone(); Py::Bool({ let t694 = Py::Int(0i64); if py::lt(&t693, &t694)? { true } else { false } }) }; if t697.truthy() { t697 } else { { let t695 = v_d.clone(); Py::Bool({ let t696 = Py::Int(360i64); if py::ge(&t695, &t696)? { true } else { false } }) } } }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_t = py::math1("radians", &v_d.clone())?;
        v_res = py::iadd(&v_res, &py::add(&py::truediv(&py::abs(&py::sub(&py::mul(&v_R.clone(), &py::math1("cos", &v_t.clone())?)?, &v_Rx.clone())?)?, &v_scale.clone())?, &py::truediv(&py::abs(&py::sub(&py::mul(&v_R.clone(), &py::math1("sin", &v_t.clone())?)?, &v_Ry.clone())?)?, &v_scale.clone())?)?)?;
    } else {
        v_res = py::iadd(&v_res, &py::abs(&v_d.clone())?)?;
    }
    return Ok(v_res.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_verify_young(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_dl: Py = Py::None;
    let mut v_sn: Py = Py::None;
    let mut v_st: Py = Py::None;
    let t703 = (Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("stress"))?, py::getitem(&v_o.clone(), &Py::str("strain"))?, py::getitem(&v_o.clone(), &Py::str("dL"))?])).items()?;
    if t703.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_st = t703[0].clone();
    v_sn = t703[1].clone();
    v_dl = t703[2].clone();
    if (py::not(&{ let t705 = py::math1("isfinite", &v_st.clone())?; if !t705.truthy() { t705 } else { { let t704 = py::math1("isfinite", &v_sn.clone())?; if !t704.truthy() { t704 } else { py::math1("isfinite", &v_dl.clone())? } } } })).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    if ({ let t706 = py::getitem(&v_i.clone(), &Py::str("F"))?; Py::Bool({ let t707 = Py::Int(0i64); if py::eq(&t706, &t707) { true } else { false } }) }).truthy() {
        return Ok(py::add(&py::add(&py::abs(&v_st.clone())?, &py::abs(&v_sn.clone())?)?, &py::abs(&v_dl.clone())?)?);
    }
    return Ok(py::add(&py::add(&f__rel(py::mul(&v_st.clone(), &py::getitem(&v_i.clone(), &Py::str("A"))?)?, py::getitem(&v_i.clone(), &Py::str("F"))?)?, &f__rel(py::mul(&v_sn.clone(), &py::getitem(&v_i.clone(), &Py::str("E"))?)?, v_st.clone())?)?, &f__rel(v_dl.clone(), py::mul(&v_sn.clone(), &py::getitem(&v_i.clone(), &Py::str("L0"))?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_young_deformation(mut v_F: Py, mut v_A: Py, mut v_L0: Py, mut v_E: Py) -> R {
    let mut v_strain: Py = Py::None;
    let mut v_stress: Py = Py::None;
    let _ = f__positive(v_A.clone(), Py::str("the cross-sectional area A"))?;
    let _ = f__positive(v_L0.clone(), Py::str("the original length L0"))?;
    let _ = f__positive(v_E.clone(), Py::str("Young's modulus E"))?;
    v_stress = py::truediv(&v_F.clone(), &v_A.clone())?;
    v_strain = py::truediv(&v_stress.clone(), &v_E.clone())?;
    if (py::not(&{ let t701 = py::abs(&v_strain.clone())?; Py::Bool({ let t702 = Py::Int(1i64); if py::lt(&t701, &t702)? { true } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a change of length as large as the original length is outside linear elasticity"))));
    }
    return Ok(Py::dict(vec![(Py::str("stress"), v_stress.clone()), (Py::str("strain"), v_strain.clone()), (Py::str("dL"), py::mul(&v_strain.clone(), &v_L0.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}
