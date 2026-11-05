# classical-physics-ai999x: offline, verified classical physics solver with step-by-step answers

[![tests](https://github.com/AI999X/classical-physics-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/classical-physics-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/classical-physics-ai999x)](https://pypi.org/project/classical-physics-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/classical-physics-ai999x)

**[Desktop app & command line](https://www.ai999x.com/desktop-cli/)** · **[Compute in your browser](https://www.ai999x.com/models/classical-physics-ai999x/#try)** · **[Install (Python)](https://pypi.org/project/classical-physics-ai999x/)** · **[Guides](https://huggingface.co/AI999X/classical-physics-ai999x/tree/main/docs)** · **[Evaluation](https://huggingface.co/AI999X/classical-physics-ai999x/tree/main/evaluation)** · **[AI999X on Hugging Face](https://huggingface.co/AI999X)** · **[Website](https://ai999x.com)** · **[Support: Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi)**

License: Apache-2.0 · Author: Ibnul Jaif Farabi (creator and engineer) · Organization: AI999X · Version 1.5.0

Field: **Physical Sciences** · Tasks: Question Answering

classical-physics-ai999x is an offline, deterministic classical physics solver written in Rust: a desktop app, a command-line program, a browser module and a Python package. It answers classical physics questions with verified results from 196 certified capabilities, gives 1177 documented answers identically on Windows and Linux, and refuses rather than guesses when an answer cannot be verified.

**classical-physics-ai999x** reads a question in plain English (or a typed call), picks a certified classical physics capability, computes the answer exactly, checks it, and returns the result with its steps, units, LaTeX and assumptions, or a refusal that says what is missing. It runs fully offline on any CPU, needs no GPU, no API key and no internet, and is built to give **zero wrong answers**: when it is not sure, it refuses instead of guessing.

[![Buy me a coffee](https://img.shields.io/badge/Buy%20me%20a%20coffee-support%20development-FFDD00?logo=buymeacoffee&logoColor=black)](https://buymeacoffee.com/ibnuljaiffarabi) ![License](https://img.shields.io/badge/license-Apache--2.0-blue) ![Python](https://img.shields.io/badge/python-3.10%20%7C%203.11%20%7C%203.12%20%7C%203.13%20%7C%203.14-blue) ![Offline](https://img.shields.io/badge/runs-offline%20on%20CPU-green) ![Rust](https://img.shields.io/badge/Rust-Windows%20%7C%20Linux%20%7C%20WebAssembly-orange)

**Run it your way.** classical-physics-ai999x comes as the AI999X desktop app and as the command-line program `classical-physics` for Windows and Linux, runs in your browser as WebAssembly, and installs as a Python package. The desktop app, the command line and the browser run the same compiled Rust program on your own device: offline, no Python needed.

## Get it

| Way to run it | For | Get it |
|---|---|---|
| **AI999X Classical Physics desktop app** 1.5.0 | Windows, Linux. This model in its own app: search, ask in words, forms, live figures, explanations, offline | [Windows (x86-64)](https://downloads.ai999x.com/desktop/1.5.0/ai999x-classical-physics-1.5.0-windows-x86_64.zip) · [Linux (x86-64)](https://downloads.ai999x.com/desktop/1.5.0/ai999x-classical-physics-1.5.0-linux-x86_64.tar.gz) |
| **AI999X desktop app** 1.5.0 | Windows, Linux. Every AI999X model in one app: search, ask in words, forms, live figures, explanations, offline | [Windows (x86-64)](https://downloads.ai999x.com/desktop/1.5.0/ai999x-1.5.0-windows-x86_64.zip) · [Linux (x86-64)](https://downloads.ai999x.com/desktop/1.5.0/ai999x-1.5.0-linux-x86_64.tar.gz) |
| **Command line** (`classical-physics`) | Windows, Linux. One program per model, offline, no Python | [Windows (x86-64)](https://downloads.ai999x.com/desktop/1.5.0/ai999x-cli-1.5.0-windows-x86_64.zip) · [Linux (x86-64)](https://downloads.ai999x.com/desktop/1.5.0/ai999x-cli-1.5.0-linux-x86_64.tar.gz) |
| **In your browser** | any device. Nothing to install | [Compute on the model page](https://www.ai999x.com/models/classical-physics-ai999x/#try) |
| **Python** | Python 3.10+ | `pip install classical-physics-ai999x` |

Checksums: [SHA256SUMS](https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS) · every version: [https://downloads.ai999x.com/desktop/latest/](https://downloads.ai999x.com/desktop/latest/) · install guide: [desktop & command line](https://www.ai999x.com/desktop-cli/)

## Quick start

### Desktop app

Download **AI999X Classical Physics** for your system (above), unpack it and start it; type a question in words or pick a capability, fill in the form and press Compute. Figures and an explanation of the answer appear beside it. The AI999X desktop app has the same, with Classical Physics among all AI999X models.

Linux, from a terminal:

```bash
curl -LO https://downloads.ai999x.com/desktop/1.5.0/ai999x-classical-physics-1.5.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-classical-physics-1.5.0-linux-x86_64.tar.gz
./ai999x-classical-physics-1.5.0-linux-x86_64/ai999x-classical-physics
```

### Command line

Linux:

```bash
curl -LO https://downloads.ai999x.com/desktop/1.5.0/ai999x-cli-1.5.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
mkdir -p ~/.local/share && tar xzf ai999x-cli-1.5.0-linux-x86_64.tar.gz -C ~/.local/share
echo 'export PATH="$HOME/.local/share/ai999x-cli-1.5.0-linux-x86_64:$PATH"' >> ~/.bashrc && source ~/.bashrc
classical-physics --version
```

Windows (PowerShell):

```powershell
$F = "ai999x-cli-1.5.0-windows-x86_64.zip"
Invoke-WebRequest "https://downloads.ai999x.com/desktop/1.5.0/$F" -OutFile $F
Invoke-WebRequest "https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS" -OutFile SHA256SUMS
(Get-FileHash $F -Algorithm SHA256).Hash.ToLower() -eq ((Select-String -SimpleMatch $F SHA256SUMS).Line -split ' ')[0]
Expand-Archive $F -DestinationPath "$env:LOCALAPPDATA\AI999X" -Force
$D = "$env:LOCALAPPDATA\AI999X\ai999x-cli-1.5.0-windows-x86_64"; $env:Path += ";$D"
[Environment]::SetEnvironmentVariable("Path", [Environment]::GetEnvironmentVariable("Path", "User") + ";$D", "User")
classical-physics --version
```

Every command below was run as shown; the text under it is what the program printed.

**Ask in words**: Write the question as you would say it: the program reads the quantities and their units, picks the capability and answers.

```console
$ classical-physics ask "A 3 kg object moves at 4 m/s. What is its kinetic energy?"
Read as: Translational kinetic energy
kinetic energy (K) = 24 J
  by: Translational kinetic energy  (K = m*v**2/2)
  m = 3 kg   [3 kg]
  v = 4 m/s   [4 m/s]
  assumes: speeds far below the speed of light
```

**What a capability needs**: The equation or method, every input and output with its SI unit. Read this before solve: it tells you the input names.

```console
$ classical-physics info two_resistors_series_parallel
Two resistors in series and in parallel
  series resistances add; parallel conductances add
  in   r1       first resistance [ohm]
  in   r2       second resistance [ohm]
  out  rs       series equivalent [ohm]
  out  rp       parallel equivalent [ohm]
```

**Solve**: Inputs as name=value, each with any unit of the right kind. The answer lists every output, the method used, each input converted to SI, and the conditions the answer assumes.

```console
$ classical-physics solve two_resistors_series_parallel "r1=6 ohm" "r2=3 ohm"
rs = 9 ohm
rp = 2 ohm
  by: Two resistors in series and in parallel  (series resistances add; parallel conductances add)
  r1 = 6 ohm
  r2 = 3 ohm
  assumes: r1 > 0; r2 > 0
```

**Many questions at once**: One JSON request per line in, one JSON answer per line out, in order: for scripts, notebooks and agents (the program starts once). In PowerShell: Get-Content cases.jsonl | classical-physics batch

`cases.jsonl`:

```json
{"key": "two_resistors_series_parallel", "inputs": {"r1": "6 ohm", "r2": "3 ohm"}}
{"key": "rocket_delta_v", "inputs": {"ve": "3000 m/s", "m0": "1000 kg", "mf": "400 kg"}}
```

```console
$ classical-physics batch < cases.jsonl
{"outcome":"SOLVED","message":"","value":9.0,"unit":"ohm","solved_for":"rs","si":[["r1",6.0],["r2",3.0]],"values":[["rs",9.0,"ohm"],["rp",2.0,"ohm"]],"target":"rs","phrase":"series equivalent","assumptions":["r1 > 0","r2 > 0"],"name":"Two resistors in series and in parallel","equation":"series resistances add; parallel conductances add"}
{"outcome":"SOLVED","message":"","value":2748.8721956224654,"unit":"m/s","solved_for":"dv","si":[["ve",3000.0],["m0",1000.0],["mf",400.0]],"target":"dv","phrase":"change in velocity","assumptions":["no gravity or drag during the burn","a constant exhaust velocity","the situation is: rocket"],"name":"Velocity change of a rocket (Tsiolkovsky)","equation":"dv = ve*log(m0/mf)"}
```

### In your browser

Open [Classical Physics on ai999x.com](https://www.ai999x.com/models/classical-physics-ai999x/#try), choose a capability and press **Compute**: the model runs on your own device as WebAssembly; nothing is sent anywhere. Change any input (any unit of the right kind) and the answer, its explanation and the figures follow.

In your own page: `classical_physics_ai999x.wasm` and the loader `ax-wasm.js` (https://www.ai999x.com/compute/rust/ax-wasm.js).

### Python

```bash
pip install classical-physics-ai999x
```

## Worked examples in every form

The same worked example from the command line, from Python and in the browser. Each command-line output is what the program printed.

### Reactance of a capacitor

Command line:

```console
$ classical-physics solve capacitive_reactance "f=60 Hz" "C=1e-06 F"
capacitive reactance (X) = 2652.582385 ohm
  by: Reactance of a capacitor  (X = 1/(2*pi*f*C))
  f = 60 Hz   [60 Hz]
  C = 1e-06 F   [1e-06 F]
  assumes: sinusoidal AC; the situation is: capacitor
```

Python:

```python
import classical_physics_ai999x as m

a = m.solve("capacitive_reactance", f="60 Hz", C="1e-06 F")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/classical-physics-ai999x/#cap-capacitive_reactance) and press **Compute in your browser**.

_What this means:_ This calculation determines the capacitive reactance, which represents the opposition a capacitor offers to alternating current. The result 2652.58 ohm is derived using the frequency 60 Hz and the capacitance 1e-06 F. A lower value indicates that the capacitor allows more current to flow at this specific frequency, while a higher value suggests greater resistance to the signal.

### Reactance of an inductor

Command line:

```console
$ classical-physics solve inductive_reactance "f=60 Hz" "L=0.03 H"
inductive reactance (X) = 11.30973355 ohm
  by: Reactance of an inductor  (X = 2*pi*f*L)
  f = 60 Hz   [60 Hz]
  L = 0.03 H   [0.03 H]
  assumes: sinusoidal AC; the situation is: inductor
```

Python:

```python
import classical_physics_ai999x as m

a = m.solve("inductive_reactance", f="60 Hz", L="0.03 H")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/classical-physics-ai999x/#cap-inductive_reactance) and press **Compute in your browser**.

_What this means:_ This calculation determines the opposition to alternating current provided by an inductor. Using the frequency of the signal and the inductance value, the result shows that the inductive reactance equals 11.3097 ohm. This value indicates how much the component resists changes in current flow within the circuit.

### Resonant frequency of an LC circuit

Command line:

```console
$ classical-physics solve lc_resonance "L=0.01 H" "C=1e-06 F"
resonant frequency (f) = 1591.549431 Hz
  by: Resonant frequency of an LC circuit  (f = 1/(2*pi*sqrt(L*C)))
  L = 0.01 H   [0.01 H]
  C = 1e-06 F   [1e-06 F]
  assumes: ideal components; negligible resistance for the frequency; the situation is: lc circuit
```

Python:

```python
import classical_physics_ai999x as m

a = m.solve("lc_resonance", L="0.01 H", C="1e-06 F")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/classical-physics-ai999x/#cap-lc_resonance) and press **Compute in your browser**.

_What this means:_ This calculation determines the resonant frequency of an LC circuit, which is the rate at which energy oscillates between the inductor and the capacitor. Given an inductance of 0.01 H and a capacitance of 1e-06 F, the resulting frequency is 1591.55 Hz. This value represents the natural tuning point where the circuit responds most strongly to alternating current signals.

### Rms current of a sinusoidal supply

Command line:

```console
$ classical-physics solve rms_current "I0=10 A"
rms current (Irms) = 7.071067812 A
  by: Rms current of a sinusoidal supply  (Irms = I0/sqrt(2))
  I0 = 10 A   [10 A]
  assumes: a sinusoidal waveform; the situation is: alternating current
```

Python:

```python
import classical_physics_ai999x as m

a = m.solve("rms_current", I0="10 A")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/classical-physics-ai999x/#cap-rms_current) and press **Compute in your browser**.

_What this means:_ This calculation determines the effective current value for a sinusoidal alternating supply. Taking the peak current, which is 10 A, and dividing it by the square root of two gives the root mean square current of 7.07107 A. This specific value represents the amount of direct current that would produce the same heating effect in a resistor as this alternating current.

## Documentation

- **Rust user guide**: the command line, every capability with a command and what it printed: [PDF](https://huggingface.co/AI999X/classical-physics-ai999x/blob/main/docs/classical_physics_ai999x_rust_guide.pdf) · [Markdown](https://huggingface.co/AI999X/classical-physics-ai999x/blob/main/docs/classical_physics_ai999x_rust_guide.md)
- **Python guide**: every capability with a Python example: [PDF](https://huggingface.co/AI999X/classical-physics-ai999x/blob/main/docs/classical_physics_ai999x_capabilities.pdf) · [Markdown](https://huggingface.co/AI999X/classical-physics-ai999x/blob/main/docs/classical_physics_ai999x_capabilities.md)
- **Desktop app & command line**: downloads, install, ask and search, figures, offline use, checksums: https://www.ai999x.com/desktop-cli/
- **Model page**: compute in your browser, every capability explained: https://www.ai999x.com/models/classical-physics-ai999x/

## Highlights

- **Verified, not generated.** Every answer is computed by a certified classical physics capability and checked (substituted back, dimension-checked or compared with an independent computation) before it is returned.
- **Zero wrong answers by design.** Seven explicit outcomes (SOLVED, NEEDS_INPUT, AMBIGUOUS, INVALID_INPUT, OUT_OF_SCOPE, UNRECOGNIZED_FORMAT, INTERNAL_ERROR), never a silent guess. Every one of its 1177 documented answers is reproduced exactly on every tested platform.
- **Plain-English questions and a guaranteed typed interface.** `ask()` reads documented question forms; `solve(capability_id, **inputs)` always solves a supported task with valid inputs.
- **Explains itself.** Steps, formulas in LaTeX, units, assumptions and the model used, ready for notebooks, tutoring apps and homework checkers.
- **Native and in the browser.** Every capability also computes natively in Rust: the desktop app and the `classical-physics` command line for Windows and Linux, and WebAssembly on the model page, no Python, no network.
- **Small and portable.** Pure Python (sympy, mpmath); identical answers on Windows and Linux (Python 3.10, 3.11, 3.12, 3.13, 3.14).
- **Coverage:** kinematics, dynamics, energy, momentum, rotation, gravitation, oscillations, waves and sound, optics, thermodynamics, electricity and magnetism.

## Model overview

| Property | Value |
|---|---|
| Package | `classical-physics-ai999x` 1.5.0 (import `classical_physics_ai999x`) |
| Type | Deterministic neuro-symbolic classical physics engine (rule-based reading + certified symbolic capabilities) |
| Capabilities | 46 certified derivations, 16 certified model_familys, 2 certified procedures, 132 certified relations |
| Knowledge pack | version 1.0.2 |
| Golden answers | 1177 documented answers, identical on every tested platform |
| Platforms tested | Windows (Python 3.13), Linux x86-64 (Python 3.10, 3.11, 3.12, 3.13, 3.14) |
| Runtime | CPU only, offline; no GPU, no network, no API key |
| License | Apache-2.0 |
| Creator / engineer | Ibnul Jaif Farabi |
| Organization | AI999X |

## Python quick start

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

The Python package also installs a Python command, `classical-physics-ai999x "<question>"`; for the native program use `classical-physics` (above). The outputs above were produced by running this exact code on classical-physics-ai999x 1.5.0.

## Draw the answer

`classical_physics.plot(answer)` draws the answer's formula around its own result: the engine's numbers, as SVG (Jupyter shows it directly; `.figure()` gives a matplotlib figure with the `[plot]` extra, `.plotly()` a Plotly figure).

```python
a = classical_physics.ask("A 3 kg object moves at 4 m/s. What is its kinetic energy?")
classical_physics.plot(a)
# -> Translational kinetic energy: K against m, 101 points, marked at 24 J
```

## How every answer is verified

1. **Reading**: the question is turned into typed quantities (value, unit, kind, role) and one asked quantity; every stated number must be used or the question is refused.
2. **Planning**: only certified capabilities may answer; a capability applies only under the conditions the wording states (for example a pendulum, a liquid at rest, boiling).
3. **Computing**: exact symbolic or high-precision numeric evaluation.
4. **Checking**: dimensions, substitution back into the defining equation, and agreement between independent readings; any disagreement becomes a refusal, never a number.

Capabilities are certified before they ship: each must reproduce hand-worked reference cases, reject a deliberately wrong variant (a control), and leave every benchmark answer unchanged.

## Python worked examples, with their steps

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

![Projectile paths and velocity vectors, computed by classical-physics-ai999x](https://huggingface.co/AI999X/classical-physics-ai999x/resolve/main/figures/displacement-time-drawing.svg)

_Trajectories of a projectile launched at 20 m/s (g = 9.81 m/s², no air resistance) at three angles: each point's horizontal distance and height computed by the package (uniform motion and constant acceleration), the top from its maximum-height formula, and on the 45° path the velocity vectors (horizontal part constant, vertical part from v = v₀ + a t), θ = 30°: range 35.3119 m, top 5.09684 m; θ = 45°: range 40.7747 m, top 10.1937 m; θ = 60°: range 35.3119 m, top 15.2905 m._

**Forces on a block on an incline**

![Forces on a block on an incline, computed by classical-physics-ai999x](https://huggingface.co/AI999X/classical-physics-ai999x/resolve/main/figures/normal-force-incline-diagram.svg)

_Forces on a 2 kg block on a frictionless 30° incline (g = 9.81 m/s²), each computed by the package and drawn to scale (0.12 length units per newton): weight 19.62 N, normal force 16.9914 N = m g cos θ, component along the slope 9.81 N = m g sin θ, which accelerates the block at 4.905 m/s²._

**Energy conservation in a free fall**

![Energy conservation in a free fall, computed by classical-physics-ai999x](https://huggingface.co/AI999X/classical-physics-ai999x/resolve/main/figures/kinetic-energy-bars.svg)

_Energy of a 1 kg body falling from rest from 20 m (g = 9.81 m/s², no air resistance): potential energy and kinetic energy (from the speed after the fall), each computed by the package, stacked, their sum stays 196.2 J: h = 20 m: U = 196.2 J, K = 0 J (v = 0 m/s); h = 15 m: U = 147.15 J, K = 49.05 J (v = 9.90454 m/s); h = 10 m: U = 98.1 J, K = 98.1 J (v = 14.0071 m/s); h = 5 m: U = 49.05 J, K = 147.15 J (v = 17.1552 m/s); h = 0 m: U = 0 J, K = 196.2 J (v = 19.8091 m/s)._

**Resonance curves of a driven oscillator**

![Resonance curves of a driven oscillator, computed by classical-physics-ai999x](https://huggingface.co/AI999X/classical-physics-ai999x/resolve/main/figures/driven-amplitude-curve.svg)

_Steady-state amplitude of a driven damped oscillator (F₀ = 1 N, k = 4 N/m, m = 1 kg) against the driving frequency for four damping constants, 64 frequencies each from the package; dots: the amplitude-resonance frequency it computes, b = 0.25: ω_r = 1.99217 rad/s, A = 2.00392 m; b = 0.5: ω_r = 1.9685 rad/s, A = 1.00791 m; b = 1: ω_r = 1.87083 rad/s, A = 0.516398 m; b = 2: ω_r = 1.41421 rad/s, A = 0.288675 m._

**An RC circuit charging and discharging**

![An RC circuit charging and discharging, computed by classical-physics-ai999x](https://huggingface.co/AI999X/classical-physics-ai999x/resolve/main/figures/rc-charging-curve.svg)

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
- Questions outside its certified classical physics capabilities, and any question whose stated numbers it cannot all use: it refuses with the reason instead.
- High-stakes engineering, medical or legal decisions without independent review.

## Limitations

- It answers what its certified capabilities cover (kinematics, dynamics, energy, momentum, rotation, gravitation, oscillations, waves and sound, optics, thermodynamics, electricity and magnetism); anything else is refused with a reason.
- `ask()` reads documented question forms and common phrasings; unusual wording may be refused; use `solve()` for guaranteed results.
- Results are as correct as the certified relations and their stated conditions; a certified relation is a checked model, not a claim about every physical situation.

## Citation

```bibtex
@software{classical_physics_ai999x_1_5_0,
  title  = {classical-physics-ai999x: an offline, verified classical physics solver},
  author = {Farabi, Ibnul Jaif}, organization = {AI999X},
  year   = {2026}, version = {1.5.0},
  url    = {https://huggingface.co/AI999X/classical-physics-ai999x}
}
```

## Contact

Questions, bug reports and requests: open a discussion on https://huggingface.co/AI999X/classical-physics-ai999x/discussions or visit https://ai999x.com.

## Support the project

classical-physics-ai999x is free and open source. If it helps you, you can support further development on [Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi).

Created and engineered by **Ibnul Jaif Farabi** · **[AI999X](https://huggingface.co/AI999X)** · [ai999x.com](https://ai999x.com)

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/classical-physics-ai999x/) (`pip install .` builds it). Releases are tagged `v1.5.0`; the [GitHub release](https://github.com/AI999X/classical-physics-ai999x/releases/tag/v1.5.0) carries the wheel, the desktop app and the command-line program for Windows and Linux, and their checksums.

## Build the program from source

`rust/` holds the Rust source of the `classical-physics` command-line program and browser module: the AI999X engine (`rust/ax-engine`), this model (`rust/classical-physics`) and its data (`rust/models/classical_physics/pack.json`). With Rust 1.80 or later:

```bash
cd rust
cargo build --release -p classical-physics-ai999x
./target/release/classical-physics --help
cargo test -p classical-physics-ai999x
```
