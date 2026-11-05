//! Classical Physics' own live figures (ax_engine::figkit), drawn for the visitor's inputs from what the model
//! computes: the magnetic field of a long straight wire here, the charts (maps over two inputs, curve families,
//! comparisons, bars) in figures_charts.rs, the drawings (free-body diagrams, springs, orbits, circuits, rays) in
//! figures_draw.rs (docs/FIGURE_PARITY.md).

use ax_engine::figkit::{self as k, eq, q, Canvas, Solved};
use ax_engine::figures::Figure;
use ax_engine::Input;

/// The figures for one solved request (the dispatcher ax_engine calls: Pack::figures).
pub fn live(s: &Solved) -> Vec<Figure> {
    let v: Vec<Option<Figure>> = match s.key {
        "long_wire_field" => vec![wire_field_lines(s), wire_family(s)],
        _ => vec![],
    };
    let mut out: Vec<Figure> = v.into_iter().flatten().collect();
    out.extend(crate::figures_draw::live(s));
    out.extend(crate::figures_charts::live(s));
    out
}

/// B of the wire computed by the model for a current I at a distance r (the permeability as the request has it).
fn wire_b(s: &Solved, i: f64, r: f64) -> Option<f64> {
    let mut set = vec![("I", Input::Float(i)), ("r", Input::Float(r))];
    if let Some(mu) = s.inp("mu0") {
        set.push(("mu0", Input::Float(mu)));
    }
    let a = s.solve_other(s.key, &set)?;
    k::num_of(&a, "B")
}

fn wire_field_lines(s: &Solved) -> Option<Figure> {
    let (i, r, b) = (s.get("I")?, s.get("r")?, s.get("B")?);
    if !(r > 0.0) || i == 0.0 {
        return None;
    }
    let mut c = Canvas::new(420.0);
    let centre = (200.0, 220.0);
    let px = 85.0 / r;                                    // the visitor's distance is the middle ring
    let rings: Vec<(f64, f64)> = [0.4, 1.0, 2.0].iter()
        .filter_map(|f| wire_b(s, i * 1.0, r * f).map(|bb| (r * f, bb))).collect();
    if rings.len() < 2 {
        return None;
    }
    // the field lines circle anticlockwise round a current out of the page (right-hand rule); a negative current
    // flows into the page and its lines turn clockwise
    let out = i > 0.0;
    for (rr, _) in &rings {
        c.field_ring(centre, rr * px, out);
    }
    c.wire_end(centre, out);
    c.dashed((centre.0 + 12.0, centre.1), (centre.0 + 2.0 * r * px + 20.0, centre.1));
    for (k_, (rr, bb)) in rings.iter().enumerate() {
        let x = centre.0 + rr * px;
        c.dot((x, centre.1));
        let up = if k_ % 2 == 0 { 8.0 } else { 26.0 };
        c.value(x + 4.0, centre.1 - up, "start", &q(*bb, "T"));
        c.name(x + 4.0, centre.1 + 18.0, "start", &q(*rr, "m"));
    }
    c.bold(16.0, 24.0, "start", &format!("I = {} {}", q(i.abs(), "A"), if out { "out of the page" } else { "into the page" }));
    c.value(16.0, 44.0, "start", &format!("{} at r = {}", eq("B", b, "T"), q(r, "m")));
    c.name(504.0, 404.0, "end", "B falls as 1/r");
    Some(s.figure("diagram", "Magnetic field round a straight wire",
                  &format!("Field lines round a long straight wire carrying I = {} {}: circles, {} (right-hand rule). \
                            Marked on each ring: the field strength the model computes there ({}), falling as 1/r.",
                           q(i.abs(), "A"), if out { "out of the page" } else { "into the page" },
                           if out { "anticlockwise" } else { "clockwise" },
                           rings.iter().map(|(rr, bb)| format!("{} at {}", q(*bb, "T"), q(*rr, "m")))
                               .collect::<Vec<_>>().join(", ")), c.body(), c.h))
}

fn wire_family(s: &Solved) -> Option<Figure> {
    let (i, r, b) = (s.get("I")?, s.get("r")?, s.get("B")?);
    if !(r > 0.0) {
        return None;
    }
    let rs = k::span(r * 0.5, r * 1.5, 31, false);
    let mut series = vec![];
    for f in [1.0, 0.5, 2.0] {
        let pts: Vec<(f64, f64)> = rs.iter().filter_map(|x| wire_b(s, i * f, *x).map(|y| (*x, y))).collect();
        if pts.len() >= 2 {
            series.push((eq("I", i * f, "A"), pts));
        }
    }
    if series.is_empty() {
        return None;
    }
    let body = k::lines(&series, "Distance from the wire r (m)", "Magnetic field B (T)", &eq("B", b, "T"), Some((r, b)),
                        &format!("r = {}", q(r, "m")), false);
    Some(s.chart("curve", "Magnetic field against distance from the wire, for three currents",
                 &format!("B against r from {} to {}, one curve for each current (the one given, half and twice it), 31 \
                           points each computed by the model; marked: r = {}, B = {}.", q(r * 0.5, "m"), q(r * 1.5, "m"),
                          q(r, "m"), q(b, "T")), &body))
}
