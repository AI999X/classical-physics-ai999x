# Classical Physics: the desktop app, user guide

**Classical Physics**, desktop app 1.6.5 for Windows and Linux. **Creator / engineer:** Ibnul Jaif Farabi · **Company:** AI999X · support@ai999x.com

Classical Physics covers kinematics, dynamics, energy, momentum, rotation, gravitation, oscillations, waves and sound, optics, thermodynamics, electricity and magnetism. Its desktop app holds all 162 calculations in 37 topics: pick one, change its inputs and the answer, a live scene and charts follow at once. It runs on your own computer with no network connection and nothing else to install. The pictures in this guide were taken in this app.

## Contents

- 1. Get it and start it
- 2. The window
- 3. Compute a calculation
- 4. Ask in words
- 5. The live scene and the charts
- 6. Light and dark
- 7. Every calculation
- 8. Other ways to use it

## 1. Get it and start it

| system | download |
|---|---|
| Windows x86-64 | https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-classical-physics-1.6.5-windows-x86_64.zip |
| Linux x86-64 | https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-classical-physics-1.6.5-linux-x86_64.tar.gz |

Windows: Download ai999x-classical-physics-1.6.5-windows-x86_64.zip, right-click it → Extract All, then open the folder and start ai999x-classical-physics.exe. If Windows SmartScreen asks, choose More info → Run anyway.

Linux:

```shell
curl -LO https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-classical-physics-1.6.5-linux-x86_64.tar.gz
curl -LO https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-classical-physics-1.6.5-linux-x86_64.tar.gz
./ai999x-classical-physics-1.6.5-linux-x86_64/ai999x-classical-physics
```

Checksums of every download: https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/SHA256SUMS. Install steps for every app: https://www.ai999x.com/desktop-cli/

## 2. The window

![The Classical Physics app, opened on Translational kinetic energy with its worked example.](https://www.ai999x.com/?ax_shot=classical-physics-ai999x/hero.webp)

*The Classical Physics app, opened on Translational kinetic energy with its worked example.*

On the left, a search box and the topics with their calculations. In the middle, the chosen calculation: its answer at the top, then the live scene (or its charts), what the answer means and the formula. On the right, its inputs, each with a slider where that makes sense, and the Compute and Reset buttons. At the bottom, a box for a question in words.

## 3. Compute a calculation

Open Energy › Translational kinetic energy (or type part of its name in the search box). It opens with a worked example filled in The answer: K = 18 J.

Change any input (type a number with its unit, or drag its slider) and the answer, the scene and the charts follow; Compute recomputes after typing, Reset brings back the worked example. An input outside the range the formula holds for is refused with the reason, never answered with a guessed number.

![Translational kinetic energy: the live scene of the worked example.](https://www.ai999x.com/?ax_shot=classical-physics-ai999x/scene-1.webp)

*Translational kinetic energy: the live scene of the worked example.*

## 4. Ask in words

Type a question as you would say it, for example “A 3 kg object moves at 4 m/s. What is its kinetic energy?” The app reads the quantities and their units, picks the calculation and answers; the command-line program reads it the same way and printed:

```text
Read as: Translational kinetic energy
kinetic energy (K) = 24 J
  by: Translational kinetic energy  (K = m*v**2/2)
```

When a question names a quantity the app cannot place, it says what is missing instead of guessing.

## 5. The live scene and the charts

Every calculation draws a scene from its answer: it moves as an input changes. The Charts button shows charts that vary one input around your values; Enlarge opens one in a large view. Scenes from other topics:

![Angular frequency of a damped oscillator](https://www.ai999x.com/?ax_shot=classical-physics-ai999x/scene-2.webp)

*Angular frequency of a damped oscillator*

![Escape speed](https://www.ai999x.com/?ax_shot=classical-physics-ai999x/scene-3.webp)

*Escape speed*

![Velocity change of a rocket (Tsiolkovsky)](https://www.ai999x.com/?ax_shot=classical-physics-ai999x/scene-4.webp)

*Velocity change of a rocket (Tsiolkovsky)*

![Reactance of a capacitor](https://www.ai999x.com/?ax_shot=classical-physics-ai999x/scene-5.webp)

*Reactance of a capacitor*

![Charge on a capacitor](https://www.ai999x.com/?ax_shot=classical-physics-ai999x/scene-6.webp)

*Charge on a capacitor*

![Power from current and resistance](https://www.ai999x.com/?ax_shot=classical-physics-ai999x/scene-7.webp)

*Power from current and resistance*

![Angular frequency and frequency](https://www.ai999x.com/?ax_shot=classical-physics-ai999x/scene-8.webp)

*Angular frequency and frequency*

![Translational kinetic energy: a chart, enlarged.](https://www.ai999x.com/?ax_shot=classical-physics-ai999x/chart-1.webp)

*Translational kinetic energy: a chart, enlarged.*

![Angular frequency of a damped oscillator: a chart, enlarged.](https://www.ai999x.com/?ax_shot=classical-physics-ai999x/chart-2.webp)

*Angular frequency of a damped oscillator: a chart, enlarged.*

## 6. Light and dark

The app follows a light or a dark theme (the switch is in its top bar).

![The Classical Physics app in the dark theme.](https://www.ai999x.com/?ax_shot=classical-physics-ai999x/hero-dark.webp)

*The Classical Physics app in the dark theme.*

## 7. Every calculation

162 calculations in 37 topics; each has its own entry on the model's page (https://www.ai999x.com/models/classical-physics-ai999x/) with its formula, worked example and charts.

| topic | calculations |
|---|---|
| AC circuits | Reactance of a capacitor, Reactance of an inductor, Resonant frequency of an LC circuit, Rms current of a sinusoidal supply, Rms voltage of a sinusoidal supply, Series RLC circuit on an AC supply: impedance, current, phase and power |
| Capacitors | Capacitance of parallel plates, Charge on a capacitor, Energy stored in a capacitor |
| Circuits | Power from current and resistance, Power from voltage and resistance, Resistance of a uniform wire, Two resistors in series and in parallel |
| Circular | Angular frequency and frequency, Centripetal acceleration, Centripetal force, Period and frequency, Speed on a circle from angular speed |
| Collisions | Final velocity of the first body in a one-dimensional elastic collision, Final velocity of the second body in a one-dimensional elastic collision |
| DC circuits | Terminal voltage of a source with internal resistance |
| Dynamics | Acceleration down a frictionless incline, Average net force from a change of momentum, Component of the weight along an incline, Impulse of a constant force, Linear momentum, Net force and acceleration, Net force with an opposing friction force, Normal force on a level surface, Normal force on an incline, Sliding friction, Tension in a rope holding a hanging body, Weight in a uniform gravitational field |
| Elasticity | Bulk compression: volume change under a pressure change (bulk modulus), Shear: shear stress, shear strain and displacement (shear modulus), Tension or compression: stress, strain and change of length (Young's modulus) |
| Electricity | Charge carried by a steady current, Electric power, Voltage across an ohmic resistance |
| Electromagnetic induction | Faraday's law: average emf induced by a change of magnetic flux, Ideal transformer: secondary voltage and current |
| Electrostatics | Coulomb's law: force between two point charges, Electric field of a point charge, Electric potential of a point charge, Force on a charge in an electric field, Gauss's law: electric field of a line, a plane, a charged sphere or a shell, Potential difference in a uniform field |
| Energy | Average power, Energy stored in a linear spring, Force of a linear spring, Gravitational potential energy near the surface, Power of a force on a moving body, Speed of a body launched by a spring, Translational kinetic energy, Work by a constant force, Work-energy theorem |
| Fluids | Absolute pressure at a depth in a fluid, Flow rate as volume per time, Flow rate through a cross section, Mass density, Pressure of a column of fluid, Pressure of a force on an area |
| Forces | Static friction on an incline: critical angle and whether a body slides, Static friction: maximum friction force and whether a body slips |
| Geometric optics | Critical angle for total internal reflection, Magnification and image height of a thin lens or a spherical mirror, Snell's law: angle of refraction |
| Gravitation | Gravitational acceleration at a distance from a mass, Newton's law of universal gravitation: force between two masses, Period of a circular orbit, Speed in a circular orbit |
| Induction | Current dying away in an RL circuit, Emf of a rod moving through a magnetic field, Time constant of an RL circuit |
| Kinematics | Displacement from the mean of initial and final velocity, Displacement under constant acceleration, Maximum height of a vertical throw, Motion at constant velocity, Speed after a displacement under constant acceleration, Speed after falling from rest, Time to the top of a vertical throw, Velocity and displacement after constant acceleration, Velocity under constant acceleration |
| Magnetism | Magnetic field at the centre of a circular loop, Magnetic field inside a long solenoid, Magnetic field of a long straight wire, Magnetic force on a current-carrying wire, Magnetic force on a moving charge, Radius of a charged particle's circle in a magnetic field |
| Optics | Focal length of a spherical mirror, Index of refraction, Thin lens equation, Wavelength of light in a medium |
| Orbits | Escape speed, Orbital period from the semi-major axis (Kepler's third law), Orbital speed on an elliptical orbit (vis-viva) |
| Oscillations | Amplitude-resonance frequency of a driven damped oscillator, Angular frequency of a damped oscillator, Damping ratio, Higher normal mode of a double pendulum, Higher normal-mode frequency of two coupled oscillators, Lower normal mode of a double pendulum, Maximum speed of a mass on a spring, Period of a mass on a spring, Period of a physical pendulum, Period of a simple pendulum, Quality factor of a damped oscillator, Steady-state amplitude of a driven damped oscillator |
| Quantum | De Broglie wavelength, Energy of a photon, Momentum of a photon, Photon energy from wavelength |
| RC circuits | Time constant of an RC circuit, Voltage of a charging capacitor, Voltage of a discharging capacitor |
| Relativity | Rest energy |
| Rigid body | Acceleration of a body rolling without slipping down an incline, Acceleration of an Atwood machine with a massive pulley, Body-frame precession rate of a torque-free symmetric body, Moment of inertia of a solid disk about its axis, Moment of inertia of a solid sphere, Moment of inertia of a thin rod about its center, Moment of inertia of a thin rod about one end, Moment of inertia of a thin spherical shell |
| Rotating frames | Off-axis equilibrium of a bead on a rotating hoop, Precession period of a Foucault pendulum |
| Rotation | Angular momentum of a point mass on a circle, Angular momentum of a rotating body, Angular velocity and angle turned under constant angular acceleration, Angular velocity under constant angular acceleration, Centripetal acceleration from angular velocity, Rotational kinetic energy, Tangential acceleration of a point on a rotating body, Torque of a force |
| Rotational dynamics | Conservation of angular momentum: change of spin rate, Newton's second law for rotation: angular acceleration from torque |
| Sound | Change of sound level and the ratio of two intensities, Sound intensity level in decibels |
| Statics | Static equilibrium of a beam on two supports: support reactions |
| Systems of particles | Centre of mass of a set of point masses |
| Thermal | Carnot (maximum) efficiency, Coefficient of performance of a heat pump, Coefficient of performance of a refrigerator, Efficiency of a heat engine, Energy balance of a cyclic heat engine, Heat for a temperature change, Heat to boil or condense, Heat to melt or freeze, Ideal gas law, Linear thermal expansion, Thermal expansion of an area, Thermal expansion of volume |
| Variable mass | Velocity change of a rocket (Tsiolkovsky) |
| Vectors | Vector components and the resultant of two-dimensional vectors |
| Wave optics | Diffraction grating: angle of an order and the highest visible order, Double-slit interference: angle and position of a bright fringe, Malus's law: intensity through a polarizer, Single-slit diffraction: dark fringes and the width of the central maximum |
| Waves | Intensity as power per area, Speed of a periodic wave, Standing waves: harmonics of a string and of open or closed pipes, Wave speed on a stretched string |

## 8. Other ways to use it

The same engine runs on the command line (`classical-physics`, its own guide), in Python (`pip install classical-physics-ai999x`, `import classical_physics_ai999x`, its own guide) and in the browser on https://www.ai999x.com/models/classical-physics-ai999x/.

## Credits

- Coastlines: Natural Earth (public domain). Lunar maria positions and sizes: IAU Gazetteer of Planetary Nomenclature.

Licence: Apache-2.0 · AI999X · https://www.ai999x.com/models/classical-physics-ai999x/
