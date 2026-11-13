//! Classical Physics' drawings for the visitor's inputs: free-body diagrams, springs, pendulums, orbits, circuits,
//! ray diagrams and energy flows, every label from the model's own answer or further solves of it.

use crate::figs_common::{self as c, qv};
use ax_engine::figkit::{self as k, Canvas, Solved};
use ax_engine::figures::Figure;
use ax_engine::Input;
use std::f64::consts::PI;

pub fn live(s: &Solved) -> Vec<Figure> {
    let v: Vec<Option<Figure>> = match s.key {
        "two_resistors_series_parallel" => vec![resistor_pair(s)],
        "uniform_motion" => vec![uniform_steps(s)],
        "displacement_time" => vec![projectiles(s)],
        "newton_second" => vec![net_force_block(s)],
        "net_force_friction" => vec![friction_push(s)],
        "kinetic_friction" => vec![sliding_block(s)],
        "normal_on_level" => vec![normal_level(s)],
        "work" => vec![work_pull(s)],
        "spring_force" => vec![spring_stretch(s)],
        "centripetal_acceleration" => vec![circle_motion(s)],
        "centripetal_force" => vec![inward_forces(s)],
        "tangential_speed" => vec![disc_speeds(s)],
        "angular_frequency" => vec![swept_angle(s)],
        "hydrostatic_pressure" => vec![tank_wall(s)],
        "ohm" => vec![ohm_circuit(s)],
        "wave_speed" => vec![wave_snapshot(s)],
        "normal_force_incline" => vec![incline_forces(s)],
        "spring_launch_speed" => vec![spring_launch(s)],
        "tension_hanging_at_rest" => vec![hanging_mass(s)],
        "speed_after_fall_from_rest" => vec![falling_body(s)],
        "electric_force_in_field" => vec![charge_between_plates(s)],
        "point_charge_field" => vec![radial_field(s)],
        "point_charge_potential" => vec![equipotentials(s)],
        "uniform_field_potential" => vec![plate_potentials(s)],
        "magnetic_force_charge" => vec![charge_in_b(s)],
        "magnetic_force_wire" => vec![wire_in_b(s)],
        "solenoid_field" => vec![solenoid(s)],
        "orbital_speed" => vec![circular_orbits(s)],
        "thin_lens" => vec![lens(s, "ray-diagram"), lens(s, "diverging"), lens(s, "magnifier")],
        "pendulum_period" => vec![pendulums(s)],
        "spring_oscillation_period" => vec![spring_masses(s)],
        "torque_lever" => vec![lever(s)],
        "angular_momentum_point_mass" => vec![point_mass_l(s)],
        "wavelength_in_medium" => vec![medium_wave(s)],
        "mirror_focal_length" => vec![mirror(s)],
        "loop_center_field" => vec![loops(s)],
        "engine_energy_balance" => vec![flow(s, "engine")],
        "cop_refrigerator" => vec![flow(s, "refrigerator")],
        "cop_heat_pump" => vec![flow(s, "heat pump")],
        "cyclotron_radius" => vec![cyclotron(s)],
        "rc_discharging" => vec![rc_discharge(s)],
        "motional_emf" => vec![rails(s)],
        "vis_viva" => vec![ellipses(s)],
        "atwood_massive_pulley" => vec![atwood(s)],
        "bead_hoop_equilibrium" => vec![bead_hoop(s)],
        "elastic_collision_first_final" => vec![collision(s)],
        _ => vec![],
    };
    v.into_iter().flatten().collect()
}

// ------------------------------------------------------------------------------------------------ helpers
fn o(s: &Solved, key: &str, set: &[(&str, f64)], y: &str) -> Option<f64> {
    if set.iter().any(|x| !x.1.is_finite()) {
        return None;
    }
    let v: Vec<(&str, Input)> = set.iter().map(|(k_, x)| (*k_, Input::Float(*x))).collect();
    let a = s.solve_other(key, &v)?;
    k::num_of(&a, y)
}

fn with_g<'a>(s: &Solved, mut set: Vec<(&'a str, f64)>) -> Vec<(&'a str, f64)> {
    if let Some(g) = s.inp("g") {
        set.push(("g", g));
    }
    set
}

fn deg(r: f64) -> f64 {
    r * 180.0 / PI
}

/// An arrow of a force or velocity with a value label beside its tip.
fn arrow_lab(cv: &mut Canvas, a: (f64, f64), b: (f64, f64), class: &str, label: &str) {
    c::varrow(cv, a, b, class);
    let (dx, dy) = (b.0 - a.0, b.1 - a.1);
    let (x, y, anchor) = if dx.abs() >= dy.abs() {
        (b.0 + if dx >= 0.0 { 8.0 } else { -8.0 }, b.1 - 8.0, if dx >= 0.0 { "start" } else { "end" })
    } else {
        (b.0 + 10.0, b.1 + if dy >= 0.0 { 14.0 } else { -4.0 }, "start")
    };
    cv.value(x, y, anchor, label);
}

/// Pixels per unit so that the largest of `vals` is `px` long.
fn scale(vals: &[f64], px: f64) -> Option<f64> {
    let m = vals.iter().fold(0.0f64, |a, b| a.max(b.abs()));
    (m > 0.0 && m.is_finite()).then(|| px / m)
}

fn fig(s: &Solved, kind: &str, title: &str, cap: &str, cv: &Canvas) -> Option<Figure> {
    Some(s.figure(kind, title, cap, cv.body(), cv.h))
}

/// A cross (into the page) or a dot (out of it) field mark grid over a rectangle.
fn field_marks(cv: &mut Canvas, x0: f64, y0: f64, x1: f64, y1: f64, into: bool) {
    let mut y = y0;
    while y <= y1 {
        let mut x = x0;
        while x <= x1 {
            if into {
                cv.raw(&format!("<path class=\"axp-c4\" d=\"M{:.1},{:.1} L{:.1},{:.1} M{:.1},{:.1} L{:.1},{:.1}\"/>",
                                x - 4.0, y - 4.0, x + 4.0, y + 4.0, x - 4.0, y + 4.0, x + 4.0, y - 4.0));
            } else {
                cv.raw(&format!("<circle class=\"axp-d4\" cx=\"{x:.1}\" cy=\"{y:.1}\" r=\"2.2\"/>"));
            }
            x += 40.0;
        }
        y += 40.0;
    }
}

// ------------------------------------------------------------------------------------------------ circuits
fn resistor_pair(s: &Solved) -> Option<Figure> {
    let (r1, r2, rs, rp) = (s.inp("r1")?, s.inp("r2")?, s.out("rs")?, s.out("rp")?);
    let mut cv = Canvas::new(260.0);
    // series: terminal - R1 - R2 - terminal, horizontal on the left
    cv.bold(130.0, 30.0, "middle", "series");
    cv.terminal((20.0, 110.0), "");
    cv.resistor((24.0, 110.0), (130.0, 110.0));
    cv.resistor((130.0, 110.0), (236.0, 110.0));
    cv.terminal((240.0, 110.0), "");
    cv.dot((130.0, 110.0));
    cv.tag(77.0, 84.0, "middle", "R1", &q_ohm(r1));
    cv.tag(183.0, 84.0, "middle", "R2", &q_ohm(r2));
    cv.value(130.0, 170.0, "middle", &format!("Rs = {}", q_ohm(rs)));
    // parallel on the right
    cv.bold(390.0, 22.0, "middle", "parallel");
    cv.terminal((290.0, 110.0), "");
    cv.wire(&[(294.0, 110.0), (320.0, 110.0)]);
    cv.wire(&[(320.0, 80.0), (320.0, 140.0)]);
    cv.resistor((320.0, 80.0), (460.0, 80.0));
    cv.resistor((320.0, 140.0), (460.0, 140.0));
    cv.wire(&[(460.0, 80.0), (460.0, 140.0)]);
    cv.wire(&[(460.0, 110.0), (486.0, 110.0)]);
    cv.terminal((490.0, 110.0), "");
    cv.dot((320.0, 110.0));
    cv.dot((460.0, 110.0));
    cv.tag(390.0, 55.0, "middle", "R1", &q_ohm(r1));
    cv.tag(390.0, 172.0, "middle", "R2", &q_ohm(r2));
    cv.value(390.0, 228.0, "middle", &format!("Rp = {}", q_ohm(rp)));
    fig(s, "schematic", "Two resistors in series and in parallel", &format!(
        "Two resistors R1 = {} and R2 = {} connected in series (left) and in parallel (right), measured between the two \
         terminals. Marked: the equivalent resistances the model returns, Rs = {} (larger than either) and Rp = {} \
         (smaller than either).", q_ohm(r1), q_ohm(r2), q_ohm(rs), q_ohm(rp)), &cv)
}

fn q_ohm(x: f64) -> String {
    k::q(x, "ohm")
}

fn ohm_circuit(s: &Solved) -> Option<Figure> {
    let (i, r, v) = (s.get("I")?, s.get("R")?, s.get("V")?);
    let mut cv = Canvas::new(300.0);
    cv.vsource((130.0, 70.0), (130.0, 230.0), false);
    cv.wire(&[(130.0, 70.0), (130.0, 50.0), (390.0, 50.0), (390.0, 90.0)]);
    cv.resistor((390.0, 90.0), (390.0, 210.0));
    cv.wire(&[(390.0, 210.0), (390.0, 250.0), (130.0, 250.0), (130.0, 230.0)]);
    cv.arrow((230.0, 50.0), (290.0, 50.0));
    cv.value(260.0, 38.0, "middle", &format!("I = {}", k::q(i, "A")));
    cv.tag(410.0, 150.0, "start", "R", &q_ohm(r));
    cv.tag(150.0, 150.0, "start", "V", &k::q(v, "V"));
    cv.bold(260.0, 285.0, "middle", &format!("V = I R = {}", k::q(v, "V")));
    fig(s, "schematic", "Ohm's law: a resistor across a source", &format!(
        "A resistor R = {} across a source, carrying a current I = {}: Ohm's law gives the voltage across it. Marked: \
         the voltage the model returns, V = I R = {}.", q_ohm(r), k::q(i, "A"), k::q(v, "V")), &cv)
}

fn rc_discharge(s: &Solved) -> Option<Figure> {
    let (v0, t, r, cc, v) = (s.inp("V0")?, s.inp("t")?, s.inp("R")?, s.inp("C")?, s.get("V")?);
    let tau = o(s, "rc_time_constant", &[("R", r), ("C", cc)], "tau")?;
    if !(tau > 0.0) {
        return None;
    }
    let ts = k::span(0.0, (5.0 * tau).max(t * 1.2), 40, false);
    let p = s.curve("t", &ts, "V", &[]);
    if p.len() < 2 {
        return None;
    }
    let marks: Vec<(f64, f64)> = (1..=3).filter_map(|n| c::at(s, &[("t", n as f64 * tau)], "V").map(|y| (n as f64 * tau, y))).collect();
    let chart = c::fit_right(&k::lines(&[("V across the capacitor".to_string(), p), ("after 1, 2, 3 τ".to_string(), marks.clone())],
                                       "t, time (s)", "V, voltage (V)", &format!("V = {} at t = {}", k::q(v, "V"), k::q(t, "s")),
                                       Some((t, v)), &k::q(v, "V"), false));
    // sketch above: a charged capacitor across a resistor
    let mut cv = Canvas::new(434.0);
    cv.capacitor((150.0, 30.0), (150.0, 80.0));
    cv.wire(&[(150.0, 30.0), (150.0, 18.0), (370.0, 18.0), (370.0, 25.0)]);
    cv.resistor((370.0, 25.0), (370.0, 85.0));
    cv.wire(&[(370.0, 85.0), (370.0, 92.0), (150.0, 92.0), (150.0, 80.0)]);
    cv.tag(172.0, 52.0, "start", "C", &k::q(cc, "F"));
    cv.tag(390.0, 55.0, "start", "R", &q_ohm(r));
    cv.value(260.0, 60.0, "middle", &format!("τ = R C = {}", k::q(tau, "s")));
    cv.raw(&format!("<g transform=\"translate(0,94)\">{chart}</g>"));
    let ms: Vec<String> = marks.iter().enumerate().map(|(n, m)| format!("{}τ: {}", n + 1, k::q(m.1, "V"))).collect();
    fig(s, "schematic", "A capacitor discharging through a resistor", &format!(
        "A capacitor C = {} charged to V0 = {} discharging through R = {} (sketch above), and its voltage against time, 40 \
         times each a solve of the model; time constant τ = R C = {} from its formula. Marked: {}; V = {} at t = {}.",
        k::q(cc, "F"), k::q(v0, "V"), q_ohm(r), k::q(tau, "s"), ms.join(", "), k::q(v, "V"), k::q(t, "s")), &cv)
}

// ------------------------------------------------------------------------------------------------ motion
fn uniform_steps(s: &Solved) -> Option<Figure> {
    let (v, t, d) = (s.inp("v")?, s.inp("t")?, s.get("d")?);
    if !(t > 0.0) || d == 0.0 {
        return None;
    }
    let n = 6;
    let ts: Vec<f64> = (0..=n).map(|i| t * i as f64 / n as f64).collect();
    let ds: Vec<f64> = ts.iter().map(|tt| if *tt == 0.0 { Some(0.0) } else { c::at(s, &[("t", *tt)], "d") }).collect::<Option<Vec<f64>>>()?;
    let mut cv = Canvas::new(440.0);
    let px = c::lin(0.0, d, 40.0, 470.0);
    cv.line((30.0, 70.0), (490.0, 70.0));
    for (tt, dd) in ts.iter().zip(&ds) {
        c::disc(&mut cv, (px(*dd), 60.0), 8.0, true);
        cv.name(px(*dd), 90.0, "middle", &k::q(*tt, "s"));
    }
    arrow_lab(&mut cv, (px(d) + 10.0, 60.0), (px(d) + 40.0, 60.0), "axp-c2", "");
    cv.value(490.0, 40.0, "end", &format!("v = {}", qv(v, "m/s")));
    let chart = c::fit_right(&k::lines(&[(String::new(), ts.iter().cloned().zip(ds.iter().cloned()).collect())], "t, time (s)",
                                       "d, distance (m)", "", Some((t, d)), &k::q(d, "m"), false));
    cv.raw(&format!("<g transform=\"translate(0,100)\">{chart}</g>"));
    fig(s, "drawing", "Uniform motion: equal distances in equal times", &format!(
        "A body moving at a constant {}, shown at seven equal time steps along its path (equal steps: equal distances) \
         and as a distance against time line; each position a solve of the model. Marked: {} after {}.",
        qv(v, "m/s"), k::q(d, "m"), k::q(t, "s")), &cv)
}

fn projectiles(s: &Solved) -> Option<Figure> {
    // the visitor's start speed thrown at three angles: horizontal by uniform motion, vertical by this formula
    let (v0, a) = (s.inp("v0")?, s.inp("a")?);
    let x = s.get("x")?;
    if !(v0 > 0.0) {
        return None;
    }
    let g = if a < 0.0 { -a } else { 9.81 };
    let mut paths = vec![];
    let mut notes = vec![];
    for th in [30.0f64, 45.0, 60.0] {
        let (vx, vy) = (v0 * th.to_radians().cos(), v0 * th.to_radians().sin());
        let t_top = o(s, "time_to_top_vertical_throw", &[("v0", vy), ("g", g)], "t")?;
        let ts = k::span(0.0, 2.0 * t_top, 30, false);
        let mut p = vec![];
        for tt in &ts {
            let xx = if *tt == 0.0 { 0.0 } else { o(s, "uniform_motion", &[("v", vx), ("t", *tt)], "d")? };
            let yy = if *tt == 0.0 { 0.0 } else { c::at(s, &[("v0", vy), ("a", -g), ("t", *tt)], "x")? };
            p.push((xx, yy.max(0.0)));
        }
        let top = o(s, "max_height_vertical_throw", &[("v0", vy), ("g", g)], "h")?;
        notes.push(format!("θ = {th}°: range {}, top {}", k::q(p.last()?.0, "m"), k::q(top, "m")));
        paths.push((th, p, top));
    }
    let xmax = paths.iter().map(|p| p.1.last().map(|q| q.0).unwrap_or(0.0)).fold(0.0, f64::max);
    let ymax = paths.iter().map(|p| p.2).fold(0.0, f64::max);
    if !(xmax > 0.0) || !(ymax > 0.0) {
        return None;
    }
    let sc = (420.0 / xmax).min(230.0 / ymax);
    let (ox, oy) = (50.0, 300.0);
    let mut cv = Canvas::new(340.0);
    c::ground_line(&mut cv, 30.0, 500.0, oy);
    for (i, (th, p, top)) in paths.iter().enumerate() {
        let class = ["axp-c", "axp-c2", "axp-c3"][i];
        let pp: Vec<(f64, f64)> = p.iter().map(|(x_, y_)| (ox + x_ * sc, oy - y_ * sc)).collect();
        c::poly(&mut cv, &pp, class, false);
        let mid = pp[pp.len() / 2];
        cv.value(mid.0, mid.1 - 8.0, "middle", &format!("{th}°: top {}", k::q(*top, "m")));
    }
    cv.bold(16.0, 24.0, "start", &format!("launch speed {}, g = {}", qv(v0, "m/s"), qv(g, "m/s^2")));
    cv.value(16.0, 44.0, "start", &format!("given: x = {} after {}", k::q(x, "m"), k::q(s.inp("t")?, "s")));
    fig(s, "drawing", "Projectile paths at three launch angles", &format!(
        "Paths of a projectile launched at the given speed {} (g = {}, no air resistance) at three angles: each point's \
         horizontal distance from the model's uniform-motion formula and its height from this displacement formula, \
         the top from the maximum-height formula. {}.", qv(v0, "m/s"), qv(g, "m/s^2"), notes.join("; ")), &cv)
}

fn net_force_block(s: &Solved) -> Option<Figure> {
    let (m, a, f) = (s.inp("m")?, s.inp("a")?, s.get("F")?);
    let mut cv = Canvas::new(300.0);
    c::ground_line(&mut cv, 60.0, 460.0, 210.0);
    c::rect(&mut cv, 200.0, 130.0, 120.0, 80.0);
    cv.name(260.0, 175.0, "middle", &format!("m = {}", k::q(m, "kg")));
    let dir = if f >= 0.0 { 1.0 } else { -1.0 };
    let (x0, x1) = if dir > 0.0 { (320.0, 470.0) } else { (200.0, 50.0) };
    arrow_lab(&mut cv, (x0, 170.0), (x1, 170.0), "axp-c", &format!("F = {}", k::q(f, "N")));
    arrow_lab(&mut cv, (260.0 - 60.0 * dir, 90.0), (260.0 + 60.0 * dir, 90.0), "axp-c2", &format!("a = {}", qv(a, "m/s^2")));
    fig(s, "diagram", "Net force and acceleration of a body", &format!(
        "Newton's second law for a body of m = {} accelerating at a = {}: it needs the net force the model returns, in \
         the direction of the acceleration. Marked: F = {}.", k::q(m, "kg"), qv(a, "m/s^2"), k::q(f, "N")), &cv)
}

fn friction_push(s: &Solved) -> Option<Figure> {
    let (p, f, net) = (s.inp("P")?, s.inp("f")?, s.get("F")?);
    let sc = scale(&[p, f, net], 170.0)?;
    let mut cv = Canvas::new(300.0);
    c::ground_line(&mut cv, 40.0, 480.0, 170.0);
    c::rect(&mut cv, 210.0, 100.0, 100.0, 70.0);
    arrow_lab(&mut cv, (210.0 - p * sc, 135.0), (210.0, 135.0), "axp-c2", "");
    cv.value(210.0 - p * sc / 2.0, 125.0, "middle", &format!("P = {}", k::q(p, "N")));
    // friction acts at the floor, against the push (it once pointed the same way as P: classical physics review)
    arrow_lab(&mut cv, (260.0, 166.0), (260.0 - f * sc, 166.0), "axp-c3", "");
    cv.value(260.0 - f * sc / 2.0, 188.0, "middle", &format!("f = {}", k::q(f, "N")));
    // the net force below, from the block's centre
    arrow_lab(&mut cv, (260.0, 230.0), (260.0 + net * sc, 230.0), "axp-c", &format!("net {}", k::q(net, "N")));
    cv.name(260.0, 270.0, "middle", "net horizontal force F = P - f");
    fig(s, "diagram", "Applied force, friction and the net force", &format!(
        "A block pushed by an applied force P = {} against friction f = {}, arrows to scale; the model returns the net \
         horizontal force, drawn below. Marked: net force {}.", k::q(p, "N"), k::q(f, "N"), k::q(net, "N")), &cv)
}

fn sliding_block(s: &Solved) -> Option<Figure> {
    let (mu, n, f) = (s.inp("mu")?, s.inp("N")?, s.get("f")?);
    let sc = scale(&[n, f], 100.0)?;
    let mut cv = Canvas::new(300.0);
    c::ground_line(&mut cv, 40.0, 480.0, 190.0);
    c::rect(&mut cv, 210.0, 130.0, 100.0, 60.0);
    arrow_lab(&mut cv, (260.0, 130.0), (260.0, 130.0 - n * sc), "axp-c2", &format!("N = {}", k::q(n, "N")));
    arrow_lab(&mut cv, (260.0, 190.0), (260.0, 190.0 + n * sc).min_y(290.0), "axp-c4", &format!("W = {}", k::q(n, "N")));
    arrow_lab(&mut cv, (210.0, 175.0), (210.0 - f * sc, 175.0), "axp-c", &format!("f = {}", k::q(f, "N")));
    arrow_lab(&mut cv, (330.0, 110.0), (400.0, 110.0), "axp-c3", "motion");
    cv.bold(16.0, 24.0, "start", &format!("μk = {}", k::num(mu)));
    fig(s, "diagram", "Kinetic friction on a sliding block", &format!(
        "A block sliding on a level floor: normal force N = {} (balanced by the weight), coefficient of kinetic friction \
         μk = {}; the friction force the model returns acts against the motion. Arrows to scale. Marked: f = {}.",
        k::q(n, "N"), k::num(mu), k::q(f, "N")), &cv)
}

trait MinY {
    fn min_y(self, y: f64) -> (f64, f64);
}
impl MinY for (f64, f64) {
    fn min_y(self, y: f64) -> (f64, f64) {
        (self.0, self.1.min(y))
    }
}

fn normal_level(s: &Solved) -> Option<Figure> {
    let (w, n) = (s.inp("W")?, s.get("N")?);
    let sc = scale(&[w, n], 100.0)?;
    let mut cv = Canvas::new(300.0);
    c::ground_line(&mut cv, 80.0, 440.0, 160.0);
    c::rect(&mut cv, 210.0, 100.0, 100.0, 60.0);
    arrow_lab(&mut cv, (260.0, 100.0), (260.0, 100.0 - n * sc), "axp-c2", &format!("N = {}", k::q(n, "N")));
    arrow_lab(&mut cv, (260.0, 160.0), (260.0, 160.0 + w * sc), "axp-c", &format!("W = {}", k::q(w, "N")));
    fig(s, "diagram", "Normal force on a level surface", &format!(
        "A body at rest on a level surface with weight W = {}: the surface pushes back with the normal force the model \
         returns, equal and opposite. Arrows to scale. Marked: N = {}.", k::q(w, "N"), k::q(n, "N")), &cv)
}

fn work_pull(s: &Solved) -> Option<Figure> {
    let (f, d, w) = (s.inp("F")?, s.inp("d")?, s.get("W")?);
    let th = s.inp("theta").unwrap_or(0.0);
    let mut cv = Canvas::new(300.0);
    c::ground_line(&mut cv, 30.0, 490.0, 200.0);
    c::rect(&mut cv, 80.0, 150.0, 80.0, 50.0);
    c::rect(&mut cv, 360.0, 150.0, 80.0, 50.0);
    cv.dashed((120.0, 230.0), (400.0, 230.0));
    c::varrow(&mut cv, (120.0, 230.0), (400.0, 230.0), "axp-c4");
    cv.name(260.0, 250.0, "middle", &format!("d = {}", k::q(d, "m")));
    let l = 120.0;
    let tip = (160.0 + l * th.cos(), 175.0 - l * th.sin());
    arrow_lab(&mut cv, (160.0, 175.0), tip, "axp-c", &format!("F = {}", k::q(f, "N")));
    let fx = k::snap(f * th.cos(), f);
    c::varrow(&mut cv, (160.0, 175.0), (160.0 + l * th.cos(), 175.0), "axp-c2");
    cv.dashed(tip, (tip.0, 175.0));
    cv.value(168.0 + (l * th.cos()).max(0.0), 192.0, "start", &format!("F cos θ = {}", k::q(fx, "N")));
    c::arc(&mut cv, (160.0, 175.0), 30.0, 0.0, th, "axp-c4");
    cv.name(196.0, 165.0, "start", &format!("θ = {}°", k::num(deg(th))));
    cv.bold(16.0, 24.0, "start", &format!("W = F d cos θ = {}", k::q(w, "J")));
    fig(s, "diagram", "Work done by a force at an angle", &format!(
        "A force F = {} pulling at θ = {}° to the direction of a displacement d = {}: only its component along the motion, \
         F cos θ, does work. Marked: the work the model returns, W = {}.", k::q(f, "N"), k::num(deg(th)), k::q(d, "m"),
        k::q(w, "J")), &cv)
}

fn spring_stretch(s: &Solved) -> Option<Figure> {
    let (kk, x, f) = (s.inp("k")?, s.inp("x")?, s.get("F")?);
    if !(x > 0.0) {
        return None;
    }
    let mut cv = Canvas::new(520.0);
    // the wall, the relaxed spring (dashed end) and the stretched spring
    cv.raw("<rect class=\"axp-ka\" x=\"20\" y=\"30\" width=\"14\" height=\"110\"/>");
    let l0 = 200.0;
    let lx = l0 * (1.0 + (x / (x + 0.5 * x.max(1e-9))).min(0.6));
    c::spring(&mut cv, (34.0, 60.0), (34.0 + l0, 60.0), 8, 9.0);
    c::rect(&mut cv, 34.0 + l0, 45.0, 40.0, 30.0);
    cv.name(34.0 + l0 + 46.0, 64.0, "start", "natural length");
    c::spring(&mut cv, (34.0, 115.0), (34.0 + lx, 115.0), 8, 9.0);
    c::rect(&mut cv, 34.0 + lx, 100.0, 40.0, 30.0);
    cv.dashed((34.0 + l0, 80.0), (34.0 + l0, 140.0));
    c::varrow(&mut cv, (34.0 + l0, 150.0), (34.0 + lx, 150.0), "axp-c4");
    cv.name(34.0 + (l0 + lx) / 2.0, 168.0, "middle", &format!("x = {}", k::q(x, "m")));
    arrow_lab(&mut cv, (34.0 + lx + 40.0, 92.0), (34.0 + lx - 50.0, 92.0), "axp-c", "");
    cv.value(34.0 + lx + 48.0, 96.0, "start", &format!("F = {}", k::q(f, "N")));
    let xs = k::span(0.0, x * 1.5, 20, false);
    let p = s.curve("x", &xs, "F", &[]);
    let chart = c::fit_right(&k::lines(&[(String::new(), p)], "x, extension (m)", "F, force (N)",
                                       &format!("k = {}", qv(kk, "N/m")), Some((x, f)), &k::q(f, "N"), false));
    cv.raw(&format!("<g transform=\"translate(0,180)\">{chart}</g>"));
    fig(s, "drawing", "A stretched spring and Hooke's law", &format!(
        "A spring of stiffness k = {} stretched x = {} beyond its natural length: it pulls back with the force the model \
         returns (Hooke's law), and the force grows in proportion to the extension (20 extensions below, each a solve \
         of the model). Marked: F = {} at x = {}.", qv(kk, "N/m"), k::q(x, "m"), k::q(f, "N"), k::q(x, "m")), &cv)
}

fn circle_motion(s: &Solved) -> Option<Figure> {
    let (v, r, a) = (s.inp("v")?, s.inp("r")?, s.get("a")?);
    let mut cv = Canvas::new(400.0);
    let ctr = (260.0, 210.0);
    let rp = 130.0;
    c::ellipse(&mut cv, ctr, rp, rp, "axp-c4");
    cv.dot(ctr);
    for i in 0..6 {
        let th = i as f64 * PI / 3.0 + 0.3;
        let p = (ctr.0 + rp * th.cos(), ctr.1 - rp * th.sin());
        c::disc(&mut cv, p, 7.0, true);
        c::varrow(&mut cv, p, (p.0 - 55.0 * th.sin(), p.1 - 55.0 * th.cos()), "axp-c2");
        c::varrow(&mut cv, p, (p.0 - 45.0 * th.cos(), p.1 + 45.0 * th.sin()), "axp-c");
    }
    cv.name(ctr.0 + 8.0, ctr.1 + 18.0, "start", &format!("r = {}", k::q(r, "m")));
    cv.value(16.0, 44.0, "start", &format!("velocity v = {} (along the circle)", qv(v, "m/s")));
    cv.bold(16.0, 24.0, "start", &format!("a = v²/r = {} (to the centre)", qv(a, "m/s^2")));
    fig(s, "diagram", "Velocity and centripetal acceleration on a circle", &format!(
        "A body moving round a circle of radius r = {} at a constant speed v = {}, at six positions: its velocity (green) \
         is always along the circle, its acceleration (pink) always points to the centre. Marked: the centripetal \
         acceleration the model returns, a = {}.", k::q(r, "m"), qv(v, "m/s"), qv(a, "m/s^2")), &cv)
}

fn inward_forces(s: &Solved) -> Option<Figure> {
    let (m, v, r, f) = (s.inp("m")?, s.inp("v")?, s.inp("r")?, s.get("F")?);
    if !(r > 0.0) {
        return None;
    }
    let rs = [r / 2.0, r, 2.0 * r];
    let fs: Vec<f64> = rs.iter().map(|x| c::at(s, &[("r", *x)], "F")).collect::<Option<Vec<f64>>>()?;
    // each arrow inside its own circle: the scale set by the tightest of them
    let sc = rs.iter().zip(&fs).map(|(rr, ff)| 0.8 * rr * (170.0 / (2.0 * r)) / ff.abs().max(1e-300)).fold(f64::INFINITY, f64::min);
    if !sc.is_finite() {
        return None;
    }
    let mut cv = Canvas::new(420.0);
    let ctr = (190.0, 220.0);
    let pr = 170.0 / (2.0 * r);
    cv.dot(ctr);
    for (i, (rr, ff)) in rs.iter().zip(&fs).enumerate() {
        let class = ["axp-c3", "axp-c", "axp-c2"][i];
        c::ellipse(&mut cv, ctr, rr * pr, rr * pr, "axp-c4");
        let th = PI / 2.0 - 0.5 * i as f64;
        let p = (ctr.0 + rr * pr * th.cos(), ctr.1 - rr * pr * th.sin());
        c::disc(&mut cv, p, 6.0, true);
        c::varrow(&mut cv, p, (p.0 - ff * sc * th.cos(), p.1 + ff * sc * th.sin()), class);
        cv.value(400.0, 120.0 + 26.0 * i as f64, "start", &format!("r = {}: {}", k::q(*rr, "m"), k::q(*ff, "N")));
    }
    cv.bold(16.0, 24.0, "start", &format!("m = {}, v = {}", k::q(m, "kg"), qv(v, "m/s")));
    fig(s, "diagram", "Centripetal force on circles of three radii", &format!(
        "Inward (centripetal) force needed to keep a {} body moving at {} on circles of radius {}, {} and {}, each a \
         solve of the model and drawn to scale towards the centre: halving the radius doubles the force. Marked: F = {} \
         at the given r = {}.", k::q(m, "kg"), qv(v, "m/s"), k::q(rs[0], "m"), k::q(rs[1], "m"), k::q(rs[2], "m"),
        k::q(f, "N"), k::q(r, "m")), &cv)
}

fn disc_speeds(s: &Solved) -> Option<Figure> {
    let (w, r, v) = (s.inp("omega")?, s.inp("r")?, s.get("v")?);
    if !(r > 0.0) {
        return None;
    }
    let rs = [0.25 * r, 0.5 * r, 0.75 * r, r];
    let vs: Vec<f64> = rs.iter().map(|x| c::at(s, &[("r", *x)], "v")).collect::<Option<Vec<f64>>>()?;
    let sc = scale(&vs, 150.0)?;
    let mut cv = Canvas::new(420.0);
    let ctr = (150.0, 250.0);
    let pr = 130.0 / r;
    c::disc(&mut cv, ctr, r * pr, true);
    cv.dot(ctr);
    let mut tips = vec![];
    for (rr, vv) in rs.iter().zip(&vs) {
        let p = (ctr.0 + rr * pr, ctr.1);
        cv.dot(p);
        let tip = (p.0, p.1 - vv * sc);
        c::varrow(&mut cv, p, tip, "axp-c2");
        cv.value(330.0, 120.0 + 24.0 * tips.len() as f64, "start", &format!("r = {}: {}", k::q(*rr, "m"), qv(*vv, "m/s")));
        tips.push(tip);
    }
    let mut line = vec![ctr];
    line.extend(tips);
    c::poly(&mut cv, &line, "axp-c4", true);
    c::arc(&mut cv, ctr, r * pr + 14.0, 2.0, 2.8, "axp-c");
    cv.bold(16.0, 24.0, "start", &format!("ω = {}, r = {}", qv(w, "rad/s"), k::q(r, "m")));
    cv.value(16.0, 44.0, "start", &format!("v = ω r = {} at the rim", qv(v, "m/s")));
    let ms: Vec<String> = rs.iter().zip(&vs).map(|(a, b)| format!("{} at r = {}", qv(*b, "m/s"), k::q(*a, "m"))).collect();
    fig(s, "diagram", "Tangential speed across a turning disc", &format!(
        "A disc of radius {} turning at ω = {}: the speed of a point grows in proportion to its distance from the axis \
         (arrows to scale; the dashed line joins their tips). Marked: the tangential speed the model returns, {}.",
        k::q(r, "m"), qv(w, "rad/s"), ms.join(", ")), &cv)
}

fn swept_angle(s: &Solved) -> Option<Figure> {
    let (f, w) = (s.inp("f")?, s.get("omega")?);
    if !(f > 0.0) {
        return None;
    }
    let t = o(s, "period_frequency", &[("f", f)], "T")?;
    let mut cv = Canvas::new(380.0);
    let ctr = (200.0, 200.0);
    let rp = 140.0;
    c::ellipse(&mut cv, ctr, rp, rp, "axp-c4");
    // the angle swept in one second (whole turns beyond the first left out of the shading)
    let sweep = (w * 1.0).rem_euclid(2.0 * PI);
    let mut wedge = vec![ctr];
    for i in 0..=40 {
        let a = sweep * i as f64 / 40.0;
        wedge.push((ctr.0 + rp * a.cos(), ctr.1 - rp * a.sin()));
    }
    c::area(&mut cv, &wedge, "axp-s");
    for i in 0..8 {
        let a = 2.0 * PI * i as f64 / 8.0;
        cv.dot((ctr.0 + rp * a.cos(), ctr.1 - rp * a.sin()));
        cv.name(ctr.0 + (rp + 22.0) * a.cos(), ctr.1 - (rp + 22.0) * a.sin() + 4.0, "middle",
                &k::q(t * i as f64 / 8.0, "s"));
    }
    cv.bold(370.0, 60.0, "start", &format!("f = {}", qv(f, "Hz")));
    cv.value(370.0, 84.0, "start", &format!("ω = {}", qv(w, "rad/s")));
    cv.value(370.0, 106.0, "start", &format!("T = {}", k::q(t, "s")));
    cv.name(370.0, 130.0, "start", "shaded: angle in 1 s");
    fig(s, "diagram", "Angular frequency: the angle swept each second", &format!(
        "A point turning round a circle at f = {}: in one second it sweeps the angle ω times 1 s (shaded, whole turns \
         left out), with ω = 2π f the angular frequency the model returns; dots every eighth of a period T = {} (from \
         its period formula). Marked: ω = {}.", qv(f, "Hz"), k::q(t, "s"), qv(w, "rad/s")), &cv)
}

fn tank_wall(s: &Solved) -> Option<Figure> {
    let (h, p) = (s.inp("h")?, s.get("P")?);
    if !(h > 0.0) {
        return None;
    }
    let hs = [0.25 * h, 0.5 * h, 0.75 * h, h];
    let ps: Vec<f64> = hs.iter().map(|x| c::at(s, &[("h", *x)], "P")).collect::<Option<Vec<f64>>>()?;
    let sc = scale(&ps, 180.0)?;
    let mut cv = Canvas::new(360.0);
    let (top, bot) = (60.0, 320.0);
    let py = c::lin(0.0, h, top, bot);
    cv.raw(&format!("<rect class=\"axp-s3\" x=\"60\" y=\"{top}\" width=\"200\" height=\"{}\"/>", bot - top));
    cv.wire(&[(60.0, 40.0), (60.0, bot), (260.0, bot), (260.0, 40.0)]);
    let mut tips = vec![(260.0, top)];
    for (hh, pp) in hs.iter().zip(&ps) {
        let y = py(*hh);
        c::varrow(&mut cv, (260.0, y), (260.0 + pp * sc, y), "axp-c");
        cv.value(266.0 + pp * sc, y - 6.0, "start", &k::q(*pp, "Pa"));
        cv.name(54.0, y + 4.0, "end", &k::q(*hh, "m"));
        tips.push((260.0 + pp * sc, y));
    }
    c::poly(&mut cv, &tips, "axp-c4", true);
    cv.bold(16.0, 24.0, "start", &format!("gauge pressure P = ρ g h = {} at {}", k::q(p, "Pa"), k::q(h, "m")));
    let ms: Vec<String> = hs.iter().zip(&ps).map(|(a, b)| format!("{} at {}", k::q(*b, "Pa"), k::q(*a, "m"))).collect();
    fig(s, "diagram", "Pressure on a tank wall grows with depth", &format!(
        "Gauge pressure on the wall of a tank of liquid at four depths, arrows to scale, each a solve of the model: the \
         pressure grows linearly with depth (dashed line). Marked: {}.", ms.join(", ")), &cv)
}

fn wave_snapshot(s: &Solved) -> Option<Figure> {
    let (f, lam, v) = (s.inp("f")?, s.inp("lam")?, s.get("v")?);
    if !(lam > 0.0) || !(f > 0.0) {
        return None;
    }
    let t = o(s, "period_frequency", &[("f", f)], "T")?;
    let mut cv = Canvas::new(340.0);
    let px = 400.0 / (2.5 * lam);
    let wave = |shift: f64| -> Vec<(f64, f64)> {
        (0..=200).map(|i| {
            let x = 2.5 * lam * i as f64 / 200.0;
            (60.0 + x * px, 170.0 - 70.0 * (2.0 * PI * (x - shift) / lam).sin())
        }).collect()
    };
    cv.line((60.0, 170.0), (470.0, 170.0));
    c::poly(&mut cv, &wave(0.0), "axp-c", false);
    c::poly(&mut cv, &wave(lam / 4.0), "axp-c4", true);
    let crest = 60.0 + lam / 4.0 * px;
    c::varrow(&mut cv, (crest, 260.0), (crest + lam * px, 260.0), "axp-c2");
    cv.name(crest + lam * px / 2.0, 280.0, "middle", &format!("λ = {}", k::q(lam, "m")));
    cv.name(470.0, 90.0, "end", &format!("dashed: a quarter period ({}) later", k::q(t / 4.0, "s")));
    cv.bold(16.0, 24.0, "start", &format!("v = f λ = {}", qv(v, "m/s")));
    cv.value(16.0, 44.0, "start", &format!("f = {}, T = {}", qv(f, "Hz"), k::q(t, "s")));
    fig(s, "drawing", "A travelling wave: one wavelength each period", &format!(
        "A snapshot of a travelling wave of frequency f = {} and wavelength λ = {} (drawn with an arbitrary amplitude), \
         and dashed the same wave a quarter period later: each crest moves one wavelength forward every period T = {}. \
         Marked: the wave speed the model returns, v = f λ = {}.", qv(f, "Hz"), k::q(lam, "m"), k::q(t, "s"),
        qv(v, "m/s")), &cv)
}

fn incline_forces(s: &Solved) -> Option<Figure> {
    let (m, th, n) = (s.inp("m")?, s.inp("theta")?, s.get("N")?);
    if !(th > 0.0 && th < PI / 2.0) {
        return None;
    }
    let w = o(s, "weight", &with_g(s, vec![("m", m)]), "W")?;
    let fa = o(s, "weight_component_along_incline", &with_g(s, vec![("m", m), ("theta", th)]), "F")?;
    let sc = scale(&[w, n, fa], 120.0)?;
    let mut cv = Canvas::new(360.0);
    let base = (60.0, 320.0);
    let len = 420.0;
    let top = (base.0 + len * th.cos().min(0.95), base.1 - len * th.cos().min(0.95) * th.tan());
    let top = if top.1 < 60.0 { (base.0 + (base.1 - 60.0) / th.tan(), 60.0) } else { top };
    c::area(&mut cv, &[base, top, (top.0, base.1)], "axp-s4");
    cv.wire(&[base, top, (top.0, base.1), base]);
    c::arc(&mut cv, base, 40.0, 0.0, th, "axp-c4");
    let mid = ((base.0 + top.0) / 2.0, (base.1 + top.1) / 2.0);
    let (ux, uy) = (th.cos(), -th.sin());
    let (nx, ny) = (-th.sin(), -th.cos());
    let ctr = (mid.0 + nx * 22.0, mid.1 + ny * 22.0);
    let corners = [(-28.0, -22.0), (28.0, -22.0), (28.0, 22.0), (-28.0, 22.0)];
    let block: Vec<(f64, f64)> = corners.iter().map(|(a, b)| (ctr.0 + ux * a - nx * b, ctr.1 + uy * a - ny * b)).collect();
    c::area(&mut cv, &block, "axp-s4");
    c::poly(&mut cv, &[block[0], block[1], block[2], block[3], block[0]], "axp-k", false);
    arrow_lab(&mut cv, ctr, (ctr.0, ctr.1 + w * sc), "axp-c4", &format!("W = {}", k::q(w, "N")));
    arrow_lab(&mut cv, ctr, (ctr.0 + nx * n * sc, ctr.1 + ny * n * sc), "axp-c2", &format!("N = {}", k::q(n, "N")));
    arrow_lab(&mut cv, ctr, (ctr.0 - ux * fa * sc, ctr.1 - uy * fa * sc), "axp-c", &format!("{} along", k::q(fa, "N")));
    cv.bold(16.0, 24.0, "start", &format!("m = {} on a {}° incline", k::q(m, "kg"), k::num(deg(th))));
    fig(s, "diagram", "Forces on a block on an incline", &format!(
        "Forces on a {} block on a frictionless {}° incline, each a solve of the model and drawn to scale: weight {}, \
         normal force N = m g cos θ = {}, component of the weight along the slope {}.", k::q(m, "kg"), k::num(deg(th)),
        k::q(w, "N"), k::q(n, "N"), k::q(fa, "N")), &cv)
}

fn spring_launch(s: &Solved) -> Option<Figure> {
    let (kk, x, m, v) = (s.inp("k")?, s.inp("x")?, s.inp("m")?, s.get("v")?);
    let u = o(s, "spring_energy", &[("k", kk), ("x", x)], "U")?;
    let ke = o(s, "kinetic_energy", &[("m", m), ("v", v)], "K")?;
    let mut cv = Canvas::new(230.0);
    c::ground_line(&mut cv, 20.0, 500.0, 150.0);
    cv.raw("<rect class=\"axp-ka\" x=\"20\" y=\"70\" width=\"14\" height=\"80\"/>");
    c::spring(&mut cv, (34.0, 125.0), (130.0, 125.0), 7, 12.0);
    c::rect(&mut cv, 130.0, 100.0, 50.0, 50.0);
    cv.name(155.0, 92.0, "middle", "before");
    cv.name(82.0, 170.0, "middle", &format!("compressed {}", k::q(x, "m")));
    c::spring(&mut cv, (34.0, 125.0), (34.0, 125.0), 1, 0.0);
    c::rect(&mut cv, 360.0, 100.0, 50.0, 50.0);
    cv.name(385.0, 92.0, "middle", "after");
    arrow_lab(&mut cv, (410.0, 125.0), (480.0, 125.0), "axp-c2", "");
    cv.value(445.0, 175.0, "middle", &format!("v = {}", qv(v, "m/s")));
    cv.bold(16.0, 24.0, "start", &format!("stored {} becomes kinetic {}", k::q(u, "J"), k::q(ke, "J")));
    cv.value(16.0, 44.0, "start", &format!("k = {}, m = {}", qv(kk, "N/m"), k::q(m, "kg")));
    fig(s, "drawing", "A spring launching a block", &format!(
        "A spring of k = {} compressed x = {} launches a {} block on a frictionless surface: all the stored energy (from \
         the model's elastic-energy formula) becomes kinetic energy (its kinetic-energy formula). Marked: launch speed \
         v = {} (returned), stored energy {}, kinetic energy {}.", qv(kk, "N/m"), k::q(x, "m"), k::q(m, "kg"),
        qv(v, "m/s"), k::q(u, "J"), k::q(ke, "J")), &cv)
}

fn hanging_mass(s: &Solved) -> Option<Figure> {
    let (m, t) = (s.inp("m")?, s.get("T")?);
    let w = o(s, "weight", &with_g(s, vec![("m", m)]), "W")?;
    let sc = scale(&[w, t], 100.0)?;
    let mut cv = Canvas::new(340.0);
    cv.raw("<rect class=\"axp-ka\" x=\"160\" y=\"20\" width=\"200\" height=\"12\"/>");
    cv.line((260.0, 32.0), (260.0, 160.0));
    c::rect(&mut cv, 225.0, 160.0, 70.0, 60.0);
    cv.name(260.0, 195.0, "middle", &k::q(m, "kg"));
    arrow_lab(&mut cv, (300.0, 160.0), (300.0, 160.0 - t * sc), "axp-c2", &format!("T = {}", k::q(t, "N")));
    arrow_lab(&mut cv, (260.0, 220.0), (260.0, 220.0 + w * sc), "axp-c", &format!("W = {}", k::q(w, "N")));
    fig(s, "diagram", "Tension in a rope holding a mass at rest", &format!(
        "A {} mass hanging at rest from a rope: the rope tension the model returns balances the weight (also from the \
         model). Arrows to scale. Marked: T = {}, W = {}.", k::q(m, "kg"), k::q(t, "N"), k::q(w, "N")), &cv)
}

fn falling_body(s: &Solved) -> Option<Figure> {
    let (h, v) = (s.inp("h")?, s.get("v")?);
    if !(h > 0.0) {
        return None;
    }
    let drops = [0.0, 0.125, 0.25, 0.5, 0.75, 1.0];
    let mut cv = Canvas::new(380.0);
    let py = c::lin(0.0, h, 50.0, 330.0);
    c::ground_line(&mut cv, 60.0, 460.0, 340.0);
    let vmax = v.abs().max(1e-12);
    for (i, f) in drops.iter().enumerate() {
        let d = h * f;
        let vv = if d == 0.0 { 0.0 } else { c::at(s, &[("h", d)], "v")? };
        let x = 100.0 + 60.0 * i as f64;
        let y = py(d);
        c::disc(&mut cv, (x, y), 7.0, true);
        if vv > 0.0 {
            c::varrow(&mut cv, (x, y + 8.0), (x, y + 8.0 + 60.0 * vv / vmax), "axp-c2");
        }
        cv.value(x + 10.0, y - 6.0, "start", &qv(vv, "m/s"));
    }
    cv.name(40.0, 54.0, "start", "released");
    cv.bold(16.0, 24.0, "start", &format!("from {}: v = {} at the ground", k::q(h, "m"), qv(v, "m/s")));
    fig(s, "drawing", "A body falling from rest", &format!(
        "A body released from rest {} above the ground (no air resistance), drawn after falling 0, an eighth, a quarter, \
         a half, three quarters and all of the height; each arrow is to scale with its speed, a solve of the model. \
         Marked: {} on reaching the ground.", k::q(h, "m"), qv(v, "m/s")), &cv)
}

// ------------------------------------------------------------------------------------------------ fields
fn charge_between_plates(s: &Solved) -> Option<Figure> {
    let (q, e, f) = (s.inp("q")?, s.inp("Ef")?, s.get("F")?);
    let mut cv = Canvas::new(300.0);
    cv.raw("<rect class=\"axp-b\" x=\"60\" y=\"50\" width=\"10\" height=\"200\"/><rect class=\"axp-b3\" x=\"450\" y=\"50\" width=\"10\" height=\"200\"/>");
    cv.bold(65.0, 40.0, "middle", "+");
    cv.bold(455.0, 40.0, "middle", "-");
    for i in 0..5 {
        let y = 70.0 + 40.0 * i as f64;
        c::varrow(&mut cv, (75.0, y), (445.0, y), "axp-c4");
    }
    let pos = q >= 0.0;
    c::disc(&mut cv, (220.0, 150.0), 12.0, true);
    cv.bold(220.0, 155.0, "middle", if pos { "+" } else { "-" });
    let dir = if (f >= 0.0) == (e >= 0.0) { 1.0 } else { -1.0 };
    arrow_lab(&mut cv, (236.0 * 1.0 + if dir < 0.0 { -32.0 } else { 0.0 }, 150.0),
              (236.0 + 110.0 * dir, 150.0), "axp-c", &format!("F = {}", k::q(f.abs(), "N")));
    cv.name(260.0, 280.0, "middle", &format!("E = {} N/C, q = {}", k::num(e), k::q(q, "C")));
    fig(s, "diagram", "A charge in a uniform electric field", &format!(
        "A charge q = {} in a uniform field E = {} N/C between two plates: a positive charge is pushed along the field \
         lines, a negative one the other way. Marked: the force the model returns, F = q E = {}.", k::q(q, "C"),
        k::num(e), k::q(f, "N")), &cv)
}

fn radial_field(s: &Solved) -> Option<Figure> {
    let (q, r, e) = (s.inp("Q")?, s.inp("r")?, s.get("Ef")?);
    if !(r > 0.0) {
        return None;
    }
    let mut cv = Canvas::new(420.0);
    let ctr = (200.0, 220.0);
    let out = q >= 0.0;
    for i in 0..12 {
        let a = 2.0 * PI * i as f64 / 12.0;
        let (p0, p1) = ((ctr.0 + 18.0 * a.cos(), ctr.1 - 18.0 * a.sin()), (ctr.0 + 180.0 * a.cos(), ctr.1 - 180.0 * a.sin()));
        if out { c::varrow(&mut cv, p0, p1, "axp-c4") } else { c::varrow(&mut cv, p1, p0, "axp-c4") }
    }
    c::disc(&mut cv, ctr, 14.0, true);
    cv.bold(ctr.0, ctr.1 + 5.0, "middle", if out { "+" } else { "-" });
    let pr = 60.0 / r;
    for (i, f) in [0.5, 1.0, 2.0].iter().enumerate() {
        let rr = r * f;
        let ee = c::at(s, &[("r", rr)], "Ef")?;
        let x = ctr.0 + rr * pr;
        cv.dot((x, ctr.1));
        cv.name(x, ctr.1 + if i % 2 == 0 { 18.0 } else { -10.0 }, "middle", &format!("{}", i + 1));
        cv.value(504.0, 24.0 + 20.0 * i as f64, "end", &format!("{}: {} at {}", i + 1, format!("{} N/C", k::num(ee)), k::q(rr, "m")));
    }
    cv.bold(16.0, 24.0, "start", &format!("Q = {}", k::q(q, "C")));
    cv.value(16.0, 44.0, "start", &format!("E = {} N/C at r = {}", k::num(e), k::q(r, "m")));
    fig(s, "diagram", "Electric field of a point charge", &format!(
        "Electric field lines of a point charge Q = {} (pointing away from a positive charge, towards a negative one), \
         with the field strength the model returns at half, once and twice the given distance: doubling the distance \
         quarters the field. Marked: {} N/C at r = {}.", k::q(q, "C"), k::num(e), k::q(r, "m")), &cv)
}

fn equipotentials(s: &Solved) -> Option<Figure> {
    let (q, r, v) = (s.inp("Q")?, s.inp("r")?, s.get("V")?);
    if !(r > 0.0) {
        return None;
    }
    let mut cv = Canvas::new(564.0);
    let ctr = (260.0, 120.0);
    let pr = 50.0 / r;
    let mut ms = vec![];
    for f in [0.5, 1.0, 2.0] {
        let rr = r * f;
        let vv = c::at(s, &[("r", rr)], "V")?;
        c::ellipse(&mut cv, ctr, rr * pr, rr * pr, "axp-c3");
        cv.value(ctr.0 + rr * pr * 0.71, ctr.1 - rr * pr * 0.71 - 4.0, "start", &format!("{}", ms.len() + 1));
        cv.value(350.0, 60.0 + 22.0 * ms.len() as f64, "start", &format!("{}: {} at {}", ms.len() + 1, k::q(vv, "V"), k::q(rr, "m")));
        ms.push(format!("{} at {}", k::q(vv, "V"), k::q(rr, "m")));
    }
    c::disc(&mut cv, ctr, 10.0, true);
    cv.bold(ctr.0, ctr.1 + 5.0, "middle", if q >= 0.0 { "+" } else { "-" });
    let rs = k::span(r * 0.25, r * 4.0, 34, false);
    let p = s.curve("r", &rs, "V", &[]);
    let chart = c::fit_right(&k::lines(&[(String::new(), p)], "r, distance (m)", "V, potential (V)",
                                       &format!("V = {} at r = {}", k::q(v, "V"), k::q(r, "m")), Some((r, v)),
                                       &k::q(v, "V"), false));
    cv.raw(&format!("<g transform=\"translate(0,224)\">{chart}</g>"));
    fig(s, "diagram", "Equipotentials round a point charge", &format!(
        "Equipotential circles round a point charge Q = {} at half, once and twice the given distance, labelled with \
         the potential the model returns, and the potential against distance below (34 distances, each a solve of the \
         model): it falls as 1/r. Marked: {}.", k::q(q, "C"), ms.join(", ")), &cv)
}

fn plate_potentials(s: &Solved) -> Option<Figure> {
    let (e, d, v) = (s.inp("Ef")?, s.inp("d")?, s.get("V")?);
    if !(d > 0.0) {
        return None;
    }
    let mut cv = Canvas::new(320.0);
    let (x0, x1) = (80.0, 440.0);
    cv.raw(&format!("<rect class=\"axp-b\" x=\"{}\" y=\"50\" width=\"10\" height=\"220\"/><rect class=\"axp-b3\" x=\"{x1}\" y=\"50\" width=\"10\" height=\"220\"/>", x0 - 10.0));
    // the field points from the high-potential plate (right, V) to the 0 V plate (left); it once pointed the other way
    for i in 0..5 {
        c::varrow(&mut cv, (x1 - 2.0, 70.0 + 45.0 * i as f64), (x0 + 2.0, 70.0 + 45.0 * i as f64), "axp-c4");
    }
    for f in [0.25, 0.5, 0.75] {
        let vv = c::at(s, &[("d", d * f)], "V")?;
        let x = x0 + (x1 - x0) * f;
        cv.dashed((x, 50.0), (x, 270.0));
        cv.value(x, 290.0, "middle", &k::q(vv, "V"));
    }
    cv.value(x1, 290.0, "middle", &k::q(v, "V"));
    cv.name(x0, 290.0, "middle", "0 V");
    cv.bold(16.0, 24.0, "start", &format!("E = {} V/m, d = {}: V = E d = {}", k::num(e), k::q(d, "m"), k::q(v, "V")));
    fig(s, "diagram", "Potential across a uniform field", &format!(
        "A uniform field E = {} V/m between two plates d = {} apart (field lines grey), with equipotential planes \
         (dashed) a quarter, half and three quarters of the way across, each potential difference from the first plate \
         a solve of the model. The potential difference over the whole gap is V = E d = {}.", k::num(e), k::q(d, "m"),
        k::q(v, "V")), &cv)
}

fn charge_in_b(s: &Solved) -> Option<Figure> {
    let (q, v, b, f) = (s.inp("q")?, s.inp("v")?, s.inp("B")?, s.get("F")?);
    let th = s.inp("theta").unwrap_or(PI / 2.0);
    let mut cv = Canvas::new(340.0);
    field_marks(&mut cv, 60.0, 70.0, 470.0, 310.0, b >= 0.0);
    c::disc(&mut cv, (200.0, 200.0), 12.0, true);
    cv.bold(200.0, 205.0, "middle", if q >= 0.0 { "+" } else { "-" });
    arrow_lab(&mut cv, (214.0, 200.0), (340.0, 200.0), "axp-c2", &format!("v = {}", qv(v, "m/s")));
    // q v x B: v to the right, B into the page -> up for a positive charge
    let up = (q >= 0.0) == (b >= 0.0);
    arrow_lab(&mut cv, (200.0, if up { 186.0 } else { 214.0 }), (200.0, if up { 90.0 } else { 310.0 }), "axp-c",
              &format!("F = {}", k::q(f.abs(), "N")));
    cv.bold(16.0, 24.0, "start", &format!("B = {} {}, θ = {}°", k::q(b.abs(), "T"),
                                         if b >= 0.0 { "into the page" } else { "out of the page" }, k::num(deg(th))));
    cv.value(16.0, 44.0, "start", &format!("q = {}", k::q(q, "C")));
    fig(s, "diagram", "Magnetic force on a moving charge", &format!(
        "A charge q = {} moving at v = {} through a uniform magnetic field B = {} into the page, at {}° to the velocity: \
         the force is perpendicular to both (F = q v × B). Marked: the force the model returns, F = {}.", k::q(q, "C"),
        qv(v, "m/s"), k::q(b, "T"), k::num(deg(th)), k::q(f, "N")), &cv)
}

fn wire_in_b(s: &Solved) -> Option<Figure> {
    let (i, l, b, f) = (s.inp("I")?, s.inp("L")?, s.inp("B")?, s.get("F")?);
    let th = s.inp("theta").unwrap_or(PI / 2.0);
    let mut cv = Canvas::new(330.0);
    field_marks(&mut cv, 140.0, 80.0, 380.0, 300.0, b >= 0.0);
    cv.raw("<rect class=\"axp-s4\" x=\"140\" y=\"60\" width=\"240\" height=\"250\"/>");
    cv.line((30.0, 200.0), (490.0, 200.0));
    arrow_lab(&mut cv, (40.0, 220.0), (120.0, 220.0), "axp-c2", &format!("I = {}", k::q(i, "A")));
    let up = (i >= 0.0) == (b >= 0.0);
    arrow_lab(&mut cv, (260.0, 200.0), (260.0, if up { 90.0 } else { 310.0 }), "axp-c", &format!("F = {}", k::q(f.abs(), "N")));
    cv.name(260.0, 325.0, "middle", &format!("L = {} in the field", k::q(l, "m")));
    cv.bold(16.0, 24.0, "start", &format!("B = {} into the page, θ = {}°", k::q(b, "T"), k::num(deg(th))));
    fig(s, "diagram", "Magnetic force on a current-carrying wire", &format!(
        "A straight wire carrying I = {} to the right, with L = {} of it in a uniform field B = {} into the page, at {}°: \
         the force is perpendicular to wire and field (F = I L × B). Marked: the force the model returns, F = {}.",
        k::q(i, "A"), k::q(l, "m"), k::q(b, "T"), k::num(deg(th)), k::q(f, "N")), &cv)
}

fn solenoid(s: &Solved) -> Option<Figure> {
    let (n, i, b) = (s.inp("nt")?, s.inp("I")?, s.get("B")?);
    let mut cv = Canvas::new(300.0);
    for j in 0..14 {
        let x = 110.0 + 22.0 * j as f64;
        c::ellipse(&mut cv, (x, 150.0), 7.0, 60.0, "axp-k");
    }
    for y in [125.0, 150.0, 175.0] {
        c::varrow(&mut cv, (100.0, y), (420.0, y), "axp-c");
    }
    let ret = |up: bool| -> Vec<(f64, f64)> {
        (0..=30).map(|t| {
            let a = PI * t as f64 / 30.0;
            (260.0 - 190.0 * a.cos(), 150.0 + if up { -1.0 } else { 1.0 } * (75.0 + 50.0 * a.sin()))
        }).collect()
    };
    c::poly(&mut cv, &ret(true), "axp-c4", true);
    c::poly(&mut cv, &ret(false), "axp-c4", true);
    cv.bold(16.0, 24.0, "start", &format!("n = {} turns/m, I = {}", k::num(n), k::q(i, "A")));
    cv.value(260.0, 290.0, "middle", &format!("B inside = {}", k::q(b, "T")));
    fig(s, "diagram", "Field of a long solenoid", &format!(
        "A long solenoid with n = {} turns per metre carrying I = {}: inside, the field is uniform and along the axis \
         (solid arrows); outside it is weak and returns round the ends (dashed, schematic). Marked: the field inside the \
         model returns, B = {}.", k::num(n), k::q(i, "A"), k::q(b, "T")), &cv)
}

fn loops(s: &Solved) -> Option<Figure> {
    let (i, r, b) = (s.inp("I")?, s.inp("r")?, s.get("B")?);
    if !(r > 0.0) {
        return None;
    }
    let mut cv = Canvas::new(420.0);
    let ctr = (200.0, 220.0);
    let pr = 80.0 / r;
    let mut ms = vec![];
    for (j, f) in [0.5, 1.0, 2.0].iter().enumerate() {
        let rr = r * f;
        let bb = c::at(s, &[("r", rr)], "B")?;
        c::ellipse(&mut cv, ctr, (rr * pr).min(190.0), (rr * pr).min(190.0), ["axp-c3", "axp-c", "axp-c2"][j]);
        cv.value(410.0, 120.0 + 26.0 * j as f64, "start", &format!("r = {}: {}", k::q(rr, "m"), k::q(bb, "T")));
        ms.push(format!("{} for r = {}", k::q(bb, "T"), k::q(rr, "m")));
    }
    cv.wire_end(ctr, i >= 0.0);
    cv.bold(16.0, 24.0, "start", &format!("I = {} anticlockwise: B at the centre out of the page", k::q(i.abs(), "A")));
    cv.value(16.0, 44.0, "start", &format!("B = {} at r = {}", k::q(b, "T"), k::q(r, "m")));
    fig(s, "diagram", "Magnetic field at the centre of a current loop", &format!(
        "Circular loops of half, once and twice the given radius, each carrying I = {} anticlockwise: the field at the \
         centre points out of the page. Marked: the field strength at the centre the model returns, {}.",
        k::q(i, "A"), ms.join(", ")), &cv)
}

fn rails(s: &Solved) -> Option<Figure> {
    let (b, l, v, e) = (s.inp("B")?, s.inp("l")?, s.inp("v")?, s.get("emf")?);
    let mut cv = Canvas::new(300.0);
    field_marks(&mut cv, 120.0, 90.0, 480.0, 230.0, b >= 0.0);
    cv.line((60.0, 70.0), (500.0, 70.0));
    cv.line((60.0, 250.0), (500.0, 250.0));
    cv.resistor((60.0, 70.0), (60.0, 250.0));
    cv.raw("<rect class=\"axp-kd\" x=\"296\" y=\"64\" width=\"8\" height=\"192\"/>");
    arrow_lab(&mut cv, (310.0, 160.0), (390.0, 160.0), "axp-c2", &format!("v = {}", qv(v, "m/s")));
    cv.name(290.0, 160.0, "end", &format!("l = {}", k::q(l, "m")));
    cv.bold(16.0, 24.0, "start", &format!("B = {} into the page: ε = B l v = {}", k::q(b, "T"), k::q(e, "V")));
    fig(s, "diagram", "A rod sliding on rails in a magnetic field", &format!(
        "A rod of length l = {} sliding at v = {} along two rails through a uniform field B = {} into the page; the rails \
         are joined by a resistor, closing the circuit. Marked: the induced emf the model returns, ε = B l v = {}.",
        k::q(l, "m"), qv(v, "m/s"), k::q(b, "T"), k::q(e, "V")), &cv)
}

fn cyclotron(s: &Solved) -> Option<Figure> {
    let (v, r) = (s.inp("v")?, s.get("r")?);
    if !(r > 0.0) || !(v > 0.0) {
        return None;
    }
    let vs = [v / 2.0, v, 2.0 * v];
    let rs: Vec<f64> = vs.iter().map(|x| c::at(s, &[("v", *x)], "r")).collect::<Option<Vec<f64>>>()?;
    let pr = 110.0 / (2.0 * r);
    let mut cv = Canvas::new(380.0);
    field_marks(&mut cv, 40.0, 110.0, 480.0, 360.0, false);
    let start = (380.0, 200.0);
    for (j, (vv, rr)) in vs.iter().zip(&rs).enumerate() {
        let class = ["axp-c3", "axp-c", "axp-c2"][j];
        c::ellipse(&mut cv, (start.0 - rr * pr, start.1), rr * pr, rr * pr, class);
        cv.value(16.0, 48.0 + 18.0 * j as f64, "start", &format!("{}: r = {}", qv(*vv, "m/s"), k::q(*rr, "m")));
    }
    cv.dot(start);
    c::varrow(&mut cv, start, (start.0, start.1 - 40.0), "axp-k");
    cv.bold(16.0, 24.0, "start", &format!("B = {} out of the page", k::q(s.inp("B")?, "T")));
    fig(s, "drawing", "Circular paths of a charge in a magnetic field", &format!(
        "Circular paths of a charged particle launched upwards from the same point at half, once and twice the given \
         speed across a uniform magnetic field out of the page, each radius a solve of the model and drawn to scale: the \
         radius grows in proportion to the speed. Marked: r = {} at v = {}.", k::q(r, "m"), qv(v, "m/s")), &cv)
}

// ------------------------------------------------------------------------------------------------ orbits
fn circular_orbits(s: &Solved) -> Option<Figure> {
    let (m, r) = (s.inp("M")?, s.inp("r")?);
    if !(r > 0.0) {
        return None;
    }
    let rs = [r, 2.5 * r, 4.0 * r];
    let vs: Vec<f64> = rs.iter().map(|x| c::at(s, &[("r", *x)], "v")).collect::<Option<Vec<f64>>>()?;
    let mut cv = Canvas::new(440.0);
    let ctr = (220.0, 230.0);
    let pr = 190.0 / rs[2];
    c::disc(&mut cv, ctr, (rs[0] * pr * 0.5).max(6.0), true);
    let vsc = scale(&vs, 70.0)?;
    for (j, (rr, vv)) in rs.iter().zip(&vs).enumerate() {
        c::ellipse(&mut cv, ctr, rr * pr, rr * pr, "axp-c4");
        let a = 0.6 + 0.7 * j as f64;
        let p = (ctr.0 + rr * pr * a.cos(), ctr.1 - rr * pr * a.sin());
        cv.dot(p);
        c::varrow(&mut cv, p, (p.0 - vv * vsc * a.sin(), p.1 - vv * vsc * a.cos()), "axp-c2");
        cv.value(16.0, 48.0 + 20.0 * j as f64, "start", &format!("r = {}: v = {}", k::q(*rr, "m"), qv(*vv, "m/s")));
    }
    cv.bold(16.0, 24.0, "start", &format!("M = {} kg", k::num(m)));
    let ms: Vec<String> = rs.iter().zip(&vs).map(|(a, b)| format!("{} at {}", qv(*b, "m/s"), k::q(*a, "m"))).collect();
    fig(s, "orbit", "Orbital speed falls with distance", &format!(
        "Circular orbits round a body of mass M = {} kg drawn to scale at the given radius and at 2.5 and 4 times it, with \
         velocity arrows to scale: the farther out, the slower. Marked: the orbital speeds the model returns, {}.",
        k::num(m), ms.join(", ")), &cv)
}

fn ellipses(s: &Solved) -> Option<Figure> {
    let (m, r, a, v) = (s.inp("M")?, s.inp("r")?, s.inp("a")?, s.get("v")?);
    if !(r > 0.0 && a > 0.0 && r < 2.0 * a) {
        return None;
    }
    let mut cv = Canvas::new(440.0);
    let focus = (260.0, 230.0);
    let mut drawn = 0;
    let mut notes = vec![];
    // ellipses of semi-major axis a with the focus at the body, each through a point at distance r
    let emin = (1.0 - r / a).abs() + 0.05;
    let es: Vec<f64> = [0.3, 0.5, 0.7].iter().copied().filter(|e| *e >= emin && *e < 0.95).take(2).collect();
    let es = if es.is_empty() { vec![emin.min(0.9)] } else { es };
    let sc = 170.0 / (a * (1.0 + es.iter().cloned().fold(0.0, f64::max)));
    for (j, e) in es.iter().enumerate() {
        let b = a * (1.0 - e * e).sqrt();
        let ctr = (focus.0 - a * e * sc, focus.1);
        c::ellipse(&mut cv, ctr, a * sc, b * sc, ["axp-c", "axp-c3"][j]);
        // the true anomaly where the distance is r: r = a(1 - e²)/(1 + e cos ν)
        let cosn = (a * (1.0 - e * e) / r - 1.0) / e;
        if cosn.abs() <= 1.0 {
            let nu = cosn.acos();
            let p = (focus.0 + r * sc * nu.cos(), focus.1 - r * sc * nu.sin());
            cv.dot(p);
            drawn += 1;
        }
        notes.push(format!("e = {}", k::num(*e)));
    }
    if drawn == 0 {
        return None;
    }
    c::disc(&mut cv, focus, 8.0, true);
    cv.raw(&format!("<circle class=\"axp-c4\" stroke-dasharray=\"5 4\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"{:.1}\"/>", focus.0, focus.1, r * sc));
    cv.bold(16.0, 24.0, "start", &format!("a = {}, r = {}: v = {}", k::q(a, "m"), k::q(r, "m"), qv(v, "m/s")));
    cv.value(16.0, 44.0, "start", &format!("M = {} kg; {}", k::num(m), notes.join(", ")));
    cv.name(504.0, 430.0, "end", "dashed: the circle of radius r");
    fig(s, "orbit", "Same semi-major axis, same speed at the same radius", &format!(
        "Orbits round a body of mass M = {} kg with the same semi-major axis a = {} but eccentricities {}, each crossing \
         the circle of radius r = {} (dashed; the ellipses are drawn from that geometry). The vis-viva equation gives \
         every one the same speed at that radius, whatever its shape. Marked: v = {} (returned).", k::num(m),
        k::q(a, "m"), notes.join(" and "), k::q(r, "m"), qv(v, "m/s")), &cv)
}

// ------------------------------------------------------------------------------------------------ optics
/// A thin lens ray diagram; kind "ray-diagram" (the given lens and object), "diverging" (the same distances with a
/// diverging lens of the opposite focal length), "magnifier" (the object inside the focal length).
fn lens(s: &Solved, kind: &str) -> Option<Figure> {
    let (d_o0, f0) = (s.inp("d_o")?, s.inp("f")?);
    let (d_o, f, title) = match kind {
        "ray-diagram" => (d_o0, f0, "Ray diagram of a thin lens"),
        "diverging" => (d_o0, -f0.abs(), "Ray diagram of a diverging lens"),
        _ => (f0.abs() / 2.0, f0.abs(), "A converging lens as a magnifier"),
    };
    if !(d_o > 0.0) || f == 0.0 || (d_o - f).abs() < 1e-12 * f.abs() {
        return None;
    }
    let di = if kind == "ray-diagram" { s.get("d_i")? } else { o(s, "thin_lens", &[("d_o", d_o), ("f", f)], "d_i")? };
    let mag = -di / d_o;
    let ext = d_o.max(di.abs()).max(f.abs() * 1.3);
    let sc = 220.0 / ext;
    let lx = 260.0;
    let ay = 170.0;
    let px = |x: f64| lx + x * sc;
    let h = 50.0;
    let mut cv = Canvas::new(320.0);
    cv.dashed((10.0, ay), (510.0, ay));
    // the lens
    if f > 0.0 {
        c::varrow(&mut cv, (lx, ay), (lx, ay - 120.0), "axp-k");
        c::varrow(&mut cv, (lx, ay), (lx, ay + 120.0), "axp-k");
    } else {
        cv.line((lx, ay - 120.0), (lx, ay + 120.0));
        cv.line((lx - 8.0, ay - 128.0), (lx, ay - 120.0));
        cv.line((lx + 8.0, ay - 128.0), (lx, ay - 120.0));
        cv.line((lx - 8.0, ay + 128.0), (lx, ay + 120.0));
        cv.line((lx + 8.0, ay + 128.0), (lx, ay + 120.0));
    }
    for sgn in [-1.0, 1.0] {
        cv.dot((px(sgn * f.abs()), ay));
    }
    cv.name(px(f.abs()), ay + 18.0, "middle", "F'");
    cv.name(px(-f.abs()), ay + 18.0, "middle", "F");
    // object (left, upright) and image
    let (ox, oy) = (px(-d_o), ay - h);
    c::varrow(&mut cv, (ox, ay), (ox, oy), "axp-c2");
    let (ix, iy) = (px(di), ay - h * mag);
    c::varrow(&mut cv, (ix, ay), (ix, iy), "axp-c3");
    // ray 1: parallel, then through F' (converging) or away from F (diverging, its extension through the near focus)
    let fp = (px(f), ay);
    let slope = (fp.1 - oy) / (fp.0 - lx);
    let end1 = (510.0, oy + slope * (510.0 - lx));
    c::poly(&mut cv, &[(ox, oy), (lx, oy), end1], "axp-c", false);
    // ray 2: through the centre
    let s2 = (ay - oy) / (lx - ox);
    c::poly(&mut cv, &[(ox, oy), (510.0, ay + s2 * (510.0 - lx))], "axp-c", false);
    if di < 0.0 {
        // virtual image: the refracted rays traced back
        cv.dashed((lx, oy), (ix, iy));
        cv.dashed((lx, ay), (ix, iy));
    }
    cv.bold(16.0, 24.0, "start", &format!("f = {}, object at {}", k::q(f, "m"), k::q(d_o, "m")));
    cv.value(16.0, 44.0, "start", &format!("image at d_i = {} ({}), magnification {}", k::q(di, "m"),
                                          if di > 0.0 { "real" } else { "virtual" }, k::num(mag)));
    let what = match kind {
        "ray-diagram" => "the given lens and object",
        "diverging" => "a diverging lens of the same focal length magnitude with the given object distance",
        _ => "the given converging lens with an object at half its focal length, inside the focal point",
    };
    fig(s, kind, title, &format!(
        "Ray diagram for {}: f = {}, object {} in front of the lens; the ray parallel to the axis is bent through (or \
         away from) the focal point, the ray through the centre goes straight. The image distance is a solve of the \
         model's thin-lens equation: d_i = {} ({} image), magnification {} (derived as -d_i / d_o).", what, k::q(f, "m"),
        k::q(d_o, "m"), k::q(di, "m"), if di > 0.0 { "a real, inverted" } else { "a virtual, upright" }, k::num(mag)), &cv)
}

fn mirror(s: &Solved) -> Option<Figure> {
    let (r, f) = (s.inp("R")?, s.get("f")?);
    if !(r > 0.0) {
        return None;
    }
    let mut cv = Canvas::new(320.0);
    let sc = 300.0 / r;
    let (vx, ay) = (440.0, 160.0);
    let cx = vx - r * sc;
    // the mirror: an arc of the circle of radius R round C
    let half = (130.0 / (r * sc)).min(0.9).asin();
    c::arc(&mut cv, (cx, ay), r * sc, -half, half, "axp-k");
    cv.dashed((40.0, ay), (500.0, ay));
    cv.dot((cx, ay));
    cv.name(cx, ay + 18.0, "middle", "C");
    let fx = vx - f * sc;
    cv.dot((fx, ay));
    cv.name(fx, ay + 18.0, "middle", "F");
    for y in [-90.0, -50.0, 50.0, 90.0] {
        let hit_x = cx + ((r * sc).powi(2) - y * y).sqrt();
        c::poly(&mut cv, &[(40.0, ay + y), (hit_x, ay + y), (fx - (hit_x - fx) * 0.3, ay - y * 0.3)], "axp-c", false);
    }
    cv.bold(16.0, 24.0, "start", &format!("R = {}: f = R/2 = {}", k::q(r, "m"), k::q(f, "m")));
    fig(s, "ray-diagram", "Parallel rays and the focal point of a concave mirror", &format!(
        "A concave mirror with radius of curvature R = {} (centre C): rays arriving parallel to the axis reflect through \
         the focal point F, half-way to C (paraxial rays). Marked: the focal length the model returns, f = R/2 = {}.",
        k::q(r, "m"), k::q(f, "m")), &cv)
}

fn medium_wave(s: &Solved) -> Option<Figure> {
    let (lam, n, ln) = (s.inp("lam")?, s.inp("n")?, s.get("lam_n")?);
    if !(lam > 0.0) || !(ln > 0.0) {
        return None;
    }
    let mut cv = Canvas::new(340.0);
    let sc = 200.0 / (2.0 * lam);
    cv.raw("<rect class=\"axp-s3\" x=\"260\" y=\"60\" width=\"250\" height=\"220\"/>");
    let mut p = vec![];
    for i in 0..=300 {
        let x = 20.0 + 490.0 * i as f64 / 300.0;
        let phase = if x <= 260.0 { (x - 20.0) / (lam * sc) } else { (240.0) / (lam * sc) + (x - 260.0) / (ln * sc) };
        p.push((x, 170.0 - 60.0 * (2.0 * PI * phase).sin()));
    }
    c::poly(&mut cv, &p, "axp-c", false);
    c::varrow(&mut cv, (20.0, 255.0), (20.0 + lam * sc, 255.0), "axp-c2");
    cv.value(20.0, 275.0, "start", &format!("λ = {}", k::q(lam, "m")));
    c::varrow(&mut cv, (270.0, 255.0), (270.0 + ln * sc, 255.0), "axp-c2");
    cv.value(270.0, 275.0, "start", &format!("λn = {}", k::q(ln, "m")));
    cv.name(500.0, 76.0, "end", &format!("medium, n = {}", k::num(n)));
    cv.bold(16.0, 24.0, "start", &format!("λn = λ / n = {}", k::q(ln, "m")));
    fig(s, "drawing", "Light entering a denser medium", &format!(
        "Light of vacuum wavelength λ = {} entering a medium of refractive index n = {} (shaded), drawn as a sine wave of \
         each wavelength: the frequency is unchanged, so the waves are compressed. Marked: the wavelength in the medium \
         the model returns, λn = {}.", k::q(lam, "m"), k::num(n), k::q(ln, "m")), &cv)
}

// ------------------------------------------------------------------------------------------------ oscillators, rotation
fn pendulums(s: &Solved) -> Option<Figure> {
    let l = s.inp("Lp")?;
    if !(l > 0.0) {
        return None;
    }
    let ls = [l / 4.0, l, 2.25 * l];
    let ts: Vec<f64> = ls.iter().map(|x| c::at(s, &[("Lp", *x)], "T")).collect::<Option<Vec<f64>>>()?;
    let mut cv = Canvas::new(360.0);
    cv.raw("<rect class=\"axp-ka\" x=\"40\" y=\"30\" width=\"440\" height=\"10\"/>");
    let pl = 240.0 / ls[2];
    for (j, (ll, tt)) in ls.iter().zip(&ts).enumerate() {
        let x = 110.0 + 150.0 * j as f64;
        let a: f64 = 0.25;
        let bob = (x + ll * pl * a.sin(), 40.0 + ll * pl * a.cos());
        cv.dashed((x, 40.0), (x, 40.0 + ll * pl));
        cv.line((x, 40.0), bob);
        c::disc(&mut cv, bob, 9.0, true);
        cv.name(x - 6.0, 40.0 + ll * pl / 2.0, "end", &k::q(*ll, "m"));
        cv.value(bob.0, bob.1 + 26.0, "middle", &format!("T = {}", k::q(*tt, "s")));
    }
    fig(s, "drawing", "Pendulum length and period", &format!(
        "Simple pendulums of a quarter, once and 2.25 times the given length drawn to scale (small swings): each period \
         a solve of the model; the period grows with the square root of the length, so a pendulum 9 times longer swings \
         3 times slower. Marked: {}.", ls.iter().zip(&ts).map(|(a, b)| format!("{} for {}", k::q(*b, "s"), k::q(*a, "m")))
        .collect::<Vec<_>>().join(", ")), &cv)
}

fn spring_masses(s: &Solved) -> Option<Figure> {
    let (m, kk, t) = (s.inp("m")?, s.inp("k")?, s.get("T")?);
    if !(m > 0.0) {
        return None;
    }
    let ms = [m, 4.0 * m, 9.0 * m];
    let ts: Vec<f64> = ms.iter().map(|x| c::at(s, &[("m", *x)], "T")).collect::<Option<Vec<f64>>>()?;
    let mut cv = Canvas::new(340.0);
    cv.raw("<rect class=\"axp-ka\" x=\"40\" y=\"30\" width=\"440\" height=\"10\"/>");
    for (j, (mm, tt)) in ms.iter().zip(&ts).enumerate() {
        let x = 110.0 + 150.0 * j as f64;
        let len = 120.0 + 25.0 * j as f64;
        c::spring(&mut cv, (x, 40.0), (x, 40.0 + len), 7, 10.0);
        let side = 30.0 + 8.0 * j as f64;
        c::rect(&mut cv, x - side / 2.0, 40.0 + len, side, side);
        cv.name(x, 40.0 + len + side + 16.0, "middle", &k::q(*mm, "kg"));
        cv.value(x, 40.0 + len + side + 34.0, "middle", &format!("T = {}", k::q(*tt, "s")));
    }
    cv.bold(16.0, 24.0, "start", &format!("k = {}", qv(kk, "N/m")));
    fig(s, "drawing", "Mass on a spring and its period", &format!(
        "Masses of once, 4 and 9 times the given {} on identical springs of stiffness k = {}: the period of oscillation, \
         a solve of the model for each, grows with the square root of the mass, independent of the amplitude. Marked: \
         the periods {} (given: {}).", k::q(m, "kg"), qv(kk, "N/m"),
        ts.iter().map(|x| k::q(*x, "s")).collect::<Vec<_>>().join(", "), k::q(t, "s")), &cv)
}

fn lever(s: &Solved) -> Option<Figure> {
    let (r, f, tau) = (s.inp("r")?, s.inp("F")?, s.get("tau")?);
    let th = s.inp("theta").unwrap_or(PI / 2.0);
    let t90 = c::at(s, &[("theta", PI / 2.0)], "tau")?;
    let mut cv = Canvas::new(300.0);
    let piv = (90.0, 200.0);
    let end = (400.0, 200.0);
    cv.raw(&format!("<path class=\"axp-kd\" d=\"M{},{} L{},{} L{},{} Z\"/>", piv.0, piv.1, piv.0 - 14.0, piv.1 + 24.0,
                    piv.0 + 14.0, piv.1 + 24.0));
    cv.raw("<rect class=\"axp-ka\" x=\"90\" y=\"194\" width=\"320\" height=\"12\"/>");
    cv.name(245.0, 226.0, "middle", &format!("r = {}", k::q(r, "m")));
    let tip = (end.0 + 110.0 * th.cos(), end.1 - 110.0 * th.sin());
    arrow_lab(&mut cv, end, tip, "axp-c", &format!("F = {}", k::q(f, "N")));
    if (th - PI / 2.0).abs() > 1e-6 {
        c::varrow(&mut cv, end, (end.0, end.1 - 110.0 * th.sin()), "axp-c2");
        cv.value(end.0 - 6.0, end.1 - 60.0, "end", &format!("F sin θ = {}", k::q(f * th.sin(), "N")));
    }
    c::arc(&mut cv, end, 26.0, 0.0, th, "axp-c4");
    cv.name(end.0 + 30.0, end.1 + 18.0, "start", &format!("θ = {}°", k::num(deg(th))));
    cv.bold(16.0, 24.0, "start", &format!("τ = r F sin θ = {} N·m", k::num(tau)));
    cv.value(16.0, 44.0, "start", &format!("at 90°: {} N·m", k::num(t90)));
    fig(s, "diagram", "Torque of a force on a lever", &format!(
        "A force F = {} applied at r = {} from a pivot at θ = {}° to the lever: only its component perpendicular to the \
         lever, F sin θ, turns it. Marked: the torque the model returns, τ = {} N·m (at 90° it would be {} N·m, also \
         returned).", k::q(f, "N"), k::q(r, "m"), k::num(deg(th)), k::num(tau), k::num(t90)), &cv)
}

fn point_mass_l(s: &Solved) -> Option<Figure> {
    let (m, v, r, l) = (s.inp("m")?, s.inp("v")?, s.inp("r")?, s.get("L")?);
    let mut cv = Canvas::new(400.0);
    let ctr = (180.0, 250.0);
    c::ellipse(&mut cv, ctr, 150.0, 150.0, "axp-c4");
    cv.wire_end(ctr, l >= 0.0);
    let p = (ctr.0 + 150.0, ctr.1);
    cv.dashed(ctr, p);
    cv.name(ctr.0 + 75.0, ctr.1 + 18.0, "middle", &format!("r = {}", k::q(r, "m")));
    c::disc(&mut cv, p, 10.0, true);
    arrow_lab(&mut cv, (p.0, p.1 - 12.0), (p.0, p.1 - 110.0), "axp-c2", &format!("v = {}", qv(v, "m/s")));
    cv.bold(16.0, 24.0, "start", &format!("L = m v r = {} kg·m²/s, out of the page", k::num(l)));
    cv.value(16.0, 44.0, "start", &format!("m = {}", k::q(m, "kg")));
    fig(s, "diagram", "Angular momentum of a point mass", &format!(
        "A {} body moving at v = {} perpendicular to its radius r = {} about a centre: its angular momentum points out of \
         the page (right-hand rule, the dot at the centre). Marked: the value the model returns, L = m v r = {} kg·m²/s.",
        k::q(m, "kg"), qv(v, "m/s"), k::q(r, "m"), k::num(l)), &cv)
}

fn atwood(s: &Solved) -> Option<Figure> {
    let (m1, m2, ir, r, a) = (s.inp("m1")?, s.inp("m2")?, s.inp("Ir")?, s.inp("R")?, s.get("a")?);
    let a0 = c::at(s, &[("Ir", 0.0)], "a");
    let mut cv = Canvas::new(380.0);
    let ctr = (260.0, 90.0);
    cv.line((260.0, 20.0), (260.0, 50.0));
    cv.raw("<rect class=\"axp-ka\" x=\"200\" y=\"10\" width=\"120\" height=\"10\"/>");
    c::disc(&mut cv, ctr, 40.0, true);
    cv.dot(ctr);
    let (h1, h2) = if m1 >= m2 { (260.0, 190.0) } else { (190.0, 260.0) };
    cv.line((220.0, 90.0), (220.0, h1));
    cv.line((300.0, 90.0), (300.0, h2));
    let s1 = 30.0 + 20.0 * (m1 / (m1 + m2));
    let s2 = 30.0 + 20.0 * (m2 / (m1 + m2));
    c::rect(&mut cv, 220.0 - s1 / 2.0, h1, s1, s1);
    c::rect(&mut cv, 300.0 - s2 / 2.0, h2, s2, s2);
    cv.name(220.0 - s1 / 2.0 - 8.0, h1 + s1 / 2.0 + 4.0, "end", &format!("m1 = {}", k::q(m1, "kg")));
    cv.name(300.0 + s2 / 2.0 + 8.0, h2 + s2 / 2.0 + 4.0, "start", &format!("m2 = {}", k::q(m2, "kg")));
    let down1 = a >= 0.0;
    c::varrow(&mut cv, (160.0, 200.0), (160.0, if down1 { 260.0 } else { 140.0 }), "axp-c");
    c::varrow(&mut cv, (360.0, 200.0), (360.0, if down1 { 140.0 } else { 260.0 }), "axp-c");
    cv.name(310.0, 60.0, "start", &format!("I = {} kg·m², R = {}", k::num(ir), k::q(r, "m")));
    cv.bold(16.0, 330.0, "start", &format!("a = {}", qv(a, "m/s^2")));
    if let Some(a0) = a0 {
        cv.value(16.0, 352.0, "start", &format!("with a massless pulley: {}", qv(a0, "m/s^2")));
    }
    fig(s, "drawing", "An Atwood machine with a massive pulley", &format!(
        "An Atwood machine: masses m1 = {} and m2 = {} on a cord over a pulley with moment of inertia I = {} kg·m² and \
         radius R = {}. Marked: the acceleration the model returns, a = {} (the heavier mass down){}.", k::q(m1, "kg"),
        k::q(m2, "kg"), k::num(ir), k::q(r, "m"), qv(a, "m/s^2"),
        a0.map(|x| format!("; with a massless pulley it would be {} (also returned)", qv(x, "m/s^2"))).unwrap_or_default()), &cv)
}

fn bead_hoop(s: &Solved) -> Option<Figure> {
    let (w, r, th) = (s.inp("w")?, s.inp("R")?, s.get("theta")?);
    let mut cv = Canvas::new(380.0);
    let ctr = (200.0, 200.0);
    let rp = 150.0;
    c::ellipse(&mut cv, ctr, rp, rp, "axp-k");
    cv.dashed((ctr.0, 20.0), (ctr.0, 370.0));
    let bead = (ctr.0 + rp * th.sin(), ctr.1 + rp * th.cos());
    cv.dashed(ctr, bead);
    cv.dashed((ctr.0, bead.1), bead);
    c::disc(&mut cv, bead, 9.0, true);
    c::arc(&mut cv, ctr, 40.0, -PI / 2.0, -PI / 2.0 + th, "axp-c");
    cv.name(ctr.0 + 14.0, ctr.1 + 56.0, "start", &format!("θ = {}°", k::num(deg(th))));
    cv.value(bead.0 + 14.0, bead.1, "start", &format!("R sin θ = {}", k::q(r * th.sin(), "m")));
    cv.bold(370.0, 60.0, "start", &format!("ω = {}", qv(w, "rad/s")));
    cv.value(370.0, 84.0, "start", &format!("R = {}", k::q(r, "m")));
    cv.value(370.0, 108.0, "start", &format!("θ = {} rad", k::num(th)));
    fig(s, "drawing", "Where a bead rests on a spinning hoop", &format!(
        "A bead free to slide on a hoop of radius R = {} spinning about its vertical diameter at ω = {}: it settles where \
         gravity and the required centripetal force balance. Marked: the equilibrium angle the model returns, θ = {} rad \
         = {}° from the bottom, and the radius of the bead's circle R sin θ = {} (derived).", k::q(r, "m"), qv(w, "rad/s"),
        k::num(th), k::num(deg(th)), k::q(r * th.sin(), "m")), &cv)
}

fn collision(s: &Solved) -> Option<Figure> {
    let (m1, m2, v1, v2, v1f) = (s.inp("m1")?, s.inp("m2")?, s.inp("v1")?, s.inp("v2")?, s.get("v1f")?);
    let v2f = o(s, "elastic_collision_second_final", &[("m1", m1), ("m2", m2), ("v1", v1), ("v2", v2)], "v2f")?;
    let sc = scale(&[v1, v2, v1f, v2f], 90.0)?;
    let mut cv = Canvas::new(330.0);
    let row = |cv: &mut Canvas, y: f64, a: f64, b: f64, label: &str| {
        cv.name(16.0, y - 30.0, "start", label);
        cv.line((20.0, y + 22.0), (500.0, y + 22.0));
        c::disc(cv, (180.0, y), 14.0 + 8.0 * (m1 / (m1 + m2)), true);
        c::disc(cv, (320.0, y), 14.0 + 8.0 * (m2 / (m1 + m2)), true);
        if a != 0.0 {
            arrow_lab(cv, (180.0, y - 34.0), (180.0 + a * sc, y - 34.0), "axp-c", &qv(a, "m/s"));
        }
        if b != 0.0 {
            arrow_lab(cv, (320.0, y - 34.0), (320.0 + b * sc, y - 34.0), "axp-c2", &qv(b, "m/s"));
        }
    };
    row(&mut cv, 110.0, v1, v2, "before");
    row(&mut cv, 250.0, v1f, v2f, "after");
    cv.name(180.0, 140.0, "middle", &k::q(m1, "kg"));
    cv.name(320.0, 140.0, "middle", &k::q(m2, "kg"));
    let p0 = m1 * v1 + m2 * v2;
    cv.value(504.0, 318.0, "end", &format!("total momentum {} kg·m/s before and after", k::num(p0)));
    fig(s, "diagram", "A head-on elastic collision", &format!(
        "A head-on elastic collision: a {} body at {} strikes a {} body moving at {}; velocity arrows to scale. Marked: \
         the final velocities the model returns, {} for the first and {} for the second; total momentum {} kg·m/s \
         (derived from the inputs).", k::q(m1, "kg"), qv(v1, "m/s"), k::q(m2, "kg"), qv(v2, "m/s"), qv(v1f, "m/s"),
        qv(v2f, "m/s"), k::num(p0)), &cv)
}

// ------------------------------------------------------------------------------------------------ energy flows
fn flow(s: &Solved, what: &str) -> Option<Figure> {
    // (hot side, cold side, work) from the inputs and the model's answer
    let (qh, qc, w, extra) = match what {
        "engine" => {
            let (qh, qc, w) = (s.inp("Qh")?, s.inp("Qc")?, s.get("W")?);
            let eff = o(s, "engine_efficiency", &[("W", w), ("Qh", qh)], "eff");
            (qh, qc, w, eff.map(|e| format!("efficiency W/Qh = {}", k::num(e))))
        }
        "refrigerator" => {
            let (qc, w, cop) = (s.inp("Qc")?, s.inp("W")?, s.get("cop")?);
            (qc + w, qc, w, Some(format!("COP = Qc/W = {}", k::num(cop))))
        }
        _ => {
            let (qh, w, cop) = (s.inp("Qh")?, s.inp("W")?, s.get("cop")?);
            (qh, qh - w, w, Some(format!("COP = Qh/W = {}", k::num(cop))))
        }
    };
    if !(qh > 0.0 && qc >= 0.0 && w > 0.0) {
        return None;
    }
    let sc = 70.0 / qh.max(qc).max(w);
    let mut cv = Canvas::new(380.0);
    let box_ = |cv: &mut Canvas, y: f64, class: &str, label: &str| {
        cv.raw(&format!("<rect class=\"{class}\" x=\"160\" y=\"{y}\" width=\"200\" height=\"32\" rx=\"5\"/>"));
        cv.bold(260.0, y + 21.0, "middle", label);
    };
    let heat_down = what == "engine";
    box_(&mut cv, 20.0, "axp-b", if what == "heat pump" { "house (hot)" } else if what == "refrigerator" { "room (hot)" } else { "hot reservoir" });
    box_(&mut cv, 328.0, "axp-b3", if what == "heat pump" { "outdoors (cold)" } else if what == "refrigerator" { "cold space" } else { "cold reservoir" });
    cv.block(200.0, 170.0, 120.0, 36.0, what);
    let band = |cv: &mut Canvas, x: f64, y0: f64, y1: f64, wpx: f64, class: &str| {
        let wpx = wpx.max(4.0);
        let dir = if y1 > y0 { 1.0 } else { -1.0 };
        let tip = y1;
        let body = y1 - dir * 18.0;
        cv.raw(&format!("<path class=\"{class}\" d=\"M{:.1},{y0:.1} L{:.1},{body:.1} L{:.1},{body:.1} L{x:.1},{tip:.1} L{:.1},{body:.1} L{:.1},{body:.1} L{:.1},{y0:.1} Z\"/>",
                        x - wpx / 2.0, x - wpx / 2.0, x - wpx / 2.0 - 10.0, x + wpx / 2.0 + 10.0, x + wpx / 2.0, x + wpx / 2.0));
    };
    if heat_down {
        band(&mut cv, 260.0, 52.0, 168.0, qh * sc, "axp-s");
        band(&mut cv, 260.0, 208.0, 326.0, qc * sc, "axp-s3");
    } else {
        band(&mut cv, 260.0, 168.0, 52.0, qh * sc, "axp-s");
        band(&mut cv, 260.0, 326.0, 208.0, qc * sc, "axp-s3");
    }
    // the work: out of an engine, into a refrigerator or heat pump
    let wy = 188.0;
    if heat_down {
        cv.raw(&format!("<path class=\"axp-s2\" d=\"M322,{:.1} L440,{:.1} L440,{:.1} L470,{wy} L440,{:.1} L440,{:.1} L322,{:.1} Z\"/>",
                        wy - w * sc / 2.0, wy - w * sc / 2.0, wy - w * sc / 2.0 - 8.0, wy + w * sc / 2.0 + 8.0,
                        wy + w * sc / 2.0, wy + w * sc / 2.0));
    } else {
        cv.raw(&format!("<path class=\"axp-s2\" d=\"M470,{:.1} L350,{:.1} L350,{:.1} L322,{wy} L350,{:.1} L350,{:.1} L470,{:.1} Z\"/>",
                        wy - w * sc / 2.0, wy - w * sc / 2.0, wy - w * sc / 2.0 - 8.0, wy + w * sc / 2.0 + 8.0,
                        wy + w * sc / 2.0, wy + w * sc / 2.0));
    }
    cv.value(300.0, 110.0, "start", &format!("Qh = {}", k::q(qh, "J")));
    cv.value(300.0, 270.0, "start", &format!("Qc = {}", k::q(qc, "J")));
    cv.value(400.0, wy - w * sc / 2.0 - 14.0, "middle", &format!("W = {}", k::q(w, "J")));
    if let Some(e) = &extra {
        cv.bold(16.0, 190.0, "start", e);
    }
    let cap = match what {
        "engine" => format!("Energy flow through a heat engine that takes Qh = {} from a hot reservoir and rejects Qc = {} \
                             to a cold one; arrow widths to scale. Marked: the work the model returns, W = Qh - Qc = {}{}.",
                            k::q(qh, "J"), k::q(qc, "J"), k::q(w, "J"), extra.map(|e| format!(", and the {e} from its efficiency formula")).unwrap_or_default()),
        "refrigerator" => format!("A refrigerator moving Qc = {} out of the cold space with W = {} of work; it rejects Qh = \
                                   Qc + W = {} (derived) to the room. Marked: the coefficient of performance the model \
                                   returns, {}. Arrow widths to scale.", k::q(qc, "J"), k::q(w, "J"), k::q(qh, "J"),
                                  extra.unwrap_or_default()),
        _ => format!("A heat pump delivering Qh = {} into a house with W = {} of work; it draws Qc = Qh - W = {} (derived) \
                      from outdoors. Marked: the coefficient of performance the model returns, {}. Arrow widths to scale.",
                     k::q(qh, "J"), k::q(w, "J"), k::q(qc, "J"), extra.unwrap_or_default()),
    };
    fig(s, "flow", &format!("Energy flow through a {what}"), &cap, &cv)
}
