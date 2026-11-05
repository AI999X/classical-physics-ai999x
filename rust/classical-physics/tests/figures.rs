//! The second pilot of the figure kit (beyond circuits): the magnetic field of a long straight wire draws its
//! field-line diagram and its curve family for the visitor's inputs, from the model's own values.

use ax_engine::figkit::check_figure;
use ax_engine::figures::figures;
use ax_engine::Input;
use classical_physics_ai999x::pack;

fn given(pairs: &[(&str, &str)]) -> Vec<(String, Input)> {
    pairs.iter().map(|(k, v)| (k.to_string(), Input::Text(v.to_string()))).collect()
}

#[test]
fn long_wire_draws_field_lines_and_a_curve_family() {
    let p = pack();
    let g = given(&[("I", "10 A"), ("r", "5 cm")]);
    let figs = figures(p, "long_wire_field", &g, None, None);
    let diagram = figs.iter().find(|f| f.slug == "long-wire-field-diagram").expect("the field-line diagram");
    let curve = figs.iter().find(|f| f.slug == "long-wire-field-curve").expect("the curve family");
    assert_eq!(figs[0].slug, "long-wire-field-diagram");                 // the model's own figures first
    // B = mu0 I / (2 pi r): 40 µT at 5 cm, 100 µT at 2 cm, 20 µT at 10 cm, as the model computes them
    for t in ["40 µT", "100 µT", "20 µT", "out of the page"] {
        assert!(diagram.svg.contains(t), "the diagram lacks '{t}'");
    }
    assert_eq!(diagram.svg.matches("class=\"axp-c\"").count(), 3);      // three field lines
    for t in ["I = 10 A", "I = 5 A", "I = 20 A"] {
        assert!(curve.svg.contains(t), "the curve family lacks '{t}'");
    }
    check_figure(diagram).unwrap();
    check_figure(curve).unwrap();

    // another current: other labels
    let g2 = given(&[("I", "3 A"), ("r", "2 cm")]);
    let figs2 = figures(p, "long_wire_field", &g2, None, None);
    let d2 = figs2.iter().find(|f| f.slug == "long-wire-field-diagram").unwrap();
    assert_ne!(d2.svg, diagram.svg);
    assert!(d2.svg.contains("30 µT"), "B = 30 µT at 2 cm for 3 A");
    check_figure(d2).unwrap();
}

#[test]
fn long_wire_solved_for_the_distance_still_draws() {
    // asked for r from B and I: the diagram is drawn round the distance computed
    let p = pack();
    let g = given(&[("I", "10 A"), ("B", "4e-5 T")]);
    let figs = figures(p, "long_wire_field", &g, Some("r"), None);
    let d = figs.iter().find(|f| f.slug == "long-wire-field-diagram").expect("diagram");
    assert!(d.svg.contains("40 µT"));
    check_figure(d).unwrap();
}
