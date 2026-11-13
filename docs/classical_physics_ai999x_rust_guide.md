# classical-physics: the Classical Physics program, user guide

**classical-physics-ai999x 1.6.0**, Rust build: command line, desktop app and browser. **Creator / engineer:** Ibnul Jaif Farabi · **Company:** AI999X · support@ai999x.com

Classical Physics covers kinematics, dynamics, energy, momentum, rotation, gravitation, oscillations, waves and sound, optics, thermodynamics, electricity and magnetism. This guide is about `classical-physics`, the model as a self-contained program for Windows and Linux: 162 capabilities in 37 topics, answered on your own computer with no network, no Python and nothing else to install. The same compiled model runs in the AI999X desktop app and in the browser on the model's page (https://www.ai999x.com/models/classical-physics-ai999x/); the Python package (`pip install classical-physics-ai999x`) has its own guide. Every command output printed here is what the program printed.

## Contents

- 1. Get it: desktop app, command line, browser, Python
- 2. Quick start
- 3. Writing inputs
- 4. Outcomes and exit codes
- 5. Scripts and agents: batch and JSON
- 6. Figures
- 7. In a web page or Node.js: WebAssembly
- 8. Every capability

## 1. Get it

Version 1.5.0 of the AI999X desktop app and of the command-line bundle. Checksums for every file: https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS; the newest version is always under https://downloads.ai999x.com/desktop/latest/. Full instructions: https://www.ai999x.com/desktop-cli/

| what | system | download |
|---|---|---|
| Desktop app: Classical Physics | Windows x86-64 | https://downloads.ai999x.com/desktop/1.5.0/ai999x-classical-physics-1.5.0-windows-x86_64.zip |
| Desktop app: Classical Physics | Linux x86-64 | https://downloads.ai999x.com/desktop/1.5.0/ai999x-classical-physics-1.5.0-linux-x86_64.tar.gz |
| Desktop app: all models | Windows x86-64 | https://downloads.ai999x.com/desktop/1.5.0/ai999x-1.5.0-windows-x86_64.zip |
| Desktop app: all models | Linux x86-64 | https://downloads.ai999x.com/desktop/1.5.0/ai999x-1.5.0-linux-x86_64.tar.gz |
| Command line | Windows x86-64 | https://downloads.ai999x.com/desktop/1.5.0/ai999x-cli-1.5.0-windows-x86_64.zip |
| Command line | Linux x86-64 | https://downloads.ai999x.com/desktop/1.5.0/ai999x-cli-1.5.0-linux-x86_64.tar.gz |

### Desktop app

Classical Physics in its own app: search, questions in words, input forms, live figures and explanations. Windows: Download ai999x-classical-physics-1.5.0-windows-x86_64.zip, right-click it → Extract All, then open the folder and start ai999x-classical-physics.exe. If Windows SmartScreen asks, choose More info → Run anyway. Linux:

```shell
curl -LO https://downloads.ai999x.com/desktop/1.5.0/ai999x-classical-physics-1.5.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-classical-physics-1.5.0-linux-x86_64.tar.gz
./ai999x-classical-physics-1.5.0-linux-x86_64/ai999x-classical-physics
```

It opens on this model: type a question or pick a capability, fill in the form and press Compute. The AI999X desktop app with every model works the same way (choose the model first). Windows: Download ai999x-1.5.0-windows-x86_64.zip, right-click it → Extract All, then open the folder and start ai999x.exe. If Windows SmartScreen asks, choose More info → Run anyway. Linux:

```shell
curl -LO https://downloads.ai999x.com/desktop/1.5.0/ai999x-1.5.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-1.5.0-linux-x86_64.tar.gz
./ai999x-1.5.0-linux-x86_64/ai999x
```

### Command line: Linux

The bundle holds one program per model; `classical-physics` is this one. The checksum line should print OK.

```shell
curl -LO https://downloads.ai999x.com/desktop/1.5.0/ai999x-cli-1.5.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
mkdir -p ~/.local/share && tar xzf ai999x-cli-1.5.0-linux-x86_64.tar.gz -C ~/.local/share
echo 'export PATH="$HOME/.local/share/ai999x-cli-1.5.0-linux-x86_64:$PATH"' >> ~/.bashrc && source ~/.bashrc
classical-physics --version
```

### Command line: Windows (PowerShell)

The fourth line should print True.

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

### In your browser

Open https://www.ai999x.com/models/classical-physics-ai999x/, choose a capability and press Compute in your browser: the same program runs on your device as WebAssembly; nothing is sent anywhere.

### Python

```shell
pip install classical-physics-ai999x
```

## 2. Quick start

### 2.1 Check the install

Prints the model, its version and that this is the Rust build.

```shell
$ classical-physics --version
classical-physics-ai999x 1.6.0 (Rust build, ax-engine 1.6.0)
```

### 2.2 Ask in words

Write the question as you would say it: the program reads the quantities and their units, picks the capability and answers.

```shell
$ classical-physics ask "A 3 kg object moves at 4 m/s. What is its kinetic energy?"
Read as: Translational kinetic energy
kinetic energy (K) = 24 J
  by: Translational kinetic energy  (K = m*v**2/2)
  m = 3 kg   [3 kg]
  v = 4 m/s   [4 m/s]
  assumes: speeds far below the speed of light
```

### 2.3 Find a capability

Search the capabilities by what you want to know; the first column is the id to pass to info and solve.

```shell
$ classical-physics search "series rlc circuit on an ac supply: impedance, current, phase and power"
series_rlc_ac_circuit                Series RLC circuit on an AC supply: impedance, current, phase and power  (AC circuits / series RLC)
rms_current                          Rms current of a sinusoidal supply  (ac circuits)
ohm_power_current                    Power from current and resistance  (circuits)
rms_voltage                          Rms voltage of a sinusoidal supply  (ac circuits)
two_resistors_series_parallel        Two resistors in series and in parallel  (circuits)
rl_current_decay                     Current dying away in an RL circuit  (induction)
ohm_power_voltage                    Power from voltage and resistance  (circuits)
electric_power                       Electric power  (electricity)
… 12 more
```

### 2.4 List the capabilities

Every capability the program answers: its id (what you pass to solve) and its name.

```shell
$ classical-physics list
uniform_motion                           Motion at constant velocity
velocity_time                            Velocity under constant acceleration
displacement_time                        Displacement under constant acceleration
velocity_displacement                    Speed after a displacement under constant acceleration
average_velocity_displacement            Displacement from the mean of initial and final velocity
newton_second                            Net force and acceleration
weight                                   Weight in a uniform gravitational field
net_force_friction                       Net force with an opposing friction force
… 154 more
```

### 2.5 What a capability needs

The equation or method, every input and output with its SI unit. Read this before solve: it tells you the input names.

```shell
$ classical-physics info series_rlc_ac_circuit
Series RLC circuit on an AC supply: impedance, current, phase and power
  XL = 2 pi f L, XC = 1/(2 pi f C), Z = sqrt(R^2 + (XL - XC)^2), I = V/Z, tan(phi) = (XL - XC)/R, P = I^2 R
  in   R        resistance [ohm]
  in   L        inductance [H]
  in   C        capacitance [F]
  in   f        supply frequency [Hz]
  in   V        rms supply voltage [V]
  out  I        rms current [A]
  out  Z        impedance [ohm]
  out  phi      phase angle of the voltage ahead of the current (positive: inductive) [rad]
  out  phi_deg  phase angle in degrees []
  out  pf       power factor cos(phi) []
  out  P        average power [W]
  out  XL       inductive reactance [ohm]
  out  XC       capacitive reactance [ohm]
```

### 2.6 Solve

Inputs as name=value, each with any unit of the right kind. The answer lists every output, the method used, each input converted to SI, and the conditions the answer assumes.

```shell
$ classical-physics solve series_rlc_ac_circuit "R=30 ohm" "L=0.1 H" "C=10 uF" "f=50 Hz" "V=230 V"
Z = 288.458219 ohm
I = 0.7973425089 A
phi = -1.466606728 rad
phi_deg = -84.0303757
pf = 0.1040011968
P = 19.07265229 W
XL = 31.41592654 ohm
XC = 318.3098862 ohm
  by: Series RLC circuit on an AC supply: impedance, current, phase and power  (XL = 2 pi f L, XC = 1/(2 pi f C), Z = sqrt(R^2 + (XL - XC)^2), I = V/Z, tan(phi) = (XL - XC)/R, P = I^2 R)
  R = 30 ohm
  L = 0.1 H
  C = 1e-05 F
  f = 50 Hz
  V = 230 V
  assumes: R, L, C, f, V > 0; sinusoidal steady state
```

### 2.7 Machine-readable answer

--json prints the same answer as one line of JSON (values in SI): for scripts and other programs.

```shell
$ classical-physics solve series_rlc_ac_circuit "R=30 ohm" "L=0.1 H" "C=10 uF" "f=50 Hz" "V=230 V" --json
{"outcome":"SOLVED","message":"","value":0.797342508851,"unit":"A","solved_for":"I","si":[["R",30.0],["L",0.1],["C",9.999999999999999e-6],["f",50.0],["V",230.0]],"values":[["Z",288.458218955,"ohm"],["I",0.797342508851,"A"],["phi",-1.46660672772,"rad"],["phi_deg",-84.0303757039,""],["pf",0.104001196807,""],["P",19.0726522926,"W"],["XL",31.415926535897935,"ohm"],["XC",318.3098861837907,"ohm"]],"target":"I","phrase":"rms current","assumptions":["R, L, C, f, V > 0","sinusoidal steady state"],"name":"Series RLC circuit on an AC supply: impedance, current, phase and power","equation":"XL = 2 pi f L, XC = 1/(2 pi f C), Z = sqrt(R^2 + (XL - XC)^2), I = V/Z, tan(phi) = (XL - XC)/R, P = I^2 R"}
```

### 2.8 When an input is missing

No number is guessed: the answer names what is missing, shows a complete call, and the program exits with status 1.

```shell
$ classical-physics solve series_rlc_ac_circuit "R=30 ohm" "L=0.1 H" "C=10 uF" "f=50 Hz"
NEEDS_INPUT: series_rlc_ac_circuit needs V (rms supply voltage). Example: solve('series_rlc_ac_circuit', R='30 ohm', L='0.1 H', C='1.66667e-05 F', f='159.155 Hz', V='100 V').
```

### 2.9 Solve an equation for another quantity

An equation can be solved for any of its quantities: give the others and name the one you want with --target (here ve).

```shell
$ classical-physics solve rocket_delta_v "m0=1000 kg" "mf=400 kg" "dv=2748.87 m/s" --target ve
exhaust velocity (ve) = 2999.997604 m/s
  by: Velocity change of a rocket (Tsiolkovsky)  (dv = ve*log(m0/mf))
  m0 = 1000 kg   [1000 kg]
  mf = 400 kg   [400 kg]
  dv = 2748.87 m/s   [2748.87 m/s]
  assumes: no gravity or drag during the burn; a constant exhaust velocity; the situation is: rocket
```

### 2.10 Many questions at once

One JSON request per line in, one JSON answer per line out, in order: for scripts, notebooks and agents (the program starts once). In PowerShell: Get-Content cases.jsonl | classical-physics batch

`cases.jsonl`:

```json
{"key": "series_rlc_ac_circuit", "inputs": {"R": "30 ohm", "L": "0.1 H", "C": "10 uF", "f": "50 Hz", "V": "230 V"}}
{"key": "rocket_delta_v", "inputs": {"ve": "3000 m/s", "m0": "1000 kg", "mf": "400 kg"}}
```

```shell
$ classical-physics batch < cases.jsonl
{"outcome":"SOLVED","message":"","value":0.797342508851,"unit":"A","solved_for":"I","si":[["R",30.0],["L",0.1],["C",9.999999999999999e-6],["f",50.0],["V",230.0]],"values":[["Z",288.458218955,"ohm"],["I",0.797342508851,"A"],["phi",-1.46660672772,"rad"],["phi_deg",-84.0303757039,""],["pf",0.104001196807,""],["P",19.0726522926,"W"],["XL",31.415926535897935,"ohm"],["XC",318.3098861837907,"ohm"]],"target":"I","phrase":"rms current","assumptions":["R, L, C, f, V > 0","sinusoidal steady state"],"name":"Series RLC circuit on an AC supply: impedance, current, phase and power","equation":"XL = 2 pi f L, XC = 1/(2 pi f C), Z = sqrt(R^2 + (XL - XC)^2), I = V/Z, tan(phi) = (XL - XC)/R, P = I^2 R"}
{"outcome":"SOLVED","message":"","value":2748.8721956224654,"unit":"m/s","solved_for":"dv","si":[["ve",3000.0],["m0",1000.0],["mf",400.0]],"target":"dv","phrase":"change in velocity","assumptions":["no gravity or drag during the burn","a constant exhaust velocity","the situation is: rocket"],"name":"Velocity change of a rocket (Tsiolkovsky)","equation":"dv = ve*log(m0/mf)"}
```

## 3. Writing inputs

Inputs are written `name=value`, one per argument; `classical-physics info <capability>` lists the names. Quote an argument that holds a space or brackets (bash, zsh and PowerShell all accept double quotes). The answer always states each input as it was read, converted to SI, so a misread unit is visible at once.

| kind | example | how it is read |
|---|---|---|
| a number with a unit | `"R=0.128 nm"` | any unit of the same kind; converted to SI |
| a number in SI | `R=1.28e-10` | no unit means the SI unit shown by info |
| a choice | `structure=FCC` | one of the values info lists |
| a list or vector | `"v=[1, 2, 3]" or "v=1, 2, 3"` | brackets keep it a list |
| a matrix | `"A=[[1, 2], [3, 4]]"` | one list per row |
| true / false | `inclusive=true` | for yes/no options |

An equation can be solved for any of its quantities: give the others and add `--target <symbol>`. Without `--target`, the program solves for the quantity the capability is named after.

## 4. Outcomes and exit codes

The program never guesses a number: when it cannot answer, it says why and exits with status 1, so a script can tell an answer from a refusal without reading the text.

| outcome | meaning | exit status |
|---|---|---|
| SOLVED | the answer: every output with its unit, the method, the inputs in SI, the conditions assumed | 0 |
| NEEDS_INPUT | an input is missing; the message names it and shows a complete call | 1 |
| INVALID_INPUT | a value or unit of the wrong kind, an unknown input name, or a value outside the conditions | 1 |
| AMBIGUOUS | the inputs allow two different answers; say which one is meant | 1 |
| OUT_OF_SCOPE | the model has no capability for this, or the result would not be meaningful | 1 |
| (usage) | a command the program does not know: it prints its usage | 2 |

## 5. Scripts and agents: batch and JSON

`classical-physics solve ... --json` prints one answer as one line of JSON. `classical-physics batch` reads one request per line on standard input and writes one JSON answer per line, in order, starting the program once for any number of requests. A request is `{"key": "<capability>", "inputs": {"name": "value", ...}, "target": null}`. In PowerShell, pipe the file in: `Get-Content cases.jsonl | classical-physics batch`. `classical-physics catalog` prints every capability with its inputs as JSON, to build forms or tool definitions.

| answer field | meaning |
|---|---|
| outcome | one of the outcomes above |
| message | why there is no answer (empty when SOLVED) |
| value, unit | the main result in SI units |
| values | every output: [symbol, value, unit] (procedures) |
| target | the quantity solved for |
| si | each numeric input converted to SI: [symbol, value] |
| assumptions | the conditions the answer relies on |
| name, equation | the capability and its equation or method |

```python
import json, subprocess

reqs = [{"key": "<capability>", "inputs": {"name": "value"}}]
out = subprocess.run(["classical-physics", "batch"], input="\n".join(map(json.dumps, reqs)),
                     capture_output=True, text=True, check=True).stdout
answers = [json.loads(line) for line in out.splitlines()]
```

## 6. Figures

`classical-physics figures <capability> name=value ... --out DIR` draws the live figures for those inputs and writes each as an SVG file into DIR (an existing folder); without `--out` the SVG text comes back as JSON. The desktop app and the model page draw the same figures as soon as you press Compute.

## 7. In a web page or Node.js: WebAssembly

The same model is a WebAssembly module, `classical_physics_ai999x.wasm` (https://www.ai999x.com/compute/rust/classical_physics_ai999x.wasm), with a small loader, `ax-wasm.js` (https://www.ai999x.com/compute/rust/ax-wasm.js). It runs in a browser, a Web Worker or Node.js, with no server.

```html
<script src="ax-wasm.js"></script>
<script>
  AXWasm.load("classical_physics_ai999x.wasm").then(function (m) {
    var a = m.solve({key: "<capability>", inputs: {name: "value"}});
    console.log(a.outcome, a.values || a.value);
    console.log(m.catalog().capabilities.length + " capabilities");
  });
</script>
```

## 8. Every capability

By topic. For each: what it does, what it needs and gives, when it applies, and a command with what the program printed for it. `classical-physics info <id>` shows the same inputs and outputs in the terminal.

### AC circuits (6)

<a id="capacitive_reactance"></a>

#### Reactance of a capacitor: `capacitive_reactance`

One equation links capacitive reactance (X), Frequency (f) and Capacitance (C). Give all but one of them and it is solved for the one left out, by default capacitive reactance (X).

$$
X = \frac{1}{2 \pi f C}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `f` | in | Frequency | Hz |
| `C` | in | Capacitance | F |
| `X` | out | capacitive reactance | ohm |

**Applies when:** sinusoidal AC; a capacitor in an AC circuit.

```shell
$ classical-physics solve capacitive_reactance "f=60 Hz" "C=1e-06 F"
capacitive reactance (X) = 2652.582385 ohm
  by: Reactance of a capacitor  (X = 1/(2*pi*f*C))
  f = 60 Hz   [60 Hz]
  C = 1e-06 F   [1e-06 F]
  assumes: sinusoidal AC; the situation is: capacitor
```

**What the worked example means:** This calculation determines the capacitive reactance, which represents the opposition a capacitor offers to alternating current. The result 2652.58 ohm is derived using the frequency 60 Hz and the capacitance 1e-06 F. A lower value indicates that the capacitor allows more current to flow at this specific frequency, while a higher value suggests greater resistance to the signal.

<a id="inductive_reactance"></a>

#### Reactance of an inductor: `inductive_reactance`

One equation links inductive reactance (X), Frequency (f) and Inductance (L). Give all but one of them and it is solved for the one left out, by default inductive reactance (X).

$$
X = 2 \pi f L
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `f` | in | Frequency | Hz |
| `L` | in | Inductance | H |
| `X` | out | inductive reactance | ohm |

**Applies when:** sinusoidal AC; an inductor in an AC circuit.

```shell
$ classical-physics solve inductive_reactance "f=60 Hz" "L=0.03 H"
inductive reactance (X) = 11.30973355 ohm
  by: Reactance of an inductor  (X = 2*pi*f*L)
  f = 60 Hz   [60 Hz]
  L = 0.03 H   [0.03 H]
  assumes: sinusoidal AC; the situation is: inductor
```

**What the worked example means:** This calculation determines the opposition to alternating current provided by an inductor. Using the frequency of the signal and the inductance value, the result shows that the inductive reactance equals 11.3097 ohm. This value indicates how much the component resists changes in current flow within the circuit.

<a id="lc_resonance"></a>

#### Resonant frequency of an LC circuit: `lc_resonance`

One equation links resonant frequency (f), Inductance (L) and Capacitance (C). Give all but one of them and it is solved for the one left out, by default resonant frequency (f).

$$
f = \frac{1}{2 \pi \sqrt{L C}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `L` | in | Inductance | H |
| `C` | in | Capacitance | F |
| `f` | out | resonant frequency | Hz |

**Applies when:** ideal components; negligible resistance for the frequency; LC and series RLC circuits.

```shell
$ classical-physics solve lc_resonance "L=0.01 H" "C=1e-06 F"
resonant frequency (f) = 1591.549431 Hz
  by: Resonant frequency of an LC circuit  (f = 1/(2*pi*sqrt(L*C)))
  L = 0.01 H   [0.01 H]
  C = 1e-06 F   [1e-06 F]
  assumes: ideal components; negligible resistance for the frequency; the situation is: lc circuit
```

**What the worked example means:** This calculation determines the resonant frequency of an LC circuit, which is the rate at which energy oscillates between the inductor and the capacitor. Given an inductance of 0.01 H and a capacitance of 1e-06 F, the resulting frequency is 1591.55 Hz. This value represents the natural tuning point where the circuit responds most strongly to alternating current signals.

<a id="rms_current"></a>

#### Rms current of a sinusoidal supply: `rms_current`

One equation links rms current (Irms) and peak current (I0). Give all but one of them and it is solved for the one left out, by default rms current (Irms).

$$
I_{\text{rms}} = \frac{I_{0}}{\sqrt{2}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `I0` | in | peak current | A |
| `Irms` | out | rms current | A |

**Applies when:** a sinusoidal waveform; sinusoidal AC.

```shell
$ classical-physics solve rms_current "I0=10 A"
rms current (Irms) = 7.071067812 A
  by: Rms current of a sinusoidal supply  (Irms = I0/sqrt(2))
  I0 = 10 A   [10 A]
  assumes: a sinusoidal waveform; the situation is: alternating current
```

**What the worked example means:** This calculation determines the effective current value for a sinusoidal alternating supply. Taking the peak current, which is 10 A, and dividing it by the square root of two gives the root mean square current of 7.07107 A. This specific value represents the amount of direct current that would produce the same heating effect in a resistor as this alternating current.

<a id="rms_voltage"></a>

#### Rms voltage of a sinusoidal supply: `rms_voltage`

One equation links rms voltage (Vrms) and peak voltage (V0). Give all but one of them and it is solved for the one left out, by default rms voltage (Vrms).

$$
V_{\text{rms}} = \frac{V_{0}}{\sqrt{2}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `V0` | in | peak voltage | V |
| `Vrms` | out | rms voltage | V |

**Applies when:** a sinusoidal waveform; sinusoidal AC.

```shell
$ classical-physics solve rms_voltage "V0=170 V"
rms voltage (Vrms) = 120.2081528 V
  by: Rms voltage of a sinusoidal supply  (Vrms = V0/sqrt(2))
  V0 = 170 V   [170 V]
  assumes: a sinusoidal waveform; the situation is: alternating current
```

**What the worked example means:** This calculation determines the root mean square voltage of a sinusoidal alternating current supply using the peak voltage. The result, 120.208 V, represents the effective voltage that delivers the same power to a resistive load as a direct current source of the same magnitude. This value is derived from the peak voltage 170 V, which is divided by the square root of two.

<a id="series_rlc_ac_circuit"></a>

#### Series RLC circuit on an AC supply: impedance, current, phase and power, `series_rlc_ac_circuit`

Reactances, impedance, rms current, phase angle, power factor and average power of a resistor, an inductor and a capacitor in series across a sinusoidal supply.

$$
Z = \sqrt{R^2 + (X_L - X_C)^2},\quad I = \frac{V}{Z},\quad \tan\varphi = \frac{X_L - X_C}{R},\quad P = I^2R
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `R` | in | resistance | ohm |
| `L` | in | inductance | H |
| `C` | in | capacitance | F |
| `f` | in | supply frequency | Hz |
| `V` | in | rms supply voltage | V |
| `I` | out | rms current | A |
| `Z` | out | impedance | ohm |
| `phi` | out | phase angle of the voltage ahead of the current (positive: inductive) | rad |
| `phi_deg` | out | phase angle in degrees | - |
| `pf` | out | power factor cos(phi) | - |
| `P` | out | average power | W |
| `XL` | out | inductive reactance | ohm |
| `XC` | out | capacitive reactance | ohm |

**Applies when:** R, L, C, f, V > 0; sinusoidal steady state.

```shell
$ classical-physics solve series_rlc_ac_circuit "R=30 ohm" "L=0.1 H" "C=10 uF" "f=50 Hz" "V=230 V"
Z = 288.458219 ohm
I = 0.7973425089 A
phi = -1.466606728 rad
phi_deg = -84.0303757
pf = 0.1040011968
P = 19.07265229 W
XL = 31.41592654 ohm
XC = 318.3098862 ohm
  by: Series RLC circuit on an AC supply: impedance, current, phase and power  (XL = 2 pi f L, XC = 1/(2 pi f C), Z = sqrt(R^2 + (XL - XC)^2), I = V/Z, tan(phi) = (XL - XC)/R, P = I^2 R)
  R = 30 ohm
  L = 0.1 H
  C = 1e-05 F
  f = 50 Hz
  V = 230 V
  assumes: R, L, C, f, V > 0; sinusoidal steady state
```

In the worked example, R = 30 ohm, L = 0.1 H, C = 10 uF, f = 50 Hz, V = 230 V go through the equation above and give Z = 288.458219 ohm, I = 0.7973425089 A, phi = -1.466606728 rad, phi_deg = -84.0303757. It applies only when R, L, C, f, V > 0; sinusoidal steady state: outside that it says what is wrong instead of returning a number.

### Capacitors (3)

<a id="parallel_plate_capacitor"></a>

#### Capacitance of parallel plates: `parallel_plate_capacitor`

One equation links Capacitance (C), area of each plate (A), plate separation (d) and Permittivity (eps0). Give all but one of them and it is solved for the one left out, by default Capacitance (C).

$$
C = \frac{\varepsilon_{0} A}{d}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `A` | in | area of each plate | m^2 |
| `d` | in | plate separation | m |
| `eps0` | in | Permittivity (optional) | C^2/(N*m^2) |
| `C` | out | Capacitance | F |

**Applies when:** plates much wider than their separation; vacuum or air between the plates; parallel plates with air or vacuum between them.

```shell
$ classical-physics solve parallel_plate_capacitor "A=0.01 m^2" "d=0.001 m" "eps0=8.854187813e-12 C^2/(N*m^2)"
capacitance (C) = 8.854187813e-11 F
  by: Capacitance of parallel plates  (C = eps0*A/d)
  A = 0.01 m^2   [0.01 m^2]
  d = 0.001 m   [0.001 m]
  eps0 = 8.854187813e-12 C^2/(N*m^2)   [8.85419e-12 C^2/(N*m^2)]
  assumes: plates much wider than their separation; vacuum or air between the plates; the situation is: capacitor
```

In the worked example, A = 0.01 m^2, d = 0.001 m, eps0 = 8.854187813e-12 C^2/(N*m^2) go through the equation above and give C = 8.854187813e-11 F. It applies only when plates much wider than their separation; vacuum or air between the plates; parallel plates with air or vacuum between them: outside that it says what is wrong instead of returning a number.

<a id="capacitance_definition"></a>

#### Charge on a capacitor: `capacitance_definition`

One equation links charge on either plate (Q), capacitance (C) and voltage across the capacitor (V). Give all but one of them and it is solved for the one left out, by default charge on either plate (Q).

$$
Q = C V
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `C` | in | capacitance | F |
| `V` | in | voltage across the capacitor | V |
| `Q` | out | charge on either plate | C |

**Applies when:** an ideal capacitor; a charged capacitor.

```shell
$ classical-physics solve capacitance_definition "C=1e-05 F" "V=12 V"
charge on either plate (Q) = 0.00012 C
  by: Charge on a capacitor  (Q = C*V)
  C = 1e-05 F   [1e-05 F]
  V = 12 V   [12 V]
  assumes: an ideal capacitor; the situation is: capacitor
```

**What the worked example means:** This calculation determines the electric charge stored on either plate of an ideal capacitor. Multiplying the capacitance, which represents the device's ability to store charge, by the voltage across it shows that the resulting charge is 0.00012 C. This value indicates how much electrical energy is currently held in the field between the plates under these specific conditions.

<a id="capacitor_energy"></a>

#### Energy stored in a capacitor: `capacitor_energy`

One equation links energy stored (Ecap), Capacitance (C) and Voltage (V). Give all but one of them and it is solved for the one left out, by default energy stored (Ecap).

$$
\text{Ecap} = \frac{C V^{2}}{2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `C` | in | Capacitance | F |
| `V` | in | Voltage | V |
| `Ecap` | out | energy stored | J |

**Applies when:** an ideal capacitor; a charged capacitor.

```shell
$ classical-physics solve capacitor_energy "C=0.0001 F" "V=20 V"
energy stored (Ecap) = 0.02 J
  by: Energy stored in a capacitor  (Ecap = C*V**2/2)
  C = 0.0001 F   [0.0001 F]
  V = 20 V   [20 V]
  assumes: an ideal capacitor; the situation is: capacitor
```

**What the worked example means:** This calculation determines the potential energy stored within an ideal capacitor, denoted as 0.02 J. The result is derived from the capacitance of the component, represented by 0.0001 F, and the voltage across it, indicated by 20 V. This value represents the work required to charge the capacitor to that specific voltage level or the total energy available for release when the capacitor discharges through a circuit.

### Circuits (4)

<a id="ohm_power_current"></a>

#### Power from current and resistance: `ohm_power_current`

One equation links power dissipated (P), Current (I) and Resistance (R). Give all but one of them and it is solved for the one left out, by default power dissipated (P).

$$
P = I^{2} R
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `I` | in | Current | A |
| `R` | in | Resistance | ohm |
| `P` | out | power dissipated | W |

**Applies when:** an ohmic resistor; steady direct current; a resistor carries a steady current.

```shell
$ classical-physics solve ohm_power_current "I=2 A" "R=5 ohm"
power dissipated (P) = 20 W
  by: Power from current and resistance  (P = I**2*R)
  I = 2 A   [2 A]
  R = 5 ohm   [5 ohm]
  assumes: an ohmic resistor; steady direct current; the situation is: resistive
```

**What the worked example means:** This calculation determines the electrical power converted into heat within a resistor. By using the steady direct current flowing through the component and its specific resistance, the result shows that the dissipated power equals 20 W. This value is critical for understanding thermal load and ensuring components are not overloaded.

<a id="ohm_power_voltage"></a>

#### Power from voltage and resistance: `ohm_power_voltage`

One equation links power dissipated (P), Voltage (V) and Resistance (R). Give all but one of them and it is solved for the one left out, by default power dissipated (P).

$$
P = \frac{V^{2}}{R}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `V` | in | Voltage | V |
| `R` | in | Resistance | ohm |
| `P` | out | power dissipated | W |

**Applies when:** an ohmic resistor; steady direct current; a resistor has a steady voltage across it.

```shell
$ classical-physics solve ohm_power_voltage "V=12 V" "R=6 ohm"
power dissipated (P) = 24 W
  by: Power from voltage and resistance  (P = V**2/R)
  V = 12 V   [12 V]
  R = 6 ohm   [6 ohm]
  assumes: an ohmic resistor; steady direct current; the situation is: resistive
```

**What the worked example means:** This calculation determines the power dissipated in a resistor using the voltage across it and its resistance. The result, 24 W, represents the rate at which electrical energy is converted into heat under steady direct current conditions. This method is particularly useful when voltage measurements are more accessible than current measurements.

<a id="resistivity"></a>

#### Resistance of a uniform wire: `resistivity`

One equation links Resistance (R), resistivity of the material (rho_e), length of the wire (L) and cross-sectional area (A). Give all but one of them and it is solved for the one left out, by default Resistance (R).

$$
R = \frac{\rho_{e} L}{A}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `rho_e` | in | resistivity of the material | ohm*m |
| `L` | in | length of the wire | m |
| `A` | in | cross-sectional area | m^2 |
| `R` | out | Resistance | ohm |

**Applies when:** a uniform wire; constant temperature; an ohmic conductor of uniform cross-section.

```shell
$ classical-physics solve resistivity "rho_e=1.68e-08 ohm*m" "L=10 m" "A=1e-06 m^2"
resistance (R) = 0.168 ohm
  by: Resistance of a uniform wire  (R = rho_e*L/A)
  rho_e = 1.68e-08 ohm*m   [1.68e-08 ohm*m]
  L = 10 m   [10 m]
  A = 1e-06 m^2   [1e-06 m^2]
  assumes: a uniform wire; constant temperature; the situation is: wire
```

**What the worked example means:** This calculation determines the electrical resistance of a uniform wire using its material resistivity, physical length, and cross-sectional area. The resulting resistance value is 0.168 ohm. This outcome reflects how the specific dimensions and material properties of the conductor impede the flow of electric current under constant temperature conditions.

<a id="two_resistors_series_parallel"></a>

#### Two resistors in series and in parallel: `two_resistors_series_parallel`

The equivalent resistance of two resistors connected in series and connected in parallel.

$$
r_{s} = r_{1} + r_{2},\quad r_{p} \left(r_{1} + r_{2}\right) = r_{1} r_{2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `r1` | in | first resistance | ohm |
| `r2` | in | second resistance | ohm |
| `rs` | out | series equivalent | ohm |
| `rp` | out | parallel equivalent | ohm |

**Applies when:** r1 > 0; r2 > 0.

```shell
$ classical-physics solve two_resistors_series_parallel "r1=6 ohm" "r2=3 ohm"
rs = 9 ohm
rp = 2 ohm
  by: Two resistors in series and in parallel  (series resistances add; parallel conductances add)
  r1 = 6 ohm
  r2 = 3 ohm
  assumes: r1 > 0; r2 > 0
```

**What the worked example means:** This calculation determines the single equivalent resistance for two resistors connected in series and in parallel. Given a first resistance of 6 ohm and a second resistance of 3 ohm, the series configuration yields an equivalent resistance of 9 ohm, while the parallel configuration results in an equivalent resistance of 2 ohm. These values represent the total opposition to current flow in each respective circuit arrangement.

### Circular (5)

<a id="angular_frequency"></a>

#### Angular frequency and frequency: `angular_frequency`

One equation links angular frequency (omega) and frequency (f). Give all but one of them and it is solved for the one left out, by default angular frequency (omega).

$$
\omega = 2 \pi f
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `f` | in | frequency | Hz |
| `omega` | out | angular frequency | rad/s |

```shell
$ classical-physics solve angular_frequency "f=0.5 Hz"
angular frequency (omega) = 3.141592654 rad/s
  by: Angular frequency and frequency  (omega = 2*pi*f)
  f = 0.5 Hz   [0.5 Hz]
```

**What the worked example means:** This calculation determines the angular frequency, represented as 3.14159 rad/s, by multiplying the standard frequency 0.5 Hz by two pi. Angular frequency describes how fast a periodic system rotates through its phase cycle in radians per second, whereas standard frequency counts complete cycles per second. This conversion is fundamental for analyzing oscillations and waves where phase angles are more useful than simple repetition counts.

<a id="centripetal_acceleration"></a>

#### Centripetal acceleration: `centripetal_acceleration`

One equation links centripetal acceleration (a), speed (v) and radius of the circle (r). Give all but one of them and it is solved for the one left out, by default centripetal acceleration (a).

$$
a = \frac{v^{2}}{r}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `v` | in | speed | m/s |
| `r` | in | radius of the circle | m |
| `a` | out | centripetal acceleration | m/s^2 |

**Applies when:** uniform circular motion.

```shell
$ classical-physics solve centripetal_acceleration "v=6 m/s" "r=3 m"
centripetal acceleration (a) = 12 m/s^2
  by: Centripetal acceleration  (a = v**2/r)
  v = 6 m/s   [6 m/s]
  r = 3 m   [3 m]
  assumes: uniform circular motion
```

**What the worked example means:** This calculation determines the centripetal acceleration required to maintain uniform circular motion. Using the object's speed of 6 m/s and the path radius of 3 m shows that the inward acceleration is 12 m/s^2. This value represents how rapidly the direction of velocity changes, indicating the strength of the force needed to keep the object on its curved trajectory rather than moving in a straight line.

<a id="centripetal_force"></a>

#### Centripetal force: `centripetal_force`

One equation links centripetal force (F), mass (m), speed (v) and radius of the circle (r). Give all but one of them and it is solved for the one left out, by default centripetal force (F).

$$
F = \frac{m v^{2}}{r}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `m` | in | mass | kg |
| `v` | in | speed | m/s |
| `r` | in | radius of the circle | m |
| `F` | out | centripetal force | N |

**Applies when:** uniform circular motion.

```shell
$ classical-physics solve centripetal_force "m=2 kg" "v=6 m/s" "r=3 m"
centripetal force (F) = 24 N
  by: Centripetal force  (F = m*v**2/r)
  m = 2 kg   [2 kg]
  v = 6 m/s   [6 m/s]
  r = 3 m   [3 m]
  assumes: uniform circular motion
```

**What the worked example means:** This calculation finds the net inward force needed to maintain uniform circular motion. Using the mass of the object, its speed, and the radius of the path shows that the required centripetal force is 24 N. This value represents the strength of the pull necessary to keep the object from moving in a straight line due to inertia.

<a id="period_frequency"></a>

#### Period and frequency: `period_frequency`

One equation links period (T) and frequency (f). Give all but one of them and it is solved for the one left out, by default period (T).

$$
T = \frac{1}{f}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `f` | in | frequency | Hz |
| `T` | out | period | s |

**Applies when:** periodic motion.

```shell
$ classical-physics solve period_frequency "f=4 Hz"
period (T) = 0.25 s
  by: Period and frequency  (T = 1/f)
  f = 4 Hz   [4 Hz]
  assumes: periodic motion
```

**What the worked example means:** This calculation determines the period, defined as the time required for one complete cycle of motion. Using the inverse relationship between period and frequency shows that for a frequency of 4 Hz, the resulting period is 0.25 s. This means each individual repetition takes this specific duration to complete.

<a id="tangential_speed"></a>

#### Speed on a circle from angular speed: `tangential_speed`

One equation links speed (v), angular speed (omega) and radius of the circle (r). Give all but one of them and it is solved for the one left out, by default speed (v).

$$
v = \omega r
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `omega` | in | angular speed | rad/s |
| `r` | in | radius of the circle | m |
| `v` | out | speed | m/s |

**Applies when:** rotation about a fixed axis.

```shell
$ classical-physics solve tangential_speed "omega=4 rad/s" "r=0.5 m"
speed (v) = 2 m/s
  by: Speed on a circle from angular speed  (v = omega*r)
  omega = 4 rad/s   [4 rad/s]
  r = 0.5 m   [0.5 m]
  assumes: rotation about a fixed axis
```

**What the worked example means:** This calculation determines the linear speed of an object moving along a circular path. It uses the given angular speed, which measures how quickly the object rotates around the axis, and the radius, which defines the distance from that axis to the object. The result, 2 m/s, represents the actual velocity at which the object travels along the edge of its circle.

### Collisions (2)

<a id="elastic_collision_first_final"></a>

#### Final velocity of the first body in a one-dimensional elastic collision, `elastic_collision_first_final`

One equation links final velocity of the first body (v1f), mass of the first body (m1), mass of the second body (m2), initial velocity of the first body (v1) and initial velocity of the second body (v2). Give all but one of them and it is solved for the one left out, by default final velocity of the first body (v1f).

$$
v_{1f} = \frac{\left(m_{1} - m_{2}\right) v_{1} + 2 m_{2} v_{2}}{m_{1} + m_{2}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `m1` | in | mass of the first body | kg |
| `m2` | in | mass of the second body | kg |
| `v1` | in | initial velocity of the first body | m/s |
| `v2` | in | initial velocity of the second body | m/s |
| `v1f` | out | final velocity of the first body | m/s |

**Applies when:** a head-on (one-dimensional) collision; kinetic energy conserved; no external forces; two bodies colliding elastically along a line.

```shell
$ classical-physics solve elastic_collision_first_final "m1=2 kg" "m2=1 kg" "v1=3 m/s" "v2=0 m/s"
final velocity of the first body (v1f) = 1 m/s
  by: Final velocity of the first body in a one-dimensional elastic collision  (v1f = ((m1 - m2)*v1 + 2*m2*v2)/(m1 + m2))
  m1 = 2 kg   [2 kg]
  m2 = 1 kg   [1 kg]
  v1 = 3 m/s   [3 m/s]
  v2 = 0 m/s   [0 m/s]
  assumes: a head-on (one-dimensional) collision; kinetic energy conserved; no external forces; the situation is: elastic collision
```

**What the worked example means:** This calculation determines the final velocity of the first body, denoted as v1f. It uses the masses of both bodies, m1 and m2, along with their initial velocities, v1 and v2. The result, 1 m/s, represents how fast and in which direction the first object moves after a head-on elastic collision.

<a id="elastic_collision_second_final"></a>

#### Final velocity of the second body in a one-dimensional elastic collision, `elastic_collision_second_final`

One equation links final velocity of the second body (v2f), mass of the first body (m1), mass of the second body (m2), initial velocity of the first body (v1) and initial velocity of the second body (v2). Give all but one of them and it is solved for the one left out, by default final velocity of the second body (v2f).

$$
v_{2f} = \frac{\left(m_{2} - m_{1}\right) v_{2} + 2 m_{1} v_{1}}{m_{1} + m_{2}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `m1` | in | mass of the first body | kg |
| `m2` | in | mass of the second body | kg |
| `v1` | in | initial velocity of the first body | m/s |
| `v2` | in | initial velocity of the second body | m/s |
| `v2f` | out | final velocity of the second body | m/s |

**Applies when:** a head-on (one-dimensional) collision; kinetic energy conserved; no external forces; two bodies colliding elastically along a line.

```shell
$ classical-physics solve elastic_collision_second_final "m1=2 kg" "m2=1 kg" "v1=3 m/s" "v2=0 m/s"
final velocity of the second body (v2f) = 4 m/s
  by: Final velocity of the second body in a one-dimensional elastic collision  (v2f = ((m2 - m1)*v2 + 2*m1*v1)/(m1 + m2))
  m1 = 2 kg   [2 kg]
  m2 = 1 kg   [1 kg]
  v1 = 3 m/s   [3 m/s]
  v2 = 0 m/s   [0 m/s]
  assumes: a head-on (one-dimensional) collision; kinetic energy conserved; no external forces; the situation is: elastic collision
```

**What the worked example means:** This calculation determines the final velocity of the second body after a one-dimensional elastic collision. It uses the masses of both bodies and their initial velocities to find that the second body ends up moving at 4 m/s. This result assumes kinetic energy is conserved and no external forces act on the system.

### DC circuits (1)

<a id="terminal_voltage_internal_resistance"></a>

#### Terminal voltage of a source with internal resistance, `terminal_voltage_internal_resistance`

Current, terminal voltage, lost volts, load power and efficiency of a source of emf with internal resistance r driving a load resistance R.

$$
I = \frac{\mathcal{E}}{R + r},\quad V_t = \mathcal{E} - Ir
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `emf` | in | emf of the source | V |
| `r` | in | internal resistance | ohm |
| `R` | in | load resistance | ohm |
| `I` | out | current in the circuit | A |
| `V_t` | out | terminal voltage (across the load) | V |
| `V_lost` | out | voltage lost inside the source (I r) | V |
| `P_load` | out | power delivered to the load | W |
| `efficiency` | out | fraction of the source's power delivered to the load | - |

**Applies when:** emf > 0; r, R >= 0 and not both 0.

```shell
$ classical-physics solve terminal_voltage_internal_resistance "emf=12 V" "r=0.5 ohm" "R=5.5 ohm"
I = 2 A
V_t = 11 V
V_lost = 1 V
P_load = 22 W
efficiency = 0.9166666667
  by: Terminal voltage of a source with internal resistance  (I = emf / (R + r); V_t = emf - I r = I R)
  emf = 12 V
  r = 0.5 ohm
  R = 5.5 ohm
  assumes: emf > 0; r, R >= 0 and not both 0
```

In the worked example, emf = 12 V, r = 0.5 ohm, R = 5.5 ohm go through the equation above and give I = 2 A, V_t = 11 V, V_lost = 1 V, P_load = 22 W. It applies only when emf > 0; r, R >= 0 and not both 0: outside that it says what is wrong instead of returning a number.

### Dynamics (12)

<a id="acceleration_frictionless_incline"></a>

#### Acceleration down a frictionless incline: `acceleration_frictionless_incline`

One equation links acceleration down the incline (a), gravitational acceleration (g) and angle of the incline to the horizontal (theta). Give all but one of them and it is solved for the one left out, by default acceleration down the incline (a).

$$
a = g \sin\left(\theta\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `g` | in | gravitational acceleration (optional) | m/s^2 |
| `theta` | in | angle of the incline to the horizontal | rad |
| `a` | out | acceleration down the incline | m/s^2 |

**Applies when:** no friction; the only forces are gravity and the normal force; the surface is stated to be frictionless.

```shell
$ classical-physics solve acceleration_frictionless_incline "g=9.8 m/s^2" "theta=30 deg"
acceleration down the incline (a) = 4.9 m/s^2
  by: Acceleration down a frictionless incline  (a = g*sin(theta))
  g = 9.8 m/s^2   [9.8 m/s^2]
  theta = 0.5235987756 rad   [30 deg]
  assumes: no friction; the only forces are gravity and the normal force; the situation is: inclined, frictionless
```

**What the worked example means:** This calculation determines the acceleration of an object sliding down a frictionless slope. Applying the formula for gravitational component along the plane shows that the acceleration depends on gravity and the angle of inclination. The resulting value of 4.9 m/s^2 represents how quickly the object speeds up under these specific conditions.

<a id="average_force_from_momentum_change"></a>

#### Average net force from a change of momentum, `average_force_from_momentum_change`

One equation links average net force (F), final momentum (p2), initial momentum (p1) and time over which the momentum changes (t). Give all but one of them and it is solved for the one left out, by default average net force (F).

$$
F = \frac{p_2 - p_1}{t}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `p2` | in | final momentum | kg*m/s |
| `p1` | in | initial momentum | kg*m/s |
| `t` | in | time over which the momentum changes | s |
| `F` | out | average net force | N |

**Applies when:** F is the net force averaged over the interval; motion along a line; the momentum before and after is known.

```shell
$ classical-physics solve average_force_from_momentum_change "p2=25 kg*m/s" "p1=10 kg*m/s" "t=3 s"
average net force (F) = 5 N
  by: Average net force from a change of momentum  (F = (p2 - p1)/t)
  p2 = 25 kg*m/s   [25 kg*m/s]
  p1 = 10 kg*m/s   [10 kg*m/s]
  t = 3 s   [3 s]
  assumes: F is the net force averaged over the interval; motion along a line
```

**What the worked example means:** This calculation determines the average net force acting on an object by dividing the difference between its final and initial momentum by the duration of the interaction. The result, 5 N, represents the steady force that would produce the same change in motion as the actual varying forces during the interval. This approach is particularly useful for analyzing impacts or collisions where instantaneous forces are difficult to measure but the total momentum transfer is known.

<a id="weight_component_along_incline"></a>

#### Component of the weight along an incline: `weight_component_along_incline`

One equation links component of the weight along the incline (F), mass (m), gravitational acceleration (g) and angle of the incline to the horizontal (theta). Give all but one of them and it is solved for the one left out, by default component of the weight along the incline (F).

$$
F = m g \sin\left(\theta\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `m` | in | mass | kg |
| `g` | in | gravitational acceleration (optional) | m/s^2 |
| `theta` | in | angle of the incline to the horizontal | rad |
| `F` | out | component of the weight along the incline | N |

**Applies when:** uniform gravitational field; a body is on a straight incline.

```shell
$ classical-physics solve weight_component_along_incline "m=10 kg" "g=9.8 m/s^2" "theta=30 deg"
component of the weight along the incline (F) = 49 N
  by: Component of the weight along an incline  (F = m*g*sin(theta))
  m = 10 kg   [10 kg]
  g = 9.8 m/s^2   [9.8 m/s^2]
  theta = 0.5235987756 rad   [30 deg]
  assumes: uniform gravitational field; the situation is: inclined
```

**What the worked example means:** This calculation determines the force pulling the object down the slope, denoted as 49 N. It is derived from the mass 10 kg, gravitational acceleration 9.8 m/s^2, and the incline angle 0.523599 rad. This parallel force component is critical for analyzing sliding motion and determining the minimum friction required to keep the body stationary on the ramp.

<a id="impulse"></a>

#### Impulse of a constant force: `impulse`

One equation links impulse (J), force (F) and time the force acts (t). Give all but one of them and it is solved for the one left out, by default impulse (J).

$$
J = F t
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `F` | in | force | N |
| `t` | in | time the force acts | s |
| `J` | out | impulse | N*s |

**Applies when:** constant force.

```shell
$ classical-physics solve impulse "F=15 N" "t=4 s"
impulse (J) = 60 N*s
  by: Impulse of a constant force  (J = F*t)
  F = 15 N   [15 N]
  t = 4 s   [4 s]
  assumes: constant force
```

**What the worked example means:** This calculation determines the total impulse delivered to an object by multiplying a constant force by the duration for which it acts. The resulting value, 60 N*s, represents the change in momentum experienced by the object during this interaction. It links the magnitude of the applied force and the time of application directly to the alteration of the object's motion.

<a id="momentum"></a>

#### Linear momentum: `momentum`

One equation links momentum (p), mass (m) and velocity (v). Give all but one of them and it is solved for the one left out, by default momentum (p).

$$
p = m v
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `m` | in | mass | kg |
| `v` | in | velocity | m/s |
| `p` | out | momentum | kg*m/s |

**Applies when:** speeds far below the speed of light.

```shell
$ classical-physics solve momentum "m=2 kg" "v=7 m/s"
momentum (p) = 14 kg*m/s
  by: Linear momentum  (p = m*v)
  m = 2 kg   [2 kg]
  v = 7 m/s   [7 m/s]
  assumes: speeds far below the speed of light
```

**What the worked example means:** This calculation determines the linear momentum of an object by multiplying its mass, which represents the amount of matter in the object, by its velocity, which describes how fast and in what direction it is moving. The resulting value, 14 kg*m/s, indicates the quantity of motion the object possesses. This measure is crucial for predicting outcomes in interactions such as collisions or propulsion events within systems where speeds remain well below the speed of light.

<a id="newton_second"></a>

#### Net force and acceleration: `newton_second`

One equation links net force (F), mass (m) and acceleration (a). Give all but one of them and it is solved for the one left out, by default net force (F).

$$
F = m a
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `m` | in | mass | kg |
| `a` | in | acceleration | m/s^2 |
| `F` | out | net force | N |

**Applies when:** constant mass; inertial frame.

```shell
$ classical-physics solve newton_second "m=4 kg" "a=2.5 m/s^2"
net force (F) = 10 N
  by: Net force and acceleration  (F = m*a)
  m = 4 kg   [4 kg]
  a = 2.5 m/s^2   [2.5 m/s^2]
  assumes: constant mass; inertial frame
```

**What the worked example means:** This calculation determines the net force acting on an object by multiplying its mass by its acceleration. The result, 10 N, represents the total push or pull required to produce the observed change in motion for an object with inertia equal to 4 kg. This value assumes the mass remains constant and the measurement occurs within a non-accelerating reference frame.

<a id="net_force_friction"></a>

#### Net force with an opposing friction force: `net_force_friction`

One equation links net force (F), applied force (P) and friction force (f). Give all but one of them and it is solved for the one left out, by default net force (F).

$$
F = P - f
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `P` | in | applied force | N |
| `f` | in | friction force | N |
| `F` | out | net force | N |

**Applies when:** applied force and friction act along the same line, in opposite directions.

```shell
$ classical-physics solve net_force_friction "P=50 N" "f=12 N"
net force (F) = 38 N
  by: Net force with an opposing friction force  (F = P - f)
  P = 50 N   [50 N]
  f = 12 N   [12 N]
  assumes: applied force and friction act along the same line, in opposite directions
```

**What the worked example means:** This calculation determines the net force acting on an object moving in one dimension. It takes the applied force, which pushes the object forward, and subtracts the opposing friction force that resists motion. The result, 38 N, represents the total unbalanced force available to accelerate the object or maintain its velocity against resistance.

<a id="normal_on_level"></a>

#### Normal force on a level surface: `normal_on_level`

One equation links normal force (N) and weight (W). Give all but one of them and it is solved for the one left out, by default normal force (N).

$$
N = W
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `W` | in | weight | N |
| `N` | out | normal force | N |

**Applies when:** level surface; no other vertical forces.

```shell
$ classical-physics solve normal_on_level "W=70 N"
normal force (N) = 70 N
  by: Normal force on a level surface  (N = W)
  W = 70 N   [70 N]
  assumes: level surface; no other vertical forces
```

**What the worked example means:** This calculation determines the normal force, denoted as 70 N, by equating it to the object's weight, given as 70 N. The result represents the upward support force exerted by a level surface to counteract gravity. This value is critical for understanding how much friction can be generated or how structural loads are distributed when an object rests on a flat plane without other vertical influences.

<a id="normal_force_incline"></a>

#### Normal force on an incline: `normal_force_incline`

One equation links normal force from the incline (N), mass (m), gravitational acceleration (g) and angle of the incline to the horizontal (theta). Give all but one of them and it is solved for the one left out, by default normal force from the incline (N).

$$
N = m g \cos\left(\theta\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `m` | in | mass | kg |
| `g` | in | gravitational acceleration (optional) | m/s^2 |
| `theta` | in | angle of the incline to the horizontal | rad |
| `N` | out | normal force from the incline | N |

**Applies when:** no acceleration perpendicular to the incline; no other force with a component normal to the surface; the body rests or slides on a straight incline.

```shell
$ classical-physics solve normal_force_incline "m=10 kg" "g=9.8 m/s^2" "theta=60 deg"
normal force from the incline (N) = 49 N
  by: Normal force on an incline  (N = m*g*cos(theta))
  m = 10 kg   [10 kg]
  g = 9.8 m/s^2   [9.8 m/s^2]
  theta = 1.047197551 rad   [60 deg]
  assumes: no acceleration perpendicular to the incline; no other force with a component normal to the surface; the situation is: inclined
```

**What the worked example means:** This calculation determines the normal force, which is the support force exerted by the incline on the object. It equals 49 N. This value represents the magnitude of the contact force perpendicular to the surface, arising from the object's mass and the gravitational pull acting at the specified angle.

<a id="kinetic_friction"></a>

#### Sliding friction: `kinetic_friction`

One equation links friction force (f), coefficient of friction (mu) and normal force (N). Give all but one of them and it is solved for the one left out, by default friction force (f).

$$
f = \mu N
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `mu` | in | coefficient of friction | - |
| `N` | in | normal force | N |
| `f` | out | friction force | N |

**Applies when:** the surfaces slide over each other.

```shell
$ classical-physics solve kinetic_friction "mu=0.25 1" "N=80 N"
friction force (f) = 20 N
  by: Sliding friction  (f = mu*N)
  mu = 0.25   [0.25 1]
  N = 80 N   [80 N]
  assumes: the surfaces slide over each other
```

**What the worked example means:** This calculation determines the resistive force acting between two sliding surfaces. Multiplying the dimensionless coefficient of friction 0.25 by the normal force 80 N gives the friction force 20 N. This value represents the magnitude of the force opposing motion, which is crucial for understanding how quickly an object will slow down or what driving force is needed to maintain its speed.

<a id="tension_hanging_at_rest"></a>

#### Tension in a rope holding a hanging body: `tension_hanging_at_rest`

One equation links tension in the rope (T), mass of the hanging body (m) and gravitational acceleration (g). Give all but one of them and it is solved for the one left out, by default tension in the rope (T).

$$
T = m g
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `m` | in | mass of the hanging body | kg |
| `g` | in | gravitational acceleration (optional) | m/s^2 |
| `T` | out | tension in the rope | N |

**Applies when:** one vertical rope carries the whole weight; the body is not accelerating; the body hangs at rest or moves at constant velocity from a single vertical rope.

```shell
$ classical-physics solve tension_hanging_at_rest "m=10 kg" "g=9.8 m/s^2"
tension in the rope (T) = 98 N
  by: Tension in a rope holding a hanging body  (T = m*g)
  m = 10 kg   [10 kg]
  g = 9.8 m/s^2   [9.8 m/s^2]
  assumes: one vertical rope carries the whole weight; the body is not accelerating; the situation is: hanging
```

**What the worked example means:** This calculation determines the tension force within a single vertical rope that supports a stationary body. Applying Newton's first law shows that the upward tension exactly counteracts the downward gravitational pull on the mass. The result, 98 N, represents the magnitude of this holding force required to keep the object at rest or moving at a constant velocity without acceleration.

<a id="weight"></a>

#### Weight in a uniform gravitational field: `weight`

One equation links weight (W), mass (m) and gravitational acceleration (g). Give all but one of them and it is solved for the one left out, by default weight (W).

$$
W = m g
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `m` | in | mass | kg |
| `g` | in | gravitational acceleration (optional) | m/s^2 |
| `W` | out | weight | N |

**Applies when:** uniform gravitational field.

```shell
$ classical-physics solve weight "m=10 kg" "g=9.8 m/s^2"
weight (W) = 98 N
  by: Weight in a uniform gravitational field  (W = m*g)
  m = 10 kg   [10 kg]
  g = 9.8 m/s^2   [9.8 m/s^2]
  assumes: uniform gravitational field
```

**What the worked example means:** This calculation determines the weight, which is the force of gravity acting on an object. It is derived by multiplying the object's mass, representing its inertia, by the local gravitational acceleration. The result, 98 N, indicates how strongly gravity pulls on the body in this uniform field.

### Elasticity (3)

<a id="bulk_modulus_compression"></a>

#### Bulk compression: volume change under a pressure change (bulk modulus), `bulk_modulus_compression`

Volumetric strain, change of volume and final volume of a body of volume V0 when the pressure on it rises by dP.

$$
\Delta V = -\frac{V_0\,\Delta P}{B}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `dP` | in | increase of pressure (negative: decrease) | Pa |
| `V0` | in | original volume | m^3 |
| `B` | in | bulk modulus | Pa |
| `dV` | out | change of volume (negative: compression) | m^3 |
| `strain` | out | volumetric strain dV / V0 | - |
| `V` | out | final volume | m^3 |

**Applies when:** V0, B > 0; linear elastic (|dP / B| < 1).

```shell
$ classical-physics solve bulk_modulus_compression "dP=20 MPa" "V0=1 m^3" "B=2.2 GPa"
dV = -0.009090909091 m^3
strain = -0.009090909091
V = 0.9909090909 m^3
  by: Bulk compression: volume change under a pressure change (bulk modulus)  (dV = -V0 dP / B)
  dP = 20000000 Pa
  V0 = 1 m^3
  B = 2200000000 Pa
  assumes: V0, B > 0; linear elastic (|dP / B| < 1)
```

In the worked example, dP = 20 MPa, V0 = 1 m^3, B = 2.2 GPa go through the equation above and give dV = -0.009090909091 m^3, strain = -0.009090909091, V = 0.9909090909 m^3. It applies only when V0, B > 0; linear elastic (|dP / B| < 1): outside that it says what is wrong instead of returning a number.

<a id="shear_modulus_deformation"></a>

#### Shear: shear stress, shear strain and displacement (shear modulus), `shear_modulus_deformation`

Shear stress, shear strain and the sideways displacement of one face of a block of height h when a force F acts parallel to a face of area A.

$$
\Delta x = \frac{F h}{A S}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `F` | in | force parallel to the face | N |
| `A` | in | area of the face the force acts on | m^2 |
| `h` | in | distance between the sheared faces | m |
| `S` | in | shear modulus | Pa |
| `dx` | out | sideways displacement of the face | m |
| `stress` | out | shear stress F / A | Pa |
| `strain` | out | shear strain dx / h | - |

**Applies when:** A, h, S > 0; linear elastic (|strain| < 1).

```shell
$ classical-physics solve shear_modulus_deformation "F=40 kN" "A=20 cm^2" "h=10 cm" "S=25 GPa"
stress = 20000000 Pa
strain = 0.0008
dx = 8e-05 m
  by: Shear: shear stress, shear strain and displacement (shear modulus)  (stress = F/A; strain = stress/S; dx = strain h)
  F = 40000 N
  A = 0.002 m^2
  h = 0.1 m
  S = 2.5e+10 Pa
  assumes: A, h, S > 0; linear elastic (|strain| < 1)
```

In the worked example, F = 40 kN, A = 20 cm^2, h = 10 cm, S = 25 GPa go through the equation above and give stress = 20000000 Pa, strain = 0.0008, dx = 8e-05 m. It applies only when A, h, S > 0; linear elastic (|strain| < 1): outside that it says what is wrong instead of returning a number.

<a id="youngs_modulus_deformation"></a>

#### Tension or compression: stress, strain and change of length (Young's modulus), `youngs_modulus_deformation`

Stress, strain and change of length of a rod or wire of original length L0 and cross-section A under an axial force F.

$$
\Delta L = \frac{F L_0}{A E}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `F` | in | axial force (positive: tension, negative: compression) | N |
| `A` | in | cross-sectional area | m^2 |
| `L0` | in | original length | m |
| `E` | in | Young's modulus | Pa |
| `dL` | out | change of length (positive: extension) | m |
| `stress` | out | tensile stress F / A | Pa |
| `strain` | out | strain dL / L0 | - |

**Applies when:** A, L0, E > 0; linear elastic (|strain| < 1).

```shell
$ classical-physics solve youngs_modulus_deformation "F=1000 N" "A=1 mm^2" "L0=2 m" "E=200 GPa"
stress = 1000000000 Pa
strain = 0.005
dL = 0.01 m
  by: Tension or compression: stress, strain and change of length (Young's modulus)  (stress = F/A; strain = stress/E; dL = strain L0)
  F = 1000 N
  A = 1e-06 m^2
  L0 = 2 m
  E = 2e+11 Pa
  assumes: A, L0, E > 0; linear elastic (|strain| < 1)
```

In the worked example, F = 1000 N, A = 1 mm^2, L0 = 2 m, E = 200 GPa go through the equation above and give stress = 1000000000 Pa, strain = 0.005, dL = 0.01 m. It applies only when A, L0, E > 0; linear elastic (|strain| < 1): outside that it says what is wrong instead of returning a number.

### Electricity (3)

<a id="charge_current"></a>

#### Charge carried by a steady current: `charge_current`

One equation links charge (Q), current (I) and elapsed time (t). Give all but one of them and it is solved for the one left out, by default charge (Q).

$$
Q = I t
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `I` | in | current | A |
| `t` | in | elapsed time | s |
| `Q` | out | charge | C |

**Applies when:** steady current.

```shell
$ classical-physics solve charge_current "I=2 A" "t=30 s"
charge (Q) = 60 C
  by: Charge carried by a steady current  (Q = I*t)
  I = 2 A   [2 A]
  t = 30 s   [30 s]
  assumes: steady current
```

**What the worked example means:** This calculation determines the total electric charge, denoted as Q, that has passed through a conductor. It is derived by multiplying the steady current, represented by I, by the elapsed time, labeled as t. The result, 60 C, indicates the cumulative quantity of electricity transported during this interval. This relationship holds because current is defined as the rate of flow of charge, so integrating a constant rate over a specific duration yields the total amount transferred.

<a id="electric_power"></a>

#### Electric power: `electric_power`

One equation links electric power (P), voltage (V) and current (I). Give all but one of them and it is solved for the one left out, by default electric power (P).

$$
P = V I
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `V` | in | voltage | V |
| `I` | in | current | A |
| `P` | out | electric power | W |

**Applies when:** steady direct current.

```shell
$ classical-physics solve electric_power "V=12 V" "I=3 A"
electric power (P) = 36 W
  by: Electric power  (P = V*I)
  V = 12 V   [12 V]
  I = 3 A   [3 A]
  assumes: steady direct current
```

**What the worked example means:** This calculation determines the electric power, which represents the rate at which electrical energy is transferred by a circuit. It is derived from the product of voltage and current under steady direct current conditions. The resulting value of 36 W indicates how much work the electrical device performs per unit of time, serving as a key metric for sizing power supplies and assessing system efficiency.

<a id="ohm"></a>

#### Voltage across an ohmic resistance: `ohm`

One equation links voltage (V), current (I) and resistance (R). Give all but one of them and it is solved for the one left out, by default voltage (V).

$$
V = I R
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `I` | in | current | A |
| `R` | in | resistance | ohm |
| `V` | out | voltage | V |

**Applies when:** ohmic element at a fixed temperature.

```shell
$ classical-physics solve ohm "I=3 A" "R=4 ohm"
voltage (V) = 12 V
  by: Voltage across an ohmic resistance  (V = I*R)
  I = 3 A   [3 A]
  R = 4 ohm   [4 ohm]
  assumes: ohmic element at a fixed temperature
```

**What the worked example means:** This calculation determines the electrical potential difference, or voltage, that develops across a resistive component when a specific electric current passes through it. Multiplying the measured current 3 A flowing through the element by its resistance value 4 ohm gives the resulting voltage drop 12 V. This relationship assumes the material behaves linearly at a constant temperature, meaning the resistance does not change as the current varies. The result indicates how much energy per unit charge is dissipated as heat within the resistor due to the flow of electrons.

### Electromagnetic induction (2)

<a id="faraday_induced_emf"></a>

#### Faraday's law: average emf induced by a change of magnetic flux, `faraday_induced_emf`

The flux through a coil before and after a change of field or area, and the average emf induced in its N turns during the change (with Lenz's sign).

$$
\mathcal{E} = -N\,\frac{\Delta\Phi}{\Delta t},\quad \Phi = BA\cos\theta
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `N` | in | number of turns | - |
| `A1` | in | initial area of the coil | m^2 |
| `B1` | in | initial magnetic field | T |
| `B2` | in | final magnetic field | T |
| `dt` | in | duration of the change | s |
| `A2` | in | final area of the coil (default: unchanged) (optional) | m^2 |
| `theta` | in | angle between the field and the coil's axis (default 0) (optional) | rad |
| `emf` | out | magnitude of the average induced emf | V |
| `emf_signed` | out | average induced emf with Lenz's sign, -N dPhi/dt | V |
| `phi1` | out | initial flux through one turn | Wb |
| `phi2` | out | final flux through one turn | Wb |
| `dphi` | out | change of flux through one turn | Wb |

**Applies when:** N >= 1; dt > 0; areas >= 0; uniform field over the coil.

```shell
$ classical-physics solve faraday_induced_emf N=200 "A1=100 cm^2" "B1=0 T" "B2=0.5 T" "dt=0.1 s"
phi1 = 0 Wb
phi2 = 0.005 Wb
dphi = 0.005 Wb
emf = 10 V
emf_signed = -10 V
  by: Faraday's law: average emf induced by a change of magnetic flux  (Phi = B A cos(theta); emf = -N (Phi2 - Phi1) / dt)
  A1 = 0.01 m^2
  B1 = 0 T
  B2 = 0.5 T
  dt = 0.1 s
  assumes: N >= 1; dt > 0; areas >= 0; uniform field over the coil
```

In the worked example, N = 200, A1 = 100 cm^2, B1 = 0 T, B2 = 0.5 T, dt = 0.1 s go through the equation above and give phi1 = 0 Wb, phi2 = 0.005 Wb, dphi = 0.005 Wb, emf = 10 V. It applies only when N >= 1; dt > 0; areas >= 0; uniform field over the coil: outside that it says what is wrong instead of returning a number.

<a id="ideal_transformer_secondary"></a>

#### Ideal transformer: secondary voltage and current, `ideal_transformer_secondary`

Secondary voltage (and, when the primary current is given, the secondary current) of an ideal transformer from its turns, and whether it steps the voltage up or down.

$$
\frac{V_s}{V_p} = \frac{N_s}{N_p} = \frac{I_p}{I_s}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Np` | in | number of turns of the primary | - |
| `Ns` | in | number of turns of the secondary | - |
| `Vp` | in | primary voltage (rms) | V |
| `Ip` | in | primary current (rms), if known (optional) | A |
| `Vs` | out | secondary voltage | V |
| `Is` | out | secondary current (when Ip is given) | A |
| `ratio` | out | turns ratio Ns / Np | - |
| `type` | out | step-up, step-down or isolation | - |

**Applies when:** Np, Ns >= 1; Vp > 0; ideal: no losses, no magnetizing current.

```shell
$ classical-physics solve ideal_transformer_secondary Np=500 Ns=50 "Vp=230 V" "Ip=0.5 A"
Vs = 23 V
ratio = 0.1
type = step-down
Is = 5 A
  by: Ideal transformer: secondary voltage and current  (Vs / Vp = Ns / Np; Np Ip = Ns Is)
  Vp = 230 V
  Ip = 0.5 A
  assumes: Np, Ns >= 1; Vp > 0; ideal: no losses, no magnetizing current
```

In the worked example, Np = 500, Ns = 50, Vp = 230 V, Ip = 0.5 A go through the equation above and give Vs = 23 V, ratio = 0.1, type = step-down, Is = 5 A. It applies only when Np, Ns >= 1; Vp > 0; ideal: no losses, no magnetizing current: outside that it says what is wrong instead of returning a number.

### Electrostatics (6)

<a id="coulomb_force"></a>

#### Coulomb's law: force between two point charges, `coulomb_force`

The electrostatic force between two point charges a distance r apart: its magnitude and its sign along the line joining them (positive: repulsion, negative: attraction).

$$
F = k\,\frac{|q_1 q_2|}{r^2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `q1` | in | first charge (with its sign) | C |
| `q2` | in | second charge (with its sign) | C |
| `r` | in | distance between the charges | m |
| `F` | out | magnitude of the electrostatic force | N |
| `F_radial` | out | force along the line joining the charges (positive: repulsion) | N |

**Applies when:** r > 0; point charges in vacuum (or air).

```shell
$ classical-physics solve coulomb_force "q1=2 uC" "q2=3 uC" "r=10 cm"
F = 5.392531075 N
F_radial = 5.392531075 N
  by: Coulomb's law: force between two point charges  (F = k q1 q2 / r^2 with k = 8.9875517923e9 N m^2/C^2; the sign of q1 q2 tells repulsion or attraction)
  q1 = 2e-06 C
  q2 = 3e-06 C
  r = 0.1 m
  assumes: r > 0; point charges in vacuum (or air)
```

In the worked example, q1 = 2 uC, q2 = 3 uC, r = 10 cm go through the equation above and give F = 5.392531075 N, F_radial = 5.392531075 N. It applies only when r > 0; point charges in vacuum (or air): outside that it says what is wrong instead of returning a number.

<a id="point_charge_field"></a>

#### Electric field of a point charge: `point_charge_field`

One equation links field strength at distance r (Ef), magnitude of the source charge (Q), distance from the charge (r) and Coulomb constant (ke). Give all but one of them and it is solved for the one left out, by default field strength at distance r (Ef).

$$
E_{f} = \frac{k_{e} Q}{r^{2}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Q` | in | magnitude of the source charge | C |
| `r` | in | distance from the charge | m |
| `ke` | in | Coulomb constant (optional) | N*m^2/C^2 |
| `Ef` | out | field strength at distance r | N/C |

**Applies when:** a point (or spherically symmetric) charge; vacuum or air; outside a point or spherical charge.

```shell
$ classical-physics solve point_charge_field "Q=2e-06 C" "r=0.3 m" "ke=8987551792 N*m^2/C^2"
field strength at distance r (Ef) = 199723.3732 N/C
  by: Electric field of a point charge  (Ef = ke*Q/r**2)
  Q = 2e-06 C   [2e-06 C]
  r = 0.3 m   [0.3 m]
  ke = 8987551792 N*m^2/C^2   [8.98755e+09 N*m^2/C^2]
  assumes: a point (or spherically symmetric) charge; vacuum or air; the situation is: electric
```

**What the worked example means:** This calculation determines the electric field strength generated by a point charge. Applying Coulomb's constant to the source charge magnitude and dividing by the square of the distance shows that the field strength is 199723 N/C N/C. This result reflects the inverse-square nature of electrostatic forces in a vacuum or air.

<a id="point_charge_potential"></a>

#### Electric potential of a point charge: `point_charge_potential`

One equation links electric potential relative to infinity (V), source charge (Q), distance from the charge (r) and Coulomb constant (ke). Give all but one of them and it is solved for the one left out, by default electric potential relative to infinity (V).

$$
V = \frac{k_{e} Q}{r}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Q` | in | source charge | C |
| `r` | in | distance from the charge | m |
| `ke` | in | Coulomb constant (optional) | N*m^2/C^2 |
| `V` | out | electric potential relative to infinity | V |

**Applies when:** a point charge; potential zero at infinity; vacuum or air; outside a point or spherical charge.

```shell
$ classical-physics solve point_charge_potential "Q=3e-09 C" "r=0.5 m" "ke=8987551792 N*m^2/C^2"
electric potential relative to infinity (V) = 53.92531075 V
  by: Electric potential of a point charge  (V = ke*Q/r)
  Q = 3e-09 C   [3e-09 C]
  r = 0.5 m   [0.5 m]
  ke = 8987551792 N*m^2/C^2   [8.98755e+09 N*m^2/C^2]
  assumes: a point charge; potential zero at infinity; vacuum or air; the situation is: electric
```

**What the worked example means:** This calculation finds the electric potential relative to infinity for a point charge. Using the source charge 3e-09 C and the observation distance 0.5 m, the result is 53.9253 V. This value represents the work per unit charge required to bring a test charge from infinity to that specific location in the field.

<a id="electric_force_in_field"></a>

#### Force on a charge in an electric field: `electric_force_in_field`

One equation links magnitude of the electric force (F), magnitude of the charge (q) and electric field strength at the charge (Ef). Give all but one of them and it is solved for the one left out, by default magnitude of the electric force (F).

$$
F = q E_{f}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `q` | in | magnitude of the charge | C |
| `Ef` | in | electric field strength at the charge | N/C |
| `F` | out | magnitude of the electric force | N |

**Applies when:** magnitudes only; a charge sits in a known electric field.

```shell
$ classical-physics solve electric_force_in_field "q=2e-06 C" "Ef=30000 N/C"
magnitude of the electric force (F) = 0.06 N
  by: Force on a charge in an electric field  (F = q*Ef)
  q = 2e-06 C   [2e-06 C]
  Ef = 30000 N/C   [30000 N/C]
  assumes: magnitudes only; the situation is: electric
```

**What the worked example means:** This calculation determines the magnitude of the electric force exerted on a charged particle situated within an external electric field. The result, 0.06 N, is derived by multiplying the magnitude of the charge, 2e-06 C, by the electric field strength at that location, 30000 N/C. This relationship defines how strongly the field acts upon the charge, which is essential for predicting particle motion in devices like capacitors.

<a id="gauss_law_symmetric_field"></a>

#### Gauss's law: electric field of a line, a plane, a charged sphere or a shell, `gauss_law_symmetric_field`

The electric field of a symmetric charge distribution by Gauss's law: an infinite line (charge Q on a length L), an infinite plane sheet (charge Q on an area A), a uniformly charged solid sphere or a thin spherical shell of radius R, inside or outside; with the enclosed charge, the flux through the Gaussian surface and the charge density.

$$
\oint \vec E\cdot d\vec A = \frac{Q_{enc}}{\varepsilon_0}:\quad E_{line} = \frac{\lambda}{2\pi\varepsilon_0 r},\ E_{plane} = \frac{\sigma}{2\varepsilon_0},\ E_{sphere} = \frac{kQr}{R^3}\ (r<R)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `geometry` | in | line, plane, sphere (uniformly charged solid ball) or shell (thin shell, or a conducting sphere) | - |
| `Q` | in | charge (with its sign): on the length L, the area A, or the whole sphere or shell | C |
| `r` | in | distance from the line, or from the centre (not needed for a plane) (optional) | m |
| `L` | in | length of line that carries Q (line only; lambda = Q/L) (optional) | m |
| `A` | in | area of plane that carries Q (plane only; sigma = Q/A) (optional) | m^2 |
| `R` | in | radius of the sphere or shell (sphere and shell only) (optional) | m |
| `E` | out | magnitude of the electric field | N/C |
| `E_radial` | out | field component away from the line, plane or centre | N/C |
| `Q_enc` | out | charge enclosed by the Gaussian surface through the point | C |
| `flux` | out | electric flux through the Gaussian surface, Q_enc/eps0 in N m^2/C | - |
| `density` | out | charge density: lambda in C/m (line), sigma in C/m^2 (plane, shell), rho in C/m^3 (sphere) | - |

**Applies when:** r > 0, L > 0, A > 0, R > 0; line and plane treated as infinite; uniform charge; on a shell (r = R) the field is refused; charges in vacuum (or air).

```shell
$ classical-physics solve gauss_law_symmetric_field geometry=line "Q=2 uC" "L=1 m" "r=10 cm"
E = 359502.0717 N/C
E_radial = 359502.0717 N/C
Q_enc = 2e-06 C
flux = 225881.8135
density = 2e-06
  by: Gauss's law: electric field of a line, a plane, a charged sphere or a shell  (line E = lambda/(2 pi eps0 r); plane E = sigma/(2 eps0); sphere E = k Q r/R^3 inside, k Q/r^2 outside; shell E = 0 inside, k Q/r^2 outside; flux = Q_enc/eps0)
  Q = 2e-06 C
  L = 1 m
  r = 0.1 m
  geometry = line
  assumes: r > 0, L > 0, A > 0, R > 0; line and plane treated as infinite; uniform charge; on a shell (r = R) the field is refused; charges in vacuum (or air)
```

In the worked example, geometry = line, Q = 2 uC, L = 1 m, r = 10 cm go through the equation above and give E = 359502.0717 N/C, E_radial = 359502.0717 N/C, Q_enc = 2e-06 C, flux = 225881.8135. It applies only when r > 0, L > 0, A > 0, R > 0; line and plane treated as infinite; uniform charge; on a shell (r = R) the field is refused; charges in vacuum (or air): outside that it says what is wrong instead of returning a number.

<a id="uniform_field_potential"></a>

#### Potential difference in a uniform field: `uniform_field_potential`

One equation links potential difference between the plates (V), uniform field strength (Ef) and separation along the field (d). Give all but one of them and it is solved for the one left out, by default potential difference between the plates (V).

$$
V = E_{f} d
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Ef` | in | uniform field strength | N/C |
| `d` | in | separation along the field | m |
| `V` | out | potential difference between the plates | V |

**Applies when:** a uniform electric field; between parallel plates far from their edges.

```shell
$ classical-physics solve uniform_field_potential "Ef=2000 N/C" "d=0.005 m"
potential difference between the plates (V) = 10 V
  by: Potential difference in a uniform field  (V = Ef*d)
  Ef = 2000 N/C   [2000 N/C]
  d = 0.005 m   [0.005 m]
  assumes: a uniform electric field; the situation is: uniform field
```

**What the worked example means:** This calculation determines the potential difference between two points in a uniform electric field. Multiplying the uniform field strength, measured as 2000 N/C, by the distance along the field lines, given as 0.005 m, gives the voltage difference of 10 V. This result represents the work done per unit charge to move between the plates or points, assuming the field remains constant throughout the region.

### Energy (9)

<a id="power_energy"></a>

#### Average power: `power_energy`

One equation links average power (P), energy transferred (E) and elapsed time (t). Give all but one of them and it is solved for the one left out, by default average power (P).

$$
P = \frac{E}{t}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `E` | in | energy transferred | J |
| `t` | in | elapsed time | s |
| `P` | out | average power | W |

**Applies when:** power averaged over the interval.

```shell
$ classical-physics solve power_energy "E=600 J" "t=20 s"
average power (P) = 30 W
  by: Average power  (P = E/t)
  E = 600 J   [600 J]
  t = 20 s   [20 s]
  assumes: power averaged over the interval
```

**What the worked example means:** This calculation determines the average rate at which energy is transferred or converted. Dividing the total energy transferred, represented as 600 J, by the elapsed time duration, shown as 20 s, gives the average power value of 30 W. This result indicates how quickly work was done or fuel was consumed during that specific period, providing a single metric for performance over time rather than an instantaneous snapshot.

<a id="spring_energy"></a>

#### Energy stored in a linear spring: `spring_energy`

One equation links elastic potential energy (U), spring constant (k) and extension or compression (x). Give all but one of them and it is solved for the one left out, by default elastic potential energy (U).

$$
U = \frac{k x^{2}}{2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `k` | in | spring constant | N/m |
| `x` | in | extension or compression | m |
| `U` | out | elastic potential energy | J |

**Applies when:** linear elastic spring within its elastic limit.

```shell
$ classical-physics solve spring_energy "k=400 N/m" "x=0.1 m"
elastic potential energy (U) = 2 J
  by: Energy stored in a linear spring  (U = k*x**2/2)
  k = 400 N/m   [400 N/m]
  x = 0.1 m   [0.1 m]
  assumes: linear elastic spring within its elastic limit
```

**What the worked example means:** This calculation determines the elastic potential energy stored in a linear spring based on its stiffness and deformation. Using the spring constant of 400 N/m and an extension or compression of 0.1 m, the resulting energy is 2 J. This value represents the work done to deform the spring from its natural length, indicating how much energy is available to be recovered when the spring returns to equilibrium.

<a id="spring_force"></a>

#### Force of a linear spring: `spring_force`

One equation links spring force (magnitude) (F), spring constant (k) and extension or compression (x). Give all but one of them and it is solved for the one left out, by default spring force (magnitude) (F).

$$
F = k x
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `k` | in | spring constant | N/m |
| `x` | in | extension or compression | m |
| `F` | out | spring force (magnitude) | N |

**Applies when:** linear elastic spring within its elastic limit.

```shell
$ classical-physics solve spring_force "k=300 N/m" "x=5 cm"
spring force (magnitude) (F) = 15 N
  by: Force of a linear spring  (F = k*x)
  k = 300 N/m   [300 N/m]
  x = 0.05 m   [5 cm]
  assumes: linear elastic spring within its elastic limit
```

**What the worked example means:** This calculation determines the restoring force exerted by an ideal linear spring. It uses the given spring constant and the magnitude of extension or compression to find that the resulting force is 15 N. This value represents the effort required to maintain the deformation or the push back against it.

<a id="gravitational_pe"></a>

#### Gravitational potential energy near the surface, `gravitational_pe`

One equation links gravitational potential energy (U), mass (m), gravitational acceleration (g) and height above the reference level (h). Give all but one of them and it is solved for the one left out, by default gravitational potential energy (U).

$$
U = m g h
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `m` | in | mass | kg |
| `g` | in | gravitational acceleration (optional) | m/s^2 |
| `h` | in | height above the reference level | m |
| `U` | out | gravitational potential energy | J |

**Applies when:** uniform gravitational field; zero of potential energy at h = 0.

```shell
$ classical-physics solve gravitational_pe "m=2 kg" "g=9.8 m/s^2" "h=5 m"
gravitational potential energy (U) = 98 J
  by: Gravitational potential energy near the surface  (U = m*g*h)
  m = 2 kg   [2 kg]
  g = 9.8 m/s^2   [9.8 m/s^2]
  h = 5 m   [5 m]
  assumes: uniform gravitational field; zero of potential energy at h = 0
```

**What the worked example means:** This calculation determines the gravitational potential energy of an object based on its mass, the local gravitational acceleration, and its height above a chosen reference level. The result, 98 J, represents the energy stored in the system due to the object's position within a uniform gravitational field. This value indicates how much work would be required to lift the object from the reference height to its current position, or how much energy could be released if it fell back down.

<a id="power_force_speed"></a>

#### Power of a force on a moving body: `power_force_speed`

One equation links power (P), force along the motion (F) and speed (v). Give all but one of them and it is solved for the one left out, by default power (P).

$$
P = F v
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `F` | in | force along the motion | N |
| `v` | in | speed | m/s |
| `P` | out | power | W |

**Applies when:** force parallel to the velocity; constant speed.

```shell
$ classical-physics solve power_force_speed "F=40 N" "v=3 m/s"
power (P) = 120 W
  by: Power of a force on a moving body  (P = F*v)
  F = 40 N   [40 N]
  v = 3 m/s   [3 m/s]
  assumes: force parallel to the velocity; constant speed
```

**What the worked example means:** This calculation finds the instantaneous power 120 W delivered to a body. It uses the force along the motion 40 N and the constant speed 3 m/s. The result shows how quickly energy is transferred by the force acting on the moving object.

<a id="spring_launch_speed"></a>

#### Speed of a body launched by a spring: `spring_launch_speed`

One equation links spring constant (k), compression of the spring before release (x), mass of the body (m) and speed when the spring has returned to its natural length (v). Give all but one of them and it is solved for the one left out, by default .

$$
\frac{k x^{2}}{2} = \frac{m v^{2}}{2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `k` | in | spring constant | N/m |
| `x` | in | compression of the spring before release | m |
| `m` | in | mass of the body | kg |
| `v` | in | speed when the spring has returned to its natural length | m/s |

**Applies when:** all the energy stored in the spring becomes kinetic energy of the body; no friction; the spring is massless; the body starts at rest against the compressed spring and is released; the surface is frictionless and level.

```shell
$ classical-physics solve spring_launch_speed "k=100 N/m" "x=0.2 m" "m=1 kg"
speed when the spring has returned to its natural length (v) = 2 m/s
  by: Speed of a body launched by a spring  (k*x**2/2 = m*v**2/2)
  k = 100 N/m   [100 N/m]
  x = 0.2 m   [0.2 m]
  m = 1 kg   [1 kg]
  assumes: all the energy stored in the spring becomes kinetic energy of the body; no friction; the spring is massless; the situation is: spring launch, frictionless
```

**What the worked example means:** This calculation finds the final speed 2 m/s of a body launched by a spring. It uses the principle that all elastic potential energy stored in the compressed spring converts into kinetic energy of the body. The result depends on the spring constant 100 N/m, the initial compression distance 0.2 m, and the mass of the body 1 kg.

<a id="kinetic_energy"></a>

#### Translational kinetic energy: `kinetic_energy`

One equation links kinetic energy (K), mass (m) and speed (v). Give all but one of them and it is solved for the one left out, by default kinetic energy (K).

$$
K = \frac{m v^{2}}{2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `m` | in | mass | kg |
| `v` | in | speed | m/s |
| `K` | out | kinetic energy | J |

**Applies when:** speeds far below the speed of light.

```shell
$ classical-physics solve kinetic_energy "m=4 kg" "v=3 m/s"
kinetic energy (K) = 18 J
  by: Translational kinetic energy  (K = m*v**2/2)
  m = 4 kg   [4 kg]
  v = 3 m/s   [3 m/s]
  assumes: speeds far below the speed of light
```

**What the worked example means:** This calculation determines the translational kinetic energy, which represents the work needed to accelerate an object from rest to its current state. Using the mass of 4 kg and the speed of 3 m/s, the resulting energy is 18 J. This value quantifies the capacity of the moving body to do work upon impact or interaction with other objects.

<a id="work"></a>

#### Work by a constant force: `work`

One equation links work done (W), force (F), displacement (d) and angle between force and displacement (theta). Give all but one of them and it is solved for the one left out, by default work done (W).

$$
W = F d \cos\left(\theta\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `F` | in | force | N |
| `d` | in | displacement | m |
| `theta` | in | angle between force and displacement (optional) | rad |
| `W` | out | work done | J |

**Applies when:** constant force.

```shell
$ classical-physics solve work "F=10 N" "d=5 m" "theta=60 deg"
work done (W) = 25 J
  by: Work by a constant force  (W = F*d*cos(theta))
  F = 10 N   [10 N]
  d = 5 m   [5 m]
  theta = 1.047197551 rad   [60 deg]
  assumes: constant force
```

**What the worked example means:** This calculation determines the work done, which represents the energy transferred when a constant force acts on an object. The result of 25 J comes from multiplying the force magnitude of 10 N by the displacement distance of 5 m and the cosine of the angle theta between them. This value indicates how much effort was converted into motion or other forms of energy during the displacement.

<a id="net_work_kinetic_energy"></a>

#### Work-energy theorem: `net_work_kinetic_energy`

One equation links net work done on the body, equal to its change in kinetic energy (W), mass (m), final speed (v) and initial speed (v0). Give all but one of them and it is solved for the one left out, by default net work done on the body, equal to its change in kinetic energy (W).

$$
W = \frac{m \left(v^{2} - v_0^{2}\right)}{2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `m` | in | mass | kg |
| `v` | in | final speed | m/s |
| `v0` | in | initial speed | m/s |
| `W` | out | net work done on the body, equal to its change in kinetic energy | J |

**Applies when:** W is the work of ALL forces on the body; constant mass; speeds far below the speed of light; the work stated is the net (total) work on the body.

```shell
$ classical-physics solve net_work_kinetic_energy "m=6 kg" "v0=1 m/s" "v=3 m/s"
net work done on the body, equal to its change in kinetic energy (W) = 24 J
  by: Work-energy theorem  (W = m*(v**2 - v0**2)/2)
  m = 6 kg   [6 kg]
  v0 = 1 m/s   [1 m/s]
  v = 3 m/s   [3 m/s]
  assumes: W is the work of ALL forces on the body; constant mass; speeds far below the speed of light
```

**What the worked example means:** This calculation determines the net work performed on an object by analyzing its change in kinetic energy. Using the mass of the body, its starting speed, and its ending speed, the model computes that the total work equals 24 J. This result represents the combined effect of all forces acting on the object during its motion, showing how much energy was transferred to or from it without needing to know the specific path or individual force details.

### Fluids (6)

<a id="absolute_pressure_depth"></a>

#### Absolute pressure at a depth in a fluid: `absolute_pressure_depth`

One equation links absolute pressure at the depth (P), pressure at the surface (P0), density of the fluid (rho), Acceleration (g) and depth below the surface (h). Give all but one of them and it is solved for the one left out, by default absolute pressure at the depth (P).

$$
P = P_{0} + \rho g h
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `P0` | in | pressure at the surface | Pa |
| `rho` | in | density of the fluid | kg/m^3 |
| `g` | in | Acceleration (optional) | m/s^2 |
| `h` | in | depth below the surface | m |
| `P` | out | absolute pressure at the depth | Pa |

**Applies when:** an incompressible fluid at rest; uniform gravity; a liquid at rest.

```shell
$ classical-physics solve absolute_pressure_depth "P0=101325 Pa" "rho=1000 kg/m^3" "g=9.80665 m/s^2" "h=10 m"
absolute pressure at the depth (P) = 199391.5 Pa
  by: Absolute pressure at a depth in a fluid  (P = P0 + rho*g*h)
  P0 = 101325 Pa   [101325 Pa]
  rho = 1000 kg/m^3   [1000 kg/m^3]
  g = 9.80665 m/s^2   [9.80665 m/s^2]
  h = 10 m   [10 m]
  assumes: an incompressible fluid at rest; uniform gravity; the situation is: fluid depth
```

**What the worked example means:** This calculation determines the total pressure experienced at a specific depth within a stationary, incompressible fluid. It combines the external pressure acting on the surface, represented by 101325 Pa, with the hydrostatic pressure generated by the weight of the fluid column above. The result, 199392 Pa, reflects how density, gravity, and depth contribute to the overall force per unit area at that location.

<a id="flow_rate_volume_time"></a>

#### Flow rate as volume per time: `flow_rate_volume_time`

One equation links volume flow rate (Q), volume delivered (V) and time taken (t). Give all but one of them and it is solved for the one left out, by default volume flow rate (Q).

$$
Q = \frac{V}{t}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `V` | in | volume delivered | m^3 |
| `t` | in | time taken | s |
| `Q` | out | volume flow rate | m^3/s |

**Applies when:** a steady flow; a fluid flows through a pipe or opening.

```shell
$ classical-physics solve flow_rate_volume_time "V=0.002 m^3" "t=10 s"
volume flow rate (Q) = 0.0002 m^3/s
  by: Flow rate as volume per time  (Q = V/t)
  V = 0.002 m^3   [0.002 m^3]
  t = 10 s   [10 s]
  assumes: a steady flow; the situation is: flow
```

**What the worked example means:** The volume flow rate Q is calculated by dividing the total volume of fluid V delivered by the time interval t over which it flowed. This result, 0.0002 m^3/s, represents the average rate at which the fluid passes through the pipe or opening during that steady period.

<a id="flow_rate_area_speed"></a>

#### Flow rate through a cross section: `flow_rate_area_speed`

One equation links volume flow rate (Q), cross-sectional area (A) and average speed of the fluid (v). Give all but one of them and it is solved for the one left out, by default volume flow rate (Q).

$$
Q = A v
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `A` | in | cross-sectional area | m^2 |
| `v` | in | average speed of the fluid | m/s |
| `Q` | out | volume flow rate | m^3/s |

**Applies when:** a steady flow; the average speed over the cross section; a fluid flows through a pipe.

```shell
$ classical-physics solve flow_rate_area_speed "A=0.0003 m^2" "v=2 m/s"
volume flow rate (Q) = 0.0006 m^3/s
  by: Flow rate through a cross section  (Q = A*v)
  A = 0.0003 m^2   [0.0003 m^2]
  v = 2 m/s   [2 m/s]
  assumes: a steady flow; the average speed over the cross section; the situation is: flow
```

**What the worked example means:** This calculation determines the volume flow rate by multiplying the cross-sectional area of the pipe by the average speed of the fluid. The result, 0.0006 m^3/s, represents how much fluid passes through a specific section per unit of time, assuming steady flow conditions where the velocity profile is uniform or averaged across the entire area.

<a id="density"></a>

#### Mass density: `density`

One equation links density (rho), mass (m) and volume (V). Give all but one of them and it is solved for the one left out, by default density (rho).

$$
\rho = \frac{m}{V}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `m` | in | mass | kg |
| `V` | in | volume | m^3 |
| `rho` | out | density | kg/m^3 |

**Applies when:** uniform material, or mean density.

```shell
$ classical-physics solve density "m=6 kg" "V=2 L"
density (rho) = 3000 kg/m^3
  by: Mass density  (rho = m/V)
  m = 6 kg   [6 kg]
  V = 0.002 m^3   [2 L]
  assumes: uniform material, or mean density
```

**What the worked example means:** This calculation determines the mean mass density by dividing the total mass of the object, given as 6 kg, by its total volume, given as 0.002 m^3. The resulting value of 3000 kg/m^3 indicates how much matter is packed into each unit of space. This metric helps identify the material composition and predicts whether the object will float or sink when placed in various fluids.

<a id="hydrostatic_pressure"></a>

#### Pressure of a column of fluid: `hydrostatic_pressure`

One equation links gauge pressure (P), fluid density (rho), gravitational acceleration (g) and depth below the surface (h). Give all but one of them and it is solved for the one left out, by default gauge pressure (P).

$$
P = \rho g h
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `rho` | in | fluid density | kg/m^3 |
| `g` | in | gravitational acceleration (optional) | m/s^2 |
| `h` | in | depth below the surface | m |
| `P` | out | gauge pressure | Pa |

**Applies when:** fluid at rest with uniform density; pressure measured relative to the surface.

```shell
$ classical-physics solve hydrostatic_pressure "rho=1000 kg/m^3" "g=9.8 m/s^2" "h=2 m"
gauge pressure (P) = 19600 Pa
  by: Pressure of a column of fluid  (P = rho*g*h)
  rho = 1000 kg/m^3   [1000 kg/m^3]
  g = 9.8 m/s^2   [9.8 m/s^2]
  h = 2 m   [2 m]
  assumes: fluid at rest with uniform density; pressure measured relative to the surface
```

**What the worked example means:** This calculation determines the gauge pressure at a specific depth within a stationary fluid. It uses the formula P equals rho times g times h, where rho represents the fluid density, g is the gravitational acceleration, and h is the vertical distance below the surface. The resulting pressure value of 19600 Pa indicates how much greater the pressure is at that depth compared to the surface level, assuming the fluid has uniform density.

<a id="pressure_force"></a>

#### Pressure of a force on an area: `pressure_force`

One equation links pressure (P), force normal to the surface (F) and area (A). Give all but one of them and it is solved for the one left out, by default pressure (P).

$$
P = \frac{F}{A}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `F` | in | force normal to the surface | N |
| `A` | in | area | m^2 |
| `P` | out | pressure | Pa |

**Applies when:** force spread evenly and normal to the surface.

```shell
$ classical-physics solve pressure_force "F=200 N" "A=0.04 m^2"
pressure (P) = 5000 Pa
  by: Pressure of a force on an area  (P = F/A)
  F = 200 N   [200 N]
  A = 0.04 m^2   [0.04 m^2]
  assumes: force spread evenly and normal to the surface
```

**What the worked example means:** This calculation determines the pressure exerted by a normal force distributed evenly over a specific surface area. The result, 5000 Pa, indicates how concentrated the force of 200 N is across the area of 0.04 m^2. A higher value signifies that the same force is applied over a smaller region, leading to greater intensity per unit area.

### Forces (2)

<a id="static_friction_incline"></a>

#### Static friction on an incline: critical angle and whether a body slides, `static_friction_incline`

The steepest incline on which a body can rest (tan(theta_c) = mu_s), whether it slides at the given angle and, with its mass, the normal force, the friction needed and the friction available.

$$
\tan\theta_c = \mu_s
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `mu_s` | in | coefficient of static friction | - |
| `theta` | in | angle of the incline above the horizontal | rad |
| `m` | in | mass of the body, if the forces are wanted (optional) | kg |
| `theta_c` | out | critical (steepest holding) angle | rad |
| `theta_c_deg` | out | critical angle in degrees | - |
| `slips` | out | whether the body slides at the given angle | - |
| `N` | out | normal force (when m is given) | N |
| `f_needed` | out | friction needed to hold it (when m is given) | N |
| `f_max` | out | maximum static friction available (when m is given) | N |

**Applies when:** mu_s >= 0; 0 <= theta < 90 degrees; g = 9.80665 m/s^2.

```shell
$ classical-physics solve static_friction_incline "mu_s=0.5 1" "theta=20 deg"
theta_c = 0.463647609 rad
theta_c_deg = 26.56505118
slips = false
  by: Static friction on an incline: critical angle and whether a body slides  (theta_c = atan(mu_s); slides when tan(theta) > mu_s; N = m g cos(theta), f_needed = m g sin(theta), f_max = mu_s N)
  mu_s = 0.5
  theta = 0.3490658504 rad
  assumes: mu_s >= 0; 0 <= theta < 90 degrees; g = 9.80665 m/s^2
```

In the worked example, mu_s = 0.5 1, theta = 20 deg go through the equation above and give theta_c = 0.463647609 rad, theta_c_deg = 26.56505118, slips = False. It applies only when mu_s >= 0; 0 <= theta < 90 degrees; g = 9.80665 m/s^2: outside that it says what is wrong instead of returning a number.

<a id="static_friction_limit"></a>

#### Static friction: maximum friction force and whether a body slips, `static_friction_limit`

The largest static friction force mu_s N and whether an applied force parallel to the surface makes the body slip.

$$
f_s \le \mu_s N
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `mu_s` | in | coefficient of static friction | - |
| `N` | in | normal force | N |
| `F` | in | applied force parallel to the surface (magnitude) | N |
| `f_max` | out | maximum static friction force | N |
| `slips` | out | whether the body slips | - |
| `margin` | out | f_max - F (positive: it holds) | N |

**Applies when:** mu_s >= 0; N > 0; F >= 0.

```shell
$ classical-physics solve static_friction_limit "mu_s=0.5 1" "N=100 N" "F=40 N"
f_max = 50 N
slips = false
margin = 10 N
  by: Static friction: maximum friction force and whether a body slips  (f_max = mu_s N; it slips when F > f_max (a force equal to the limit holds it on the verge))
  mu_s = 0.5
  N = 100 N
  F = 40 N
  assumes: mu_s >= 0; N > 0; F >= 0
```

In the worked example, mu_s = 0.5 1, N = 100 N, F = 40 N go through the equation above and give f_max = 50 N, slips = False, margin = 10 N. It applies only when mu_s >= 0; N > 0; F >= 0: outside that it says what is wrong instead of returning a number.

### Geometric optics (3)

<a id="critical_angle_total_internal_reflection"></a>

#### Critical angle for total internal reflection, `critical_angle_total_internal_reflection`

The smallest angle of incidence (from the normal) at which light in the denser medium is totally reflected at the boundary with a less dense one.

$$
\sin\theta_c = \frac{n_2}{n_1}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `n1` | in | refractive index of the medium the light travels in (the denser one) | - |
| `n2` | in | refractive index of the medium beyond the boundary | - |
| `theta_c` | out | critical angle from the normal | rad |
| `theta_c_deg` | out | critical angle in degrees | - |

**Applies when:** n1 > n2 >= 1.

```shell
$ classical-physics solve critical_angle_total_internal_reflection "n1=1.5 1" "n2=1 1"
theta_c = 0.7297276562 rad
theta_c_deg = 41.8103149
  by: Critical angle for total internal reflection  (sin(theta_c) = n2 / n1)
  n1 = 1.5
  n2 = 1
  assumes: n1 > n2 >= 1
```

In the worked example, n1 = 1.5 1, n2 = 1 1 go through the equation above and give theta_c = 0.7297276562 rad, theta_c_deg = 41.8103149. It applies only when n1 > n2 >= 1: outside that it says what is wrong instead of returning a number.

<a id="lens_mirror_magnification"></a>

#### Magnification and image height of a thin lens or a spherical mirror, `lens_mirror_magnification`

Magnification, image height, image distance and the nature of the image (real or virtual, upright or inverted) from the object distance and either the image distance or the focal length.

$$
m = \frac{h_i}{h_o} = -\frac{d_i}{d_o},\quad \frac{1}{d_o} + \frac{1}{d_i} = \frac{1}{f}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `d_o` | in | object distance (positive) | m |
| `h_o` | in | object height | m |
| `d_i` | in | image distance (positive: real image), if known (optional) | m |
| `f` | in | focal length (positive: converging lens or concave mirror), if d_i is not given (optional) | m |
| `m` | out | magnification (negative: inverted image) | - |
| `h_i` | out | image height (negative: inverted) | m |
| `d_i` | out | image distance (positive: real, negative: virtual) | m |
| `image` | out | real or virtual, upright or inverted | - |

**Applies when:** d_o > 0, h_o > 0; exactly one of d_i and f; the object not at the focal point.

```shell
$ classical-physics solve lens_mirror_magnification "d_o=30 cm" "f=10 cm" "h_o=2 cm"
m = -0.5
h_i = -0.01 m
d_i = 0.15 m
image = real, inverted
  by: Magnification and image height of a thin lens or a spherical mirror  (d_i = d_o f / (d_o - f) when f is given; m = -d_i / d_o; h_i = m h_o)
  d_o = 0.3 m
  f = 0.1 m
  h_o = 0.02 m
  assumes: d_o > 0, h_o > 0; exactly one of d_i and f; the object not at the focal point
```

In the worked example, d_o = 30 cm, f = 10 cm, h_o = 2 cm go through the equation above and give m = -0.5 1, h_i = -0.01 m, d_i = 0.15 m, image = real, inverted. It applies only when d_o > 0, h_o > 0; exactly one of d_i and f; the object not at the focal point: outside that it says what is wrong instead of returning a number.

<a id="snell_refraction_angle"></a>

#### Snell's law: angle of refraction, `snell_refraction_angle`

The angle of refraction of a ray passing from a medium of index n1 into one of index n2; refused with a reason when the ray is totally internally reflected.

$$
n_1\sin\theta_1 = n_2\sin\theta_2
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `n1` | in | refractive index of the incident medium | - |
| `n2` | in | refractive index of the second medium | - |
| `theta1` | in | angle of incidence from the normal (deg or rad) | rad |
| `theta2` | out | angle of refraction from the normal | rad |
| `theta2_deg` | out | angle of refraction in degrees | - |

**Applies when:** n1, n2 >= 1; 0 <= theta1 <= 90 degrees; n1 sin(theta1) <= n2 (no total internal reflection).

```shell
$ classical-physics solve snell_refraction_angle "n1=1 1" "n2=1.5 1" "theta1=30 deg"
theta2 = 0.3398369095 rad
theta2_deg = 19.47122063
  by: Snell's law: angle of refraction  (sin(theta2) = n1 sin(theta1) / n2)
  n1 = 1
  n2 = 1.5
  theta1 = 0.5235987756 rad
  assumes: n1, n2 >= 1; 0 <= theta1 <= 90 degrees; n1 sin(theta1) <= n2 (no total internal reflection)
```

In the worked example, n1 = 1 1, n2 = 1.5 1, theta1 = 30 deg go through the equation above and give theta2 = 0.3398369095 rad, theta2_deg = 19.47122063. It applies only when n1, n2 >= 1; 0 <= theta1 <= 90 degrees; n1 sin(theta1) <= n2 (no total internal reflection): outside that it says what is wrong instead of returning a number.

### Gravitation (4)

<a id="surface_gravity"></a>

#### Gravitational acceleration at a distance from a mass, `surface_gravity`

One equation links gravitational acceleration (g), mass of the planet or star (M), distance from the centre (r) and Gravitational constant (G). Give all but one of them and it is solved for the one left out, by default gravitational acceleration (g).

$$
g = \frac{G M}{r^{2}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `M` | in | mass of the planet or star | kg |
| `r` | in | distance from the centre | m |
| `G` | in | Gravitational constant (optional) | N*m^2/kg^2 |
| `g` | out | gravitational acceleration | m/s^2 |

**Applies when:** a spherically symmetric body; outside the body; at or above the surface of a planet or star.

```shell
$ classical-physics solve surface_gravity "M=5.97e+24 kg" "r=6370000 m" "G=6.6743e-11 N*m^2/kg^2"
gravitational acceleration (g) = 9.819767158 m/s^2
  by: Gravitational acceleration at a distance from a mass  (g = G*M/r**2)
  M = 5.97e+24 kg   [5.97e+24 kg]
  r = 6370000 m   [6.37e+06 m]
  G = 6.6743e-11 N*m^2/kg^2   [6.6743e-11 N*m^2/kg^2]
  assumes: a spherically symmetric body; outside the body; the situation is: celestial
```

**What the worked example means:** This calculation determines the gravitational acceleration 9.81977 m/s^2 at a specific location. It uses the universal gravitational constant, the mass of the central body 5.97e+24 kg, and the distance from its center 6.37e+06 m. The result indicates how strongly gravity pulls on an object placed at that point in space.

<a id="gravitational_force_two_masses"></a>

#### Newton's law of universal gravitation: force between two masses, `gravitational_force_two_masses`

The attractive gravitational force between two masses (point masses or spheres) whose centres are r apart.

$$
F = G\,\frac{m_1 m_2}{r^2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `m1` | in | first mass | kg |
| `m2` | in | second mass | kg |
| `r` | in | distance between the centres | m |
| `F` | out | magnitude of the gravitational attraction | N |

**Applies when:** m1 > 0, m2 > 0, r > 0; point masses or spherically symmetric bodies.

```shell
$ classical-physics solve gravitational_force_two_masses "m1=100 kg" "m2=50 kg" "r=2 m"
F = 8.342875e-08 N
  by: Newton's law of universal gravitation: force between two masses  (F = G m1 m2 / r^2 with G = 6.6743e-11 N m^2/kg^2)
  m1 = 100 kg
  m2 = 50 kg
  r = 2 m
  assumes: m1 > 0, m2 > 0, r > 0; point masses or spherically symmetric bodies
```

In the worked example, m1 = 100 kg, m2 = 50 kg, r = 2 m go through the equation above and give F = 8.342875e-08 N. It applies only when m1 > 0, m2 > 0, r > 0; point masses or spherically symmetric bodies: outside that it says what is wrong instead of returning a number.

<a id="orbital_period"></a>

#### Period of a circular orbit: `orbital_period`

One equation links orbital period (T), orbital radius (r), mass of the central body (M) and Gravitational constant (G). Give all but one of them and it is solved for the one left out, by default orbital period (T).

$$
T = 2 \pi \sqrt{\frac{r^{3}}{G M}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `r` | in | orbital radius | m |
| `M` | in | mass of the central body | kg |
| `G` | in | Gravitational constant (optional) | N*m^2/kg^2 |
| `T` | out | orbital period | s |

**Applies when:** a circular orbit; the orbiting body much lighter than the central one; a satellite in a circular orbit.

```shell
$ classical-physics solve orbital_period "r=42200000 m" "M=5.97e+24 kg" "G=6.6743e-11 N*m^2/kg^2"
orbital period (T) = 86289.61224 s
  by: Period of a circular orbit  (T = 2*pi*sqrt(r**3/(G*M)))
  r = 42200000 m   [4.22e+07 m]
  M = 5.97e+24 kg   [5.97e+24 kg]
  G = 6.6743e-11 N*m^2/kg^2   [6.6743e-11 N*m^2/kg^2]
  assumes: a circular orbit; the orbiting body much lighter than the central one; the situation is: orbit
```

**What the worked example means:** This calculation determines the time required for an object to complete one full revolution in a circular orbit. It uses the orbital radius and the mass of the central body, assuming the orbiting object is significantly lighter than the center. The result, 86289.6 s, represents the duration of this cycle, linking the size of the path directly to the speed of travel under gravity.

<a id="orbital_speed"></a>

#### Speed in a circular orbit: `orbital_speed`

One equation links orbital speed (v), mass of the central body (M), orbital radius (r) and Gravitational constant (G). Give all but one of them and it is solved for the one left out, by default orbital speed (v).

$$
v = \sqrt{\frac{G M}{r}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `M` | in | mass of the central body | kg |
| `r` | in | orbital radius | m |
| `G` | in | Gravitational constant (optional) | N*m^2/kg^2 |
| `v` | out | orbital speed | m/s |

**Applies when:** a circular orbit; the orbiting body much lighter than the central one; a satellite in a circular orbit.

```shell
$ classical-physics solve orbital_speed "M=5.97e+24 kg" "r=6770000 m" "G=6.6743e-11 N*m^2/kg^2"
orbital speed (v) = 7671.7721 m/s
  by: Speed in a circular orbit  (v = sqrt(G*M/r))
  M = 5.97e+24 kg   [5.97e+24 kg]
  r = 6770000 m   [6.77e+06 m]
  G = 6.6743e-11 N*m^2/kg^2   [6.6743e-11 N*m^2/kg^2]
  assumes: a circular orbit; the orbiting body much lighter than the central one; the situation is: orbit
```

**What the worked example means:** This calculation finds the orbital speed 7671.77 m/s required for a body to maintain a circular path. It derives this value from the gravitational constant, the mass of the central body 5.97e+24 kg, and the orbital radius 6.77e+06 m. The result represents the precise velocity needed to balance the inward pull of gravity with the object's tendency to move in a straight line, ensuring it stays in orbit without falling back or escaping.

### Induction (3)

<a id="rl_current_decay"></a>

#### Current dying away in an RL circuit: `rl_current_decay`

One equation links current at time t (I), initial current (I0), time since the circuit was switched off (t) and time constant (tau). Give all but one of them and it is solved for the one left out, by default current at time t (I).

$$
I = I_{0} e^{\frac{-t}{\tau}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `I0` | in | initial current | A |
| `t` | in | time since the circuit was switched off | s |
| `tau` | in | time constant | s |
| `I` | out | current at time t | A |

**Applies when:** a single inductor and resistor; the source is removed at t = 0; an RL circuit after the source is switched off.

```shell
$ classical-physics solve rl_current_decay "I0=10 A" "t=0.0025 s" "tau=0.0025 s"
current at time t (I) = 3.678794412 A
  by: Current dying away in an RL circuit  (I = I0*exp(-t/tau))
  I0 = 10 A   [10 A]
  t = 0.0025 s   [0.0025 s]
  tau = 0.0025 s   [0.0025 s]
  assumes: a single inductor and resistor; the source is removed at t = 0; the situation is: inductor, current decay
```

**What the worked example means:** This calculation determines the current flowing through an RL circuit at a specific moment after the power source has been removed. The result, represented as 3.67879 A, shows how much charge continues to flow due to the energy stored in the inductor. This value depends on the starting current 10 A and decays exponentially over time 0.0025 s based on the circuit's characteristic time constant 0.0025 s. A larger time constant means the current persists longer before fading away.

<a id="motional_emf"></a>

#### Emf of a rod moving through a magnetic field, `motional_emf`

One equation links induced emf (emf), magnetic field (B), length of the rod (l) and speed of the rod (v). Give all but one of them and it is solved for the one left out, by default induced emf (emf).

$$
\text{emf} = B l v
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `B` | in | magnetic field | T |
| `l` | in | length of the rod | m |
| `v` | in | speed of the rod | m/s |
| `emf` | out | induced emf | V |

**Applies when:** B, the rod and its velocity mutually perpendicular; a uniform field; a straight rod moving through a uniform magnetic field.

```shell
$ classical-physics solve motional_emf "B=0.5 T" "l=0.2 m" "v=3 m/s"
induced emf (emf) = 0.3 V
  by: Emf of a rod moving through a magnetic field  (emf = B*l*v)
  B = 0.5 T   [0.5 T]
  l = 0.2 m   [0.2 m]
  v = 3 m/s   [3 m/s]
  assumes: B, the rod and its velocity mutually perpendicular; a uniform field; the situation is: moving rod
```

**What the worked example means:** This calculation determines the electromotive force generated across a straight conductor moving through a uniform magnetic field. The result, 0.3 V, arises from the interaction between the magnetic flux density of 0.5 T tesla, the effective rod length of 0.2 m meters, and the perpendicular velocity of 3 m/s meters per second. This voltage represents the conversion of mechanical energy into electrical potential difference due to electromagnetic induction.

<a id="rl_time_constant"></a>

#### Time constant of an RL circuit: `rl_time_constant`

One equation links time constant (tau), inductance (L) and resistance (R). Give all but one of them and it is solved for the one left out, by default time constant (tau).

$$
\tau = \frac{L}{R}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `L` | in | inductance | H |
| `R` | in | resistance | ohm |
| `tau` | out | time constant | s |

**Applies when:** a single inductor and resistor; an RL circuit switched on or off.

```shell
$ classical-physics solve rl_time_constant "L=0.5 H" "R=100 ohm"
time constant (tau) = 0.005 s
  by: Time constant of an RL circuit  (tau = L/R)
  L = 0.5 H   [0.5 H]
  R = 100 ohm   [100 ohm]
  assumes: a single inductor and resistor; the situation is: inductor
```

**What the worked example means:** This calculation determines the time constant for a simple RL circuit using the inductance and resistance values. The resulting time constant, 0.005 s, indicates how quickly the current changes when the circuit is switched on or off.

### Kinematics (9)

<a id="average_velocity_displacement"></a>

#### Displacement from the mean of initial and final velocity, `average_velocity_displacement`

One equation links displacement (x), initial velocity (v0), final velocity (v) and elapsed time (t). Give all but one of them and it is solved for the one left out, by default displacement (x).

$$
x = \frac{\left(v_0 + v\right) t}{2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `v0` | in | initial velocity | m/s |
| `v` | in | final velocity | m/s |
| `t` | in | elapsed time | s |
| `x` | out | displacement | m |

**Applies when:** constant acceleration along a line.

```shell
$ classical-physics solve average_velocity_displacement "v0=4 m/s" "v=10 m/s" "t=3 s"
displacement (x) = 21 m
  by: Displacement from the mean of initial and final velocity  (x = (v0 + v)*t/2)
  v0 = 4 m/s   [4 m/s]
  v = 10 m/s   [10 m/s]
  t = 3 s   [3 s]
  assumes: constant acceleration along a line
```

**What the worked example means:** This calculation determines the total displacement of an object moving in a straight line with constant acceleration. Taking the average of the initial velocity and the final velocity, then multiplying by the elapsed time gives the distance covered without needing to know the specific rate of acceleration. The result indicates that the object moved 21 m during the interval.

<a id="displacement_time"></a>

#### Displacement under constant acceleration: `displacement_time`

One equation links displacement (x), initial velocity (v0), acceleration (a) and elapsed time (t). Give all but one of them and it is solved for the one left out, by default displacement (x).

$$
x = v_0 t + \frac{a t^{2}}{2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `v0` | in | initial velocity | m/s |
| `a` | in | acceleration | m/s^2 |
| `t` | in | elapsed time | s |
| `x` | out | displacement | m |

**Applies when:** constant acceleration along a line.

```shell
$ classical-physics solve displacement_time "v0=3 m/s" "a=2 m/s^2" "t=4 s"
displacement (x) = 28 m
  by: Displacement under constant acceleration  (x = v0*t + a*t**2/2)
  v0 = 3 m/s   [3 m/s]
  a = 2 m/s^2   [2 m/s^2]
  t = 4 s   [4 s]
  assumes: constant acceleration along a line
```

**What the worked example means:** This calculation determines the displacement 28 m by combining the effect of the initial velocity 3 m/s and the constant acceleration 2 m/s^2 acting over the elapsed time 4 s. The result represents how far the object has moved from its starting point along a straight line, assuming the acceleration does not change during this period.

<a id="max_height_vertical_throw"></a>

#### Maximum height of a vertical throw: `max_height_vertical_throw`

One equation links maximum height above the launch point (h), launch speed (v0) and gravitational acceleration (g). Give all but one of them and it is solved for the one left out, by default maximum height above the launch point (h).

$$
h = \frac{v_0^{2}}{2 g}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `v0` | in | launch speed | m/s |
| `g` | in | gravitational acceleration (optional) | m/s^2 |
| `h` | out | maximum height above the launch point | m |

**Applies when:** thrown straight up; no air resistance; uniform gravitational field; the launch is vertical; the height is measured from the launch point.

```shell
$ classical-physics solve max_height_vertical_throw "v0=14 m/s" "g=9.8 m/s^2"
maximum height above the launch point (h) = 10 m
  by: Maximum height of a vertical throw  (h = v0**2/(2*g))
  v0 = 14 m/s   [14 m/s]
  g = 9.8 m/s^2   [9.8 m/s^2]
  assumes: thrown straight up; no air resistance; uniform gravitational field; the situation is: thrown up
```

**What the worked example means:** This calculation determines the peak altitude attained by an object projected vertically upward. By using the initial velocity 14 m/s and the local gravitational acceleration 9.8 m/s^2, the model computes the distance traveled against gravity before the object momentarily stops at its highest point, which is found to be 10 m. This result represents the total vertical displacement from the release point under ideal conditions.

<a id="uniform_motion"></a>

#### Motion at constant velocity: `uniform_motion`

One equation links distance travelled (d), constant speed (v) and elapsed time (t). Give all but one of them and it is solved for the one left out, by default distance travelled (d).

$$
d = v t
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `v` | in | constant speed | m/s |
| `t` | in | elapsed time | s |
| `d` | out | distance travelled | m |

**Applies when:** constant velocity.

```shell
$ classical-physics solve uniform_motion "v=20 m/s" "t=30 s"
distance travelled (d) = 600 m
  by: Motion at constant velocity  (d = v*t)
  v = 20 m/s   [20 m/s]
  t = 30 s   [30 s]
  assumes: constant velocity
```

**What the worked example means:** This calculation determines the total distance travelled by an object moving in a straight line at a steady speed. Multiplying the constant speed of 20 m/s by the elapsed time of 30 s shows that the object covers a distance of 600 m. This result assumes that the velocity remains unchanged throughout the duration, meaning there is no acceleration or change in direction during this period.

<a id="velocity_displacement"></a>

#### Speed after a displacement under constant acceleration, `velocity_displacement`

One equation links final speed (v), initial speed (v0), acceleration (a) and displacement (x). Give all but one of them and it is solved for the one left out, by default .

$$
v^{2} = v_0^{2} + 2 a x
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `v` | in | final speed | m/s |
| `v0` | in | initial speed | m/s |
| `a` | in | acceleration | m/s^2 |
| `x` | in | displacement | m |

**Applies when:** constant acceleration along a line; motion does not reverse.

```shell
$ classical-physics solve velocity_displacement "v0=6 m/s" "a=2 m/s^2" "x=16 m"
final speed (v) = 10 m/s
  by: Speed after a displacement under constant acceleration  (v**2 = v0**2 + 2*a*x)
  v0 = 6 m/s   [6 m/s]
  a = 2 m/s^2   [2 m/s^2]
  x = 16 m   [16 m]
  assumes: constant acceleration along a line; motion does not reverse
```

**What the worked example means:** This calculation determines the final speed 10 m/s given an initial speed 6 m/s, constant acceleration 2 m/s^2, and displacement 16 m. It uses the kinematic equation that relates velocity to position without needing time, assuming straight-line motion where direction does not reverse.

<a id="speed_after_fall_from_rest"></a>

#### Speed after falling from rest: `speed_after_fall_from_rest`

One equation links speed after the fall (v), gravitational acceleration (g) and height fallen (h). Give all but one of them and it is solved for the one left out, by default speed after the fall (v).

$$
v = \sqrt{2 g h}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `g` | in | gravitational acceleration (optional) | m/s^2 |
| `h` | in | height fallen | m |
| `v` | out | speed after the fall | m/s |

**Applies when:** released from rest; no air resistance; the body starts from rest and falls freely.

```shell
$ classical-physics solve speed_after_fall_from_rest "g=9.8 m/s^2" "h=10 m"
speed after the fall (v) = 14 m/s
  by: Speed after falling from rest  (v = sqrt(2*g*h))
  g = 9.8 m/s^2   [9.8 m/s^2]
  h = 10 m   [10 m]
  assumes: released from rest; no air resistance; the situation is: free fall, from rest
```

**What the worked example means:** This calculation determines the final speed of an object that has fallen from a stationary position under the influence of gravity alone. The result, 14 m/s, represents the velocity achieved after descending through a height of 10 m with gravitational acceleration at 9.8 m/s^2. This value indicates how fast the object is moving just before impact or at the end of the fall, assuming no air resistance affects its motion.

<a id="time_to_top_vertical_throw"></a>

#### Time to the top of a vertical throw: `time_to_top_vertical_throw`

One equation links time from launch to the highest point (t), launch speed (v0) and gravitational acceleration (g). Give all but one of them and it is solved for the one left out, by default time from launch to the highest point (t).

$$
t = \frac{v_0}{g}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `v0` | in | launch speed | m/s |
| `g` | in | gravitational acceleration (optional) | m/s^2 |
| `t` | out | time from launch to the highest point | s |

**Applies when:** thrown straight up; no air resistance; uniform gravitational field; the launch is vertical.

```shell
$ classical-physics solve time_to_top_vertical_throw "v0=29.4 m/s" "g=9.8 m/s^2"
time from launch to the highest point (t) = 3 s
  by: Time to the top of a vertical throw  (t = v0/g)
  v0 = 29.4 m/s   [29.4 m/s]
  g = 9.8 m/s^2   [9.8 m/s^2]
  assumes: thrown straight up; no air resistance; uniform gravitational field; the situation is: thrown up
```

**What the worked example means:** This calculation determines the duration it takes for an object thrown straight up to reach its highest point. The result, 3 s, represents this specific interval of flight before gravity causes the object to stop rising and begin falling back down.

<a id="uniform_acceleration_state"></a>

#### Velocity and displacement after constant acceleration, `uniform_acceleration_state`

The final velocity and the displacement of a body that starts at velocity u and accelerates uniformly at a for a time t.

$$
v = u + a t,\quad 2 s = \left(u + v\right) t
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `u` | in | initial velocity | m/s |
| `a` | in | constant acceleration | m/s^2 |
| `t` | in | elapsed time | s |
| `v` | out | final velocity | m/s |
| `s` | out | displacement | m |

**Applies when:** t >= 0.

```shell
$ classical-physics solve uniform_acceleration_state "u=4 m/s" "a=2 m/s^2" "t=5 s"
v = 14 m/s
s = 45 m
  by: Velocity and displacement after constant acceleration  (v = u + a*t; s from the average velocity (u + v)/2 over the time t)
  u = 4 m/s
  a = 2 m/s^2
  t = 5 s
  assumes: t >= 0
```

**What the worked example means:** Using an initial velocity of 4 m/s, a constant acceleration of 2 m/s^2, and an elapsed time of 5 s shows the final velocity to be 14 m/s and the total displacement to be 45 m. This result assumes the object moves in a straight line with unchanging acceleration, making it possible to predict its state after this specific duration.

<a id="velocity_time"></a>

#### Velocity under constant acceleration: `velocity_time`

One equation links final velocity (v), initial velocity (v0), acceleration (a) and elapsed time (t). Give all but one of them and it is solved for the one left out, by default final velocity (v).

$$
v = v_0 + a t
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `v0` | in | initial velocity | m/s |
| `a` | in | acceleration | m/s^2 |
| `t` | in | elapsed time | s |
| `v` | out | final velocity | m/s |

**Applies when:** constant acceleration along a line.

```shell
$ classical-physics solve velocity_time "v0=5 m/s" "a=2 m/s^2" "t=4 s"
final velocity (v) = 13 m/s
  by: Velocity under constant acceleration  (v = v0 + a*t)
  v0 = 5 m/s   [5 m/s]
  a = 2 m/s^2   [2 m/s^2]
  t = 4 s   [4 s]
  assumes: constant acceleration along a line
```

**What the worked example means:** This calculation determines the final velocity of an object moving in a straight line with constant acceleration. It uses the initial velocity, the acceleration rate, and the elapsed time to find the resulting speed. The computed final velocity is 13 m/s meters per second, which represents how fast the object is moving at the end of the specified duration.

### Magnetism (6)

<a id="loop_center_field"></a>

#### Magnetic field at the centre of a circular loop, `loop_center_field`

One equation links field at the centre (B), Current (I), radius of the loop (r) and Permeability (mu0). Give all but one of them and it is solved for the one left out, by default field at the centre (B).

$$
B = \frac{\mu_{0} I}{2 r}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `I` | in | Current | A |
| `r` | in | radius of the loop | m |
| `mu0` | in | Permeability (optional) | T*m/A |
| `B` | out | field at the centre | T |

**Applies when:** a single circular loop; vacuum or air; the centre of a circular current loop.

```shell
$ classical-physics solve loop_center_field "I=5 A" "r=0.1 m" "mu0=1.256637062e-06 T*m/A"
field at the centre (B) = 3.141592655e-05 T
  by: Magnetic field at the centre of a circular loop  (B = mu0*I/(2*r))
  I = 5 A   [5 A]
  r = 0.1 m   [0.1 m]
  mu0 = 1.256637062e-06 T*m/A   [1.25664e-06 T*m/A]
  assumes: a single circular loop; vacuum or air; the situation is: loop
```

**What the worked example means:** This calculation determines the magnetic field strength at the exact geometric center of a single circular loop carrying current. The result, 3.14159e-05 T, represents the magnitude of this field in teslas, derived from the current 5 A and the loop radius 0.1 m. This value serves as a fundamental reference point for understanding how current loops generate magnetic fields under ideal conditions.

<a id="solenoid_field"></a>

#### Magnetic field inside a long solenoid: `solenoid_field`

One equation links Magnetic field (B), turns per unit length (nt), Current (I) and Permeability (mu0). Give all but one of them and it is solved for the one left out, by default Magnetic field (B).

$$
B = \mu_{0} n_{t} I
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `nt` | in | turns per unit length | 1/m |
| `I` | in | Current | A |
| `mu0` | in | Permeability (optional) | T*m/A |
| `B` | out | Magnetic field | T |

**Applies when:** a long solenoid; air core; inside a long solenoid.

```shell
$ classical-physics solve solenoid_field "nt=1000 1/m" "I=2 A" "mu0=1.256637062e-06 T*m/A"
magnetic field (B) = 0.002513274124 T
  by: Magnetic field inside a long solenoid  (B = mu0*nt*I)
  nt = 1000 1/m   [1000 1/m]
  I = 2 A   [2 A]
  mu0 = 1.256637062e-06 T*m/A   [1.25664e-06 T*m/A]
  assumes: a long solenoid; air core; the situation is: solenoid
```

**What the worked example means:** This calculation determines the magnetic field B inside a long solenoid with an air core. It uses the permeability mu0, the turns per unit length nt, and the current I to find that the field strength is 0.00251327 T. This result represents the uniform magnetic field generated within the coil's interior.

<a id="long_wire_field"></a>

#### Magnetic field of a long straight wire: `long_wire_field`

One equation links Magnetic field (B), Current (I), distance from the wire (r) and Permeability (mu0). Give all but one of them and it is solved for the one left out, by default Magnetic field (B).

$$
B = \frac{\mu_{0} I}{2 \pi r}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `I` | in | Current | A |
| `r` | in | distance from the wire | m |
| `mu0` | in | Permeability (optional) | T*m/A |
| `B` | out | Magnetic field | T |

**Applies when:** a long straight wire; vacuum or air; near a long straight wire, far from its ends.

```shell
$ classical-physics solve long_wire_field "I=10 A" "r=0.05 m" "mu0=1.256637062e-06 T*m/A"
magnetic field (B) = 4.000000002e-05 T
  by: Magnetic field of a long straight wire  (B = mu0*I/(2*pi*r))
  I = 10 A   [10 A]
  r = 0.05 m   [0.05 m]
  mu0 = 1.256637062e-06 T*m/A   [1.25664e-06 T*m/A]
  assumes: a long straight wire; vacuum or air; the situation is: wire
```

**What the worked example means:** This calculation determines the magnetic field strength generated by a long straight wire carrying an electric current. The result, 4e-05 T, represents the magnitude of the field at a specific point located 0.05 m away from the center of the wire, assuming the current flowing through it is 10 A. This value indicates how strongly the wire influences nearby magnetic materials or other currents in a vacuum or air environment.

<a id="magnetic_force_wire"></a>

#### Magnetic force on a current-carrying wire: `magnetic_force_wire`

One equation links Force (F), Current (I), length of wire in the field (L), Magnetic field (B) and angle between the wire and the field (theta). Give all but one of them and it is solved for the one left out, by default Force (F).

$$
F = I L B \sin\left(\theta\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `I` | in | Current | A |
| `L` | in | length of wire in the field | m |
| `B` | in | Magnetic field | T |
| `theta` | in | angle between the wire and the field | rad |
| `F` | out | Force | N |

**Applies when:** a straight wire; a uniform field; a straight wire in a uniform magnetic field.

```shell
$ classical-physics solve magnetic_force_wire "I=5 A" "L=0.2 m" "B=0.4 T" "theta=1.570796327 rad"
force (F) = 0.4 N
  by: Magnetic force on a current-carrying wire  (F = I*L*B*sin(theta))
  I = 5 A   [5 A]
  L = 0.2 m   [0.2 m]
  B = 0.4 T   [0.4 T]
  theta = 1.570796327 rad   [1.5708 rad]
  assumes: a straight wire; a uniform field; the situation is: magnetic, wire
```

**What the worked example means:** This calculation finds the net magnetic force acting on a straight segment of wire carrying an electric current through a uniform magnetic field. The result, 0.4 N, represents the magnitude of this force in newtons. It depends directly on the current 5 A flowing through the wire, the length of the wire within the field 0.2 m, and the strength of the magnetic field 0.4 T. The direction and magnitude are also influenced by the angle 1.5708 rad between the wire and the field lines, where maximum force occurs when they are perpendicular.

<a id="magnetic_force_charge"></a>

#### Magnetic force on a moving charge: `magnetic_force_charge`

One equation links magnitude of the magnetic force (F), Charge (q), speed of the charge (v), Magnetic field (B) and angle between velocity and field (theta). Give all but one of them and it is solved for the one left out, by default magnitude of the magnetic force (F).

$$
F = q v B \sin\left(\theta\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `q` | in | Charge | C |
| `v` | in | speed of the charge | m/s |
| `B` | in | Magnetic field | T |
| `theta` | in | angle between velocity and field | rad |
| `F` | out | magnitude of the magnetic force | N |

**Applies when:** a uniform magnetic field; no electric field; a charge moving in a magnetic field.

```shell
$ classical-physics solve magnetic_force_charge "q=1.6e-19 C" "v=2000000 m/s" "B=0.5 T" "theta=1.570796327 rad"
magnitude of the magnetic force (F) = 1.6e-13 N
  by: Magnetic force on a moving charge  (F = q*v*B*sin(theta))
  q = 1.6e-19 C   [1.6e-19 C]
  v = 2000000 m/s   [2e+06 m/s]
  B = 0.5 T   [0.5 T]
  theta = 1.570796327 rad   [1.5708 rad]
  assumes: a uniform magnetic field; no electric field; the situation is: magnetic
```

**What the worked example means:** This calculation finds the magnitude of the magnetic force acting on a moving charged particle within a uniform magnetic field. The result, 1.6e-13 N, depends directly on the charge magnitude 1.6e-19 C, the particle's speed 2e+06 m/s, and the magnetic field strength 0.5 T. It also varies with the sine of the angle 1.5708 rad between the velocity vector and the field lines. This force represents the deflection experienced by the particle as it moves through the field.

<a id="cyclotron_radius"></a>

#### Radius of a charged particle's circle in a magnetic field, `cyclotron_radius`

One equation links radius of the circular path (r), mass of the particle (m), speed perpendicular to the field (v), magnitude of the charge (q), field strength (B) and angle between the velocity and the field (theta). Give all but one of them and it is solved for the one left out, by default radius of the circular path (r).

$$
r = \frac{m v}{q B \sin\left(\theta\right)}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `m` | in | mass of the particle | kg |
| `v` | in | speed perpendicular to the field | m/s |
| `q` | in | magnitude of the charge | C |
| `B` | in | field strength | T |
| `theta` | in | angle between the velocity and the field | rad |
| `r` | out | radius of the circular path | m |

**Applies when:** a uniform field; non-relativistic speed; the path is the circle traced by the perpendicular velocity component; a charged particle in a uniform magnetic field.

```shell
$ classical-physics solve cyclotron_radius "m=1.67e-27 kg" "v=1000000 m/s" "q=1.6e-19 C" "B=0.5 T" "theta=1.570796327 rad"
radius of the circular path (r) = 0.020875 m
  by: Radius of a charged particle's circle in a magnetic field  (r = m*v/(q*B*sin(theta)))
  m = 1.67e-27 kg   [1.67e-27 kg]
  v = 1000000 m/s   [1e+06 m/s]
  q = 1.6e-19 C   [1.6e-19 C]
  B = 0.5 T   [0.5 T]
  theta = 1.570796327 rad   [1.5708 rad]
  assumes: a uniform field; non-relativistic speed; the path is the circle traced by the perpendicular velocity component; the situation is: magnetic, charged particle
```

**What the worked example means:** This calculation determines the radius of the circular trajectory traced by a charged particle moving through a uniform magnetic field. The result, 0.020875 m, depends on the particle's mass 1.67e-27 kg, its speed perpendicular to the field 1e+06 m/s, the magnitude of its charge 1.6e-19 C, and the magnetic field strength 0.5 T. A larger radius indicates that the particle follows a wider curve, which occurs when the particle is heavier or faster, while a stronger magnetic field or greater charge results in a tighter turn.

### Optics (4)

<a id="mirror_focal_length"></a>

#### Focal length of a spherical mirror: `mirror_focal_length`

One equation links focal length (f) and radius of curvature (R). Give all but one of them and it is solved for the one left out, by default focal length (f).

$$
f = \frac{R}{2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `R` | in | radius of curvature | m |
| `f` | out | focal length | m |

**Applies when:** paraxial rays; spherical mirrors with small apertures.

```shell
$ classical-physics solve mirror_focal_length "R=0.4 m"
focal length (f) = 0.2 m
  by: Focal length of a spherical mirror  (f = R/2)
  R = 0.4 m   [0.4 m]
  assumes: paraxial rays; the situation is: mirror
```

**What the worked example means:** This calculation determines the focal length 0.2 m by taking half of the mirror's radius of curvature 0.4 m. The result indicates where parallel light rays will converge after reflecting off the spherical surface, a key parameter for optical design.

<a id="index_of_refraction"></a>

#### Index of refraction: `index_of_refraction`

One equation links index of refraction (n), Speed (c) and speed of light in the medium (v). Give all but one of them and it is solved for the one left out, by default index of refraction (n).

$$
n = \frac{c}{v}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `c` | in | Speed (optional) | m/s |
| `v` | in | speed of light in the medium | m/s |
| `n` | out | index of refraction | - |

**Applies when:** light in a transparent medium; transparent materials.

```shell
$ classical-physics solve index_of_refraction "c=299792458 m/s" "v=200000000 m/s"
index of refraction (n) = 1.49896229
  by: Index of refraction  (n = c/v)
  c = 299792458 m/s   [2.99792e+08 m/s]
  v = 200000000 m/s   [2e+08 m/s]
  assumes: light in a transparent medium; the situation is: medium
```

**What the worked example means:** This calculation determines the index of refraction, which represents how much a material slows down light relative to its speed in a vacuum. Dividing the constant speed of light in a vacuum 2.99792e+08 m/s by the measured speed of light within the specific transparent medium 2e+08 m/s gives the dimensionless ratio 1.49896. This value indicates that light travels slower in the medium than in empty space, a factor critical for understanding optical behavior such as bending at interfaces.

<a id="thin_lens"></a>

#### Thin lens equation: `thin_lens`

One equation links object distance (d_o), image distance (d_i) and focal length (f). Give all but one of them and it is solved for the one left out, by default .

$$
\frac{1}{d_{o}} + \frac{1}{d_{i}} = \frac{1}{f}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `d_o` | in | object distance | m |
| `d_i` | in | image distance | m |
| `f` | in | focal length | m |

**Applies when:** a thin lens; paraxial rays; a thin lens.

```shell
$ classical-physics solve thin_lens "d_o=0.3 m" "f=0.1 m"
image distance (d_i) = 0.15 m
  by: Thin lens equation  (1/d_o + 1/d_i = 1/f)
  d_o = 0.3 m   [0.3 m]
  f = 0.1 m   [0.1 m]
  assumes: a thin lens; paraxial rays; the situation is: lens
```

**What the worked example means:** This calculation determines the image distance using the thin lens equation. Substituting the known object distance and focal length into the formula shows that the image forms at a distance of 0.15 m from the lens center. This value indicates where light rays converge or appear to diverge from after passing through the optical element.

<a id="wavelength_in_medium"></a>

#### Wavelength of light in a medium: `wavelength_in_medium`

One equation links wavelength in the medium (lam_n), wavelength in vacuum (lam) and index of refraction (n). Give all but one of them and it is solved for the one left out, by default wavelength in the medium (lam_n).

$$
\lambda_{n} = \frac{\lambda}{n}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `lam` | in | wavelength in vacuum | m |
| `n` | in | index of refraction | - |
| `lam_n` | out | wavelength in the medium | m |

**Applies when:** the frequency is unchanged across the boundary; light entering a transparent medium.

```shell
$ classical-physics solve wavelength_in_medium "lam=6e-07 m" "n=1.5 1"
wavelength in the medium (lam_n) = 4e-07 m
  by: Wavelength of light in a medium  (lam_n = lam/n)
  lam = 6e-07 m   [6e-07 m]
  n = 1.5   [1.5 1]
  assumes: the frequency is unchanged across the boundary; the situation is: medium
```

**What the worked example means:** This calculation determines the wavelength of light inside a transparent material, denoted as 4e-07 m. It uses the vacuum wavelength 6e-07 m and the index of refraction 1.5. The result shows how much the wave compresses due to the slower speed of light in that specific medium.

### Orbits (3)

<a id="escape_speed"></a>

#### Escape speed: `escape_speed`

One equation links escape speed (v), Gravitational constant (G), mass of the body (M) and distance from its center (r). Give all but one of them and it is solved for the one left out, by default escape speed (v).

$$
v = \sqrt{\frac{2 G M}{r}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `G` | in | Gravitational constant (optional) | N*m^2/kg^2 |
| `M` | in | mass of the body | kg |
| `r` | in | distance from its center | m |
| `v` | out | escape speed | m/s |

**Applies when:** no air resistance; a spherically symmetric body; starting from rest relative to it; leaving a body's gravity from distance r.

```shell
$ classical-physics solve escape_speed "G=6.6743e-11 N*m^2/kg^2" "M=5.972e+24 kg" "r=6371000 m"
escape speed (v) = 11185.97789 m/s
  by: Escape speed  (v = sqrt(2*G*M/r))
  G = 6.6743e-11 N*m^2/kg^2   [6.6743e-11 N*m^2/kg^2]
  M = 5.972e+24 kg   [5.972e+24 kg]
  r = 6371000 m   [6.371e+06 m]
  assumes: no air resistance; a spherically symmetric body; starting from rest relative to it; the situation is: escape
```

In the worked example, G = 6.6743e-11 N*m^2/kg^2, M = 5.972e+24 kg, r = 6371000 m go through the equation above and give v = 11185.97789 m/s. It applies only when no air resistance; a spherically symmetric body; starting from rest relative to it; leaving a body's gravity from distance r: outside that it says what is wrong instead of returning a number.

<a id="kepler_period_semi_major"></a>

#### Orbital period from the semi-major axis (Kepler's third law), `kepler_period_semi_major`

One equation links orbital period (T), semi-major axis (a), Gravitational constant (G) and mass of the central body (M). Give all but one of them and it is solved for the one left out, by default orbital period (T).

$$
T = 2 \pi \sqrt{\frac{a^{3}}{G M}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `a` | in | semi-major axis | m |
| `G` | in | Gravitational constant (optional) | N*m^2/kg^2 |
| `M` | in | mass of the central body | kg |
| `T` | out | orbital period | s |

**Applies when:** two bodies, the orbiting one much lighter; any bound elliptical orbit.

```shell
$ classical-physics solve kepler_period_semi_major "a=1.496e+11 m" "G=6.6743e-11 N*m^2/kg^2" "M=1.989e+30 kg"
orbital period (T) = 31554187.75 s
  by: Orbital period from the semi-major axis (Kepler's third law)  (T = 2*pi*sqrt(a**3/(G*M)))
  a = 1.496e+11 m   [1.496e+11 m]
  G = 6.6743e-11 N*m^2/kg^2   [6.6743e-11 N*m^2/kg^2]
  M = 1.989e+30 kg   [1.989e+30 kg]
  assumes: two bodies, the orbiting one much lighter; the situation is: orbit
```

**What the worked example means:** This calculation determines the time required for an object to complete one full orbit, known as the orbital period. The result, 3.15542e+07 s, is computed using Kepler's third law, which relates this duration to the size of the elliptical path, represented by the semi-major axis 1.496e+11 m, and the gravitational influence of the central body, quantified by its mass 1.989e+30 kg. Essentially, it predicts how long a satellite takes to circle a planet or star based on how far away it orbits and how massive the central object is.

<a id="vis_viva"></a>

#### Orbital speed on an elliptical orbit (vis-viva), `vis_viva`

One equation links orbital speed at distance r (v), Gravitational constant (G), mass of the central body (M), distance from the center of the central body (r) and semi-major axis (a). Give all but one of them and it is solved for the one left out, by default orbital speed at distance r (v).

$$
v = \sqrt{G M \left(\frac{2}{r} - \frac{1}{a}\right)}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `G` | in | Gravitational constant (optional) | N*m^2/kg^2 |
| `M` | in | mass of the central body | kg |
| `r` | in | distance from the center of the central body | m |
| `a` | in | semi-major axis | m |
| `v` | out | orbital speed at distance r | m/s |

**Applies when:** two bodies, the orbiting one much lighter; a bound (elliptical) orbit; a satellite or planet on an elliptical orbit.

```shell
$ classical-physics solve vis_viva "G=6.6743e-11 N*m^2/kg^2" "M=5.972e+24 kg" "r=7000000 m" "a=8000000 m"
orbital speed at distance r (v) = 8003.685272 m/s
  by: Orbital speed on an elliptical orbit (vis-viva)  (v = sqrt(G*M*(2/r - 1/a)))
  G = 6.6743e-11 N*m^2/kg^2   [6.6743e-11 N*m^2/kg^2]
  M = 5.972e+24 kg   [5.972e+24 kg]
  r = 7000000 m   [7e+06 m]
  a = 8000000 m   [8e+06 m]
  assumes: two bodies, the orbiting one much lighter; a bound (elliptical) orbit; the situation is: orbit
```

**What the worked example means:** This calculation determines the instantaneous velocity of a satellite or planet moving along an elliptical path around a central body. Applying the vis-viva equation combines the gravitational constant with the mass of the central object and the specific geometric parameters of the orbit. The result shows that the speed at the current position is 8003.69 m/s. This value reflects how kinetic energy varies as the object moves closer to or farther from the center of gravity within its bound trajectory.

### Oscillations (12)

<a id="amplitude_resonance_frequency"></a>

#### Amplitude-resonance frequency of a driven damped oscillator, `amplitude_resonance_frequency`

One equation links driving frequency of maximum amplitude (wr), spring constant (k), oscillating mass (m) and viscous damping coefficient (b). Give all but one of them and it is solved for the one left out, by default driving frequency of maximum amplitude (wr).

$$
\omega_{r} = \sqrt{\frac{k}{m} - \frac{b^{2}}{2 m^{2}}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `k` | in | spring constant | N/m |
| `m` | in | oscillating mass | kg |
| `b` | in | viscous damping coefficient | kg/s |
| `wr` | out | driving frequency of maximum amplitude | rad/s |

**Applies when:** sinusoidal driving; linear viscous damping; b^2 < 2 k m; the driving frequency that maximises the steady-state amplitude.

```shell
$ classical-physics solve amplitude_resonance_frequency "k=50 N/m" "m=2 kg" "b=4 kg/s"
driving frequency of maximum amplitude (wr) = 4.795831523 rad/s
  by: Amplitude-resonance frequency of a driven damped oscillator  (wr = sqrt(k/m - b**2/(2*m**2)))
  k = 50 N/m   [50 N/m]
  m = 2 kg   [2 kg]
  b = 4 kg/s   [4 kg/s]
  assumes: sinusoidal driving; linear viscous damping; b^2 < 2 k m; the situation is: driven
```

**What the worked example means:** This calculation determines the specific angular frequency at which a driven damped oscillator exhibits its maximum steady-state amplitude. The result, 4.79583 rad/s, is derived from the spring constant 50 N/m, the oscillating mass 2 kg, and the viscous damping coefficient 4 kg/s. This resonant frequency is lower than the undamped natural frequency because energy dissipation shifts the peak response to a slower oscillation rate.

<a id="damped_angular_frequency"></a>

#### Angular frequency of a damped oscillator: `damped_angular_frequency`

One equation links angular frequency of the damped oscillation (wd), spring constant (k), oscillating mass (m) and viscous damping coefficient (b). Give all but one of them and it is solved for the one left out, by default angular frequency of the damped oscillation (wd).

$$
\omega_{d} = \sqrt{\frac{k}{m} - \frac{b^{2}}{4 m^{2}}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `k` | in | spring constant | N/m |
| `m` | in | oscillating mass | kg |
| `b` | in | viscous damping coefficient | kg/s |
| `wd` | out | angular frequency of the damped oscillation | rad/s |

**Applies when:** linear viscous damping; underdamped (b^2 < 4 k m); a mass on a linear spring with viscous damping.

```shell
$ classical-physics solve damped_angular_frequency "k=50 N/m" "m=2 kg" "b=4 kg/s"
angular frequency of the damped oscillation (wd) = 4.898979486 rad/s
  by: Angular frequency of a damped oscillator  (wd = sqrt(k/m - b**2/(4*m**2)))
  k = 50 N/m   [50 N/m]
  m = 2 kg   [2 kg]
  b = 4 kg/s   [4 kg/s]
  assumes: linear viscous damping; underdamped (b^2 < 4 k m); the situation is: damped
```

**What the worked example means:** This calculation determines the angular frequency of a damped oscillator, denoted as 4.89898 rad/s. It uses the spring constant 50 N/m, the oscillating mass 2 kg, and the viscous damping coefficient 4 kg/s. The result indicates how fast the system vibrates while losing energy due to friction. This value is critical for understanding the settling time of mechanical systems.

<a id="damping_ratio"></a>

#### Damping ratio: `damping_ratio`

One equation links damping ratio (zeta), viscous damping coefficient (b), spring constant (k) and oscillating mass (m). Give all but one of them and it is solved for the one left out, by default damping ratio (zeta).

$$
\zeta = \frac{b}{2 \sqrt{k m}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `b` | in | viscous damping coefficient | kg/s |
| `k` | in | spring constant | N/m |
| `m` | in | oscillating mass | kg |
| `zeta` | out | damping ratio | - |

**Applies when:** linear viscous damping; a damped mass-spring system.

```shell
$ classical-physics solve damping_ratio "b=4 kg/s" "k=50 N/m" "m=2 kg"
damping ratio (zeta) = 0.2
  by: Damping ratio  (zeta = b/(2*sqrt(k*m)))
  b = 4 kg/s   [4 kg/s]
  k = 50 N/m   [50 N/m]
  m = 2 kg   [2 kg]
  assumes: linear viscous damping; the situation is: damped
```

**What the worked example means:** The damping ratio 0.2 is calculated using the viscous damping coefficient 4 kg/s, spring constant 50 N/m, and oscillating mass 2 kg. This dimensionless value determines whether the mass-spring system will oscillate, return slowly without overshooting, or settle as quickly as possible. A value below one suggests underdamping where oscillations occur, while a value of one represents critical damping.

<a id="double_pendulum_fast_mode"></a>

#### Higher normal mode of a double pendulum: `double_pendulum_fast_mode`

One equation links higher normal-mode angular frequency (w), Acceleration (g) and length of each arm (L). Give all but one of them and it is solved for the one left out, by default higher normal-mode angular frequency (w).

$$
\omega = \sqrt{\frac{\left(2 + \sqrt{2}\right) g}{L}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `g` | in | Acceleration (optional) | m/s^2 |
| `L` | in | length of each arm | m |
| `w` | out | higher normal-mode angular frequency | rad/s |

**Applies when:** equal masses and arm lengths; small oscillations; a double pendulum with equal masses and arms.

```shell
$ classical-physics solve double_pendulum_fast_mode "g=9.80665 m/s^2" "L=1 m"
higher normal-mode angular frequency (w) = 5.786363057 rad/s
  by: Higher normal mode of a double pendulum  (w = sqrt((2 + sqrt(2))*g/L))
  g = 9.80665 m/s^2   [9.80665 m/s^2]
  L = 1 m   [1 m]
  assumes: equal masses and arm lengths; small oscillations; the situation is: double pendulum
```

**What the worked example means:** This calculation determines the angular frequency of the higher-frequency normal mode for a double pendulum with equal masses and arm lengths undergoing small oscillations. The result, 5.78636 rad/s, represents how rapidly the system oscillates in its out-of-phase motion where the upper and lower bobs move in opposite directions relative to their equilibrium positions.

<a id="coupled_fast_mode"></a>

#### Higher normal-mode frequency of two coupled oscillators, `coupled_fast_mode`

One equation links higher normal-mode angular frequency (w2), each outer spring's constant (k), coupling spring constant (kc) and each mass (m). Give all but one of them and it is solved for the one left out, by default higher normal-mode angular frequency (w2).

$$
\omega_{2} = \sqrt{\frac{k + 2 k_{c}}{m}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `k` | in | each outer spring's constant | N/m |
| `kc` | in | coupling spring constant | N/m |
| `m` | in | each mass | kg |
| `w2` | out | higher normal-mode angular frequency | rad/s |

**Applies when:** two identical masses; identical outer springs; small oscillations, no damping; two equal masses joined by a coupling spring, each also tied to a wall.

```shell
$ classical-physics solve coupled_fast_mode "k=40 N/m" "kc=10 N/m" "m=2 kg"
higher normal-mode angular frequency (w2) = 5.477225575 rad/s
  by: Higher normal-mode frequency of two coupled oscillators  (w2 = sqrt((k + 2*kc)/m))
  k = 40 N/m   [40 N/m]
  kc = 10 N/m   [10 N/m]
  m = 2 kg   [2 kg]
  assumes: two identical masses; identical outer springs; small oscillations, no damping; the situation is: coupled
```

**What the worked example means:** This calculation determines the higher normal-mode angular frequency, denoted as w2, for a system of two identical masses coupled by springs. Using the spring constant of each outer spring k, the coupling spring constant kc, and the mass m, the formula yields a result of 5.47723 rad/s. This value represents the rate at which the masses oscillate in the mode where they move away from each other, indicating how the coupling stiffens the system compared to independent oscillators.

<a id="double_pendulum_slow_mode"></a>

#### Lower normal mode of a double pendulum: `double_pendulum_slow_mode`

One equation links lower normal-mode angular frequency (w), Acceleration (g) and length of each arm (L). Give all but one of them and it is solved for the one left out, by default lower normal-mode angular frequency (w).

$$
\omega = \sqrt{\frac{\left(2 - \sqrt{2}\right) g}{L}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `g` | in | Acceleration (optional) | m/s^2 |
| `L` | in | length of each arm | m |
| `w` | out | lower normal-mode angular frequency | rad/s |

**Applies when:** equal masses and arm lengths; small oscillations; a double pendulum with equal masses and arms.

```shell
$ classical-physics solve double_pendulum_slow_mode "g=9.80665 m/s^2" "L=1 m"
lower normal-mode angular frequency (w) = 2.396790055 rad/s
  by: Lower normal mode of a double pendulum  (w = sqrt((2 - sqrt(2))*g/L))
  g = 9.80665 m/s^2   [9.80665 m/s^2]
  L = 1 m   [1 m]
  assumes: equal masses and arm lengths; small oscillations; the situation is: double pendulum
```

**What the worked example means:** This calculation determines the angular frequency of the lower normal mode for a double pendulum with equal masses and arm lengths. The result, 2.39679 rad/s, represents the rate at which the system oscillates when both bobs move largely in phase under small-angle conditions. This value depends on the gravitational acceleration 9.80665 m/s^2 and the length of each arm 1 m.

<a id="shm_max_speed"></a>

#### Maximum speed of a mass on a spring: `shm_max_speed`

One equation links maximum speed (at equilibrium) (v), amplitude (X), spring constant (k) and oscillating mass (m). Give all but one of them and it is solved for the one left out, by default maximum speed (at equilibrium) (v).

$$
v = X \sqrt{\frac{k}{m}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `X` | in | amplitude | m |
| `k` | in | spring constant | N/m |
| `m` | in | oscillating mass | kg |
| `v` | out | maximum speed (at equilibrium) | m/s |

**Applies when:** undamped simple harmonic motion; an ideal massless spring; a mass oscillating on a spring.

```shell
$ classical-physics solve shm_max_speed "X=0.1 m" "k=100 N/m" "m=1 kg"
maximum speed (at equilibrium) (v) = 1 m/s
  by: Maximum speed of a mass on a spring  (v = X*sqrt(k/m))
  X = 0.1 m   [0.1 m]
  k = 100 N/m   [100 N/m]
  m = 1 kg   [1 kg]
  assumes: undamped simple harmonic motion; an ideal massless spring; the situation is: shm
```

**What the worked example means:** This calculation determines the peak velocity reached by an oscillating mass during simple harmonic motion. Combining the amplitude, spring constant, and mass shows that the maximum speed is 1 m/s. This value represents the fastest point in the cycle, occurring as the mass passes through its equilibrium position.

<a id="spring_oscillation_period"></a>

#### Period of a mass on a spring: `spring_oscillation_period`

One equation links Time (T), Mass (m) and Spring constant (k). Give all but one of them and it is solved for the one left out, by default Time (T).

$$
T = 2 \pi \sqrt{\frac{m}{k}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `m` | in | Mass | kg |
| `k` | in | Spring constant | N/m |
| `T` | out | Time | s |

**Applies when:** an ideal spring; small oscillations; no damping; a mass oscillating on an ideal spring.

```shell
$ classical-physics solve spring_oscillation_period "m=0.5 kg" "k=200 N/m"
time (T) = 0.3141592654 s
  by: Period of a mass on a spring  (T = 2*pi*sqrt(m/k))
  m = 0.5 kg   [0.5 kg]
  k = 200 N/m   [200 N/m]
  assumes: an ideal spring; small oscillations; no damping; the situation is: spring oscillation
```

**What the worked example means:** This calculation determines the time required for one complete cycle of a mass attached to an ideal spring. The result, 0.314159 s, indicates how long the system takes to return to its starting position after being displaced. This duration depends on the mass 0.5 kg and the stiffness of the spring, represented by the constant 200 N/m. A longer period implies slower oscillations, while a shorter period signifies faster movement.

<a id="physical_pendulum_period"></a>

#### Period of a physical pendulum: `physical_pendulum_period`

One equation links small-oscillation period (T), moment of inertia about the pivot (Ir), mass of the body (m), Acceleration (g) and distance from the pivot to the center of mass (d). Give all but one of them and it is solved for the one left out, by default small-oscillation period (T).

$$
T = 2 \pi \sqrt{\frac{I_{r}}{m g d}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Ir` | in | moment of inertia about the pivot | kg*m^2 |
| `m` | in | mass of the body | kg |
| `g` | in | Acceleration (optional) | m/s^2 |
| `d` | in | distance from the pivot to the center of mass | m |
| `T` | out | small-oscillation period | s |

**Applies when:** small oscillations; no friction at the pivot; a rigid body swinging about a fixed pivot.

```shell
$ classical-physics solve physical_pendulum_period "Ir=0.5 kg*m^2" "m=2 kg" "g=9.80665 m/s^2" "d=0.3 m"
small-oscillation period (T) = 1.831592715 s
  by: Period of a physical pendulum  (T = 2*pi*sqrt(Ir/(m*g*d)))
  Ir = 0.5 kg*m^2   [0.5 kg*m^2]
  m = 2 kg   [2 kg]
  g = 9.80665 m/s^2   [9.80665 m/s^2]
  d = 0.3 m   [0.3 m]
  assumes: small oscillations; no friction at the pivot; the situation is: physical pendulum
```

**What the worked example means:** This calculation determines the time it takes for a rigid body to complete one full swing about a fixed pivot under small-angle conditions. The result, 1.83159 s, depends on the distribution of mass relative to the pivot, specifically the moment of inertia 0.5 kg*m^2 and the distance from the pivot to the center of mass 0.3 m, as well as the total mass 2 kg and local gravitational acceleration 9.80665 m/s^2. A longer period indicates a slower oscillation, which typically occurs when the mass is distributed farther from the pivot or when the center of mass is closer to the pivot point.

<a id="pendulum_period"></a>

#### Period of a simple pendulum: `pendulum_period`

One equation links period of the swing (T), length of the pendulum (Lp) and Acceleration (g). Give all but one of them and it is solved for the one left out, by default period of the swing (T).

$$
T = 2 \pi \sqrt{\frac{L_{p}}{g}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Lp` | in | length of the pendulum | m |
| `g` | in | Acceleration (optional) | m/s^2 |
| `T` | out | period of the swing | s |

**Applies when:** small swings; a point mass on a light string; small-angle swings.

```shell
$ classical-physics solve pendulum_period "Lp=1 m" "g=9.80665 m/s^2"
period of the swing (T) = 2.006409293 s
  by: Period of a simple pendulum  (T = 2*pi*sqrt(Lp/g))
  Lp = 1 m   [1 m]
  g = 9.80665 m/s^2   [9.80665 m/s^2]
  assumes: small swings; a point mass on a light string; the situation is: pendulum
```

**What the worked example means:** This calculation determines the time required for a simple pendulum to complete one full back-and-forth oscillation. Using the length of the pendulum, which is 1 m, and the acceleration due to gravity, given as 9.80665 m/s^2, shows that the resulting period is 2.00641 s. This result highlights that the timing of the swing depends on the physical length and local gravity but remains independent of the mass of the bob.

<a id="quality_factor"></a>

#### Quality factor of a damped oscillator: `quality_factor`

One equation links quality factor (Qf), spring constant (k), oscillating mass (m) and viscous damping coefficient (b). Give all but one of them and it is solved for the one left out, by default quality factor (Qf).

$$
Q_{f} = \frac{\sqrt{k m}}{b}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `k` | in | spring constant | N/m |
| `m` | in | oscillating mass | kg |
| `b` | in | viscous damping coefficient | kg/s |
| `Qf` | out | quality factor | - |

**Applies when:** linear viscous damping; a damped mass-spring system.

```shell
$ classical-physics solve quality_factor "k=50 N/m" "m=2 kg" "b=4 kg/s"
quality factor (Qf) = 2.5
  by: Quality factor of a damped oscillator  (Qf = sqrt(k*m)/b)
  k = 50 N/m   [50 N/m]
  m = 2 kg   [2 kg]
  b = 4 kg/s   [4 kg/s]
  assumes: linear viscous damping; the situation is: damped
```

**What the worked example means:** This calculation determines the quality factor of a damped oscillator using the formula Q equals the square root of the product of the spring constant and the mass divided by the viscous damping coefficient. With a spring constant of 50 N/m and an oscillating mass of 2 kg, combined with a damping coefficient of 4 kg/s, the resulting quality factor is 2.5. A higher value indicates that the system loses energy slowly and resonates sharply, while a lower value suggests significant damping and rapid decay of motion.

<a id="driven_amplitude"></a>

#### Steady-state amplitude of a driven damped oscillator, `driven_amplitude`

One equation links steady-state amplitude (A), driving force amplitude (F0), spring constant (k), oscillating mass (m), viscous damping coefficient (b) and driving angular frequency (w). Give all but one of them and it is solved for the one left out, by default steady-state amplitude (A).

$$
A = \frac{F_{0}}{\sqrt{\left(k - m \omega^{2}\right)^{2} + \left(b \omega\right)^{2}}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `F0` | in | driving force amplitude | N |
| `k` | in | spring constant | N/m |
| `m` | in | oscillating mass | kg |
| `b` | in | viscous damping coefficient | kg/s |
| `w` | in | driving angular frequency | rad/s |
| `A` | out | steady-state amplitude | m |

**Applies when:** sinusoidal driving force; linear viscous damping; steady state (transients have died out); a damped mass-spring system driven by a sinusoidal force.

```shell
$ classical-physics solve driven_amplitude "F0=10 N" "k=50 N/m" "m=2 kg" "b=4 kg/s" "w=3 rad/s"
steady-state amplitude (A) = 0.292602868 m
  by: Steady-state amplitude of a driven damped oscillator  (A = F0/sqrt((k - m*w**2)**2 + (b*w)**2))
  F0 = 10 N   [10 N]
  k = 50 N/m   [50 N/m]
  m = 2 kg   [2 kg]
  b = 4 kg/s   [4 kg/s]
  w = 3 rad/s   [3 rad/s]
  assumes: sinusoidal driving force; linear viscous damping; steady state (transients have died out); the situation is: driven
```

**What the worked example means:** This calculation determines the steady-state amplitude A of a driven damped oscillator. It uses the driving force amplitude F0, spring constant k, oscillating mass m, viscous damping coefficient b, and driving angular frequency w. The result shows that for the given inputs, the displacement magnitude settles at 0.292603 m, representing the long-term response after initial transients have decayed.

### Quantum (4)

<a id="de_broglie"></a>

#### De Broglie wavelength: `de_broglie`

One equation links de Broglie wavelength (lam), momentum of the particle (p) and Planck constant (h). Give all but one of them and it is solved for the one left out, by default de Broglie wavelength (lam).

$$
\lambda = \frac{h}{p}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `p` | in | momentum of the particle | kg*m/s |
| `h` | in | Planck constant (optional) | J*s |
| `lam` | out | de Broglie wavelength | m |

**Applies when:** a free particle; electrons, protons, neutrons and other particles.

```shell
$ classical-physics solve de_broglie "p=1e-24 kg*m/s" "h=6.62607015e-34 J*s"
de Broglie wavelength (lam) = 6.62607015e-10 m
  by: De Broglie wavelength  (lam = h/p)
  p = 1e-24 kg*m/s   [1e-24 kg*m/s]
  h = 6.62607015e-34 J*s   [6.62607e-34 J*s]
  assumes: a free particle; the situation is: matter wave
```

**What the worked example means:** The de Broglie wavelength of a particle with momentum 1e-24 kg*m/s is calculated as 6.62607e-10 m. This value represents the spatial period of the matter wave associated with the particle, indicating its wave-like properties. A larger wavelength corresponds to a lower momentum, which is crucial for understanding quantum interference effects in particles like electrons and neutrons.

<a id="photon_energy"></a>

#### Energy of a photon: `photon_energy`

One equation links photon energy (E), frequency of the light (f) and Planck constant (h). Give all but one of them and it is solved for the one left out, by default photon energy (E).

$$
E = h f
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `f` | in | frequency of the light | Hz |
| `h` | in | Planck constant (optional) | J*s |
| `E` | out | photon energy | J |

**Applies when:** a single photon; light or other electromagnetic radiation.

```shell
$ classical-physics solve photon_energy "f=5e+14 Hz" "h=6.62607015e-34 J*s"
photon energy (E) = 3.313035075e-19 J
  by: Energy of a photon  (E = h*f)
  f = 5e+14 Hz   [5e+14 Hz]
  h = 6.62607015e-34 J*s   [6.62607e-34 J*s]
  assumes: a single photon; the situation is: photon
```

**What the worked example means:** This calculation determines the discrete energy packet carried by a single photon using its frequency. The result, 3.31304e-19 J, represents the specific amount of energy associated with that quantum of light or electromagnetic radiation.

<a id="photon_momentum"></a>

#### Momentum of a photon: `photon_momentum`

One equation links photon momentum (p), Length (lam) and Planck constant (h). Give all but one of them and it is solved for the one left out, by default photon momentum (p).

$$
p = \frac{h}{\lambda}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `lam` | in | Length | m |
| `h` | in | Planck constant (optional) | J*s |
| `p` | out | photon momentum | kg*m/s |

**Applies when:** a single photon; light.

```shell
$ classical-physics solve photon_momentum "lam=5e-07 m" "h=6.62607015e-34 J*s"
photon momentum (p) = 1.32521403e-27 kg*m/s
  by: Momentum of a photon  (p = h/lam)
  lam = 5e-07 m   [5e-07 m]
  h = 6.62607015e-34 J*s   [6.62607e-34 J*s]
  assumes: a single photon; the situation is: photon
```

**What the worked example means:** This calculation determines the momentum carried by a single photon using its wavelength. Dividing Planck's constant by the wavelength shows that the momentum is 1.32521e-27 kg*m/s. This result confirms that light exerts pressure and transfers impulse during collisions, which is essential for understanding phenomena like radiation pressure.

<a id="photon_energy_wavelength"></a>

#### Photon energy from wavelength: `photon_energy_wavelength`

One equation links Energy (E), wavelength in vacuum (lam), Planck constant (h) and Speed (c). Give all but one of them and it is solved for the one left out, by default Energy (E).

$$
E = \frac{h c}{\lambda}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `lam` | in | wavelength in vacuum | m |
| `h` | in | Planck constant (optional) | J*s |
| `c` | in | Speed (optional) | m/s |
| `E` | out | Energy | J |

**Applies when:** a single photon; wavelength in vacuum; light in vacuum or air.

```shell
$ classical-physics solve photon_energy_wavelength "lam=5e-07 m" "h=6.62607015e-34 J*s" "c=299792458 m/s"
energy (E) = 3.972891714e-19 J
  by: Photon energy from wavelength  (E = h*c/lam)
  lam = 5e-07 m   [5e-07 m]
  h = 6.62607015e-34 J*s   [6.62607e-34 J*s]
  c = 299792458 m/s   [2.99792e+08 m/s]
  assumes: a single photon; wavelength in vacuum; the situation is: photon
```

**What the worked example means:** This calculation determines the energy of a single photon, denoted as E, using its wavelength in vacuum, lam. The result, 3.97289e-19 J, represents the quantum of energy carried by that specific photon. This value is derived from the fundamental relationship between light's spatial period and its energy content.

### RC circuits (3)

<a id="rc_time_constant"></a>

#### Time constant of an RC circuit: `rc_time_constant`

One equation links time constant (tau), resistance (R) and capacitance (C). Give all but one of them and it is solved for the one left out, by default time constant (tau).

$$
\tau = R C
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `R` | in | resistance | ohm |
| `C` | in | capacitance | F |
| `tau` | out | time constant | s |

**Applies when:** a single resistor and capacitor; an RC circuit charging or discharging.

```shell
$ classical-physics solve rc_time_constant "R=1000 ohm" "C=8e-06 F"
time constant (tau) = 0.008 s
  by: Time constant of an RC circuit  (tau = R*C)
  R = 1000 ohm   [1000 ohm]
  C = 8e-06 F   [8e-06 F]
  assumes: a single resistor and capacitor; the situation is: rc circuit
```

**What the worked example means:** This calculation determines the time constant tau, which represents the characteristic time scale for voltage changes in a simple resistor-capacitor circuit. Multiplying the resistance R, measured at 1000 ohm, by the capacitance C, measured at 8e-06 F, gives a result of 0.008 s. This value indicates how quickly the capacitor charges or discharges, serving as a critical parameter for designing timing circuits and filtering signals in electronics.

<a id="rc_charging"></a>

#### Voltage of a charging capacitor: `rc_charging`

One equation links voltage across the capacitor at time t (V), emf of the source (emf), time since charging began (t), resistance (R) and capacitance (C). Give all but one of them and it is solved for the one left out, by default voltage across the capacitor at time t (V).

$$
V = \text{emf} \left(1 - e^{\frac{-t}{R C}}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `emf` | in | emf of the source | V |
| `t` | in | time since charging began | s |
| `R` | in | resistance | ohm |
| `C` | in | capacitance | F |
| `V` | out | voltage across the capacitor at time t | V |

**Applies when:** a single resistor and capacitor; the capacitor is uncharged at t = 0; an uncharged capacitor charged through a resistor by a constant emf.

```shell
$ classical-physics solve rc_charging "emf=9 V" "t=0.008 s" "R=1000 ohm" "C=8e-06 F"
voltage across the capacitor at time t (V) = 5.689085029 V
  by: Voltage of a charging capacitor  (V = emf*(1 - exp(-t/(R*C))))
  emf = 9 V   [9 V]
  t = 0.008 s   [0.008 s]
  R = 1000 ohm   [1000 ohm]
  C = 8e-06 F   [8e-06 F]
  assumes: a single resistor and capacitor; the capacitor is uncharged at t = 0; the situation is: rc circuit, charging
```

**What the worked example means:** This calculation determines the voltage across a charging capacitor, denoted as V. It uses the source electromotive force (emf), resistance (R), capacitance (C), and elapsed time (t). The result shows that after 0.008 s, the capacitor has accumulated enough charge to produce a potential difference of 5.68909 V. This value represents how close the circuit is to its steady state where the capacitor voltage equals the source emf.

<a id="rc_discharging"></a>

#### Voltage of a discharging capacitor: `rc_discharging`

One equation links voltage across the capacitor at time t (V), initial voltage across the capacitor (V0), time since the discharge began (t), resistance (R) and capacitance (C). Give all but one of them and it is solved for the one left out, by default voltage across the capacitor at time t (V).

$$
V = V_{0} e^{\frac{-t}{R C}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `V0` | in | initial voltage across the capacitor | V |
| `t` | in | time since the discharge began | s |
| `R` | in | resistance | ohm |
| `C` | in | capacitance | F |
| `V` | out | voltage across the capacitor at time t | V |

**Applies when:** a single resistor and capacitor; the discharge starts at t = 0; a charged capacitor discharging through a resistor.

```shell
$ classical-physics solve rc_discharging "V0=12 V" "t=0.008 s" "R=1000 ohm" "C=8e-06 F"
voltage across the capacitor at time t (V) = 4.414553294 V
  by: Voltage of a discharging capacitor  (V = V0*exp(-t/(R*C)))
  V0 = 12 V   [12 V]
  t = 0.008 s   [0.008 s]
  R = 1000 ohm   [1000 ohm]
  C = 8e-06 F   [8e-06 F]
  assumes: a single resistor and capacitor; the discharge starts at t = 0; the situation is: rc circuit, discharging
```

**What the worked example means:** This calculation determines the instantaneous voltage across a discharging capacitor, denoted as 4.41455 V. The result is derived from the exponential decay formula using the initial voltage 12 V, the elapsed time 0.008 s, the resistance 1000 ohm, and the capacitance 8e-06 F. It signifies how much electrical potential remains stored in the capacitor at this specific moment after the discharge began.

### Relativity (1)

<a id="rest_energy"></a>

#### Rest energy: `rest_energy`

One equation links rest energy (E0), rest mass (m) and Speed (c). Give all but one of them and it is solved for the one left out, by default rest energy (E0).

$$
E_{0} = m c^{2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `m` | in | rest mass | kg |
| `c` | in | Speed (optional) | m/s |
| `E0` | out | rest energy | J |

**Applies when:** energy of the mass at rest; mass converted entirely into energy, or the rest energy of a body.

```shell
$ classical-physics solve rest_energy "m=0.001 kg" "c=299792458 m/s"
rest energy (E0) = 8.987551787e+13 J
  by: Rest energy  (E0 = m*c**2)
  m = 0.001 kg   [0.001 kg]
  c = 299792458 m/s   [2.99792e+08 m/s]
  assumes: energy of the mass at rest; the situation is: mass energy
```

**What the worked example means:** This calculation determines the intrinsic energy contained within an object solely due to its mass, known as rest energy. Multiplying the rest mass 0.001 kg by the square of the speed of light 2.99792e+08 m/s gives the total energy 8.98755e+13 J that would be released if the entire mass were converted into energy. This result highlights the profound equivalence between matter and energy, showing how even small amounts of mass correspond to vast quantities of energy.

### Rigid body (8)

<a id="rolling_incline_acceleration"></a>

#### Acceleration of a body rolling without slipping down an incline, `rolling_incline_acceleration`

One equation links acceleration of the center of mass along the incline (a), Acceleration (g), angle of the incline (theta), moment of inertia about the center (Ir), mass (m) and rolling radius (R). Give all but one of them and it is solved for the one left out, by default acceleration of the center of mass along the incline (a).

$$
a = \frac{g \sin\left(\theta\right)}{1 + \frac{I_{r}}{m R^{2}}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `g` | in | Acceleration (optional) | m/s^2 |
| `theta` | in | angle of the incline | rad |
| `Ir` | in | moment of inertia about the center | kg*m^2 |
| `m` | in | mass | kg |
| `R` | in | rolling radius | m |
| `a` | out | acceleration of the center of mass along the incline | m/s^2 |

**Applies when:** rolling without slipping; no rolling resistance; a round body rolling down a straight incline.

```shell
$ classical-physics solve rolling_incline_acceleration "g=9.80665 m/s^2" "theta=0.5235987756 rad" "Ir=0.04 kg*m^2" "m=2 kg" "R=0.2 m"
acceleration of the center of mass along the incline (a) = 3.268883333 m/s^2
  by: Acceleration of a body rolling without slipping down an incline  (a = g*sin(theta)/(1 + Ir/(m*R**2)))
  g = 9.80665 m/s^2   [9.80665 m/s^2]
  theta = 0.5235987756 rad   [0.523599 rad]
  Ir = 0.04 kg*m^2   [0.04 kg*m^2]
  m = 2 kg   [2 kg]
  R = 0.2 m   [0.2 m]
  assumes: rolling without slipping; no rolling resistance; the situation is: rolling
```

**What the worked example means:** This calculation determines the linear acceleration of a round object rolling down a slope without slipping. The result, 3.26888 m/s^2, represents how quickly the body speeds up along the incline. It accounts for rotational inertia, which reduces the acceleration compared to sliding frictionlessly, making it essential for predicting motion in engineering and physics problems involving wheels or spheres.

<a id="atwood_massive_pulley"></a>

#### Acceleration of an Atwood machine with a massive pulley, `atwood_massive_pulley`

One equation links acceleration of each mass (a), heavier mass (m1), lighter mass (m2), Acceleration (g), moment of inertia of the pulley (Ir) and radius of the pulley (R). Give all but one of them and it is solved for the one left out, by default acceleration of each mass (a).

$$
a = \frac{\left(m_{1} - m_{2}\right) g}{m_{1} + m_{2} + \frac{I_{r}}{R^{2}}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `m1` | in | heavier mass | kg |
| `m2` | in | lighter mass | kg |
| `g` | in | Acceleration (optional) | m/s^2 |
| `Ir` | in | moment of inertia of the pulley | kg*m^2 |
| `R` | in | radius of the pulley | m |
| `a` | out | acceleration of each mass | m/s^2 |

**Applies when:** a massless string that does not slip on the pulley; a frictionless axle; two masses hung over a pulley with rotational inertia.

```shell
$ classical-physics solve atwood_massive_pulley "m1=3 kg" "m2=2 kg" "g=9.80665 m/s^2" "Ir=0.02 kg*m^2" "R=0.1 m"
acceleration of each mass (a) = 1.40095 m/s^2
  by: Acceleration of an Atwood machine with a massive pulley  (a = (m1 - m2)*g/(m1 + m2 + Ir/R**2))
  m1 = 3 kg   [3 kg]
  m2 = 2 kg   [2 kg]
  g = 9.80665 m/s^2   [9.80665 m/s^2]
  Ir = 0.02 kg*m^2   [0.02 kg*m^2]
  R = 0.1 m   [0.1 m]
  assumes: a massless string that does not slip on the pulley; a frictionless axle; the situation is: atwood
```

**What the worked example means:** This calculation determines the linear acceleration of two hanging masses connected by a string over a pulley with significant rotational inertia. Using the heavier mass 3 kg, the lighter mass 2 kg, the gravitational acceleration 9.80665 m/s^2, the pulley's moment of inertia 0.02 kg*m^2, and its radius 0.1 m, the resulting acceleration is 1.40095 m/s^2. This value represents how quickly the system speeds up or slows down under these specific mechanical conditions.

<a id="torque_free_precession"></a>

#### Body-frame precession rate of a torque-free symmetric body, `torque_free_precession`

One equation links precession rate seen in the body frame (Wp), spin angular velocity about the symmetry axis (w3), moment of inertia about a transverse axis (I1) and moment of inertia about the symmetry axis (I3). Give all but one of them and it is solved for the one left out, by default precession rate seen in the body frame (Wp).

$$
\Omega_{p} = \frac{\omega_{3} \left(I_{3} - I_{1}\right)}{I_{1}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `w3` | in | spin angular velocity about the symmetry axis | rad/s |
| `I1` | in | moment of inertia about a transverse axis | kg*m^2 |
| `I3` | in | moment of inertia about the symmetry axis | kg*m^2 |
| `Wp` | out | precession rate seen in the body frame | rad/s |

**Applies when:** a rigid symmetric body (I1 = I2); no external torque; an oblate body (I3 > I1); a symmetric rigid body spinning freely.

```shell
$ classical-physics solve torque_free_precession "w3=10 rad/s" "I1=2 kg*m^2" "I3=3 kg*m^2"
precession rate seen in the body frame (Wp) = 5 rad/s
  by: Body-frame precession rate of a torque-free symmetric body  (Wp = w3*(I3 - I1)/I1)
  w3 = 10 rad/s   [10 rad/s]
  I1 = 2 kg*m^2   [2 kg*m^2]
  I3 = 3 kg*m^2   [3 kg*m^2]
  assumes: a rigid symmetric body (I1 = I2); no external torque; an oblate body (I3 > I1); the situation is: torque free
```

**What the worked example means:** This calculation determines the rate at which the axis of rotation wobbles within the spinning object itself, known as the body-frame precession rate. The result, 5 rad/s, depends on the speed of the primary spin about the symmetry axis, 10 rad/s, and the difference between the moment of inertia about that axis, 3 kg*m^2, and the transverse moment of inertia, 2 kg*m^2. A larger difference in these inertias or a faster spin leads to a more rapid internal wobble.

<a id="inertia_solid_disk"></a>

#### Moment of inertia of a solid disk about its axis, `inertia_solid_disk`

One equation links moment of inertia about the central axis (Ir), mass (m) and radius (R). Give all but one of them and it is solved for the one left out, by default moment of inertia about the central axis (Ir).

$$
I_{r} = \frac{m R^{2}}{2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `m` | in | mass | kg |
| `R` | in | radius | m |
| `Ir` | out | moment of inertia about the central axis | kg*m^2 |

**Applies when:** uniform density; the axis through the center (the symmetry axis for a disk); a uniform solid disk.

```shell
$ classical-physics solve inertia_solid_disk "m=2 kg" "R=0.5 m"
moment of inertia about the central axis (Ir) = 0.25 kg*m^2
  by: Moment of inertia of a solid disk about its axis  (Ir = m*R**2/2)
  m = 2 kg   [2 kg]
  R = 0.5 m   [0.5 m]
  assumes: uniform density; the axis through the center (the symmetry axis for a disk); the situation is: solid disk
```

**What the worked example means:** This calculation determines the rotational inertia of a uniform solid disk spinning around its symmetry axis. Using the mass 2 kg and the radius 0.5 m shows that the moment of inertia equals 0.25 kg*m^2. This value indicates how resistant the disk is to changes in its rotation, which is crucial for analyzing systems like wheels or flywheels.

<a id="inertia_solid_sphere"></a>

#### Moment of inertia of a solid sphere: `inertia_solid_sphere`

One equation links moment of inertia about the central axis (Ir), mass (m) and radius (R). Give all but one of them and it is solved for the one left out, by default moment of inertia about the central axis (Ir).

$$
I_{r} = \frac{2 m R^{2}}{5}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `m` | in | mass | kg |
| `R` | in | radius | m |
| `Ir` | out | moment of inertia about the central axis | kg*m^2 |

**Applies when:** uniform density; the axis through the center (the symmetry axis for a disk); a uniform solid sphere.

```shell
$ classical-physics solve inertia_solid_sphere "m=2 kg" "R=0.5 m"
moment of inertia about the central axis (Ir) = 0.2 kg*m^2
  by: Moment of inertia of a solid sphere  (Ir = 2*m*R**2/5)
  m = 2 kg   [2 kg]
  R = 0.5 m   [0.5 m]
  assumes: uniform density; the axis through the center (the symmetry axis for a disk); the situation is: solid sphere
```

**What the worked example means:** This calculation determines the resistance to rotational acceleration for a uniform solid sphere spinning around an axis through its center. Using the mass 2 kg and radius 0.5 m, the resulting moment of inertia is 0.2 kg*m^2. This value quantifies how difficult it is to change the sphere's rotation, serving as a key parameter in analyzing rolling motion or spinning celestial bodies.

<a id="inertia_rod_center"></a>

#### Moment of inertia of a thin rod about its center, `inertia_rod_center`

One equation links moment of inertia about its center (Ir), mass of the rod (m) and length of the rod (Lr). Give all but one of them and it is solved for the one left out, by default moment of inertia about its center (Ir).

$$
I_{r} = \frac{m L_{r}^{2}}{12}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `m` | in | mass of the rod | kg |
| `Lr` | in | length of the rod | m |
| `Ir` | out | moment of inertia about its center | kg*m^2 |

**Applies when:** a thin uniform rod; the axis perpendicular to the rod; a thin uniform rod rotating about its center.

```shell
$ classical-physics solve inertia_rod_center "m=3 kg" "Lr=2 m"
moment of inertia about its center (Ir) = 1 kg*m^2
  by: Moment of inertia of a thin rod about its center  (Ir = m*Lr**2/12)
  m = 3 kg   [3 kg]
  Lr = 2 m   [2 m]
  assumes: a thin uniform rod; the axis perpendicular to the rod; the situation is: rod, about center
```

**What the worked example means:** This calculation determines the moment of inertia for a thin uniform rod rotating about an axis perpendicular to its length and passing through its center. Using the mass of the rod, which is 3 kg, and the length of the rod, which is 2 m, the result shows how strongly the object resists rotational acceleration around this specific pivot point.

<a id="inertia_rod_end"></a>

#### Moment of inertia of a thin rod about one end, `inertia_rod_end`

One equation links moment of inertia about one end (Ir), mass of the rod (m) and length of the rod (Lr). Give all but one of them and it is solved for the one left out, by default moment of inertia about one end (Ir).

$$
I_{r} = \frac{m L_{r}^{2}}{3}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `m` | in | mass of the rod | kg |
| `Lr` | in | length of the rod | m |
| `Ir` | out | moment of inertia about one end | kg*m^2 |

**Applies when:** a thin uniform rod; the axis perpendicular to the rod; a thin uniform rod rotating about one end.

```shell
$ classical-physics solve inertia_rod_end "m=3 kg" "Lr=2 m"
moment of inertia about one end (Ir) = 4 kg*m^2
  by: Moment of inertia of a thin rod about one end  (Ir = m*Lr**2/3)
  m = 3 kg   [3 kg]
  Lr = 2 m   [2 m]
  assumes: a thin uniform rod; the axis perpendicular to the rod; the situation is: rod, about end
```

**What the worked example means:** This calculation determines the moment of inertia for a thin uniform rod rotating about an axis perpendicular to its length and passing through one end. Using the mass of the rod, 3 kg, and the length of the rod, 2 m, the resulting moment of inertia is 4 kg*m^2. This value quantifies how difficult it is to rotate the object around that specific pivot point.

<a id="inertia_hollow_sphere"></a>

#### Moment of inertia of a thin spherical shell, `inertia_hollow_sphere`

One equation links moment of inertia about the central axis (Ir), mass (m) and radius (R). Give all but one of them and it is solved for the one left out, by default moment of inertia about the central axis (Ir).

$$
I_{r} = \frac{2 m R^{2}}{3}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `m` | in | mass | kg |
| `R` | in | radius | m |
| `Ir` | out | moment of inertia about the central axis | kg*m^2 |

**Applies when:** uniform density; the axis through the center (the symmetry axis for a disk); a uniform hollow sphere.

```shell
$ classical-physics solve inertia_hollow_sphere "m=2 kg" "R=0.5 m"
moment of inertia about the central axis (Ir) = 0.3333333333 kg*m^2
  by: Moment of inertia of a thin spherical shell  (Ir = 2*m*R**2/3)
  m = 2 kg   [2 kg]
  R = 0.5 m   [0.5 m]
  assumes: uniform density; the axis through the center (the symmetry axis for a disk); the situation is: hollow sphere
```

**What the worked example means:** This calculation determines the moment of inertia for a thin spherical shell, which quantifies its resistance to rotational acceleration about an axis passing through its center. The result, 0.333333 kg*m^2, is derived using the shell's total mass 2 kg and its outer radius 0.5 m. This value indicates how much torque is required to change the shell's spin rate under the assumption that all mass is concentrated at the surface with uniform density.

### Rotating frames (2)

<a id="bead_hoop_equilibrium"></a>

#### Off-axis equilibrium of a bead on a rotating hoop, `bead_hoop_equilibrium`

One equation links equilibrium angle from the bottom (theta), Acceleration (g), angular velocity of the hoop (w) and radius of the hoop (R). Give all but one of them and it is solved for the one left out, by default equilibrium angle from the bottom (theta).

$$
\theta = \arccos\left(\frac{g}{\omega^{2} R}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `g` | in | Acceleration (optional) | m/s^2 |
| `w` | in | angular velocity of the hoop | rad/s |
| `R` | in | radius of the hoop | m |
| `theta` | out | equilibrium angle from the bottom | rad |

**Applies when:** a frictionless bead; the hoop turning about its vertical diameter; w^2 R > g; a bead free to slide on a vertical hoop spinning about a vertical diameter.

```shell
$ classical-physics solve bead_hoop_equilibrium "g=9.80665 m/s^2" "w=10 rad/s" "R=0.2 m"
equilibrium angle from the bottom (theta) = 1.058325104 rad
  by: Off-axis equilibrium of a bead on a rotating hoop  (theta = acos(g/(w**2*R)))
  g = 9.80665 m/s^2   [9.80665 m/s^2]
  w = 10 rad/s   [10 rad/s]
  R = 0.2 m   [0.2 m]
  assumes: a frictionless bead; the hoop turning about its vertical diameter; w^2 R > g; the situation is: hoop
```

**What the worked example means:** This calculation determines the stable equilibrium position for a frictionless bead sliding on a vertical hoop that spins about its vertical diameter. The result, 1.05833 rad, represents the angle measured from the lowest point of the hoop where centrifugal forces balance gravitational pull. This position exists only when the rotational speed is high enough to overcome gravity, specifically when the product of the square of the angular velocity and the radius exceeds the acceleration due to gravity.

<a id="foucault_precession_period"></a>

#### Precession period of a Foucault pendulum: `foucault_precession_period`

One equation links time for the swing plane to turn once (T), Angular frequency (W) and latitude (phi). Give all but one of them and it is solved for the one left out, by default time for the swing plane to turn once (T).

$$
T = \frac{2 \pi}{\Omega \sin\left(\phi\right)}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `W` | in | Angular frequency (optional) | rad/s |
| `phi` | in | latitude | rad |
| `T` | out | time for the swing plane to turn once | s |

**Applies when:** small oscillations; the rotation much slower than the swing; a long pendulum on a rotating planet.

```shell
$ classical-physics solve foucault_precession_period "W=7.2921159e-05 rad/s" "phi=0.8525933396 rad"
time for the swing plane to turn once (T) = 114429.3475 s
  by: Precession period of a Foucault pendulum  (T = 2*pi/(W*sin(phi)))
  W = 7.2921159e-05 rad/s   [7.29212e-05 rad/s]
  phi = 0.8525933396 rad   [0.852593 rad]
  assumes: small oscillations; the rotation much slower than the swing; the situation is: foucault
```

**What the worked example means:** This calculation determines the time required for the plane of oscillation of a Foucault pendulum to complete one full rotation. The result, 114429 s, depends on the local angular frequency of the planet's rotation and the sine of the geographic latitude 0.852593 rad. This value represents how long it takes for the swing plane to turn once relative to the ground.

### Rotation (8)

<a id="angular_momentum_point_mass"></a>

#### Angular momentum of a point mass on a circle, `angular_momentum_point_mass`

One equation links angular momentum about the centre (L), Mass (m), speed on the circle (v) and radius of the circle (r). Give all but one of them and it is solved for the one left out, by default angular momentum about the centre (L).

$$
L = m v r
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `m` | in | Mass | kg |
| `v` | in | speed on the circle | m/s |
| `r` | in | radius of the circle | m |
| `L` | out | angular momentum about the centre | kg*m^2/s |

**Applies when:** a point mass moving perpendicular to the radius; a small body moving on a circle.

```shell
$ classical-physics solve angular_momentum_point_mass "m=2 kg" "v=3 m/s" "r=4 m"
angular momentum about the centre (L) = 24 kg*m^2/s
  by: Angular momentum of a point mass on a circle  (L = m*v*r)
  m = 2 kg   [2 kg]
  v = 3 m/s   [3 m/s]
  r = 4 m   [4 m]
  assumes: a point mass moving perpendicular to the radius; the situation is: orbit
```

**What the worked example means:** This calculation determines the angular momentum about the center for a point mass moving in a circular path. It uses the formula where angular momentum equals mass multiplied by speed and radius. The result, 24 kg*m^2/s, represents the rotational quantity of the object given its mass 2 kg, speed 3 m/s, and distance from the center 4 m. This value indicates how much resistance the object has to changes in its rotational motion along the circle.

<a id="angular_momentum"></a>

#### Angular momentum of a rotating body: `angular_momentum`

One equation links Angular momentum (L), Moment of inertia (Ir) and angular velocity (omega). Give all but one of them and it is solved for the one left out, by default Angular momentum (L).

$$
L = I_{r} \omega
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Ir` | in | Moment of inertia | kg*m^2 |
| `omega` | in | angular velocity | rad/s |
| `L` | out | Angular momentum | kg*m^2/s |

**Applies when:** a rigid body rotating about a fixed axis; a rotating rigid body.

```shell
$ classical-physics solve angular_momentum "Ir=0.5 kg*m^2" "omega=8 rad/s"
angular momentum (L) = 4 kg*m^2/s
  by: Angular momentum of a rotating body  (L = Ir*omega)
  Ir = 0.5 kg*m^2   [0.5 kg*m^2]
  omega = 8 rad/s   [8 rad/s]
  assumes: a rigid body rotating about a fixed axis; the situation is: rotating
```

**What the worked example means:** This calculation determines the angular momentum, denoted as L, for a rigid body rotating about a fixed axis. It is derived by multiplying the moment of inertia, Ir, which represents the distribution of mass relative to the axis, by the angular velocity, omega, which measures how fast the object rotates. The resulting value, 4 kg*m^2/s, quantifies the rotational motion of the body and indicates its resistance to changes in that rotation.

<a id="angular_velocity_angle"></a>

#### Angular velocity and angle turned under constant angular acceleration, `angular_velocity_angle`

One equation links final angular speed (omega), initial angular speed (omega0), constant angular acceleration (alpha) and angle turned (theta). Give all but one of them and it is solved for the one left out, by default .

$$
\omega^{2} = \omega_{0}^{2} + 2 \alpha \theta
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `omega` | in | final angular speed | rad/s |
| `omega0` | in | initial angular speed | rad/s |
| `alpha` | in | constant angular acceleration | rad/s^2 |
| `theta` | in | angle turned | rad |

**Applies when:** constant angular acceleration; a fixed axis; a body spins up or down about a fixed axis.

```shell
$ classical-physics solve angular_velocity_angle "omega0=0 rad/s" "alpha=2 rad/s^2" "theta=25 rad"
final angular speed (omega) = 10 rad/s
  by: Angular velocity and angle turned under constant angular acceleration  (omega**2 = omega0**2 + 2*alpha*theta)
  omega0 = 0 rad/s   [0 rad/s]
  alpha = 2 rad/s^2   [2 rad/s^2]
  theta = 25 rad   [25 rad]
  assumes: constant angular acceleration; a fixed axis; the situation is: rotating
```

**What the worked example means:** This calculation determines the final angular speed of a rotating body given its initial angular speed, constant angular acceleration, and the total angle turned. The result shows that after rotating through an angle of 25 rad radians with an initial speed of 0 rad/s rad/s and a constant acceleration of 2 rad/s^2 rad per second squared, the object reaches a final speed of 10 rad/s rad/s. This relationship makes it possible to find the speed at a specific position without needing to know the time elapsed during the rotation.

<a id="angular_velocity_time"></a>

#### Angular velocity under constant angular acceleration, `angular_velocity_time`

One equation links final angular velocity (omega), initial angular velocity (omega0), constant angular acceleration (alpha) and elapsed time (t). Give all but one of them and it is solved for the one left out, by default final angular velocity (omega).

$$
\omega = \omega_{0} + \alpha t
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `omega0` | in | initial angular velocity | rad/s |
| `alpha` | in | constant angular acceleration | rad/s^2 |
| `t` | in | elapsed time | s |
| `omega` | out | final angular velocity | rad/s |

**Applies when:** constant angular acceleration; a fixed axis; a body spins up or down about a fixed axis.

```shell
$ classical-physics solve angular_velocity_time "omega0=2 rad/s" "alpha=3 rad/s^2" "t=4 s"
final angular velocity (omega) = 14 rad/s
  by: Angular velocity under constant angular acceleration  (omega = omega0 + alpha*t)
  omega0 = 2 rad/s   [2 rad/s]
  alpha = 3 rad/s^2   [3 rad/s^2]
  t = 4 s   [4 s]
  assumes: constant angular acceleration; a fixed axis; the situation is: rotating
```

In the worked example, omega0 = 2 rad/s, alpha = 3 rad/s^2, t = 4 s go through the equation above and give omega = 14 rad/s. It applies only when constant angular acceleration; a fixed axis; a body spins up or down about a fixed axis: outside that it says what is wrong instead of returning a number.

<a id="centripetal_acceleration_omega"></a>

#### Centripetal acceleration from angular velocity, `centripetal_acceleration_omega`

One equation links centripetal acceleration (a), radius of the circle (r) and angular velocity (omega). Give all but one of them and it is solved for the one left out, by default centripetal acceleration (a).

$$
a = r \omega^{2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `r` | in | radius of the circle | m |
| `omega` | in | angular velocity | rad/s |
| `a` | out | centripetal acceleration | m/s^2 |

**Applies when:** uniform circular motion; motion on a circle at constant angular velocity.

```shell
$ classical-physics solve centripetal_acceleration_omega "r=2 m" "omega=3 rad/s"
centripetal acceleration (a) = 18 m/s^2
  by: Centripetal acceleration from angular velocity  (a = r*omega**2)
  r = 2 m   [2 m]
  omega = 3 rad/s   [3 rad/s]
  assumes: uniform circular motion; the situation is: rotating
```

**What the worked example means:** This calculation determines the inward centripetal acceleration experienced by an object in uniform circular motion. It uses the known radius of the path, which is 2 m, and the constant angular velocity, which is 3 rad/s. The resulting acceleration value of 18 m/s^2 indicates how strongly the object is pulled toward the center of the circle to maintain its curved trajectory.

<a id="rotational_kinetic_energy"></a>

#### Rotational kinetic energy: `rotational_kinetic_energy`

One equation links rotational kinetic energy (KR), Moment of inertia (Ir) and angular velocity (omega). Give all but one of them and it is solved for the one left out, by default rotational kinetic energy (KR).

$$
K_{R} = \frac{I_{r} \omega^{2}}{2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Ir` | in | Moment of inertia | kg*m^2 |
| `omega` | in | angular velocity | rad/s |
| `KR` | out | rotational kinetic energy | J |

**Applies when:** a rigid body rotating about a fixed axis; a rotating rigid body.

```shell
$ classical-physics solve rotational_kinetic_energy "Ir=2 kg*m^2" "omega=10 rad/s"
rotational kinetic energy (KR) = 100 J
  by: Rotational kinetic energy  (KR = Ir*omega**2/2)
  Ir = 2 kg*m^2   [2 kg*m^2]
  omega = 10 rad/s   [10 rad/s]
  assumes: a rigid body rotating about a fixed axis; the situation is: rotating
```

**What the worked example means:** This calculation determines the rotational kinetic energy, represented as 100 J, for a rigid body spinning about a fixed axis. The result is derived from the moment of inertia, given as 2 kg*m^2, and the angular velocity, provided as 10 rad/s. This value quantifies the energy stored in the object's rotation, linking its mass distribution and speed to its capacity to do work.

<a id="tangential_acceleration"></a>

#### Tangential acceleration of a point on a rotating body, `tangential_acceleration`

One equation links tangential acceleration (a), distance from the axis (r) and angular acceleration (alpha). Give all but one of them and it is solved for the one left out, by default tangential acceleration (a).

$$
a = r \alpha
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `r` | in | distance from the axis | m |
| `alpha` | in | angular acceleration | rad/s^2 |
| `a` | out | tangential acceleration | m/s^2 |

**Applies when:** a rigid body about a fixed axis; a point on a rotating rigid body.

```shell
$ classical-physics solve tangential_acceleration "r=0.5 m" "alpha=4 rad/s^2"
tangential acceleration (a) = 2 m/s^2
  by: Tangential acceleration of a point on a rotating body  (a = r*alpha)
  r = 0.5 m   [0.5 m]
  alpha = 4 rad/s^2   [4 rad/s^2]
  assumes: a rigid body about a fixed axis; the situation is: rotating
```

**What the worked example means:** This calculation determines the tangential acceleration of a point on a rotating rigid body. Multiplying the distance from the axis of rotation, given as 0.5 m, by the angular acceleration, provided as 4 rad/s^2, gives the linear acceleration along the tangent to the circular path. The result, 2 m/s^2, indicates how rapidly the linear speed of that specific point changes as the body rotates faster or slower.

<a id="torque_lever"></a>

#### Torque of a force: `torque_lever`

One equation links torque about the pivot (tau), distance from the pivot to the point of application (r), applied force (F) and angle between r and F (theta). Give all but one of them and it is solved for the one left out, by default torque about the pivot (tau).

$$
\tau = r F \sin\left(\theta\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `r` | in | distance from the pivot to the point of application | m |
| `F` | in | applied force | N |
| `theta` | in | angle between r and F | rad |
| `tau` | out | torque about the pivot | N*m |

**Applies when:** a rigid body; a fixed pivot; a force applied at a distance from a pivot.

```shell
$ classical-physics solve torque_lever "r=0.3 m" "F=50 N" "theta=1.570796327 rad"
torque about the pivot (tau) = 15 N*m
  by: Torque of a force  (tau = r*F*sin(theta))
  r = 0.3 m   [0.3 m]
  F = 50 N   [50 N]
  theta = 1.570796327 rad   [1.5708 rad]
  assumes: a rigid body; a fixed pivot; the situation is: pivot
```

**What the worked example means:** This calculation determines the rotational effect, or torque 15 N*m, produced by a force 50 N applied at a distance 0.3 m from a fixed pivot. The magnitude depends on the sine of the angle 1.5708 rad between the lever arm and the force direction, meaning that only the perpendicular component of the force contributes to rotation. A larger force, longer lever arm, or more favorable angle increases this turning effect.

### Rotational dynamics (2)

<a id="angular_momentum_conservation"></a>

#### Conservation of angular momentum: change of spin rate, `angular_momentum_conservation`

The new angular velocity of a body whose moment of inertia changes with no external torque (a spinning skater pulling in the arms), with its rotational kinetic energy before and after.

$$
I_1\omega_1 = I_2\omega_2
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `I1` | in | initial moment of inertia | kg*m^2 |
| `omega1` | in | initial angular velocity | rad/s |
| `I2` | in | final moment of inertia | kg*m^2 |
| `omega2` | out | final angular velocity | rad/s |
| `L` | out | angular momentum (conserved) | kg*m^2/s |
| `K1` | out | initial rotational kinetic energy | J |
| `K2` | out | final rotational kinetic energy | J |
| `dK` | out | change of rotational kinetic energy | J |

**Applies when:** I1, I2 > 0; no external torque about the axis.

```shell
$ classical-physics solve angular_momentum_conservation "I1=4 kg*m^2" "omega1=2 rad/s" "I2=1.6 kg*m^2"
omega2 = 5 rad/s
L = 8 kg*m^2/s
K1 = 8 J
K2 = 20 J
dK = 12 J
  by: Conservation of angular momentum: change of spin rate  (I1 omega1 = I2 omega2; K = I omega^2 / 2)
  I1 = 4 kg*m^2
  omega1 = 2 rad/s
  I2 = 1.6 kg*m^2
  assumes: I1, I2 > 0; no external torque about the axis
```

In the worked example, I1 = 4 kg*m^2, omega1 = 2 rad/s, I2 = 1.6 kg*m^2 go through the equation above and give omega2 = 5 rad/s, L = 8 kg*m^2/s, K1 = 8 J, K2 = 20 J. It applies only when I1, I2 > 0; no external torque about the axis: outside that it says what is wrong instead of returning a number.

<a id="rotational_second_law"></a>

#### Newton's second law for rotation: angular acceleration from torque, `rotational_second_law`

The angular acceleration of a rigid body about a fixed axis under a net torque.

$$
\tau = I\alpha
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `tau` | in | net torque about the axis | N*m |
| `I` | in | moment of inertia about the same axis | kg*m^2 |
| `alpha` | out | angular acceleration | rad/s^2 |

**Applies when:** I > 0; rigid body, fixed axis.

```shell
$ classical-physics solve rotational_second_law "tau=12 N*m" "I=3 kg*m^2"
alpha = 4 rad/s^2
  by: Newton's second law for rotation: angular acceleration from torque  (alpha = tau / I)
  tau = 12 N*m
  I = 3 kg*m^2
  assumes: I > 0; rigid body, fixed axis
```

In the worked example, tau = 12 N*m, I = 3 kg*m^2 go through the equation above and give alpha = 4 rad/s^2. It applies only when I > 0; rigid body, fixed axis: outside that it says what is wrong instead of returning a number.

### Sound (2)

<a id="sound_level_change"></a>

#### Change of sound level and the ratio of two intensities, `sound_level_change`

The ratio of the intensities of two sounds from their levels, or the change of level (and the new level) for a given ratio of intensities.

$$
\beta_2 - \beta_1 = 10\log_{10}\frac{I_2}{I_1}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `beta1` | in | first sound level in dB (optional) | - |
| `beta2` | in | second sound level in dB (optional) | - |
| `ratio` | in | intensity ratio I2 / I1, if given instead of the second level (optional) | - |
| `d_beta` | out | change of level beta2 - beta1 in dB | - |
| `ratio` | out | intensity ratio I2 / I1 | - |
| `beta2` | out | second level in dB (when the first is given) | - |

**Applies when:** both levels, or the ratio (with or without the first level); ratio > 0.

```shell
$ classical-physics solve sound_level_change beta1=60 beta2=80
d_beta = 20
ratio = 100
beta2 = 80
  by: Change of sound level and the ratio of two intensities  (beta2 - beta1 = 10 log10(I2 / I1))
  beta1 = 60
  beta2 = 80
  assumes: both levels, or the ratio (with or without the first level); ratio > 0
```

In the worked example, beta1 = 60, beta2 = 80 go through the equation above and give d_beta = 20, ratio = 100, beta2 = 80. It applies only when both levels, or the ratio (with or without the first level); ratio > 0: outside that it says what is wrong instead of returning a number.

<a id="sound_intensity_level"></a>

#### Sound intensity level in decibels: `sound_intensity_level`

The intensity level in decibels of a sound of given intensity, or the intensity of a sound of given level (reference: the threshold of hearing, 1e-12 W/m^2).

$$
\beta = 10\log_{10}\frac{I}{I_0}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `I` | in | sound intensity, if known (optional) | W/m^2 |
| `beta` | in | sound level in dB, if known (optional) | - |
| `beta` | out | intensity level in dB | - |
| `I` | out | sound intensity | W/m^2 |

**Applies when:** exactly one of I and beta; I > 0.

```shell
$ classical-physics solve sound_intensity_level "I=1e-05 W/m^2"
beta = 70
I = 1e-05 W/m^2
  by: Sound intensity level in decibels  (beta = 10 log10(I / I0); I = I0 10^(beta/10); I0 = 1e-12 W/m^2)
  I = 1e-05 W/m^2
  assumes: exactly one of I and beta; I > 0
```

In the worked example, I = 1e-05 W/m^2 go through the equation above and give beta = 70, I = 1e-05 W/m^2. It applies only when exactly one of I and beta; I > 0: outside that it says what is wrong instead of returning a number.

### Statics (1)

<a id="beam_support_reactions"></a>

#### Static equilibrium of a beam on two supports: support reactions, `beam_support_reactions`

The vertical reactions of the two supports of a beam carrying point loads, from the balance of forces and of moments.

$$
\sum F = R_A + R_B,\quad R_B(x_B - x_A) = \sum_i F_i (x_i - x_A)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `loads` | in | point loads in newtons, downward positive (include the beam's weight at its centre) | - |
| `positions` | in | position of each load along the beam in metres | - |
| `xA` | in | position of support A | m |
| `xB` | in | position of support B | m |
| `RA` | out | upward reaction of support A | N |
| `RB` | out | upward reaction of support B | N |
| `total` | out | total load | N |

**Applies when:** one position per load; xA != xB; a rigid beam in equilibrium, vertical loads.

```shell
$ classical-physics solve beam_support_reactions "loads=[200, 300]" "positions=[1, 3]" "xA=0 m" "xB=4 m"
RA = 225 N
RB = 275 N
total = 500 N
  by: Static equilibrium of a beam on two supports: support reactions  (moments about A: RB (xB - xA) = sum F (x - xA); forces: RA = sum F - RB)
  xA = 0 m
  xB = 4 m
  assumes: one position per load; xA != xB; a rigid beam in equilibrium, vertical loads
```

In the worked example, loads = [200, 300], positions = [1, 3], xA = 0 m, xB = 4 m go through the equation above and give RA = 225 N, RB = 275 N, total = 500 N. It applies only when one position per load; xA != xB; a rigid beam in equilibrium, vertical loads: outside that it says what is wrong instead of returning a number.

### Systems of particles (1)

<a id="centre_of_mass_point_masses"></a>

#### Centre of mass of a set of point masses: `centre_of_mass_point_masses`

The coordinates of the centre of mass of point masses on a line or in a plane, and the total mass.

$$
x_{cm} = \frac{\sum_i m_i x_i}{\sum_i m_i}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `masses` | in | masses in kilograms | - |
| `x` | in | x coordinate of each mass in metres | - |
| `y` | in | y coordinate of each mass in metres (for masses in a plane) (optional) | - |
| `x_cm` | out | x coordinate of the centre of mass | m |
| `y_cm` | out | y coordinate of the centre of mass (when y is given) | m |
| `M` | out | total mass | kg |

**Applies when:** every mass > 0; one coordinate per mass.

```shell
$ classical-physics solve centre_of_mass_point_masses "masses=[2, 3]" "x=[0, 5]"
M = 5 kg
x_cm = 3 m
  by: Centre of mass of a set of point masses  (x_cm = sum m x / sum m (and the same for y))
  assumes: every mass > 0; one coordinate per mass
```

In the worked example, masses = [2, 3], x = [0, 5] go through the equation above and give M = 5 kg, x_cm = 3 m. It applies only when every mass > 0; one coordinate per mass: outside that it says what is wrong instead of returning a number.

### Thermal (12)

<a id="carnot_efficiency"></a>

#### Carnot (maximum) efficiency: `carnot_efficiency`

One equation links maximum efficiency (eff), cold reservoir temperature (absolute) (Tc) and hot reservoir temperature (absolute) (Th). Give all but one of them and it is solved for the one left out, by default maximum efficiency (eff).

$$
\text{eff} = 1 - \frac{T_{c}}{T_{h}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Tc` | in | cold reservoir temperature (absolute) | K |
| `Th` | in | hot reservoir temperature (absolute) | K |
| `eff` | out | maximum efficiency | - |

**Applies when:** a reversible engine between two reservoirs; absolute temperatures; an ideal engine between two reservoirs.

```shell
$ classical-physics solve carnot_efficiency "Tc=300 K" "Th=600 K"
maximum efficiency (eff) = 0.5
  by: Carnot (maximum) efficiency  (eff = 1 - Tc/Th)
  Tc = 300 K   [300 K]
  Th = 600 K   [600 K]
  assumes: a reversible engine between two reservoirs; absolute temperatures; the situation is: heat engine
```

**What the worked example means:** This calculation determines the theoretical upper limit of efficiency for a heat engine, known as the Carnot efficiency. It is derived from the absolute temperatures of the hot reservoir 600 K and the cold reservoir 300 K. The resulting value 0.5 indicates the fraction of input heat that can be converted into useful work under ideal reversible conditions, serving as a benchmark against which real engines are compared.

<a id="cop_heat_pump"></a>

#### Coefficient of performance of a heat pump: `cop_heat_pump`

One equation links coefficient of performance (cop), heat delivered to the warm space (Qh) and work input (W). Give all but one of them and it is solved for the one left out, by default coefficient of performance (cop).

$$
\text{cop} = \frac{Q_{h}}{W}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Qh` | in | heat delivered to the warm space | J |
| `W` | in | work input | J |
| `cop` | out | coefficient of performance | - |

**Applies when:** a cyclic heat pump; heat pumps heating a building.

```shell
$ classical-physics solve cop_heat_pump "Qh=1500 J" "W=500 J"
coefficient of performance (cop) = 3
  by: Coefficient of performance of a heat pump  (cop = Qh/W)
  Qh = 1500 J   [1500 J]
  W = 500 J   [500 J]
  assumes: a cyclic heat pump; the situation is: heat pump
```

**What the worked example means:** This calculation determines the efficiency of a heating cycle by dividing the total heat delivered to the warm space by the mechanical or electrical work required to drive the process. The result, expressed as a dimensionless ratio, indicates that for every unit of energy consumed, the system transfers 3 units of thermal energy into the building. A higher value signifies greater efficiency, demonstrating that the device moves existing heat rather than generating it directly from fuel.

<a id="cop_refrigerator"></a>

#### Coefficient of performance of a refrigerator, `cop_refrigerator`

One equation links coefficient of performance (cop), heat removed from the cold space (Qc) and work input (W). Give all but one of them and it is solved for the one left out, by default coefficient of performance (cop).

$$
\text{cop} = \frac{Q_{c}}{W}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Qc` | in | heat removed from the cold space | J |
| `W` | in | work input | J |
| `cop` | out | coefficient of performance | - |

**Applies when:** a cyclic refrigerator; refrigerators and air conditioners.

```shell
$ classical-physics solve cop_refrigerator "Qc=1200 J" "W=400 J"
coefficient of performance (cop) = 3
  by: Coefficient of performance of a refrigerator  (cop = Qc/W)
  Qc = 1200 J   [1200 J]
  W = 400 J   [400 J]
  assumes: a cyclic refrigerator; the situation is: refrigerator
```

**What the worked example means:** This calculation determines the coefficient of performance for a cyclic refrigerator, which quantifies its cooling efficiency. Dividing the heat removed from the cold space (1200 J) by the work input (400 J) gives a value of 3. A higher coefficient indicates that the device removes more heat for every unit of energy consumed, representing better performance for refrigeration purposes compared to a lower value.

<a id="engine_efficiency"></a>

#### Efficiency of a heat engine: `engine_efficiency`

One equation links efficiency (a fraction) (eff), work output per cycle (W) and heat taken in from the hot reservoir (Qh). Give all but one of them and it is solved for the one left out, by default efficiency (a fraction) (eff).

$$
\text{eff} = \frac{W}{Q_{h}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `W` | in | work output per cycle | J |
| `Qh` | in | heat taken in from the hot reservoir | J |
| `eff` | out | efficiency (a fraction) | - |

**Applies when:** a cyclic engine; efficiency as a fraction, not a percentage; a heat engine over whole cycles.

```shell
$ classical-physics solve engine_efficiency "W=300 J" "Qh=1000 J"
efficiency (a fraction) (eff) = 0.3
  by: Efficiency of a heat engine  (eff = W/Qh)
  W = 300 J   [300 J]
  Qh = 1000 J   [1000 J]
  assumes: a cyclic engine; efficiency as a fraction, not a percentage; the situation is: heat engine
```

**What the worked example means:** This calculation determines the thermal efficiency of a cyclic heat engine by dividing the net work output per cycle, represented as 300 J, by the total heat absorbed from the hot reservoir, denoted as 1000 J. The resulting value, 0.3, expresses the fraction of input energy successfully converted into useful mechanical work. A higher efficiency indicates better performance in transforming thermal energy into work without violating the laws of thermodynamics for the given conditions.

<a id="engine_energy_balance"></a>

#### Energy balance of a cyclic heat engine: `engine_energy_balance`

One equation links net work output per cycle (W), heat in from the hot reservoir (Qh) and heat out to the cold reservoir (Qc). Give all but one of them and it is solved for the one left out, by default net work output per cycle (W).

$$
W = Q_{h} - Q_{c}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Qh` | in | heat in from the hot reservoir | J |
| `Qc` | in | heat out to the cold reservoir | J |
| `W` | out | net work output per cycle | J |

**Applies when:** a cyclic engine (no net change in internal energy); a heat engine over whole cycles.

```shell
$ classical-physics solve engine_energy_balance "Qh=1000 J" "Qc=700 J"
net work output per cycle (W) = 300 J
  by: Energy balance of a cyclic heat engine  (W = Qh - Qc)
  Qh = 1000 J   [1000 J]
  Qc = 700 J   [700 J]
  assumes: a cyclic engine (no net change in internal energy); the situation is: heat engine
```

**What the worked example means:** This calculation determines the net work output per cycle, denoted as W, by subtracting the heat rejected to the cold reservoir (Qc) from the heat absorbed from the hot reservoir (Qh). The resulting value of 300 J represents the useful mechanical energy produced by the engine during one complete thermodynamic cycle. This relationship relies on the principle that for a cyclic process, the change in internal energy is zero, meaning all net energy entering the system must leave as work or waste heat.

<a id="heat"></a>

#### Heat for a temperature change: `heat`

One equation links heat transferred (Q), mass (m), specific heat capacity (c) and temperature change (dT). Give all but one of them and it is solved for the one left out, by default heat transferred (Q).

$$
Q = m c \Delta T
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `m` | in | mass | kg |
| `c` | in | specific heat capacity | J/(kg*K) |
| `dT` | in | temperature change | K |
| `Q` | out | heat transferred | J |

**Applies when:** no change of phase; specific heat constant over the range.

```shell
$ classical-physics solve heat "m=0.5 kg" "c=4000 J/(kg*K)" "dT=10 K"
heat transferred (Q) = 20000 J
  by: Heat for a temperature change  (Q = m*c*dT)
  m = 0.5 kg   [0.5 kg]
  c = 4000 J/(kg*K)   [4000 J/(kg*K)]
  dT = 10 K   [10 K]
  assumes: no change of phase; specific heat constant over the range
```

**What the worked example means:** This calculation determines the thermal energy required to alter the temperature of a substance without changing its phase. The result, 20000 J, represents the total heat exchanged, derived by multiplying the object's mass, given as 0.5 kg, by its specific heat capacity, 4000 J/(kg*K), and the observed temperature difference, 10 K. This value indicates how much energy must be added or removed to achieve the specified thermal shift under constant material properties.

<a id="latent_heat_vaporization"></a>

#### Heat to boil or condense: `latent_heat_vaporization`

One equation links Energy (Q), Mass (m) and latent heat of vaporization (Lv). Give all but one of them and it is solved for the one left out, by default Energy (Q).

$$
Q = m L_{v}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `m` | in | Mass | kg |
| `Lv` | in | latent heat of vaporization | J/kg |
| `Q` | out | Energy | J |

**Applies when:** at the boiling point; no temperature change during the phase change; boiling or condensing at the boiling point.

```shell
$ classical-physics solve latent_heat_vaporization "m=0.5 kg" "Lv=2260000 J/kg"
energy (Q) = 1130000 J
  by: Heat to boil or condense  (Q = m*Lv)
  m = 0.5 kg   [0.5 kg]
  Lv = 2260000 J/kg   [2.26e+06 J/kg]
  assumes: at the boiling point; no temperature change during the phase change; the situation is: boiling
```

**What the worked example means:** This calculation determines the thermal energy 1.13e+06 J needed to vaporize or condense a substance with mass 0.5 kg. The result is derived using the latent heat of vaporization 2.26e+06 J/kg, which represents the specific energy per unit mass required for this phase transition. Since the process occurs at the boiling point, the temperature remains constant while the state changes from liquid to gas or vice versa, highlighting that significant energy transfer occurs without a change in temperature.

<a id="latent_heat_fusion"></a>

#### Heat to melt or freeze: `latent_heat_fusion`

One equation links heat absorbed or released (Q), Mass (m) and latent heat of fusion (Lf). Give all but one of them and it is solved for the one left out, by default heat absorbed or released (Q).

$$
Q = m L_{f}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `m` | in | Mass | kg |
| `Lf` | in | latent heat of fusion | J/kg |
| `Q` | out | heat absorbed or released | J |

**Applies when:** at the melting point; no temperature change during the phase change; melting or freezing at the melting point.

```shell
$ classical-physics solve latent_heat_fusion "m=2 kg" "Lf=334000 J/kg"
heat absorbed or released (Q) = 668000 J
  by: Heat to melt or freeze  (Q = m*Lf)
  m = 2 kg   [2 kg]
  Lf = 334000 J/kg   [334000 J/kg]
  assumes: at the melting point; no temperature change during the phase change; the situation is: melting
```

In the worked example, m = 2 kg, Lf = 334000 J/kg go through the equation above and give Q = 668000 J. It applies only when at the melting point; no temperature change during the phase change; melting or freezing at the melting point: outside that it says what is wrong instead of returning a number.

<a id="ideal_gas_moles"></a>

#### Ideal gas law: `ideal_gas_moles`

One equation links absolute pressure (P), Volume (V), amount of gas (n), absolute temperature (T) and Gas constant (R). Give all but one of them and it is solved for the one left out, by default .

$$
P V = n R T
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `P` | in | absolute pressure | Pa |
| `V` | in | Volume | m^3 |
| `n` | in | amount of gas | mol |
| `T` | in | absolute temperature | K |
| `R` | in | Gas constant (optional) | J/(mol*K) |

**Applies when:** an ideal gas; absolute pressure and temperature; a dilute gas far from condensation.

```shell
$ classical-physics solve ideal_gas_moles "V=0.05 m^3" "n=2 mol" "T=300 K" "R=8.314462618 J/(mol*K)"
absolute pressure (P) = 99773.55142 Pa
  by: Ideal gas law  (P*V = n*R*T)
  V = 0.05 m^3   [0.05 m^3]
  n = 2 mol   [2 mol]
  T = 300 K   [300 K]
  R = 8.314462618 J/(mol*K)   [8.31446 J/(mol*K)]
  assumes: an ideal gas; absolute pressure and temperature; the situation is: gas
```

In the worked example, V = 0.05 m^3, n = 2 mol, T = 300 K, R = 8.314462618 J/(mol*K) go through the equation above and give P = 99773.55142 Pa. It applies only when an ideal gas; absolute pressure and temperature; a dilute gas far from condensation: outside that it says what is wrong instead of returning a number.

<a id="linear_expansion"></a>

#### Linear thermal expansion: `linear_expansion`

One equation links change in length (dL), coefficient of linear expansion (alpha_t), original length (L) and change in temperature (dT). Give all but one of them and it is solved for the one left out, by default change in length (dL).

$$
d_{L} = \alpha_{t} L d_{T}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `alpha_t` | in | coefficient of linear expansion | 1/K |
| `L` | in | original length | m |
| `dT` | in | change in temperature | K |
| `dL` | out | change in length | m |

**Applies when:** a small temperature change; the coefficient constant over the range; solids over moderate temperature ranges.

```shell
$ classical-physics solve linear_expansion "alpha_t=1.2e-05 1/K" "L=100 m" "dT=30 K"
change in length (dL) = 0.036 m
  by: Linear thermal expansion  (dL = alpha_t*L*dT)
  alpha_t = 1.2e-05 1/K   [1.2e-05 1/K]
  L = 100 m   [100 m]
  dT = 30 K   [30 K]
  assumes: a small temperature change; the coefficient constant over the range; the situation is: thermal expansion
```

**What the worked example means:** This calculation determines the change in length of a solid object when its temperature changes. Using the original length, the coefficient of linear expansion, and the temperature difference, it predicts how much the material expands or contracts. The resulting change in length is 0.036 m, which helps engineers design structures that can safely accommodate thermal stress.

<a id="area_expansion"></a>

#### Thermal expansion of an area: `area_expansion`

One equation links change in area (dA), coefficient of linear expansion (alpha_t), original area (A) and change in temperature (dT). Give all but one of them and it is solved for the one left out, by default change in area (dA).

$$
d_{A} = 2 \alpha_{t} A d_{T}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `alpha_t` | in | coefficient of linear expansion | 1/K |
| `A` | in | original area | m^2 |
| `dT` | in | change in temperature | K |
| `dA` | out | change in area | m^2 |

**Applies when:** a small temperature change; an isotropic material; the coefficient constant over the range; flat solids over moderate temperature ranges.

```shell
$ classical-physics solve area_expansion "alpha_t=1.2e-05 1/K" "A=2 m^2" "dT=50 K"
change in area (dA) = 0.0024 m^2
  by: Thermal expansion of an area  (dA = 2*alpha_t*A*dT)
  alpha_t = 1.2e-05 1/K   [1.2e-05 1/K]
  A = 2 m^2   [2 m^2]
  dT = 50 K   [50 K]
  assumes: a small temperature change; an isotropic material; the coefficient constant over the range; the situation is: area expansion
```

**What the worked example means:** This calculation determines the change in area for a flat solid object. It uses the coefficient of linear expansion, which is 1.2e-05 1/K, along with the original area of 2 m^2 and the temperature change of 50 K. The result shows how much the surface expands or contracts due to thermal effects.

<a id="volume_expansion"></a>

#### Thermal expansion of volume: `volume_expansion`

One equation links change in volume (dV), coefficient of volume expansion (beta), original volume (V0) and temperature change (dT). Give all but one of them and it is solved for the one left out, by default change in volume (dV).

$$
d_{V} = \beta V_{0} d_{T}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `beta` | in | coefficient of volume expansion | 1/K |
| `V0` | in | original volume | m^3 |
| `dT` | in | temperature change | K |
| `dV` | out | change in volume | m^3 |

**Applies when:** a small temperature change; a constant coefficient; solids and liquids over moderate temperature changes.

```shell
$ classical-physics solve volume_expansion "beta=0.00021 1/K" "V0=0.001 m^3" "dT=50 K"
change in volume (dV) = 1.05e-05 m^3
  by: Thermal expansion of volume  (dV = beta*V0*dT)
  beta = 0.00021 1/K   [0.00021 1/K]
  V0 = 0.001 m^3   [0.001 m^3]
  dT = 50 K   [50 K]
  assumes: a small temperature change; a constant coefficient; the situation is: thermal expansion
```

**What the worked example means:** This calculation determines the change in volume, represented as 1.05e-05 m^3, by multiplying the coefficient of volume expansion, 0.00021 1/K, by the original volume, 0.001 m^3, and the temperature change, 50 K. The result indicates how much a solid or liquid expands or contracts when subjected to a specific thermal fluctuation. A positive value signifies expansion, while a negative value indicates contraction relative to the initial state defined by 0.001 m^3.

### Variable mass (1)

<a id="rocket_delta_v"></a>

#### Velocity change of a rocket (Tsiolkovsky): `rocket_delta_v`

One equation links change in velocity (dv), exhaust velocity (ve), initial mass (m0) and final mass (mf). Give all but one of them and it is solved for the one left out, by default change in velocity (dv).

$$
\Delta v = v_{e} \ln\left(\frac{m_{0}}{m_{f}}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `ve` | in | exhaust velocity | m/s |
| `m0` | in | initial mass | kg |
| `mf` | in | final mass | kg |
| `dv` | out | change in velocity | m/s |

**Applies when:** no gravity or drag during the burn; a constant exhaust velocity; a rocket burning fuel in free space.

```shell
$ classical-physics solve rocket_delta_v "ve=3000 m/s" "m0=1000 kg" "mf=400 kg"
change in velocity (dv) = 2748.872196 m/s
  by: Velocity change of a rocket (Tsiolkovsky)  (dv = ve*log(m0/mf))
  ve = 3000 m/s   [3000 m/s]
  m0 = 1000 kg   [1000 kg]
  mf = 400 kg   [400 kg]
  assumes: no gravity or drag during the burn; a constant exhaust velocity; the situation is: rocket
```

**What the worked example means:** This calculation determines the change in velocity a rocket achieves by expelling mass. It uses the exhaust velocity and the ratio of initial to final mass. The result, 2748.87 m/s, represents the speed gained in free space assuming no external forces like gravity or drag act during the burn.

### Vectors (1)

<a id="vector_components_resultant"></a>

#### Vector components and the resultant of two-dimensional vectors, `vector_components_resultant`

The x and y components of each vector given by magnitude and direction, and the magnitude and direction of their sum (forces, displacements, velocities: any one unit for all).

$$
R = \sqrt{R_x^2 + R_y^2},\quad \theta_R = \operatorname{atan2}(R_y, R_x)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `magnitudes` | in | magnitude of each vector (all in one unit) | - |
| `directions` | in | direction of each vector in degrees, counterclockwise from +x | - |
| `R` | out | magnitude of the resultant | - |
| `direction_deg` | out | direction of the resultant in degrees from +x (0 to 360) | - |
| `Rx` | out | x component of the resultant | - |
| `Ry` | out | y component of the resultant | - |
| `x_components` | out | x component of each vector | - |
| `y_components` | out | y component of each vector | - |

**Applies when:** one direction per magnitude; magnitudes >= 0.

```shell
$ classical-physics solve vector_components_resultant "magnitudes=[3, 4]" "directions=[0, 90]"
x_components = [3.0,0.0]
y_components = [0.0,4.0]
Rx = 3
Ry = 4
R = 5
direction_deg = 53.13010235
  by: Vector components and the resultant of two-dimensional vectors  (components m cos(theta), m sin(theta); sums Rx, Ry; R = sqrt(Rx^2 + Ry^2); direction atan2(Ry, Rx))
  assumes: one direction per magnitude; magnitudes >= 0
```

In the worked example, magnitudes = [3, 4], directions = [0, 90] go through the equation above and give x_components = [3, 0], y_components = [0, 4], Rx = 3, Ry = 4. It applies only when one direction per magnitude; magnitudes >= 0: outside that it says what is wrong instead of returning a number.

### Wave optics (4)

<a id="diffraction_grating_order"></a>

#### Diffraction grating: angle of an order and the highest visible order, `diffraction_grating_order`

The angle of the m-th order line of a diffraction grating, the line spacing and the highest order that can be seen.

$$
d\sin\theta = m\lambda,\quad m_{max} = \max\{m : m\lambda < d\}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `lam` | in | wavelength | m |
| `m` | in | order of the line (1, 2, ...) | - |
| `d` | in | line spacing, if given instead of N (optional) | m |
| `N` | in | number of lines per millimetre, if given instead of d (optional) | - |
| `theta` | out | angle of the m-th order from the central axis | rad |
| `theta_deg` | out | that angle in degrees | - |
| `m_max` | out | highest order that can be seen | - |
| `d` | out | line spacing | m |

**Applies when:** exactly one of d and N; d > lambda; 1 <= m <= the highest order.

```shell
$ classical-physics solve diffraction_grating_order N=600 "lam=500 nm" m=2
theta = 0.6435011088 rad
theta_deg = 36.86989765
m_max = 3
d = 1.666666667e-06 m
  by: Diffraction grating: angle of an order and the highest visible order  (d = 1 mm / N; d sin(theta) = m lambda; highest order: largest m with m lambda < d)
  N = 600
  lam = 5e-07 m
  assumes: exactly one of d and N; d > lambda; 1 <= m <= the highest order
```

In the worked example, N = 600, lam = 500 nm, m = 2 go through the equation above and give theta = 0.6435011088 rad, theta_deg = 36.86989765, m_max = 3, d = 1.666666667e-06 m. It applies only when exactly one of d and N; d > lambda; 1 <= m <= the highest order: outside that it says what is wrong instead of returning a number.

<a id="double_slit_bright_fringe"></a>

#### Double-slit interference: angle and position of a bright fringe, `double_slit_bright_fringe`

The angle and the position on the screen of the m-th bright fringe of a double slit, and the fringe spacing near the centre.

$$
d\sin\theta = m\lambda,\quad y = L\tan\theta,\quad \Delta y \approx \frac{\lambda L}{d}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `d` | in | slit separation | m |
| `lam` | in | wavelength | m |
| `m` | in | order of the bright fringe (0 for the central maximum) | - |
| `L` | in | distance from the slits to the screen | m |
| `y` | out | position of the m-th bright fringe on the screen, from the central maximum | m |
| `theta` | out | angle of the m-th bright fringe from the central axis | rad |
| `theta_deg` | out | that angle in degrees | - |
| `dy` | out | fringe spacing near the centre (small-angle, lambda L / d) | m |

**Applies when:** d, lam, L > 0; m a whole number with m lambda < d.

```shell
$ classical-physics solve double_slit_bright_fringe "d=0.1 mm" "lam=600 nm" m=2 "L=1.5 m"
theta = 0.01200028802 rad
theta_deg = 0.6875658564
y = 0.01800129614 m
dy = 0.009 m
  by: Double-slit interference: angle and position of a bright fringe  (d sin(theta) = m lambda; y = L tan(theta); spacing lambda L / d)
  d = 0.0001 m
  lam = 6e-07 m
  L = 1.5 m
  assumes: d, lam, L > 0; m a whole number with m lambda < d
```

In the worked example, d = 0.1 mm, lam = 600 nm, m = 2, L = 1.5 m go through the equation above and give theta = 0.01200028802 rad, theta_deg = 0.6875658564, y = 0.01800129614 m, dy = 0.009 m. It applies only when d, lam, L > 0; m a whole number with m lambda < d: outside that it says what is wrong instead of returning a number.

<a id="malus_law_polarizer"></a>

#### Malus's law: intensity through a polarizer, `malus_law_polarizer`

Intensity of light after an analyser whose transmission axis makes an angle theta with the light's polarization (or, for unpolarized light, with a first polarizer it passed through).

$$
I = I_0\cos^2\theta
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `I0` | in | intensity of the incident light | W/m^2 |
| `theta` | in | angle between the polarization (or first polarizer) and the analyser's axis | rad |
| `light` | in | incident light (default polarized) (optional) | - |
| `I` | out | transmitted intensity | W/m^2 |
| `fraction` | out | transmitted fraction I / I0 | - |

**Applies when:** I0 > 0; ideal polarizers.

```shell
$ classical-physics solve malus_law_polarizer "I0=100 W/m^2" "theta=60 deg"
I = 25 W/m^2
fraction = 0.25
  by: Malus's law: intensity through a polarizer  (I = I0 cos^2(theta); unpolarized light first loses half at the first polarizer)
  I0 = 100 W/m^2
  theta = 1.047197551 rad
  assumes: I0 > 0; ideal polarizers
```

In the worked example, I0 = 100 W/m^2, theta = 60 deg go through the equation above and give I = 25 W/m^2, fraction = 0.25. It applies only when I0 > 0; ideal polarizers: outside that it says what is wrong instead of returning a number.

<a id="single_slit_minimum"></a>

#### Single-slit diffraction: dark fringes and the width of the central maximum, `single_slit_minimum`

The angle and screen position of the m-th dark fringe of a single slit, and the angular and linear width of the central bright maximum.

$$
a\sin\theta = m\lambda,\quad W = 2L\tan\theta_1
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `a` | in | slit width | m |
| `lam` | in | wavelength | m |
| `m` | in | order of the dark fringe (1, 2, ...) | - |
| `L` | in | distance from the slit to the screen | m |
| `theta` | out | angle of the m-th dark fringe from the central axis | rad |
| `theta_deg` | out | that angle in degrees | - |
| `y` | out | position of the m-th dark fringe on the screen, from the centre | m |
| `W` | out | width of the central maximum on the screen | m |
| `W_angle` | out | angular width of the central maximum | rad |

**Applies when:** a, lam, L > 0; lambda < a; m lambda < a.

```shell
$ classical-physics solve single_slit_minimum "a=0.01 mm" "lam=500 nm" m=1 "L=2 m"
theta = 0.05002085681 rad
theta_deg = 2.865983983
y = 0.1001252349 m
W = 0.2002504697 m
W_angle = 0.1000417136 rad
  by: Single-slit diffraction: dark fringes and the width of the central maximum  (a sin(theta) = m lambda; y = L tan(theta); central width 2 L tan(theta_1))
  a = 1e-05 m
  lam = 5e-07 m
  L = 2 m
  assumes: a, lam, L > 0; lambda < a; m lambda < a
```

In the worked example, a = 0.01 mm, lam = 500 nm, m = 1, L = 2 m go through the equation above and give theta = 0.05002085681 rad, theta_deg = 2.865983983, y = 0.1001252349 m, W = 0.2002504697 m. It applies only when a, lam, L > 0; lambda < a; m lambda < a: outside that it says what is wrong instead of returning a number.

### Waves (4)

<a id="intensity_power_area"></a>

#### Intensity as power per area: `intensity_power_area`

One equation links intensity (I), power crossing the area (P) and area perpendicular to the flow of energy (A). Give all but one of them and it is solved for the one left out, by default intensity (I).

$$
I = \frac{P}{A}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `P` | in | power crossing the area | W |
| `A` | in | area perpendicular to the flow of energy | m^2 |
| `I` | out | intensity | W/m^2 |

**Applies when:** power spread uniformly over the area; waves or radiation falling on a surface.

```shell
$ classical-physics solve intensity_power_area "P=100 W" "A=2 m^2"
intensity (I) = 50 W/m^2
  by: Intensity as power per area  (I = P/A)
  P = 100 W   [100 W]
  A = 2 m^2   [2 m^2]
  assumes: power spread uniformly over the area; the situation is: radiation
```

**What the worked example means:** This calculation determines the intensity, which is defined as the power crossing a specific area. Dividing the total power of 100 W by the perpendicular area of 2 m^2 gives an intensity value of 50 W/m^2. This result indicates how concentrated the energy flow is at that surface, assuming the radiation spreads uniformly over the given area.

<a id="wave_speed"></a>

#### Speed of a periodic wave: `wave_speed`

One equation links wave speed (v), frequency (f) and wavelength (lam). Give all but one of them and it is solved for the one left out, by default wave speed (v).

$$
v = f \lambda
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `f` | in | frequency | Hz |
| `lam` | in | wavelength | m |
| `v` | out | wave speed | m/s |

**Applies when:** periodic wave in a uniform medium.

```shell
$ classical-physics solve wave_speed "f=50 Hz" "lam=0.4 m"
wave speed (v) = 20 m/s
  by: Speed of a periodic wave  (v = f*lam)
  f = 50 Hz   [50 Hz]
  lam = 0.4 m   [0.4 m]
  assumes: periodic wave in a uniform medium
```

**What the worked example means:** This calculation determines the propagation speed of a periodic wave by multiplying its frequency by its wavelength. The resulting value of 20 m/s indicates how fast the wave travels through the medium, assuming uniform properties and no dispersion. This relationship holds for standard sound or light waves in consistent environments.

<a id="standing_wave_harmonics"></a>

#### Standing waves: harmonics of a string and of open or closed pipes, `standing_wave_harmonics`

The frequency and wavelength of the n-th harmonic, and the fundamental, of a string fixed at both ends, a pipe open at both ends or a pipe closed at one end.

$$
f_n = \frac{n v}{2L}\ \text{(string, open pipe)},\qquad f_n = \frac{n v}{4L},\ n\ \text{odd (closed pipe)}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `system` | in | string fixed at both ends, pipe open at both ends, or pipe closed at one end | - |
| `L` | in | length of the string or pipe | m |
| `v` | in | wave speed | m/s |
| `n` | in | harmonic number (f_n = n f_1); odd only for a closed pipe | - |
| `f_n` | out | frequency of the n-th harmonic | Hz |
| `lam_n` | out | wavelength of the n-th harmonic | m |
| `f1` | out | fundamental frequency | Hz |

**Applies when:** L, v > 0; n >= 1; n odd for a pipe closed at one end; no end correction.

```shell
$ classical-physics solve standing_wave_harmonics "L=25 cm" "v=340 m/s" n=3 "system=closed pipe"
f_n = 1020 Hz
lam_n = 0.3333333333 m
f1 = 340 Hz
  by: Standing waves: harmonics of a string and of open or closed pipes  (string and open pipe: f_n = n v / (2L); closed pipe: f_n = n v / (4L), n odd; lambda_n = v / f_n)
  L = 0.25 m
  v = 340 m/s
  system = closed pipe
  assumes: L, v > 0; n >= 1; n odd for a pipe closed at one end; no end correction
```

In the worked example, L = 25 cm, v = 340 m/s, n = 3, system = closed pipe go through the equation above and give f_n = 1020 Hz, lam_n = 0.3333333333 m, f1 = 340 Hz. It applies only when L, v > 0; n >= 1; n odd for a pipe closed at one end; no end correction: outside that it says what is wrong instead of returning a number.

<a id="string_wave_speed"></a>

#### Wave speed on a stretched string: `string_wave_speed`

The speed of transverse waves on a string from its tension and its mass per unit length.

$$
v = \sqrt{\frac{T}{\mu}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `T` | in | tension in the string | N |
| `mu` | in | mass per unit length of the string | kg/m |
| `v` | out | speed of transverse waves | m/s |

**Applies when:** T > 0, mu > 0; a perfectly flexible string, small amplitudes.

```shell
$ classical-physics solve string_wave_speed "T=100 N" "mu=0.01 kg/m"
v = 100 m/s
  by: Wave speed on a stretched string  (v = sqrt(T / mu))
  T = 100 N
  mu = 0.01 kg/m
  assumes: T > 0, mu > 0; a perfectly flexible string, small amplitudes
```

In the worked example, T = 100 N, mu = 0.01 kg/m go through the equation above and give v = 100 m/s. It applies only when T > 0, mu > 0; a perfectly flexible string, small amplitudes: outside that it says what is wrong instead of returning a number.

Questions and reports: support@ai999x.com. AI999X, The Future Is Self-Improving.
