//! Classical Physics' charts for the visitor's inputs: a value over two inputs (field), curve families (graph,
//! curve-2), comparisons and bars, every point a solve of the model (this capability again, or a neighbouring one).

use crate::figs_common::{self as c, qv};
use ax_engine::figkit::{self as k, Solved};
use ax_engine::figures::Figure;
use ax_engine::Input;
use std::f64::consts::PI;

pub fn live(s: &Solved) -> Vec<Figure> {
    let v: Vec<Option<Figure>> = match s.key {
        "momentum" => vec![c::field_map(s, "m", "v", "p")],
        "kinetic_energy" => vec![fall_energy(s), c::field_map(s, "v", "m", "K")],
        "power_energy" => vec![c::field_map(s, "E", "t", "P")],
        "power_force_speed" => vec![c::field_map(s, "F", "v", "P")],
        "density" => vec![c::field_map(s, "m", "V", "rho")],
        "pressure_force" => vec![c::field_map(s, "F", "A", "P")],
        "electric_power" => vec![c::field_map(s, "V", "I", "P")],
        "charge_current" => vec![c::field_map(s, "I", "t", "Q")],
        "acceleration_frictionless_incline" => vec![c::field_map(s, "g", "theta", "a")],
        "ohm_power_current" => vec![c::field_map(s, "I", "R", "P")],
        "ohm_power_voltage" => vec![c::field_map(s, "V", "R", "P")],
        "rotational_kinetic_energy" => vec![c::field_map(s, "omega", "Ir", "KR")],
        "angular_momentum" => vec![c::field_map(s, "Ir", "omega", "L")],
        "tangential_acceleration" => vec![c::field_map(s, "r", "alpha", "a")],
        "centripetal_acceleration_omega" => vec![c::field_map(s, "omega", "r", "a")],
        "flow_rate_volume_time" => vec![c::field_map(s, "V", "t", "Q")],
        "flow_rate_area_speed" => vec![c::field_map(s, "A", "v", "Q")],
        "engine_efficiency" => vec![c::field_map(s, "W", "Qh", "eff")],
        "carnot_efficiency" => vec![c::field_map(s, "Tc", "Th", "eff")],
        "intensity_power_area" => vec![c::field_map(s, "P", "A", "I")],
        "rl_time_constant" => vec![c::field_map(s, "L", "R", "tau")],
        "inductive_reactance" => vec![c::field_map(s, "f", "L", "X")],
        "capacitive_reactance" => vec![c::field_map(s, "f", "C", "X")],
        "rc_time_constant" => vec![c::field_map(s, "R", "C", "tau")],
        "rc_charging" => vec![c::family_std(s, "curve-2", "t", "V", "emf", 0.1, 10.0, true)],
        "driven_amplitude" => vec![c::family_std(s, "curve-2", "m", "A", "F0", 0.1, 10.0, true)],
        // graphs
        "uniform_acceleration_state" => vec![state_graph(s)],
        "velocity_time" => vec![velocity_lines(s)],
        "displacement_time" => vec![displacement_parabola(s)],
        "velocity_displacement" => vec![speed_distance(s)],
        "average_velocity_displacement" => vec![trapezoid(s)],
        "impulse" => vec![impulse_area(s)],
        "work" => vec![work_angle(s)],
        "spring_energy" => vec![spring_triangle(s)],
        "period_frequency" => vec![period_wave(s)],
        "max_height_vertical_throw" => vec![throws(s)],
        "time_to_top_vertical_throw" => vec![throw_velocity(s)],
        "average_force_from_momentum_change" => vec![momentum_slope(s)],
        "capacitance_definition" => vec![charge_lines(s)],
        "capacitor_energy" => vec![capacitor_triangle(s)],
        "parallel_plate_capacitor" => vec![plate_gap(s)],
        "orbital_period" => vec![period_radius(s)],
        "latent_heat_fusion" => vec![heating_curve(s)],
        "photon_energy_wavelength" => vec![photon_spectrum(s)],
        "absolute_pressure_depth" => vec![pressure_depth(s)],
        "rms_voltage" => vec![rms_wave(s, "V0", "Vrms", "V", "voltage")],
        "rms_current" => vec![rms_wave(s, "I0", "Irms", "A", "current")],
        "lc_resonance" => vec![reactances(s)],
        "rl_current_decay" => vec![decay(s)],
        "damped_angular_frequency" => vec![damped(s)],
        "damping_ratio" => vec![regimes(s)],
        "kepler_period_semi_major" => vec![kepler(s)],
        "bead_hoop_equilibrium" => vec![bead_graph(s)],
        // bars
        "newton_second" => vec![force_bars(s)],
        "weight" => vec![weight_bars(s)],
        "surface_gravity" => vec![gravity_bars(s)],
        "latent_heat_vaporization" => vec![water_heats(s)],
        "index_of_refraction" => vec![index_bars(s)],
        // comparisons
        "weight_component_along_incline" => vec![incline_parts(s)],
        "double_pendulum_slow_mode" => vec![pendulum_modes(s)],
        "escape_speed" => vec![escape_orbit(s)],
        "inertia_solid_sphere" | "inertia_hollow_sphere" | "inertia_solid_disk" | "inertia_rod_center"
        | "inertia_rod_end" => vec![inertia_shapes(s)],
        "rolling_incline_acceleration" => vec![rolling_race(s)],
        "elastic_collision_second_final" => vec![collision_ratio(s)],
        _ => vec![],
    };
    v.into_iter().flatten().collect()
}

// ------------------------------------------------------------------------------------------------ helpers
/// Another capability solved with numbers (SI); its value `y`.
fn o(s: &Solved, key: &str, set: &[(&str, f64)], y: &str) -> Option<f64> {
    if set.iter().any(|x| !x.1.is_finite()) {
        return None;
    }
    let v: Vec<(&str, Input)> = set.iter().map(|(k_, x)| (*k_, Input::Float(*x))).collect();
    let a = s.solve_other(key, &v)?;
    k::num_of(&a, y)
}

/// The gravitational acceleration of the request when stated, for neighbouring formulas that default it.
fn with_g<'a>(s: &Solved, mut set: Vec<(&'a str, f64)>) -> Vec<(&'a str, f64)> {
    if let Some(g) = s.inp("g") {
        set.push(("g", g));
    }
    set
}

fn pts(xs: &[f64], f: impl Fn(f64) -> Option<f64>) -> Vec<(f64, f64)> {
    xs.iter().filter_map(|x| f(*x).map(|y| (*x, y))).collect()
}

fn ser(name: &str, p: Vec<(f64, f64)>) -> (String, Vec<(f64, f64)>) {
    (name.to_string(), p)
}

fn deg(r: f64) -> f64 {
    r * 180.0 / PI
}

/// A chart from series (each needs two points), the generic frame.
#[allow(clippy::too_many_arguments)]
fn chart(s: &Solved, kind: &str, title: &str, cap: &str, series: Vec<(String, Vec<(f64, f64)>)>, xlab: &str,
         ylab: &str, header: &str, marker: Option<(f64, f64)>, mlabel: &str, log_x: bool) -> Option<Figure> {
    let series: Vec<(String, Vec<(f64, f64)>)> = series.into_iter().filter(|x| x.1.len() >= 2).collect();
    if series.is_empty() {
        return None;
    }
    let body = c::fit_right(&k::lines(&series, xlab, ylab, header, marker, mlabel, log_x));
    Some(s.chart(kind, title, cap, &body))
}

fn bars_fig(s: &Solved, kind: &str, title: &str, cap: &str, labels: Vec<String>, vals: Vec<f64>, hl: Vec<usize>,
            ylab: &str, header: &str) -> Option<Figure> {
    if vals.len() < 2 || vals.iter().any(|v| !v.is_finite()) {
        return None;
    }
    let body = c::fit_right(&k::bars(&labels, &vals, &hl, "", ylab, header));
    Some(s.chart(kind, title, cap, &body))
}

// ------------------------------------------------------------------------------------------------ graphs
fn state_graph(s: &Solved) -> Option<Figure> {
    let (u, a, t) = (s.inp("u")?, s.inp("a")?, s.inp("t")?);
    let (v, d) = (s.out("v")?, s.out("s")?);
    if !(t > 0.0) {
        return None;
    }
    let ts = k::span(0.0, t * 1.5, 24, false);
    let mut vp = vec![];
    let mut sp = vec![];
    for tt in &ts {
        if let Some(b) = s.with(&[("t", *tt)]) {
            if let (Some(vv), Some(ss)) = (k::num_of(&b, "v"), k::num_of(&b, "s")) {
                vp.push((*tt, vv));
                sp.push((*tt, ss));
            }
        }
    }
    if vp.len() < 2 {
        return None;
    }
    let top = k::lines(&[ser("velocity v", vp)], "", "v, velocity (m/s)", &format!("v = {}", qv(v, "m/s")),
                       Some((t, v)), &qv(v, "m/s"), false);
    let bottom = k::lines(&[ser("displacement s", sp)], "t, time (s)", "s, displacement (m)", &format!("s = {}", qv(d, "m")),
                          Some((t, d)), &qv(d, "m"), false);
    let (body, h) = k::stack(&[c::fit_right(&top), c::fit_right(&bottom)]);
    Some(s.figure("graph", "Velocity and displacement under constant acceleration",
                  &format!("Velocity (top) and displacement (bottom) against time for a body starting at u = {} with a \
                            constant acceleration a = {}, 24 times each a solve of the model; the area under the \
                            velocity line up to t = {} is the displacement. Marked: v = {} and s = {} at t = {}.",
                           qv(u, "m/s"), qv(a, "m/s^2"), qv(t, "s"), qv(v, "m/s"), qv(d, "m"), qv(t, "s")), &body, h))
}

fn velocity_lines(s: &Solved) -> Option<Figure> {
    let (v0, a, t, v) = (s.inp("v0")?, s.inp("a")?, s.inp("t")?, s.get("v")?);
    if !(t > 0.0) {
        return None;
    }
    let ts = k::span(0.0, t * 1.5, 12, false);
    let aa = if a != 0.0 { [a * 0.25, a, -a * 0.5] } else { [1.0, 0.0, -1.0] };
    let mut series = vec![];
    let mut marks = vec![];
    for x in aa {
        let p = s.curve("t", &ts, "v", &[("a", x)]);
        if let Some(vt) = c::at(s, &[("a", x), ("t", t)], "v") {
            marks.push(format!("a = {}: v = {}", qv(x, "m/s^2"), qv(vt, "m/s")));
        }
        series.push((format!("a = {}", qv(x, "m/s^2")), p));
    }
    chart(s, "graph", "Velocity against time and the slope of each line",
          &format!("Velocity against time from v0 = {} for three constant accelerations, 12 times each a solve of the \
                    model; the slope of each line is its acceleration. Marked at t = {}: {}.", qv(v0, "m/s"),
                   qv(t, "s"), marks.join("; ")),
          series, "t, time (s)", "v, velocity (m/s)", &format!("v = {} at t = {}", qv(v, "m/s"), qv(t, "s")),
          Some((t, v)), &qv(v, "m/s"), false)
}

fn displacement_parabola(s: &Solved) -> Option<Figure> {
    let (v0, a, t, x) = (s.inp("v0")?, s.inp("a")?, s.inp("t")?, s.get("x")?);
    if !(t > 0.0) {
        return None;
    }
    let ts = k::span(0.0, t * 1.25, 20, false);
    let acc = s.curve("t", &ts, "x", &[]);
    let flat = s.curve("t", &ts, "x", &[("a", 0.0)]);
    let x_flat = c::at(s, &[("a", 0.0)], "x")?;
    chart(s, "graph", "Displacement grows as a parabola",
          &format!("Displacement against time for v0 = {} and a = {} (20 times, each a solve of the model), beside the \
                    straight line the same start speed gives with no acceleration (also from the model); the gap \
                    between them at t = {} is the part due to the acceleration. Marked: x = {} at t = {}, {} without \
                    acceleration.", qv(v0, "m/s"), qv(a, "m/s^2"), qv(t, "s"), qv(x, "m"), qv(t, "s"), qv(x_flat, "m")),
          vec![ser(&format!("a = {}", qv(a, "m/s^2")), acc), ser("a = 0", flat)], "t, time (s)", "x, displacement (m)",
          &format!("x = {}", qv(x, "m")), Some((t, x)), &qv(x, "m"), false)
}

fn speed_distance(s: &Solved) -> Option<Figure> {
    let (v0, a, x, v) = (s.inp("v0")?, s.inp("a")?, s.inp("x")?, s.get("v")?);
    if !(x > 0.0) {
        return None;
    }
    let xs = k::span(0.0, x * 1.5, 24, false);
    let mut series = vec![];
    let mut marks = vec![];
    for f in [0.5, 1.0, 2.0] {
        let p = s.curve("x", &xs, "v", &[("a", a * f)]);
        if let Some(vx) = c::at(s, &[("a", a * f), ("x", x)], "v") {
            marks.push(format!("a = {}: {}", qv(a * f, "m/s^2"), qv(vx, "m/s")));
        }
        series.push((format!("a = {}", qv(a * f, "m/s^2")), p));
    }
    chart(s, "graph", "Speed against distance under constant acceleration",
          &format!("Speed reached after a displacement x from v0 = {}, for three constant accelerations (24 \
                    displacements each, a solve of the model): the speed grows as the square root of the distance, \
                    not linearly. Marked at x = {}: {}.", qv(v0, "m/s"), qv(x, "m"), marks.join("; ")),
          series, "x, displacement (m)", "v, speed (m/s)", &format!("v = {}", qv(v, "m/s")), Some((x, v)),
          &qv(v, "m/s"), false)
}

fn trapezoid(s: &Solved) -> Option<Figure> {
    let (v0, v, t, x) = (s.inp("v0")?, s.inp("v")?, s.inp("t")?, s.get("x")?);
    if !(t > 0.0) {
        return None;
    }
    let vavg = x / t;
    let vel = vec![(0.0, v0), (t, v)];
    let avg = vec![(0.0, vavg), (t, vavg)];
    let mut body = k::lines(&[ser("velocity", vel), ser("average velocity", avg)], "t, time (s)",
                            "v, velocity (m/s)", &format!("Δx = {}", qv(x, "m")), Some((t, v)), &qv(v, "m/s"), false);
    body = c::fit_right(&body);
    Some(s.chart("graph", "Displacement as the area under a velocity line", &format!(
        "Velocity changing uniformly from v0 = {} to v = {} in t = {}: the trapezoid under the line is the \
         displacement the model returns, Δx = {}; the second line is the average velocity Δx / t = {}, which encloses \
         the same area.", qv(v0, "m/s"), qv(v, "m/s"), qv(t, "s"), qv(x, "m"), qv(vavg, "m/s")), &body))
}

fn impulse_area(s: &Solved) -> Option<Figure> {
    let (f, t, j) = (s.inp("F")?, s.inp("t")?, s.get("J")?);
    if !(t > 0.0) {
        return None;
    }
    let force = vec![(0.0, 0.0), (0.0, f), (t, f), (t, 0.0), (t * 1.3, 0.0)];
    chart(s, "graph", "Impulse as the area under force against time",
          &format!("A constant force F = {} acting for t = {}: the impulse is the area of the rectangle under the \
                    force against time. Marked: the impulse the model returns, J = {} (the change of momentum it \
                    causes).", qv(f, "N"), qv(t, "s"), qv(j, "N*s")),
          vec![ser("", force)], "t, time (s)", "F, force (N)", &format!("J = F t = {}", qv(j, "N*s")),
          Some((t / 2.0, f)), &format!("area {}", qv(j, "N*s")), false)
}

fn work_angle(s: &Solved) -> Option<Figure> {
    let (f, d, w) = (s.inp("F")?, s.inp("d")?, s.get("W")?);
    let th = s.inp("theta").unwrap_or(0.0);
    let xs = k::span(0.0, PI, 37, false);
    let p: Vec<(f64, f64)> = s.curve("theta", &xs, "W", &[]).into_iter().map(|(x, y)| (deg(x), k::snap(y, f * d))).collect();
    chart(s, "graph", "Work against the angle of the force",
          &format!("Work of a force F = {} over d = {} against the angle between them (37 angles, each a solve of the \
                    model): positive below 90°, zero for a perpendicular force, negative when the force opposes the \
                    motion. Marked: {} at {}°.", qv(f, "N"), qv(d, "m"), qv(w, "J"), k::num(deg(th))),
          vec![ser("", p)], "θ, angle between force and displacement (°)", "W, work (J)",
          &format!("W = {}", qv(w, "J")), Some((deg(th), w)), &qv(w, "J"), false)
}

fn spring_triangle(s: &Solved) -> Option<Figure> {
    let (kk, x, u) = (s.inp("k")?, s.inp("x")?, s.get("U")?);
    if x == 0.0 {
        return None;
    }
    let xs = k::span(0.0, x * 1.4, 15, false);
    let p = pts(&xs, |xx| o(s, "spring_force", &[("k", kk), ("x", xx)], "F"));
    let fx = o(s, "spring_force", &[("k", kk), ("x", x)], "F")?;
    let tri = vec![(0.0, 0.0), (x, fx), (x, 0.0)];
    chart(s, "graph", "Elastic energy as the area under force against extension",
          &format!("Force of a spring with k = {} against its extension (from the model's Hooke's-law formula): the \
                    energy stored when it is stretched x = {} is the triangle under the line up to x. Marked: stored \
                    energy U = {} (returned) and F = {} at x.", qv(kk, "N/m"), qv(x, "m"), qv(u, "J"), qv(fx, "N")),
          vec![ser("spring force F = k x", p), ser("area up to x", tri)], "x, extension (m)", "F, force (N)",
          &format!("U = {}", qv(u, "J")), Some((x, fx)), &qv(fx, "N"), false)
}

fn period_wave(s: &Solved) -> Option<Figure> {
    let (f, t) = (s.inp("f")?, s.get("T")?);
    if !(f > 0.0) || !(t > 0.0) {
        return None;
    }
    let w = o(s, "angular_frequency", &[("f", f)], "omega")?;
    let ts = k::span(0.0, 3.0 * t, 181, false);
    let wave: Vec<(f64, f64)> = ts.iter().map(|x| (*x, (w * x).sin())).collect();
    let crest = t / 4.0;
    let mark = vec![(crest, 1.0), (crest + t, 1.0)];
    chart(s, "graph", "Period of an oscillation",
          &format!("An oscillation of frequency f = {} drawn as sin(ω t) over three cycles, with ω = {} from the \
                    model's angular-frequency formula; the time between two equal crests is the period the model \
                    returns. Marked: T = 1/f = {}.", qv(f, "Hz"), qv(w, "rad/s"), qv(t, "s")),
          vec![ser("sin(ω t)", wave), ser("one period", mark)], "t, time (s)", "displacement (relative)",
          &format!("T = {}", qv(t, "s")), Some((crest + t, 1.0)), &format!("T = {}", qv(t, "s")), false)
}

fn throws(s: &Solved) -> Option<Figure> {
    let (v0, h) = (s.inp("v0")?, s.get("h")?);
    if !(v0 > 0.0) {
        return None;
    }
    let g = s.inp("g").unwrap_or(9.80665);
    let mut series = vec![];
    let mut marks = vec![];
    let mut tops = vec![];
    for f in [0.5, 1.0, 1.5] {
        let vv = v0 * f;
        let top_t = o(s, "time_to_top_vertical_throw", &with_g(s, vec![("v0", vv)]), "t")?;
        let top_h = o(s, "max_height_vertical_throw", &with_g(s, vec![("v0", vv)]), "h")?;
        let ts = k::span(0.0, 2.0 * top_t, 24, false);
        let p = pts(&ts, |tt| o(s, "displacement_time", &[("v0", vv), ("a", -g), ("t", tt)], "x"));
        marks.push(format!("v0 = {}: top {} at {}", qv(vv, "m/s"), qv(top_h, "m"), qv(top_t, "s")));
        tops.push((top_t, top_h));
        series.push((format!("v0 = {}", qv(vv, "m/s")), p));
    }
    series.push(("highest points".to_string(), tops));
    chart(s, "graph", "Vertical throws and their highest points",
          &format!("Height against time of a body thrown straight up (g = {}, no air resistance) at three launch \
                    speeds: each point from the model's constant-acceleration displacement, the tops from its \
                    maximum-height and time-to-top formulas; doubling v0 quadruples the height. Marked: {}.",
                   qv(g, "m/s^2"), marks.join("; ")),
          series, "t, time (s)", "height (m)", &format!("h = {}", qv(h, "m")),
          Some((o(s, "time_to_top_vertical_throw", &with_g(s, vec![("v0", v0)]), "t")?, h)), &qv(h, "m"), false)
}

fn throw_velocity(s: &Solved) -> Option<Figure> {
    let (v0, t) = (s.inp("v0")?, s.get("t")?);
    if !(t > 0.0) {
        return None;
    }
    let g = s.inp("g").unwrap_or(9.80665);
    let ts = k::span(0.0, 2.0 * t, 25, false);
    let p: Vec<(f64, f64)> = pts(&ts, |tt| o(s, "velocity_time", &[("v0", v0), ("a", -g), ("t", tt)], "v"))
        .into_iter().map(|(x, y)| (x, k::snap(y, v0))).collect();
    chart(s, "graph", "Velocity of a vertical throw crosses zero at the top",
          &format!("Upward velocity of a body thrown up at v0 = {} (g = {}), 25 times, each a solve of the model: it \
                    falls by g every second, is zero at the top and returns with the launch speed downwards. Marked: \
                    time to the top {}.", qv(v0, "m/s"), qv(g, "m/s^2"), qv(t, "s")),
          vec![ser("", p)], "t, time (s)", "v, upward velocity (m/s)", &format!("t = {}", qv(t, "s")),
          Some((t, 0.0)), &format!("top at {}", qv(t, "s")), false)
}

fn momentum_slope(s: &Solved) -> Option<Figure> {
    let (p2, p1, t, f) = (s.inp("p2")?, s.inp("p1")?, s.inp("t")?, s.get("F")?);
    if !(t > 0.0) {
        return None;
    }
    chart(s, "graph", "Average force as the rate of change of momentum",
          &format!("Momentum changing from p1 = {} to p2 = {} in Δt = {} (the inputs): the average net force is the \
                    slope of momentum against time. Marked: the average force the model returns, {}.",
                   qv(p1, "kg*m/s"), qv(p2, "kg*m/s"), qv(t, "s"), qv(f, "N")),
          vec![ser("", vec![(0.0, p1), (t, p2)])], "t, time (s)", "p, momentum (kg·m/s)",
          &format!("F = {}", qv(f, "N")), Some((t, p2)), &format!("slope {}", qv(f, "N")), false)
}

fn charge_lines(s: &Solved) -> Option<Figure> {
    let (cc, v, q) = (s.inp("C")?, s.inp("V")?, s.get("Q")?);
    if v == 0.0 {
        return None;
    }
    let vs = k::span(0.0, v * 1.5, 10, false);
    let mut series = vec![];
    let mut marks = vec![];
    for f in [0.5, 1.0, 2.0] {
        series.push((format!("C = {}", qv(cc * f, "F")), s.curve("V", &vs, "Q", &[("C", cc * f)])));
        if let Some(qq) = c::at(s, &[("C", cc * f)], "Q") {
            marks.push(format!("C = {}: Q = {}", qv(cc * f, "F"), qv(qq, "C")));
        }
    }
    chart(s, "graph", "Charge against voltage for three capacitors",
          &format!("Charge stored on three capacitors against the voltage across them (10 voltages each, a solve of \
                    the model): Q = C V, so the capacitance is the slope. Marked at V = {}: {}.", qv(v, "V"),
                   marks.join("; ")),
          series, "V, voltage (V)", "Q, charge (C)", &format!("Q = {}", qv(q, "C")), Some((v, q)), &qv(q, "C"), false)
}

fn capacitor_triangle(s: &Solved) -> Option<Figure> {
    let (cc, v, e) = (s.inp("C")?, s.inp("V")?, s.get("Ecap")?);
    if v == 0.0 {
        return None;
    }
    let vs = k::span(0.0, v * 1.4, 15, false);
    let p = pts(&vs, |vv| o(s, "capacitance_definition", &[("C", cc), ("V", vv)], "Q"));
    let q = o(s, "capacitance_definition", &[("C", cc), ("V", v)], "Q")?;
    chart(s, "graph", "Energy stored in a capacitor as an area",
          &format!("Charge on a C = {} capacitor against its voltage (from the model's Q = C V): the energy stored when \
                    it is charged to {} is the triangle under the line, half of Q V. Marked: the energy the model \
                    returns, U = {}, and Q = {}.", qv(cc, "F"), qv(v, "V"), qv(e, "J"), qv(q, "C")),
          vec![ser("Q = C V", p), ser("area up to V", vec![(0.0, 0.0), (v, q), (v, 0.0)])], "V, voltage (V)",
          "Q, charge (C)", &format!("U = {}", qv(e, "J")), Some((v, q)), &qv(q, "C"), false)
}

fn plate_gap(s: &Solved) -> Option<Figure> {
    let (a, d, cc) = (s.inp("A")?, s.inp("d")?, s.get("C")?);
    if !(d > 0.0) {
        return None;
    }
    let ds = k::span(d / 4.0, d * 4.0, 25, true);
    let p = s.curve("d", &ds, "C", &[]);
    chart(s, "graph", "Capacitance against plate separation",
          &format!("A parallel-plate capacitor of plate area A = {} with the gap from a quarter to four times d \
                    ({} to {}), 25 gaps each a solve of the model: the capacitance is inversely proportional to the \
                    gap. Marked: C = {} at d = {}.", qv(a, "m^2"), qv(d / 4.0, "m"), qv(d * 4.0, "m"), qv(cc, "F"),
                   qv(d, "m")),
          vec![ser("", p)], "d, plate separation (m)", "C, capacitance (F)", &format!("C = {}", qv(cc, "F")),
          Some((d, cc)), &qv(cc, "F"), true)
}

fn period_radius(s: &Solved) -> Option<Figure> {
    let (r, t) = (s.inp("r")?, s.get("T")?);
    if !(r > 0.0) {
        return None;
    }
    let rs = k::span(r * 0.156, r * 1.5, 40, false);
    let p = s.curve("r", &rs, "T", &[]);
    chart(s, "graph", "Orbital period against radius",
          &format!("Period of a circular orbit round the given central mass against its radius, 40 radii from {} to \
                    {}, each a solve of the model (T grows as r to the power 3/2). Marked: the given orbit, T = {} at \
                    r = {}.", qv(rs[0], "m"), qv(rs[39], "m"), qv(t, "s"), qv(r, "m")),
          vec![ser("", p)], "r, orbital radius (m)", "T, period (s)", &format!("T = {}", qv(t, "s")), Some((r, t)),
          &qv(t, "s"), false)
}

fn heating_curve(s: &Solved) -> Option<Figure> {
    // water from ice at -20 °C to steam at 120 °C (stated specific heats), each stage's heat from the model
    let (m, lf, q) = (s.inp("m")?, s.inp("Lf")?, s.get("Q")?);
    if !(m > 0.0) {
        return None;
    }
    let (c_ice, c_w, c_st, lv) = (2100.0, 4186.0, 2010.0, 2.26e6);
    let q1 = o(s, "heat", &[("m", m), ("c", c_ice), ("dT", 20.0)], "Q")?;
    let q2 = q;
    let q3 = o(s, "heat", &[("m", m), ("c", c_w), ("dT", 100.0)], "Q")?;
    let q4 = o(s, "latent_heat_vaporization", &[("m", m), ("Lv", lv)], "Q")?;
    let q5 = o(s, "heat", &[("m", m), ("c", c_st), ("dT", 20.0)], "Q")?;
    let mut acc = 0.0;
    let mut p = vec![(0.0, -20.0)];
    for (dq, temp) in [(q1, 0.0), (q2, 0.0), (q3, 100.0), (q4, 100.0), (q5, 120.0)] {
        acc += dq;
        p.push((acc, temp));
    }
    chart(s, "graph", "Heating curve of water",
          &format!("Heating curve of {} of water from ice at -20 °C to steam at 120 °C at atmospheric pressure (stated: \
                    c of ice 2100, of water 4186, of steam 2010 J/(kg·K), latent heat of vaporization 2.26e6 J/kg, \
                    latent heat of fusion Lf = {} as given): each stage's heat from the model, accumulated; the \
                    temperature stays put while the phase changes. Marked: melting needs {}, boiling {}.",
                   qv(m, "kg"), qv(lf, "J/kg"), qv(q, "J"), qv(q4, "J")),
          vec![ser("", p)], "heat added (J)", "temperature (°C)", &format!("Q = {} to melt", qv(q, "J")),
          Some((q1 + q2, 0.0)), &format!("melted, {}", qv(q, "J")), false)
}

fn photon_spectrum(s: &Solved) -> Option<Figure> {
    let (lam, e) = (s.inp("lam")?, s.get("E")?);
    if !(lam > 0.0) {
        return None;
    }
    let ls = k::span(lam * 0.4, lam * 2.0, 41, false);
    let p = s.curve("lam", &ls, "E", &[]);
    chart(s, "graph", "Photon energy across the spectrum",
          &format!("Energy of one photon against its wavelength from {} to {}, 41 wavelengths, each a solve of the \
                    model: the shorter the wavelength, the more energy per photon. Marked: E = {} at {}.",
                   qv(ls[0], "m"), qv(ls[40], "m"), qv(e, "J"), qv(lam, "m")),
          vec![ser("", p)], "λ, wavelength (m)", "E, photon energy (J)", &format!("E = {}", qv(e, "J")),
          Some((lam, e)), &qv(e, "J"), false)
}

fn pressure_depth(s: &Solved) -> Option<Figure> {
    let (p0, h, p) = (s.inp("P0")?, s.inp("h")?, s.get("P")?);
    if !(h > 0.0) {
        return None;
    }
    let hs = k::span(0.0, h * 1.5, 31, false);
    let tot = s.curve("h", &hs, "P", &[]);
    let surf = vec![(0.0, p0), (h * 1.5, p0)];
    chart(s, "graph", "Absolute pressure: the surface pressure plus the liquid column",
          &format!("Absolute pressure in a liquid with P0 = {} at the surface, 31 depths each a solve of the model: \
                    the surface pressure's share is constant, the liquid column adds ρ g h. Marked: {} at {}.",
                   qv(p0, "Pa"), qv(p, "Pa"), qv(h, "m")),
          vec![ser("absolute pressure", tot), ser("surface pressure P0", surf)], "h, depth (m)", "P, pressure (Pa)",
          &format!("P = {}", qv(p, "Pa")), Some((h, p)), &qv(p, "Pa"), false)
}

fn rms_wave(s: &Solved, peak: &str, rms: &str, u: &str, what: &str) -> Option<Figure> {
    let (p0, r) = (s.inp(peak)?, s.get(rms)?);
    if !(p0 > 0.0) {
        return None;
    }
    let f = 50.0;
    let ts = k::span(0.0, 2.0 / f, 161, false);
    let wave: Vec<(f64, f64)> = ts.iter().map(|t| (*t, p0 * (2.0 * PI * f * t).sin())).collect();
    let line = vec![(0.0, r), (2.0 / f, r)];
    let unit_word = if u == "V" { "voltage" } else { "current" };
    chart(s, "graph", &format!("Peak and rms {what} of a sine wave"),
          &format!("An alternating {what} of peak {} (drawn at 50 Hz for two cycles, a stated frequency) with the \
                    root-mean-square value the model returns: the steady value that delivers the same average power. \
                    Marked: rms = {}, the peak divided by the square root of 2.", qv(p0, u), qv(r, u)),
          vec![ser(&format!("{unit_word}, peak {}", qv(p0, u)), wave), ser(&format!("rms {}", qv(r, u)), line)],
          "t, time (s)", &format!("{unit_word} ({u})"), &format!("{rms} = {}", qv(r, u)), Some((2.0 / f, r)),
          &qv(r, u), false)
}

fn reactances(s: &Solved) -> Option<Figure> {
    let (l, cc, f0) = (s.inp("L")?, s.inp("C")?, s.get("f")?);
    if !(f0 > 0.0) {
        return None;
    }
    let fs = k::span(f0 / 10.0, f0 * 10.0, 21, true);
    let xl = pts(&fs, |f| o(s, "inductive_reactance", &[("f", f), ("L", l)], "X"));
    let xc = pts(&fs, |f| o(s, "capacitive_reactance", &[("f", f), ("C", cc)], "X"));
    let x0 = o(s, "inductive_reactance", &[("f", f0), ("L", l)], "X")?;
    chart(s, "graph", "Resonance where the two reactances meet",
          &format!("Reactance of an inductor L = {} and a capacitor C = {} against frequency over two decades (21 \
                    frequencies each from the model's reactance formulas): they are equal where the LC circuit \
                    resonates. Marked: the resonant frequency the model returns, f0 = {}, where both reactances are {}.",
                   qv(l, "H"), qv(cc, "F"), qv(f0, "Hz"), qv(x0, "ohm")),
          vec![ser("inductor XL", xl), ser("capacitor XC", xc)], "f, frequency (Hz)", "reactance (Ω)",
          &format!("f0 = {}", qv(f0, "Hz")), Some((f0, x0)), &qv(f0, "Hz"), true)
}

fn decay(s: &Solved) -> Option<Figure> {
    let (i0, tau, t, i) = (s.inp("I0")?, s.inp("tau")?, s.inp("t")?, s.get("I")?);
    if !(tau > 0.0) {
        return None;
    }
    let ts = k::span(0.0, (5.0 * tau).max(t * 1.2), 40, false);
    let p = s.curve("t", &ts, "I", &[]);
    let mut marks = vec![];
    for n in 1..=4 {
        if let Some(v) = c::at(s, &[("t", n as f64 * tau)], "I") {
            marks.push(format!("{} at {n}τ", qv(v, "A")));
        }
    }
    let taus: Vec<(f64, f64)> = (1..=4).filter_map(|n| c::at(s, &[("t", n as f64 * tau)], "I").map(|v| (n as f64 * tau, v))).collect();
    chart(s, "graph", "Current decay in an RL circuit",
          &format!("Current dying away in an inductive circuit from I0 = {} with time constant τ = {}, 40 times, each \
                    a solve of the model: each τ leaves about 37 % of the current before it. Marked: {}.",
                   qv(i0, "A"), qv(tau, "s"), marks.join(", ")),
          vec![ser("current", p), ser("after 1, 2, 3, 4 τ", taus)], "t, time (s)", "I, current (A)",
          &format!("I = {} at t = {}", qv(i, "A"), qv(t, "s")), Some((t, i)), &qv(i, "A"), false)
}

fn damped(s: &Solved) -> Option<Figure> {
    let (kk, m, b, wd) = (s.inp("k")?, s.inp("m")?, s.inp("b")?, s.get("wd")?);
    if !(m > 0.0) || !(wd > 0.0) {
        return None;
    }
    let t_end = 3.0 * 2.0 * PI / wd;
    let ts = k::span(0.0, t_end, 161, false);
    let mut series = vec![];
    let mut marks = vec![];
    for bb in [b, b / 4.0, (b * 2.5)] {
        let Some(w) = c::at(s, &[("b", bb)], "wd") else { continue };
        let z = o(s, "damping_ratio", &[("b", bb), ("k", kk), ("m", m)], "zeta");
        let p: Vec<(f64, f64)> = ts.iter().map(|t| (*t, (-bb * t / (2.0 * m)).exp() * (w * t).cos())).collect();
        marks.push(format!("b = {}: ωd = {}{}", qv(bb, "kg/s"), qv(w, "rad/s"),
                           z.map(|z| format!(", ζ = {}", k::num(z))).unwrap_or_default()));
        series.push((format!("b = {}", qv(bb, "kg/s")), p));
    }
    if series.is_empty() {
        return None;
    }
    let env: Vec<(f64, f64)> = ts.iter().map(|t| (*t, (-b * t / (2.0 * m)).exp())).collect();
    series.push(("envelope".to_string(), env));
    chart(s, "graph", "Damped oscillations",
          &format!("Free motion of a damped oscillator (k = {}, m = {}) starting at x = 1 for three damping constants, \
                    drawn as exp(-b t / 2m) cos(ωd t), with the damped angular frequency ωd and the damping ratio ζ \
                    returned by the model; the last curve is the decay envelope of the given case. Marked: {}.",
                   qv(kk, "N/m"), qv(m, "kg"), marks.join("; ")),
          series, "t, time (s)", "x, displacement (relative)", &format!("ωd = {}", qv(wd, "rad/s")), None, "", false)
}

fn regimes(s: &Solved) -> Option<Figure> {
    let (b, z) = (s.inp("b")?, s.get("zeta")?);
    if !(b > 0.0) || !(z > 0.0) {
        return None;
    }
    let bcrit = b / z;
    let bs = k::span(0.0, bcrit * 2.0, 40, false);
    let p = s.curve("b", &bs, "zeta", &[]);
    let one = vec![(0.0, 1.0), (bcrit * 2.0, 1.0)];
    chart(s, "graph", "Damping ratio and the three regimes",
          &format!("Damping ratio of the oscillator against its damping constant, 40 values each a solve of the model: \
                    below ζ = 1 the motion oscillates, above it it does not; critical damping is at b = {} (derived \
                    as b / ζ). Marked: ζ = {} at b = {}.", qv(bcrit, "kg/s"), k::num(z), qv(b, "kg/s")),
          vec![ser("damping ratio ζ", p), ser("critical damping ζ = 1", one)], "b, damping constant (kg/s)",
          "ζ, damping ratio", &format!("ζ = {}", k::num(z)), Some((b, z)), &format!("ζ = {}", k::num(z)), false)
}

fn kepler(s: &Solved) -> Option<Figure> {
    let (a, t) = (s.inp("a")?, s.get("T")?);
    if !(a > 0.0) {
        return None;
    }
    let planets = [("Mercury", 5.79e10), ("Venus", 1.082e11), ("Earth", 1.496e11), ("Mars", 2.279e11),
                   ("Jupiter", 7.786e11), ("Saturn", 1.4335e12), ("Uranus", 2.8725e12), ("Neptune", 4.495e12)];
    let p: Vec<(f64, f64)> = planets.iter().filter_map(|(_, x)| c::at(s, &[("a", *x)], "T").map(|y| (*x, y))).collect();
    if p.len() < 2 {
        return None;
    }
    let names: Vec<String> = planets.iter().zip(&p).map(|((n, _), (_, y))| format!("{n} {}", qv(*y, "s"))).collect();
    let mut body = k::lines(&[ser("the eight planets' distances", p)], "a, semi-major axis (m)", "T, period (s)",
                            &format!("T = {}", qv(t, "s")), Some((a, t)), &qv(t, "s"), true);
    body = c::fit_right(&body);
    Some(s.chart("graph", "Kepler's third law for the planets' distances", &format!(
        "Orbital period against semi-major axis for the eight planets' distances (stated, from 5.79e10 m to 4.495e12 m) \
         round the given central mass, each period a solve of the model; on logarithmic axes the points lie on one \
         straight line of slope 3/2. Periods: {}. Marked: the given orbit, T = {} at a = {}.", names.join(", "),
        qv(t, "s"), qv(a, "m")), &body))
}

fn bead_graph(s: &Solved) -> Option<Figure> {
    let (w, th) = (s.inp("w")?, s.get("theta")?);
    if !(w > 0.0) {
        return None;
    }
    let ws = k::span(w * 0.3, w * 2.0, 40, false);
    let p: Vec<(f64, f64)> = s.curve("w", &ws, "theta", &[]).into_iter().map(|(x, y)| (x, deg(y))).collect();
    chart(s, "graph", "Equilibrium angle against spin rate",
          &format!("Equilibrium angle of a bead on the hoop against the spin rate, 40 rates each a solve of the model \
                    (angles converted to degrees): too slow a spin has no off-bottom equilibrium, and no point is \
                    drawn there. Marked: {}° at ω = {}.", k::num(deg(th)), qv(w, "rad/s")),
          vec![ser("", p)], "ω, spin rate of the hoop (rad/s)", "θ, angle from the bottom (°)",
          &format!("θ = {}°", k::num(deg(th))), Some((w, deg(th))), &format!("{}°", k::num(deg(th))), false)
}

// ------------------------------------------------------------------------------------------------ bars
fn fall_energy(s: &Solved) -> Option<Figure> {
    // a body of the given mass falling from rest from a height giving its speed v at the bottom
    let (m, v) = (s.inp("m")?, s.inp("v")?);
    if !(m > 0.0) || v == 0.0 {
        return None;
    }
    let g = 9.81;
    let h0 = v * v / (2.0 * g);
    let mut labels = vec![];
    let mut us = vec![];
    let mut ks = vec![];
    let mut marks = vec![];
    for f in [1.0, 0.75, 0.5, 0.25, 0.0] {
        let h = h0 * f;
        let u = o(s, "gravitational_pe", &[("m", m), ("g", g), ("h", h)], "U")?;
        let vv = if f < 1.0 { o(s, "speed_after_fall_from_rest", &[("g", g), ("h", h0 - h)], "v")? } else { 0.0 };
        let kk = c::at(s, &[("v", vv)], "K")?;
        labels.push(qv(h, "m"));
        marks.push(format!("h = {}: U = {}, K = {}", qv(h, "m"), qv(u, "J"), qv(kk, "J")));
        us.push(Some(u));
        ks.push(Some(kk));
    }
    let body = c::fit_right(&k::bar_groups(&labels, &[("potential energy U".to_string(), us),
                                                      ("kinetic energy K".to_string(), ks)],
                                           "height above the ground (m)", "energy (J)",
                                           &format!("K = {} at v = {}", qv(s.get("K")?, "J"), qv(v, "m/s"))));
    Some(s.chart("bars", "Energy conservation in a free fall", &format!(
        "Energy of a {} body falling from rest from {} (g = 9.81 m/s², no air resistance), the height from which it \
         lands at the given speed: potential energy and kinetic energy (from the speed after the fall), each a solve \
         of the model; their sum stays the same. {}.", qv(m, "kg"), qv(h0, "m"), marks.join("; ")), &body))
}

fn force_bars(s: &Solved) -> Option<Figure> {
    let (m, a, f) = (s.inp("m")?, s.inp("a")?, s.get("F")?);
    let ms = [m / 4.0, m / 2.0, m, 2.0 * m];
    let mut labels = vec![];
    let mut vals = vec![];
    for mm in ms {
        vals.push(c::at(s, &[("m", mm)], "F")?);
        labels.push(qv(mm, "kg"));
    }
    let marks: Vec<String> = labels.iter().zip(&vals).map(|(l, v)| format!("{l}: {}", qv(*v, "N"))).collect();
    bars_fig(s, "bars", "Force needed for the same acceleration", &format!(
        "Net force needed to give bodies of four masses the same acceleration a = {}, each a solve of the model: twice \
         the mass needs twice the force. Marked: {}.", qv(a, "m/s^2"), marks.join(", ")),
             labels, vals, vec![2], "F, net force (N)", &format!("F = {}", qv(f, "N")))
}

fn weight_bars(s: &Solved) -> Option<Figure> {
    let (m, w) = (s.inp("m")?, s.get("W")?);
    let places = [("Moon", 1.62), ("Mars", 3.71), ("Earth", 9.8), ("Jupiter", 24.79)];
    let mut labels = vec![];
    let mut vals = vec![];
    for (n, g) in places {
        vals.push(c::at(s, &[("g", g)], "W")?);
        labels.push(n.to_string());
    }
    labels.push("given".into());
    vals.push(w);
    let marks: Vec<String> = labels.iter().zip(&vals).map(|(l, v)| format!("{l} {}", qv(*v, "N"))).collect();
    bars_fig(s, "bars", "One mass, four places", &format!(
        "Weight of the same {} mass where the gravitational acceleration is 1.62 m/s² (Moon), 3.71 m/s² (Mars), 9.8 m/s² \
         (Earth) and 24.79 m/s² (Jupiter), each a solve of the model, and with the given g: the mass is the same, the \
         weight scales with g. Marked: {}.", qv(m, "kg"), marks.join(", ")),
             labels, vals, vec![4], "W, weight (N)", &format!("W = {}", qv(w, "N")))
}

fn gravity_bars(s: &Solved) -> Option<Figure> {
    let g = s.get("g")?;
    let bodies = [("Moon", 7.35e22, 1.737e6), ("Mars", 6.42e23, 3.39e6), ("Earth", 5.97e24, 6.37e6),
                  ("Jupiter", 1.898e27, 6.99e7)];
    let mut labels = vec![];
    let mut vals = vec![];
    for (n, m, r) in bodies {
        vals.push(c::at(s, &[("M", m), ("r", r)], "g")?);
        labels.push(n.to_string());
    }
    labels.push("given".into());
    vals.push(g);
    let marks: Vec<String> = labels.iter().zip(&vals).map(|(l, v)| format!("{l} {}", qv(*v, "m/s^2"))).collect();
    bars_fig(s, "bars", "Surface gravity of the Moon, Mars, Earth and Jupiter", &format!(
        "Surface gravity of four bodies from their mass and radius (stated: Moon 7.35e22 kg, 1.737e6 m; Mars 6.42e23 kg, \
         3.39e6 m; Earth 5.97e24 kg, 6.37e6 m; Jupiter 1.898e27 kg, 6.99e7 m) and of the given mass and distance, each a \
         solve of the model. Marked: {}.", marks.join(", ")),
             labels, vals, vec![4], "g (m/s²)", &format!("g = {}", qv(g, "m/s^2")))
}

fn water_heats(s: &Solved) -> Option<Figure> {
    let (m, q) = (s.inp("m")?, s.get("Q")?);
    if !(m > 0.0) {
        return None;
    }
    let melt = o(s, "latent_heat_fusion", &[("m", m), ("Lf", 334000.0)], "Q")?;
    let warm = o(s, "heat", &[("m", m), ("c", 4186.0), ("dT", 100.0)], "Q")?;
    let labels = vec!["melt".to_string(), "warm".to_string(), "boil".to_string()];
    let vals = vec![melt, warm, q];
    bars_fig(s, "bars", "Boiling takes the most heat", &format!(
        "Heat needed by {} of water to melt (latent heat of fusion 334000 J/kg), to warm from 0 to 100 °C (c = 4186 \
         J/(kg·K)) and to boil away (the given latent heat), each a solve of the model. Marked: melting {}, warming {}, \
         vaporisation {}.", qv(m, "kg"), qv(melt, "J"), qv(warm, "J"), qv(q, "J")),
             labels, vals, vec![2], "Q, heat (J)", &format!("Q = {}", qv(q, "J")))
}

fn index_bars(s: &Solved) -> Option<Figure> {
    let n = s.get("n")?;
    let media = [("water", 2.25e8), ("glass", 2.0e8), ("diamond", 1.24e8)];
    let mut labels = vec![];
    let mut vals = vec![];
    for (name, v) in media {
        vals.push(c::at(s, &[("v", v)], "n")?);
        labels.push(name.to_string());
    }
    labels.push("given".into());
    vals.push(n);
    let marks: Vec<String> = labels.iter().zip(&vals).map(|(l, v)| format!("{l} {}", k::num(*v))).collect();
    bars_fig(s, "bars", "Refractive index from the speed of light", &format!(
        "Refractive index n = c/v of transparent media from the speed of light in each (stated: water 2.25e8 m/s, \
         glass 2.0e8 m/s, diamond 1.24e8 m/s) and in the given medium, each a solve of the model. Marked: {}.",
        marks.join(", ")), labels, vals, vec![3], "n, index of refraction", &format!("n = {}", k::num(n)))
}

// ------------------------------------------------------------------------------------------------ comparisons
fn incline_parts(s: &Solved) -> Option<Figure> {
    let (m, th, f) = (s.inp("m")?, s.inp("theta")?, s.get("F")?);
    let ths = k::span(0.0, PI / 2.0, 19, false);
    let along: Vec<(f64, f64)> = s.curve("theta", &ths, "F", &[]).into_iter().map(|(x, y)| (deg(x), y)).collect();
    let normal: Vec<(f64, f64)> = pts(&ths, |t| o(s, "normal_force_incline", &with_g(s, vec![("m", m), ("theta", t)]), "N"))
        .into_iter().map(|(x, y)| (deg(x), k::snap(y, m))).collect();
    let w = o(s, "weight", &with_g(s, vec![("m", m)]), "W")?;
    chart(s, "comparison", "Weight split into components on an incline",
          &format!("The two components of the weight of a {} body on an incline, against the incline's angle (19 \
                    angles each a solve of the model): the part pulling it down the slope grows from zero, the part \
                    pressing it into the surface (the normal force) falls to zero at 90°. Marked: {} along the slope \
                    at θ = {}°; weight {}.", qv(m, "kg"), qv(f, "N"), k::num(deg(th)), qv(w, "N")),
          vec![ser("along the slope", along), ser("into the slope", normal),
               ser(&format!("weight {}", qv(w, "N")), vec![(0.0, w), (90.0, w)])],
          "θ, angle of the incline (°)", "force (N)", &format!("F = {}", qv(f, "N")), Some((deg(th), f)), &qv(f, "N"),
          false)
}

fn pendulum_modes(s: &Solved) -> Option<Figure> {
    let (l, w) = (s.inp("L")?, s.get("w")?);
    if !(l > 0.0) {
        return None;
    }
    let ls = k::span(l * 0.2, l * 3.0, 29, false);
    let slow = s.curve("L", &ls, "w", &[]);
    let fast = pts(&ls, |x| o(s, "double_pendulum_fast_mode", &with_g(s, vec![("L", x)]), "w"));
    let wf = o(s, "double_pendulum_fast_mode", &with_g(s, vec![("L", l)]), "w")?;
    chart(s, "comparison", "The two normal modes of a double pendulum",
          &format!("Small-oscillation normal-mode angular frequencies of a double pendulum of two equal masses on \
                    equal lengths L, 29 lengths from {} to {}, each a solve of the model for each mode. Marked at L = \
                    {}: slow mode {}, fast mode {}.", qv(ls[0], "m"), qv(ls[28], "m"), qv(l, "m"), qv(w, "rad/s"),
                   qv(wf, "rad/s")),
          vec![ser("slow mode", slow), ser("fast mode", fast)], "L, length of each arm (m)",
          "ω, angular frequency (rad/s)", &format!("ω = {}", qv(w, "rad/s")), Some((l, w)), &qv(w, "rad/s"), false)
}

fn escape_orbit(s: &Solved) -> Option<Figure> {
    let (m, r, v) = (s.inp("M")?, s.inp("r")?, s.get("v")?);
    if !(r > 0.0) {
        return None;
    }
    let rs = k::span(r, r * 10.0, 30, false);
    let esc = s.curve("r", &rs, "v", &[]);
    let mut set_g = vec![];
    if let Some(g) = s.inp("G") {
        set_g.push(("G", g));
    }
    let orb = pts(&rs, |x| {
        let mut st = set_g.clone();
        st.push(("M", m));
        st.push(("r", x));
        o(s, "orbital_speed", &st, "v")
    });
    chart(s, "comparison", "Escape speed and orbital speed with distance",
          &format!("Escape speed and circular orbital speed against distance from the centre of a body of mass M = {}, \
                    from r = {} to ten times that, 30 distances each a solve of the model: the escape speed is always \
                    the square root of 2 times the circular speed. Marked: escape speed {} at r = {}.", qv(m, "kg"),
                   qv(r, "m"), qv(v, "m/s"), qv(r, "m")),
          vec![ser("escape speed", esc), ser("circular orbital speed", orb)], "r, distance from the centre (m)",
          "speed (m/s)", &format!("v = {}", qv(v, "m/s")), Some((r, v)), &qv(v, "m/s"), false)
}

fn inertia_shapes(s: &Solved) -> Option<Figure> {
    let m = s.inp("m")?;
    let ir = s.get("Ir")?;
    // spheres and a disc of radius R, rods of length 2R
    let r = match s.inp("R") {
        Some(r) => r,
        None => s.inp("Lr")? / 2.0,
    };
    let shapes = [("sphere", "inertia_solid_sphere", "R", r), ("shell", "inertia_hollow_sphere", "R", r),
                  ("disc", "inertia_solid_disk", "R", r), ("rod, mid", "inertia_rod_center", "Lr", 2.0 * r),
                  ("rod, end", "inertia_rod_end", "Lr", 2.0 * r)];
    let mut labels = vec![];
    let mut vals = vec![];
    let mut hl = vec![];
    for (i, (n, key, sym, x)) in shapes.iter().enumerate() {
        vals.push(o(s, key, &[("m", m), (sym, *x)], "Ir")?);
        labels.push(n.to_string());
        if *key == s.key {
            hl.push(i);
        }
    }
    let marks: Vec<String> = labels.iter().zip(&vals).map(|(l, v)| format!("{l} {}", qv(*v, "kg*m^2"))).collect();
    bars_fig(s, "comparison", "Moment of inertia compared with other shapes", &format!(
        "Moments of inertia of bodies of the same mass m = {}: spheres and a disc of radius R = {}, rods of length {} \
         (2R) about their centre and about one end, each a solve of the model; the highlighted bar is this formula's: \
         the farther the mass from the axis, the larger I. Marked: {}.", qv(m, "kg"), qv(r, "m"), qv(2.0 * r, "m"),
        marks.join(", ")), labels, vals, hl, "I, moment of inertia (kg·m²)", &format!("I = {}", qv(ir, "kg*m^2")))
}

fn rolling_race(s: &Solved) -> Option<Figure> {
    let (th, ir, m, r, a) = (s.inp("theta")?, s.inp("Ir")?, s.inp("m")?, s.inp("R")?, s.get("a")?);
    let mut labels = vec!["as given".to_string()];
    let mut vals = vec![a];
    let mut marks = vec![format!("as given (I = {}) {}", qv(ir, "kg*m^2"), qv(a, "m/s^2"))];
    for (n, key) in [("sphere", "inertia_solid_sphere"), ("disc", "inertia_solid_disk"),
                     ("shell", "inertia_hollow_sphere")] {
        let i2 = o(s, key, &[("m", m), ("R", r)], "Ir")?;
        let a2 = c::at(s, &[("Ir", i2)], "a")?;
        labels.push(n.to_string());
        vals.push(a2);
        marks.push(format!("{n} (I = {}) {}", qv(i2, "kg*m^2"), qv(a2, "m/s^2")));
    }
    bars_fig(s, "comparison", "Which body rolls down fastest", &format!(
        "Acceleration down a {}° incline (m = {}, R = {}) of the body as given (highlighted) and of a solid sphere, a \
         solid disc and a hollow sphere of the same mass and radius rolling without slipping, each moment of inertia \
         from the model's inertia formulas and each acceleration a solve of the model. Marked: {}.", k::num(deg(th)),
        qv(m, "kg"), qv(r, "m"), marks.join("; ")), labels, vals, vec![0], "a, acceleration (m/s²)",
             &format!("a = {}", qv(a, "m/s^2")))
}

fn collision_ratio(s: &Solved) -> Option<Figure> {
    let (m1, m2, v1, v2, v2f) = (s.inp("m1")?, s.inp("m2")?, s.inp("v1")?, s.inp("v2")?, s.get("v2f")?);
    if !(m2 > 0.0) || !(m1 > 0.0) {
        return None;
    }
    let ratios = k::span(0.1, 10.0, 31, true);
    let second: Vec<(f64, f64)> = pts(&ratios, |q| c::at(s, &[("m1", q * m2)], "v2f"));
    let first: Vec<(f64, f64)> = pts(&ratios, |q| o(s, "elastic_collision_first_final",
                                                    &[("m1", q * m2), ("m2", m2), ("v1", v1), ("v2", v2)], "v1f"))
        .into_iter().map(|(x, y)| (x, k::snap(y, v1.abs() + v2.abs()))).collect();
    chart(s, "comparison", "Elastic collision outcomes against the mass ratio",
          &format!("Final velocities after a head-on elastic collision of a body of mass m1 moving at v1 = {} with a {} \
                    body moving at v2 = {}, against the mass ratio m1/m2 from 0.1 to 10 (31 ratios, both velocities \
                    a solve of the model). Marked: v2' = {} for the given ratio {}.", qv(v1, "m/s"), qv(m2, "kg"),
                   qv(v2, "m/s"), qv(v2f, "m/s"), k::num(m1 / m2)),
          vec![ser("second body v2'", second), ser("first body v1'", first)], "m1 / m2, mass ratio",
          "final velocity (m/s)", &format!("v2' = {}", qv(v2f, "m/s")), Some((m1 / m2, v2f)), &qv(v2f, "m/s"), true)
}
