# Classical Physics (classical-physics-ai999x)

[![tests](https://github.com/AI999X/classical-physics-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/classical-physics-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/classical-physics-ai999x)](https://pypi.org/project/classical-physics-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/classical-physics-ai999x)

Classical Physics covers kinematics, dynamics, energy, momentum, rotation, gravitation, oscillations, waves and sound, optics, thermodynamics, electricity and magnetism: 162 calculations in 37 topics, each with its formula, its steps and its units, and a plain reason instead of a number when an input is missing or out of range. One compiled engine, four ways to use it: the desktop app, the command line, the browser and Python. Everything runs on your own computer, offline.

[![The Classical Physics desktop app: Translational kinetic energy, with its worked example and live scene](https://www.ai999x.com/?ax_shot=classical-physics-ai999x/hero.webp)](https://www.ai999x.com/models/classical-physics-ai999x/)

## Four ways to use it

| | |
|---|---|
| **Desktop app** (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.6/ai999x-classical-physics-1.6.6-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.6/ai999x-classical-physics-1.6.6-linux-x86_64.tar.gz) |
| **Command line** `classical-physics` (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.6/ai999x-cli-1.6.6-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.6/ai999x-cli-1.6.6-linux-x86_64.tar.gz) |
| **Browser** (any device) | [Try it on the model page](https://www.ai999x.com/models/classical-physics-ai999x/#capabilities) |
| **Python** 3.10+ | `pip install classical-physics-ai999x`: on Windows and Linux (x86-64) `pip` installs the same compiled engine; on other systems (macOS, ARM) it installs a pure-Python build that gives the same answers |

## See it

From the Classical Physics desktop app: every scene and chart is drawn from the computed answer.

[![Classical Physics desktop app: a slider moves an input and the answer, the scene and the charts follow](https://www.ai999x.com/?ax_shot=classical-physics-ai999x/clip-loop.webp)](https://www.ai999x.com/?ax_shot=classical-physics-ai999x/clip.mp4)

*[Watch the full clip (MP4)](https://www.ai999x.com/?ax_shot=classical-physics-ai999x/clip.mp4)*

| | |
|---|---|
| [![Translational kinetic energy: the live scene in the Classical Physics desktop app](https://www.ai999x.com/?ax_shot=classical-physics-ai999x/scene-1.webp)](https://www.ai999x.com/models/classical-physics-ai999x/#cap-kinetic_energy)<br>Translational kinetic energy | [![Angular frequency of a damped oscillator: the live scene in the Classical Physics desktop app](https://www.ai999x.com/?ax_shot=classical-physics-ai999x/scene-2.webp)](https://www.ai999x.com/models/classical-physics-ai999x/#cap-damped_angular_frequency)<br>Angular frequency of a damped oscillator |
| [![Escape speed: the live scene in the Classical Physics desktop app](https://www.ai999x.com/?ax_shot=classical-physics-ai999x/scene-3.webp)](https://www.ai999x.com/models/classical-physics-ai999x/#cap-escape_speed)<br>Escape speed | [![Velocity change of a rocket (Tsiolkovsky): the live scene in the Classical Physics desktop app](https://www.ai999x.com/?ax_shot=classical-physics-ai999x/scene-4.webp)](https://www.ai999x.com/models/classical-physics-ai999x/#cap-rocket_delta_v)<br>Velocity change of a rocket (Tsiolkovsky) |
| [![Reactance of a capacitor: the live scene in the Classical Physics desktop app](https://www.ai999x.com/?ax_shot=classical-physics-ai999x/scene-5.webp)](https://www.ai999x.com/models/classical-physics-ai999x/#cap-capacitive_reactance)<br>Reactance of a capacitor | [![Charge on a capacitor: the live scene in the Classical Physics desktop app](https://www.ai999x.com/?ax_shot=classical-physics-ai999x/scene-6.webp)](https://www.ai999x.com/models/classical-physics-ai999x/#cap-capacitance_definition)<br>Charge on a capacitor |
| [![Power from current and resistance: the live scene in the Classical Physics desktop app](https://www.ai999x.com/?ax_shot=classical-physics-ai999x/scene-7.webp)](https://www.ai999x.com/models/classical-physics-ai999x/#cap-ohm_power_current)<br>Power from current and resistance | [![Angular frequency and frequency: the live scene in the Classical Physics desktop app](https://www.ai999x.com/?ax_shot=classical-physics-ai999x/scene-8.webp)](https://www.ai999x.com/models/classical-physics-ai999x/#cap-angular_frequency)<br>Angular frequency and frequency |

Charts:

|   |   |   |   |
|---|---|---|---|
| [![Translational kinetic energy: a chart in the Classical Physics desktop app](https://www.ai999x.com/?ax_shot=classical-physics-ai999x/chart-1.webp)](https://www.ai999x.com/models/classical-physics-ai999x/#cap-kinetic_energy)<br>Translational kinetic energy | [![Angular frequency of a damped oscillator: a chart in the Classical Physics desktop app](https://www.ai999x.com/?ax_shot=classical-physics-ai999x/chart-2.webp)](https://www.ai999x.com/models/classical-physics-ai999x/#cap-damped_angular_frequency)<br>Angular frequency of a damped oscillator | [![Escape speed: a chart in the Classical Physics desktop app](https://www.ai999x.com/?ax_shot=classical-physics-ai999x/chart-3.webp)](https://www.ai999x.com/models/classical-physics-ai999x/#cap-escape_speed)<br>Escape speed | [![Velocity change of a rocket (Tsiolkovsky): a chart in the Classical Physics desktop app](https://www.ai999x.com/?ax_shot=classical-physics-ai999x/chart-4.webp)](https://www.ai999x.com/models/classical-physics-ai999x/#cap-rocket_delta_v)<br>Velocity change of a rocket (Tsiolkovsky) |

## Ask in words: 15 examples

Each question below was asked on this release; the comment under it is the line it printed.

```python
import classical_physics_ai999x as classical_physics

a = classical_physics.ask(
    "Coulomb's law: q1 = 2 uC, q2 = 3 uC, r = 10 cm"
)
print(a.outcome, a.values)
# SOLVED {'F': 5.392531075379998, 'F_radial': 5.392531075379998}

a = classical_physics.ask("Critical angle: n1 = 1.5, n2 = 1")
print(a.outcome, a.values)
# SOLVED {'theta_c': 0.729727656227, 'theta_c_deg': 41.8103148958}

a = classical_physics.ask(
    "A body moving at 4 m/s accelerates uniformly at 2 m/s^2 for 5 s. "
    "What are its final velocity and displacement?"
)
print(a.outcome, a.values)
# SOLVED {'v': 14.0, 's': 45.0}

a = classical_physics.ask(
    "Angular momentum conservation: I1 = 4 kg*m^2, omega1 = 2 rad/s, "
    "I2 = 1.6 kg*m^2"
)
print(a.outcome, a.values)
# SOLVED {'omega2': 5.0, 'L': 8.0, 'K1': 8.0, 'K2': 20.0, 'dK': 12.0}

a = classical_physics.ask(
    "Sound level change: beta1 = 60 dB, beta2 = 80 dB"
)
print(a.outcome, a.values)
# SOLVED {'d_beta': 20.0, 'ratio': 100.0, 'beta2': 80.0}

a = classical_physics.ask(
    "Beam support reactions: loads = 200, 300 N; positions = 1, 3 m; "
    "xA = 0 m; xB = 4 m"
)
print(a.outcome, a.values)
# SOLVED {'RA': 225.0, 'RB': 275.0, 'total': 500.0}

a = classical_physics.ask(
    "Centre of mass: masses = 2, 3 kg; x = 0, 5 m"
)
print(a.outcome, a.values)
# SOLVED {'M': 5.0, 'x_cm': 3.0}

a = classical_physics.ask(
    "Shear modulus deformation: F = 40 kN, A = 20 cm^2, h = 10 cm, S = "
    "25 GPa"
)
print(a.outcome, a.values)
# SOLVED {'stress': 20000000.0, 'strain': 0.0008, 'dx': 8e-05}

a = classical_physics.ask(
    "Ideal transformer: Np = 500, Ns = 50, Vp = 230 V, Ip = 0.5 A"
)
print(a.outcome, a.values)
# SOLVED {'Vs': 23.0, 'ratio': 0.1, 'type': 'step-down', 'Is': 5.0}

a = classical_physics.ask(
    "Static friction: mu_s = 0.5, N = 100 N, F = 40 N"
)
print(a.outcome, a.values)
# SOLVED {'f_max': 50.0, 'slips': False, 'margin': 10.0}

a = classical_physics.ask(
    "Image magnification: d_o = 30 cm, f = 10 cm, h_o = 2 cm"
)
print(a.outcome, a.values)
# SOLVED {'m': -0.5, 'h_i': -0.01, 'd_i': 0.15, 'image': 'real, inverted'}

a = classical_physics.ask(
    "Gravitational force: m1 = 100 kg, m2 = 50 kg, r = 2 m"
)
print(a.outcome, a.values)
# SOLVED {'F': 8.342874999999999e-08}

a = classical_physics.ask(
    "Rotational second law: tau = 12 N*m, I = 3 kg*m^2"
)
print(a.outcome, a.values)
# SOLVED {'alpha': 4.0}

a = classical_physics.ask("Sound intensity level: I = 1e-5 W/m^2")
print(a.outcome, a.values)
# SOLVED {'beta': 70.0, 'I': 1e-05}

a = classical_physics.ask(
    "Standing wave harmonic, closed pipe: L = 25 cm, v = 340 m/s, n = "
    "3"
)
print(a.outcome, a.values)
# SOLVED {'f_n': 1020.0, 'lam_n': 0.3333333333333333, 'f1': 340.0}
```

## Command line: 15 examples

Each command below was run with `classical-physics` of this release; the lines under it are the answer it printed (the full answer goes on with the method, every input in SI units and the conditions it assumes).

```text
$ classical-physics solve capacitive_reactance "f=60 Hz" "C=1e-06 F"
capacitive reactance (X) = 2652.582385 ohm

$ classical-physics solve parallel_plate_capacitor "A=0.01 m^2" "d=0.001 m" "eps0=8.854187813e-12 C^2/(N*m^2)"
capacitance (C) = 8.854187813e-11 F

$ classical-physics solve ohm_power_current "I=2 A" "R=5 ohm"
power dissipated (P) = 20 W

$ classical-physics solve angular_frequency "f=0.5 Hz"
angular frequency (omega) = 3.141592654 rad/s

$ classical-physics solve elastic_collision_first_final "m1=2 kg" "m2=1 kg" "v1=3 m/s" "v2=0 m/s"
final velocity of the first body (v1f) = 1 m/s

$ classical-physics solve terminal_voltage_internal_resistance "emf=12 V" "r=0.5 ohm" "R=5.5 ohm"
I = 2 A
V_t = 11 V
V_lost = 1 V
P_load = 22 W
efficiency = 0.9166666667

$ classical-physics solve acceleration_frictionless_incline "g=9.8 m/s^2" "theta=30 deg"
acceleration down the incline (a) = 4.9 m/s^2

$ classical-physics solve bulk_modulus_compression "dP=20 MPa" "V0=1 m^3" "B=2.2 GPa"
dV = -0.009090909091 m^3
strain = -0.009090909091
V = 0.9909090909 m^3

$ classical-physics solve charge_current "I=2 A" "t=30 s"
charge (Q) = 60 C

$ classical-physics solve faraday_induced_emf N=200 "A1=100 cm^2" "B1=0 T" "B2=0.5 T" "dt=0.1 s"
phi1 = 0 Wb
phi2 = 0.005 Wb
dphi = 0.005 Wb
emf = 10 V
emf_signed = -10 V

$ classical-physics solve coulomb_force "q1=2 uC" "q2=3 uC" "r=10 cm"
F = 5.392531075 N
F_radial = 5.392531075 N

$ classical-physics solve power_energy "E=600 J" "t=20 s"
average power (P) = 30 W

$ classical-physics solve absolute_pressure_depth "P0=101325 Pa" "rho=1000 kg/m^3" "g=9.80665 m/s^2" "h=10 m"
absolute pressure at the depth (P) = 199391.5 Pa

$ classical-physics solve static_friction_incline "mu_s=0.5 1" "theta=20 deg"
theta_c = 0.463647609 rad
theta_c_deg = 26.56505118
slips = false

$ classical-physics solve critical_angle_total_internal_reflection "n1=1.5 1" "n2=1 1"
theta_c = 0.7297276562 rad
theta_c_deg = 41.8103149
```

## Guides

- Desktop app guide: the window, every calculation, scenes and charts: [PDF](https://huggingface.co/AI999X/classical-physics-ai999x/blob/main/docs/classical_physics_ai999x_desktop_guide.pdf) · [HTML](https://huggingface.co/AI999X/classical-physics-ai999x/blob/main/docs/classical_physics_ai999x_desktop_guide.html) · [Markdown](https://huggingface.co/AI999X/classical-physics-ai999x/blob/main/docs/classical_physics_ai999x_desktop_guide.md)
- Python guide: every calculation with a Python example: [PDF](https://huggingface.co/AI999X/classical-physics-ai999x/blob/main/docs/classical_physics_ai999x_capabilities.pdf) · [HTML](https://huggingface.co/AI999X/classical-physics-ai999x/blob/main/docs/classical_physics_ai999x_capabilities.html) · [Markdown](https://huggingface.co/AI999X/classical-physics-ai999x/blob/main/docs/classical_physics_ai999x_capabilities.md)
- Command-line guide: every calculation with the command and what it printed: [PDF](https://huggingface.co/AI999X/classical-physics-ai999x/blob/main/docs/classical_physics_ai999x_rust_guide.pdf) · [HTML](https://huggingface.co/AI999X/classical-physics-ai999x/blob/main/docs/classical_physics_ai999x_rust_guide.html) · [Markdown](https://huggingface.co/AI999X/classical-physics-ai999x/blob/main/docs/classical_physics_ai999x_rust_guide.md)

Every calculation, with its formula, a worked example and its figures: **[Classical Physics on ai999x.com](https://www.ai999x.com/models/classical-physics-ai999x/)**. Install guides for the app and the command line: [https://www.ai999x.com/desktop-cli/](https://www.ai999x.com/desktop-cli/).

Version 1.6.6 · License: Apache-2.0 · Author: Ibnul Jaif Farabi · Organization: AI999X · Contact: support@ai999x.com · [Support the project](https://buymeacoffee.com/ibnuljaiffarabi)

*The Future Is Self-Improving.*

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/classical-physics-ai999x/) (`pip install .` builds it). Releases are tagged `v1.6.6`; the [GitHub release](https://github.com/AI999X/classical-physics-ai999x/releases/tag/v1.6.6) carries the wheel, the desktop app and the command-line program for Windows and Linux, and their checksums.

## Build the program from source

`rust/` holds the Rust source of the `classical-physics` command-line program and browser module: the AI999X engine (`rust/ax-engine`), this model (`rust/classical-physics`) and its data (`rust/models/classical_physics/pack.json`). With Rust 1.80 or later:

```bash
cd rust
cargo build --release -p classical-physics-ai999x
./target/release/classical-physics --help
cargo test -p classical-physics-ai999x
```
