# Classical Physics (classical-physics-ai999x)

[![tests](https://github.com/AI999X/classical-physics-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/classical-physics-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/classical-physics-ai999x)](https://pypi.org/project/classical-physics-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/classical-physics-ai999x)

Classical Physics covers kinematics, dynamics, energy, momentum, rotation, gravitation, oscillations, waves and sound, optics, thermodynamics, electricity and magnetism: 162 calculations in 37 topics, each with its formula, its steps and its units, and a plain reason instead of a number when an input is missing or out of range. One compiled engine, four ways to use it: the desktop app, the command line, the browser and Python. Everything runs on your own computer, offline.

[![The Classical Physics desktop app: Average net force from a change of momentum, with its worked example and live scene](https://www.ai999x.com/?ax_shot=classical-physics-ai999x/hero.webp)](https://www.ai999x.com/models/classical-physics-ai999x/)

## Four ways to use it

| | |
|---|---|
| **Desktop app** (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-1.6.4-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-1.6.4-linux-x86_64.tar.gz) |
| **Command line** `classical-physics` (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-cli-1.6.4-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-cli-1.6.4-linux-x86_64.tar.gz) |
| **Browser** (any device) | [Try it on the model page](https://www.ai999x.com/models/classical-physics-ai999x/#try) |
| **Python** 3.10+ | `pip install classical-physics-ai999x`: on Windows and Linux (x86-64) `pip` installs the same compiled engine; on other systems (macOS, ARM) it installs a pure-Python build that gives the same answers |

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

## Screenshots

[![The Classical Physics desktop app: Average net force from a change of momentum, with its worked example and live scene](https://www.ai999x.com/?ax_shot=classical-physics-ai999x/hero-dark.webp)](https://www.ai999x.com/models/classical-physics-ai999x/)

[![Angular frequency of a damped oscillator: the live scene in the Classical Physics desktop app](https://www.ai999x.com/?ax_shot=classical-physics-ai999x/see-1.webp)](https://www.ai999x.com/models/classical-physics-ai999x/)

*Angular frequency of a damped oscillator*

[![Coefficient of performance of a heat pump: the live scene in the Classical Physics desktop app](https://www.ai999x.com/?ax_shot=classical-physics-ai999x/see-2.webp)](https://www.ai999x.com/models/classical-physics-ai999x/)

*Coefficient of performance of a heat pump*

[![Gravitational potential energy near the surface: the live scene in the Classical Physics desktop app](https://www.ai999x.com/?ax_shot=classical-physics-ai999x/see-3.webp)](https://www.ai999x.com/models/classical-physics-ai999x/)

*Gravitational potential energy near the surface*

Every calculation, with its formula, a worked example and its figures: **[Classical Physics on ai999x.com](https://www.ai999x.com/models/classical-physics-ai999x/)**. Install guides for the app and the command line: [https://www.ai999x.com/desktop-cli/](https://www.ai999x.com/desktop-cli/).

Version 1.6.4 · License: Apache-2.0 · Author: Ibnul Jaif Farabi · Organization: AI999X · Contact: support@ai999x.com · [Support the project](https://buymeacoffee.com/ibnuljaiffarabi)

*The Future Is Self-Improving.*

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/classical-physics-ai999x/) (`pip install .` builds it). Releases are tagged `v1.6.4`; the [GitHub release](https://github.com/AI999X/classical-physics-ai999x/releases/tag/v1.6.4) carries the wheel, the desktop app and the command-line program for Windows and Linux, and their checksums.

## Build the program from source

`rust/` holds the Rust source of the `classical-physics` command-line program and browser module: the AI999X engine (`rust/ax-engine`), this model (`rust/classical-physics`) and its data (`rust/models/classical_physics/pack.json`). With Rust 1.80 or later:

```bash
cd rust
cargo build --release -p classical-physics-ai999x
./target/release/classical-physics --help
cargo test -p classical-physics-ai999x
```
