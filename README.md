# classical-physics-ai999x — offline, verified classical physics solver with step-by-step answers

[![tests](https://github.com/AI999X/classical-physics-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/classical-physics-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/classical-physics-ai999x)](https://pypi.org/project/classical-physics-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/classical-physics-ai999x)

**[Install](https://pypi.org/project/classical-physics-ai999x/)** · **[Capability guide](https://huggingface.co/AI999X/classical-physics-ai999x/tree/main/docs)** · **[Evaluation](https://huggingface.co/AI999X/classical-physics-ai999x/tree/main/evaluation)** · **[AI999X on Hugging Face](https://huggingface.co/AI999X)** · **[Website](https://ai999x.com)** · **[Support: Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi)**

License: Apache-2.0 · Author: Ibnul Jaif Farabi (creator and engineer) · Organization: AI999X · Version 1.0.3

Field: **Physical Sciences** · Tasks: Question Answering, Token Classification, Text Classification, Sentence Similarity, Feature Extraction

classical-physics-ai999x is an offline, deterministic Python solver that answers classical physics questions with verified results from 196 certified capabilities, gives 1177 documented answers identically on Windows and Linux, and refuses rather than guesses when an answer cannot be verified.

**classical-physics-ai999x** is a compact, deterministic classical physics engine for Python. It reads a question in plain English (or a typed call), picks a certified classical physics capability, computes the answer exactly, checks it, and returns the result with its steps, units, LaTeX and assumptions — or an honest refusal that says what is missing. It runs fully offline on any CPU, needs no GPU, no API key and no internet, and is built to give **zero wrong answers**: when it is not sure, it refuses instead of guessing.

[![Buy me a coffee](https://img.shields.io/badge/Buy%20me%20a%20coffee-support%20development-FFDD00?logo=buymeacoffee&logoColor=black)](https://buymeacoffee.com/ibnuljaiffarabi) ![License](https://img.shields.io/badge/license-Apache--2.0-blue) ![Python](https://img.shields.io/badge/python-3.10%20%7C%203.11%20%7C%203.12%20%7C%203.13%20%7C%203.14-blue) ![Offline](https://img.shields.io/badge/runs-offline%20on%20CPU-green)

## Highlights

- **Verified, not generated.** Every answer is computed by a certified classical physics capability and checked (substituted back, dimension-checked or compared with an independent computation) before it is returned.
- **Zero wrong answers by design.** Seven explicit outcomes — SOLVED, NEEDS_INPUT, AMBIGUOUS, INVALID_INPUT, OUT_OF_SCOPE, UNRECOGNIZED_FORMAT, INTERNAL_ERROR — never a silent guess. Every one of its 1177 documented answers is reproduced exactly on every tested platform.
- **Plain-English questions and a guaranteed typed interface.** `ask()` reads documented question forms; `solve(capability_id, **inputs)` always solves a supported task with valid inputs.
- **Explains itself.** Steps, formulas in LaTeX, units, assumptions and the model used, ready for notebooks, tutoring apps and homework checkers.
- **Small and portable.** Pure Python (sympy, mpmath); identical answers on Windows and Linux (Python 3.10, 3.11, 3.12, 3.13, 3.14).
- **Coverage:** kinematics, dynamics, energy, momentum, rotation, gravitation, oscillations, waves and sound, optics, thermodynamics, electricity and magnetism.

## Model overview

| Property | Value |
|---|---|
| Package | `classical-physics-ai999x` 1.0.3 (import `classical_physics_ai999x`) |
| Type | Deterministic neuro-symbolic classical physics engine (rule-based reading + certified symbolic capabilities) |
| Capabilities | 46 certified derivations, 16 certified model_familys, 2 certified procedures, 132 certified relations |
| Knowledge pack | version 1.0.2 |
| Golden answers | 1177 documented answers, identical on every tested platform |
| Platforms tested | Windows (Python 3.13), Linux x86-64 (Python 3.10, 3.11, 3.12, 3.13, 3.14) |
| Runtime | CPU only, offline; no GPU, no network, no API key |
| License | Apache-2.0 |
| Creator / engineer | Ibnul Jaif Farabi |
| Organization | AI999X |

## Quickstart

```bash
pip install classical-physics-ai999x
```

```python
import classical_physics_ai999x as classical_physics

classical_physics.ask("A 3 kg object moves at 4 m/s. What is its kinetic energy?").show()
# -> SOLVED — 24 J  (kinetic_energy)
classical_physics.ask("A 2 kg mass on a spring with a spring constant of 50 N/m has a damping coefficient of 4 kg/s. What is the damped angular frequency?").show()
# -> SOLVED — 4.89898 rad/s  (damped_angular_frequency)
classical_physics.ask("A ball is kicked at 20 m/s at 30 degrees above the horizontal from level ground. How far does it travel horizontally?").show()
# -> SOLVED — 35.324 m  (projectile)
classical_physics.ask("What is the escape speed from a planet of mass 5.972e24 kg and radius 6.371e6 m?").show()
# -> SOLVED — 11186 m/s  (escape_speed)
classical_physics.solve("rocket_delta_v", ve="3000 m/s", m0="1000 kg", mf="400 kg").show()
# -> SOLVED — 2748.87 m/s  (rocket_delta_v)
classical_physics.ask("Who created you?").message
# -> classical-physics-ai999x was created and engineered by Ibnul Jaif Farabi at AI999X.
```

Command line: `classical-physics-ai999x "<question>"`, `classical-physics-ai999x --capabilities`. The outputs above were produced by running this exact code on classical-physics-ai999x 1.0.3.

## Find the right capability, and draw it

`classical_physics.search()` finds which certified capability a question is about — across **every** AI999X model, so a question for another model points you to it. It runs **[ai999x-encoder](https://huggingface.co/AI999X/ai999x-encoder)**, AI999X's own capability encoder (4.7M parameters, a 4.9 MB ONNX model, offline; the root `model.safetensors` of this repository): `pip install "classical-physics-ai999x[search]"`. `ask(question, suggest=True)` adds the same pointers to a question it does not answer; an answer itself never changes.

```python
import classical_physics_ai999x as classical_physics

classical_physics.search("Solve x^2 - 5x + 6 = 0", k=2)
# -> Roots of a quadratic equation (math-foundations-ai999x, 0.63); Roots of a cubic equation (math-foundations-ai999x, 0.59)
```

`classical_physics.plot(answer)` draws the answer's formula around its own result — the engine's numbers, as SVG (Jupyter shows it directly; `.figure()` gives a matplotlib figure with the `[plot]` extra, `.plotly()` a Plotly figure).

```python
a = classical_physics.ask("A 3 kg object moves at 4 m/s. What is its kinetic energy?")
classical_physics.plot(a)
# -> Translational kinetic energy: K against m, 101 points, marked at 24 J
```

## How every answer is verified

1. **Reading** — the question is turned into typed quantities (value, unit, kind, role) and one asked quantity; every stated number must be used or the question is refused.
2. **Planning** — only certified capabilities may answer; a capability applies only under the conditions the wording states (for example a pendulum, a liquid at rest, boiling).
3. **Computing** — exact symbolic or high-precision numeric evaluation.
4. **Checking** — dimensions, substitution back into the defining equation, and agreement between independent readings; any disagreement becomes a refusal, never a number.

Capabilities are certified before they ship: each must reproduce hand-worked reference cases, reject a deliberately wrong variant (a control), and leave every benchmark answer unchanged.

## Worked examples

Each question was put to `classical-physics-ai999x`; the answer is the one it returned, with the steps it took.

**Translational kinetic energy** · _Energy_

> A 3 kg object moves at 4 m/s. What is its kinetic energy?

→ **K = 24 J**

<details><summary>Steps</summary>

- Translational kinetic energy: m = 3 kg, v = 4 m/s → K = 24 J

</details>

**Angular frequency of a damped oscillator** · _Oscillations_

> A 2 kg mass on a spring with a spring constant of 50 N/m has a damping coefficient of 4 kg/s. What is the damped angular frequency?

→ **wd = 4.898979486 rad/s**

<details><summary>Steps</summary>

- Angular frequency of a damped oscillator: k = 50 N/m, m = 2 kg, b = 4 kg/s → wd = 4.898979486 rad/s

</details>

**None** · _None_

> A ball is kicked at 20 m/s at 30 degrees above the horizontal from level ground. How far does it travel horizontally?

→ **x = 35.3240058 m**

<details><summary>Steps</summary>

- vertical motion back to the landing height: g = 9.80665 m/s^2, h0 = 0 m, th = 0.5235987756 rad, v0 = 20 m/s → t = 2.039432426 s
- horizontal motion (constant velocity): t = 2.039432426 s, th = 0.5235987756 rad, v0 = 20 m/s → x = 35.3240058 m

</details>

**Escape speed** · _Orbits_

> What is the escape speed from a planet of mass 5.972e24 kg and radius 6.371e6 m?

→ **v = 11185.97789 m/s**

<details><summary>Steps</summary>

- Escape speed: G = 6.6743e-11 N*m^2/kg^2, M = 5.972e+24 kg, r = 6371000 m → v = 11185.97789 m/s

</details>

**Velocity change of a rocket (Tsiolkovsky)** · _Variable mass_

`m.solve("rocket_delta_v", ve="3000 m/s", m0="1000 kg", mf="400 kg")`

→ **dv = 2748.872196 m/s**

<details><summary>Steps</summary>

- Velocity change of a rocket (Tsiolkovsky): ve = 3000 m/s, m0 = 1000 kg, mf = 400 kg → dv = 2748.872196 m/s

</details>

**Reactance of a capacitor** · _AC circuits_

> A capacitor with a capacitance of 1 uF is in an AC circuit at a frequency of 60 Hz. What is its reactance?

→ **X = 2652.582385 ohm**

<details><summary>Steps</summary>

- Reactance of a capacitor: f = 60 Hz, C = 1e-06 F → X = 2652.582385 ohm

</details>

## Figures

Every point was computed by `classical-physics-ai999x`.

**Projectile paths and velocity vectors**

![Projectile paths and velocity vectors — computed by classical-physics-ai999x](https://huggingface.co/AI999X/classical-physics-ai999x/resolve/main/figures/displacement-time-drawing.svg)

_Trajectories of a projectile launched at 20 m/s (g = 9.81 m/s², no air resistance) at three angles: each point's horizontal distance and height computed by the package (uniform motion and constant acceleration), the top from its maximum-height formula, and on the 45° path the velocity vectors (horizontal part constant, vertical part from v = v₀ + a t) — θ = 30°: range 35.3119 m, top 5.09684 m; θ = 45°: range 40.7747 m, top 10.1937 m; θ = 60°: range 35.3119 m, top 15.2905 m._

**Forces on a block on an incline**

![Forces on a block on an incline — computed by classical-physics-ai999x](https://huggingface.co/AI999X/classical-physics-ai999x/resolve/main/figures/normal-force-incline-diagram.svg)

_Forces on a 2 kg block on a frictionless 30° incline (g = 9.81 m/s²), each computed by the package and drawn to scale (0.12 length units per newton): weight 19.62 N, normal force 16.9914 N = m g cos θ, component along the slope 9.81 N = m g sin θ, which accelerates the block at 4.905 m/s²._

**Energy conservation in a free fall**

![Energy conservation in a free fall — computed by classical-physics-ai999x](https://huggingface.co/AI999X/classical-physics-ai999x/resolve/main/figures/kinetic-energy-bars.svg)

_Energy of a 1 kg body falling from rest from 20 m (g = 9.81 m/s², no air resistance): potential energy and kinetic energy (from the speed after the fall), each computed by the package, stacked — their sum stays 196.2 J: h = 20 m: U = 196.2 J, K = 0 J (v = 0 m/s); h = 15 m: U = 147.15 J, K = 49.05 J (v = 9.90454 m/s); h = 10 m: U = 98.1 J, K = 98.1 J (v = 14.0071 m/s); h = 5 m: U = 49.05 J, K = 147.15 J (v = 17.1552 m/s); h = 0 m: U = 0 J, K = 196.2 J (v = 19.8091 m/s)._

**Resonance curves of a driven oscillator**

![Resonance curves of a driven oscillator — computed by classical-physics-ai999x](https://huggingface.co/AI999X/classical-physics-ai999x/resolve/main/figures/driven-amplitude-curve.svg)

_Steady-state amplitude of a driven damped oscillator (F₀ = 1 N, k = 4 N/m, m = 1 kg) against the driving frequency for four damping constants, 64 frequencies each from the package; dots: the amplitude-resonance frequency it computes — b = 0.25: ω_r = 1.99217 rad/s, A = 2.00392 m; b = 0.5: ω_r = 1.9685 rad/s, A = 1.00791 m; b = 1: ω_r = 1.87083 rad/s, A = 0.516398 m; b = 2: ω_r = 1.41421 rad/s, A = 0.288675 m._

**An RC circuit charging and discharging**

![An RC circuit charging and discharging — computed by classical-physics-ai999x](https://huggingface.co/AI999X/classical-physics-ai999x/resolve/main/figures/rc-charging-curve.svg)

_Capacitor voltage in an RC circuit (R = 1000 Ω, C = 0.001 F, source 10 V) charging from 0 and discharging from 10 V, 50 times each from the package; time constant τ = 1 s; marked: t = τ, charging voltage 6.32121 V (63.2121 % of the source)._

## Capabilities

46 certified derivations, 16 certified model_familys, 2 certified procedures, 132 certified relations. A few of them:

- **Derivations:** acceleration with friction level; angular acceleration from tangential; angular frequency period; average speed equal instantaneous; centripetal acceleration from period; derive absolute pressure depth; derive acceleration frictionless incline; derive centripetal acceleration omega; derive centripetal force; derive charge current; derive cyclotron radius; derive electric power; derive impulse; derive inductive reactance; derive lc resonance; derive long wire field; derive normal on level; derive ohm power current; derive ohm power voltage; derive orbital period; derive photon energy; derive photon energy wavelength; derive photon momentum; derive rl time constant …
- **Model_familys:** A gas between two states; Apparent weight in an elevator; Block on a surface pulled over a pulley; Collisions along a line; Doppler effect for sound; Ideal banking of a curve; Maximum speed on a flat curve; Mixing to thermal equilibrium; Motion in stages; Projectile motion; RC circuits; Radioactive decay; Recoil from rest; Rolling from a height; Series and parallel resistor and capacitor networks; Two masses over a pulley
- **Procedures:** Two resistors in series and in parallel; Velocity and displacement after constant acceleration
- **Relations:** Absolute pressure at a depth in a fluid; Acceleration down a frictionless incline; Acceleration of a body rolling without slipping down an incline; Acceleration of an Atwood machine with a massive pulley; Amplitude-resonance frequency of a driven damped oscillator; Angular frequency and frequency; Angular frequency of a damped oscillator; Angular momentum of a point mass on a circle; Angular momentum of a rotating body; Angular velocity and angle turned under constant angular acceleration; Angular velocity under constant angular acceleration; Average net force from a change of momentum; Average power; Body-frame precession rate of a torque-free symmetric body; Capacitance of parallel plates; Carnot (maximum) efficiency; Centripetal acceleration; Centripetal acceleration from angular velocity; Centripetal force; Charge carried by a steady current; Charge on a capacitor; Coefficient of performance of a heat pump; Coefficient of performance of a refrigerator; Component of the weight along an incline …

`classical_physics_ai999x.templates()` lists every question form `ask()` reads; the capability guide (PDF, every example executed) is included in this repository.

## Out of scope

- Open-ended explanation, essay writing or tutoring dialogue: it is a solver, not a chatbot.
- Questions outside its certified classical physics capabilities, and any question whose stated numbers it cannot all use — it refuses with the reason instead.
- High-stakes engineering, medical or legal decisions without independent review.

## Limitations

- It answers what its certified capabilities cover (kinematics, dynamics, energy, momentum, rotation, gravitation, oscillations, waves and sound, optics, thermodynamics, electricity and magnetism); anything else is refused with a reason.
- `ask()` reads documented question forms and common phrasings; unusual wording may be refused — use `solve()` for guaranteed results.
- Results are as correct as the certified relations and their stated conditions; a certified relation is a checked model, not a claim about every physical situation.

## Citation

```bibtex
@software{classical_physics_ai999x_1_0_3,
  title  = {classical-physics-ai999x: an offline, verified classical physics solver},
  author = {Farabi, Ibnul Jaif}, organization = {AI999X},
  year   = {2026}, version = {1.0.3},
  url    = {https://huggingface.co/AI999X/classical-physics-ai999x}
}
```

## Contact

Questions, bug reports and requests: open a discussion on https://huggingface.co/AI999X/classical-physics-ai999x/discussions or visit https://ai999x.com.

## Support the project

classical-physics-ai999x is free and open source. If it helps you, you can support further development on [Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi).

Created and engineered by **Ibnul Jaif Farabi** · **[AI999X](https://huggingface.co/AI999X)** · [ai999x.com](https://ai999x.com)

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/classical-physics-ai999x/) (`pip install .` builds it). Releases are tagged `v<version>`; the wheel is attached to each [GitHub release](https://github.com/AI999X/classical-physics-ai999x/releases). The optional ONNX models (formula graphs and, where measured, a decision model) are on [Hugging Face](https://huggingface.co/AI999X/classical-physics-ai999x/tree/main/onnx).
