"""Classical physics procedures: standard first-course calculations that the formula relations (knowledge/relations.json)
do not hold — a case split (total internal reflection, slipping or holding, the highest visible order), several outputs
from one setting (an AC circuit, a lens image, a loaded beam), lists (centre of mass, support reactions, vector sums)
or a choice of system (string, open or closed pipe). They coexist with the relations and with the compiled procedures
(procedures_compiled.json) and are enabled only once tools/certify_procedures.py has certified them.

Conventions (fixed, so run and reference agree exactly):
- Constants: Coulomb constant k = 8987551792.3 N m^2/C^2 and G = 6.6743e-11 N m^2/kg^2 (the values the relations
  use), standard gravity g = 9.80665 m/s^2, threshold of hearing I0 = 1e-12 W/m^2.
- Angles are read with a unit (deg or rad; a bare number is refused) and returned in radians with a companion in
  degrees. Angles of incidence and refraction are measured from the normal; an incline angle from the horizontal; the
  angle in Faraday's law is between the field and the coil's axis (normal); Malus's angle is between the light's
  polarization and the analyser's transmission axis. Vector directions are degrees counterclockwise from +x, the
  resultant's direction in [0, 360); a zero resultant has no direction and is given direction 0 by convention.
- Lenses and mirrors: real-is-positive. f > 0 for a converging lens or concave mirror; d_i > 0 is a real image; the
  magnification m = -d_i/d_o is negative for an inverted image.
- Coulomb's law: F_radial > 0 is repulsion, < 0 attraction; F is the magnitude.
- Harmonics: n is the harmonic number (f_n = n f_1); a pipe closed at one end has odd harmonics only.
- Interference and diffraction: the angle of the m-th bright fringe (double slit, grating) or dark fringe (single
  slit) from d sin(theta) = m lambda; positions on the screen are L tan(theta) (exact geometry, no small-angle
  approximation); the double-slit fringe spacing lambda L / d is the small-angle spacing near the centre. A grating
  order exactly at 90 degrees (grazing) is not seen; the highest order is the largest m with m lambda < d (to 1e-9).
- Static friction: a body holds while the needed friction does not exceed mu_s N (a force equal to the limit, to a
  relative 1e-12, holds it on the verge).
- Elasticity: linear (Hooke) behaviour; tension, extension and pressure increase are positive; a strain of magnitude
  1 or more is refused.
- Beams: point loads downward positive (N), positions along the beam (m) from any fixed origin; reactions upward
  positive. Lists carry no unit: loads in N, positions and coordinates in m, masses in kg, directions in degrees.
- Sound levels in dB relative to I0; the change between two levels is level 2 minus level 1, the ratio I2/I1.
- Results built from sin, cos, tan, asin, atan, log or powers are rounded to 12 significant digits (identical on every
  platform and Python version); values that are zero by geometry or by balance are written as exactly 0.

Question form (one per procedure): the capability phrase, a colon, then `name = value` pairs separated by commas
(`Coulomb's law: q1 = 2 uC, q2 = 3 uC, r = 10 cm`); physical values carry a unit. Procedures with lists separate the
pairs with semicolons and the list's numbers with commas (`Centre of mass: masses = 2, 3 kg; x = 0, 5 m`). Any other
number or an unknown name is refused, never guessed.
"""
from __future__ import annotations

import math
import re

KE = 8987551792.3          # Coulomb constant, N m^2/C^2
G_NEWTON = 6.6743e-11      # gravitational constant, N m^2/kg^2
G0 = 9.80665               # standard gravity, m/s^2
I0_HEAR = 1e-12            # threshold of hearing, W/m^2


# ================================================================================================ shared helpers
def _r(x):
    """12 significant digits, no negative zero (identical results on every platform and Python version)."""
    if x == 0 or not math.isfinite(x):
        return 0.0 if x == 0 else x
    return float(f"{x:.12g}") + 0.0


def _rel(a, b):
    """|a - b| relative to the larger magnitude; 0 when both are 0; infinite for a non-finite value."""
    if not (math.isfinite(a) and math.isfinite(b)):
        return math.inf
    d = abs(a - b)
    if d == 0:
        return 0.0
    return d / max(abs(a), abs(b))


def _index_ok(n, name):
    if not (math.isfinite(n) and n >= 1):
        raise ValueError(f"{name} must be a refractive index of at least 1")


def _positive(x, what):
    if not (math.isfinite(x) and x > 0):
        raise ValueError(f"{what} must be positive")


def _cos_sin_deg(d):
    """cos and sin of an angle in degrees, exact at multiples of 90 degrees."""
    a = d % 360.0
    if a == 0:
        return 1.0, 0.0
    if a == 90:
        return 0.0, 1.0
    if a == 180:
        return -1.0, 0.0
    if a == 270:
        return 0.0, -1.0
    t = math.radians(a)
    return math.cos(t), math.sin(t)


def _cos_snap(theta):
    c = math.cos(theta)
    if abs(c) < 1e-15:
        c = 0.0
    return c


# ================================================================================================ question reader
_NUM = r"[-+]?(?:\d+(?:\.\d*)?|\.\d+)(?:[eE][-+]?\d+)?"
_SEG = re.compile(r"^\s*(?:and\s+)?([A-Za-z][A-Za-z_0-9]*)\s*=\s*(" + _NUM + r")\s*(.*?)\s*$")
_WORD = re.compile(r"^\s*(?:and\s+)?([A-Za-z][A-Za-z_0-9]*)\s*=\s*([A-Za-z][A-Za-z \-]*?)\s*$")
_UNIT = re.compile(r"^[A-Za-zµμΩ°º%][A-Za-zµμΩ°º%/*^0-9().\-]*$")
_LIST = re.compile(r"^\s*\[?\s*(" + _NUM + r"(?:\s*,\s*" + _NUM + r"|\s+" + _NUM + r")*)\s*\]?\s*([A-Za-z°º]*)\s*$")


def _name(names, written):
    """The symbol a written name stands for: the exact spelling first (r and R differ), then lower case."""
    return names.get(written, names.get(written.lower()))


def _unit(u):
    return "deg" if u in ("°", "º", "degree", "degrees") else u


def _head(question, phrase):
    """The text after '<phrase>:' (the phrase opens the question), without a final full stop; None otherwise."""
    if not isinstance(question, str):
        return None
    m = re.match(r"\s*(?:" + phrase + r")\s*:\s*", question, re.I)
    if not m:
        return None
    return re.sub(r"[.?!]\s*$", "", question[m.end():].strip()).strip()


def _fields(rest, names, phys=(), ints=(), words=(), num_units=None):
    """{symbol: value} from 'name = value, ...'. `phys`: symbols that need a unit (returned as 'value unit', read by
    the runtime); `ints`: whole numbers (returned as written, read exactly); `words`: symbols whose value is a word;
    `num_units`: {symbol: allowed unit words} for plain numbers. None for anything it does not fully read."""
    if rest is None:
        return None
    out = {}
    if not rest:
        return out
    nu = num_units or {}
    for seg in re.split(r"[,;]", rest):
        if not seg.strip():
            return None
        ms = _SEG.match(seg)
        if ms:
            key = _name(names, ms.group(1))
            if key is None or key in out or key in words:
                return None
            num, unit = ms.group(2), ms.group(3).strip()
            if unit and not _UNIT.match(unit):
                return None
            if key in phys:
                if not unit or unit == "%":
                    return None
                out[key] = num + " " + _unit(unit)
            elif unit and unit not in nu.get(key, ()):
                return None
            elif key in ints:
                out[key] = num
            else:
                out[key] = float(num)
            continue
        mw = _WORD.match(seg)
        if mw:
            key = _name(names, mw.group(1))
            if key is None or key not in words or key in out:
                return None
            out[key] = mw.group(2).strip().lower()
            continue
        return None
    return out


def _pairs(question, phrase, names, phys=(), ints=(), words=(), num_units=None):
    return _fields(_head(question, phrase), names, phys, ints, words, num_units)


def _list_pairs(question, phrase, names, lists, list_units, phys=()):
    """{symbol: value} from '<phrase>: name = 1, 2, 3 [unit]; name = value unit; ...' (pairs separated by
    semicolons). A list may end with its one conventional unit (list_units); a physical value needs a unit."""
    rest = _head(question, phrase)
    if rest is None:
        return None
    out = {}
    if not rest:
        return out
    for seg in rest.split(";"):
        if not seg.strip():
            return None
        m = re.match(r"^\s*(?:and\s+)?([A-Za-z][A-Za-z_0-9]*)\s*=\s*(.+?)\s*$", seg)
        if not m:
            return None
        key = _name(names, m.group(1))
        if key is None or key in out:
            return None
        if key in lists:
            ml = _LIST.match(m.group(2))
            if not ml:
                return None
            unit = ml.group(2)
            if unit and unit not in list_units.get(key, ()):
                return None
            out[key] = ", ".join(re.split(r"\s*,\s*|\s+", ml.group(1).strip()))
        elif key in phys:
            mv = re.match(r"^(" + _NUM + r")\s*(\S+)$", m.group(2))
            if not mv or not _UNIT.match(mv.group(2)):
                return None
            out[key] = mv.group(1) + " " + _unit(mv.group(2))
        else:
            return None
    return out


def _lu(rng, a, b):
    return math.exp(rng.uniform(math.log(a), math.log(b)))


def _sign(rng):
    return rng.choice([1.0, -1.0])


# ================================================================================================ electrostatics
def coulomb_force(q1, q2, r) -> dict:
    if not (math.isfinite(r) and r > 0):
        raise ValueError("the separation r must be positive")
    Fr = KE * q1 * q2 / (r * r)
    return {"F": abs(Fr), "F_radial": Fr + 0.0}


def verify_coulomb(i, o) -> float:
    F, Fr = o["F"], o["F_radial"]
    if not (math.isfinite(F) and math.isfinite(Fr)):
        return math.inf
    s = abs(KE * i["q1"] * i["q2"])
    if s == 0:
        return abs(F) + abs(Fr)
    return abs(Fr * i["r"] * i["r"] - KE * i["q1"] * i["q2"]) / s + _rel(F, abs(Fr))


def ref_coulomb(q1, q2, r):
    """Shell theorem by quadrature (scipy quad): q1 spread uniformly over a spherical shell of radius r/2 about its
    position; the force on q2 is the sum of the axial components of the shell elements' Coulomb forces."""
    from scipy.integrate import quad
    R = r / 2

    def axial(t):
        s2 = r * r + R * R - 2 * r * R * math.cos(t)
        return (r - R * math.cos(t)) / s2 ** 1.5 * math.sin(t) / 2

    val, _ = quad(axial, 0.0, math.pi, epsabs=0, epsrel=1e-13, limit=200)
    Fr = KE * q1 * q2 * val
    return {"F_radial": Fr, "F": abs(Fr)}


# ================================================================================================ gravitation
def gravitational_force(m1, m2, r) -> dict:
    _positive(m1, "the mass m1")
    _positive(m2, "the mass m2")
    if not (math.isfinite(r) and r > 0):
        raise ValueError("the separation r must be positive")
    return {"F": G_NEWTON * m1 * m2 / (r * r)}


def verify_gravity(i, o) -> float:
    return _rel(o["F"] * i["r"] * i["r"], G_NEWTON * i["m1"] * i["m2"])


def ref_gravity(m1, m2, r):
    """Shell theorem by quadrature (scipy quad) with scipy's G: m1 spread over a spherical shell of radius r/2; the
    force on m2 is the sum of the axial components of the shell elements' attractions."""
    from scipy.constants import G as G_lib
    from scipy.integrate import quad
    R = r / 2

    def axial(t):
        s2 = r * r + R * R - 2 * r * R * math.cos(t)
        return (r - R * math.cos(t)) / s2 ** 1.5 * math.sin(t) / 2

    val, _ = quad(axial, 0.0, math.pi, epsabs=0, epsrel=1e-13, limit=200)
    return {"F": G_lib * m1 * m2 * val}


# ================================================================================================ refraction
def snell_refraction(n1, n2, theta1) -> dict:
    _index_ok(n1, "n1")
    _index_ok(n2, "n2")
    if not (0 <= theta1 <= math.pi / 2 + 1e-12):
        raise ValueError("the angle of incidence must lie between 0 and 90 degrees (measured from the normal)")
    s = n1 * math.sin(min(theta1, math.pi / 2)) / n2
    if s > 1:
        raise ValueError("total internal reflection: n1 sin(theta1) exceeds n2, so there is no refracted ray")
    t2 = math.asin(s)
    return {"theta2": _r(t2), "theta2_deg": _r(math.degrees(t2))}


def verify_snell(i, o) -> float:
    t2, d2 = o["theta2"], o["theta2_deg"]
    if not (math.isfinite(t2) and math.isfinite(d2)) or t2 < 0 or t2 > math.pi / 2 + 1e-12:
        return math.inf
    a = i["n1"] * math.sin(min(i["theta1"], math.pi / 2))
    b = i["n2"] * math.sin(t2)
    return abs(a - b) / max(abs(a), abs(b), 1e-300) + abs(d2 - math.degrees(t2)) / max(1.0, abs(d2))


def ref_snell(n1, n2, theta1):
    """Vector form of the law of refraction (numpy): t = eta i + (eta cos_i - sqrt(1 - eta^2 sin_i^2)) n; the
    refraction angle from atan2 of the transmitted direction's components."""
    import numpy as np
    eta = n1 / n2
    nrm = np.array([0.0, 1.0])
    inc = np.array([math.sin(theta1), -math.cos(theta1)])
    ci = -float(np.dot(nrm, inc))
    k = 1 - eta * eta * (1 - ci * ci)
    t = eta * inc + (eta * ci - math.sqrt(k)) * nrm
    th = math.atan2(abs(float(t[0])), abs(float(t[1])))
    return {"theta2": th, "theta2_deg": math.degrees(th)}


def critical_angle(n1, n2) -> dict:
    _index_ok(n1, "n1")
    _index_ok(n2, "n2")
    if not n1 > n2:
        raise ValueError("total internal reflection needs light going from a denser medium (n1) into a less dense one "
                         "(n2 < n1)")
    t = math.asin(n2 / n1)
    return {"theta_c": _r(t), "theta_c_deg": _r(math.degrees(t))}


def verify_critical(i, o) -> float:
    t, d = o["theta_c"], o["theta_c_deg"]
    if not (math.isfinite(t) and math.isfinite(d)) or t <= 0 or t > math.pi / 2:
        return math.inf
    return abs(i["n1"] * math.sin(t) - i["n2"]) / i["n2"] + abs(d - math.degrees(t)) / max(1.0, abs(d))


def ref_critical(n1, n2):
    """The incidence angle at which the vector-refraction discriminant 1 - (n1/n2)^2 sin^2(theta) reaches zero
    (scipy brentq): beyond it no transmitted ray exists."""
    from scipy.optimize import brentq
    eta = n1 / n2
    t = brentq(lambda th: 1 - (eta * math.sin(th)) ** 2, 0.0, math.pi / 2, xtol=1e-16, rtol=8.9e-16, maxiter=500)
    return {"theta_c": t, "theta_c_deg": math.degrees(t)}


# ================================================================================================ images
def lens_mirror_magnification(d_o, h_o, d_i=None, f=None) -> dict:
    if not (math.isfinite(d_o) and d_o > 0):
        raise ValueError("the object distance d_o must be positive (a real object)")
    if not (math.isfinite(h_o) and h_o > 0):
        raise ValueError("the object height h_o must be positive")
    if (d_i is None) == (f is None):
        raise ValueError("give exactly one of the image distance d_i or the focal length f")
    if d_i is None:
        if f == 0:
            raise ValueError("the focal length must not be zero")
        if d_o == f:
            raise ValueError("the object is at the focal point: the image is at infinity")
        d_i = d_o * f / (d_o - f)
    elif d_i == 0:
        raise ValueError("the image distance must not be zero")
    m = -d_i / d_o
    image = ("real" if d_i > 0 else "virtual") + (", inverted" if m < 0 else ", upright")
    return {"m": m, "h_i": m * h_o, "d_i": d_i, "image": image}


def verify_magnification(i, o) -> float:
    m, h_i, d_i = o["m"], o["h_i"], o["d_i"]
    if not (math.isfinite(m) and math.isfinite(h_i) and math.isfinite(d_i)) or d_i == 0:
        return math.inf
    d_o = i["d_o"]
    res = abs(m * d_o + d_i) / abs(d_i) + _rel(h_i, m * i["h_o"])
    if i.get("d_i") is not None:
        res += _rel(d_i, i["d_i"])
    if i.get("f") is not None:
        f = i["f"]
        res += abs(1 / d_o + 1 / d_i - 1 / f) / max(1 / d_o, 1 / abs(d_i), 1 / abs(f))
    want = ("real" if d_i > 0 else "virtual") + (", inverted" if m < 0 else ", upright")
    if o["image"] != want:
        return math.inf
    return res


def ref_magnification(d_o, h_o, d_i=None, f=None):
    """Ray-transfer (ABCD) matrices with numpy: free space d_o, a thin lens of focal length f, free space d_i. The
    image plane is where the B element vanishes; the magnification is the A element there."""
    import numpy as np

    def system(fl, di):
        p_o = np.array([[1.0, d_o], [0.0, 1.0]])
        lens = np.array([[1.0, 0.0], [-1.0 / fl, 1.0]])
        p_i = np.array([[1.0, di], [0.0, 1.0]])
        return p_i @ lens @ p_o

    fl = f if f is not None else 1.0 / (1.0 / d_o + 1.0 / d_i)
    if d_i is None:
        b0 = system(fl, 0.0)[0, 1]
        b1 = system(fl, 1.0)[0, 1] - b0
        di = -float(b0) / float(b1)
    else:
        di = d_i
    A = float(system(fl, di)[0, 0])
    return {"m": A, "h_i": A * h_o, "d_i": di}


# ================================================================================================ wave optics
def double_slit(d, lam, m, L) -> dict:
    _positive(d, "the slit separation d")
    _positive(lam, "the wavelength")
    _positive(L, "the screen distance L")
    if m < 0:
        raise ValueError("the order m is a whole number 0, 1, 2, ...")
    s = m * lam / d
    if s >= 1:
        raise ValueError(f"order {m} does not exist: m lambda must be smaller than the slit separation d")
    th = math.asin(s)
    return {"theta": _r(th), "theta_deg": _r(math.degrees(th)), "y": _r(L * math.tan(th)), "dy": lam * L / d}


def verify_double_slit(i, o) -> float:
    th, y, dy, dd = o["theta"], o["y"], o["dy"], o["theta_deg"]
    if not (math.isfinite(th) and math.isfinite(y) and math.isfinite(dy) and math.isfinite(dd)):
        return math.inf
    if th < 0 or th >= math.pi / 2:
        return math.inf
    d, lam, m, L = i["d"], i["lam"], i["m"], i["L"]
    res = abs(d * math.sin(th) - m * lam) / max(m * lam, lam)
    res += abs(y - L * math.tan(th)) / max(L, abs(y))
    res += _rel(dy * d, lam * L)
    return res + abs(dd - math.degrees(th)) / max(1.0, abs(dd))


def ref_double_slit(d, lam, m, L):
    """Re-solve: the phase difference of the two slit waves, 2 pi d sin(theta)/lambda, equals 2 pi m (scipy brentq)."""
    from scipy.optimize import brentq
    if m == 0:
        th = 0.0
    else:
        th = brentq(lambda t: 2 * math.pi * d * math.sin(t) / lam - 2 * math.pi * m, 0.0, math.pi / 2,
                    xtol=1e-18, rtol=8.9e-16, maxiter=500)
    return {"theta": th, "y": L * math.tan(th), "dy": lam * L / d}


def _wrong_double_slit(d, lam, m, L):
    """The dark-fringe condition (m + 1/2) lambda used for a bright fringe."""
    o = double_slit(d, lam, m, L)
    s = (m + 0.5) * lam / d
    if s < 1:
        th = math.asin(s)
        o = {"theta": th, "theta_deg": math.degrees(th), "y": L * math.tan(th), "dy": o["dy"]}
    return o


def diffraction_grating(lam, m, d=None, N=None) -> dict:
    if (d is None) == (N is None):
        raise ValueError("give exactly one of the line spacing d or the number of lines per millimetre N")
    if d is None:
        _positive(N, "the number of lines per millimetre N")
        d = 1e-3 / N
    _positive(d, "the line spacing d")
    _positive(lam, "the wavelength")
    if m < 1:
        raise ValueError("the order m is a whole number 1, 2, 3, ...")
    ratio = d / lam
    k = math.floor(ratio)
    if abs(ratio - round(ratio)) <= 1e-9 * ratio:
        k = round(ratio) - 1                          # an order exactly at 90 degrees grazes the grating: not seen
    if k < 1:
        raise ValueError("the line spacing is not larger than the wavelength: no order beyond the central maximum")
    if m > k:
        raise ValueError(f"order {m} is not seen: the highest order is {k}")
    th = math.asin(m * lam / d)
    return {"theta": _r(th), "theta_deg": _r(math.degrees(th)), "m_max": int(k), "d": d}


def verify_grating(i, o) -> float:
    th, d, k, dd = o["theta"], o["d"], o["m_max"], o["theta_deg"]
    if not (math.isfinite(th) and math.isfinite(d) and math.isfinite(dd)) or th <= 0 or th >= math.pi / 2:
        return math.inf
    d_in = i["d"] if i.get("d") is not None else 1e-3 / i["N"]
    lam, m = i["lam"], i["m"]
    res = _rel(d, d_in) + abs(d * math.sin(th) - m * lam) / (m * lam) + abs(dd - math.degrees(th)) / max(1.0, abs(dd))
    if k != int(k) or k < 1 or not (k * lam < d * (1 - 1e-9)) or not ((k + 1) * lam >= d * (1 - 1e-9)):
        return math.inf
    return res


def ref_grating(lam, m, d=None, N=None):
    """Re-solve of the grating condition d sin(theta) = m lambda by scipy brentq; the highest order by counting the
    orders whose sine stays below 1."""
    from scipy.optimize import brentq
    dd = d if d is not None else 1.0 / (N * 1e3)
    th = brentq(lambda t: dd * math.sin(t) - m * lam, 0.0, math.pi / 2, xtol=1e-18, rtol=8.9e-16, maxiter=500)
    k = 0
    while (k + 1) * lam < dd * (1 - 1e-9):
        k += 1
    return {"theta": th, "theta_deg": math.degrees(th), "m_max": k, "d": dd}


def _wrong_grating(lam, m, d=None, N=None):
    """tan(theta) = m lambda / d: the screen-position relation used for the grating condition."""
    o = diffraction_grating(lam, m, d, N)
    th = math.atan(m * lam / o["d"])
    return {"theta": th, "theta_deg": math.degrees(th), "m_max": o["m_max"], "d": o["d"]}


def single_slit(a, lam, m, L) -> dict:
    _positive(a, "the slit width a")
    _positive(lam, "the wavelength")
    _positive(L, "the screen distance L")
    if m < 1:
        raise ValueError("the order m of a dark fringe is a whole number 1, 2, 3, ...")
    if not lam < a:
        raise ValueError("the slit must be wider than the wavelength for a dark fringe to exist")
    if m * lam >= a:
        raise ValueError(f"dark fringe {m} does not exist: m lambda must be smaller than the slit width a")
    th = math.asin(m * lam / a)
    th1 = math.asin(lam / a)
    return {"theta": _r(th), "theta_deg": _r(math.degrees(th)), "y": _r(L * math.tan(th)),
            "W": _r(2 * L * math.tan(th1)), "W_angle": _r(2 * th1)}


def verify_single_slit(i, o) -> float:
    th, y, W, Wa, dd = o["theta"], o["y"], o["W"], o["W_angle"], o["theta_deg"]
    if not (math.isfinite(th) and math.isfinite(y) and math.isfinite(W) and math.isfinite(Wa) and math.isfinite(dd)):
        return math.inf
    if th <= 0 or th >= math.pi / 2 or Wa <= 0 or Wa >= math.pi:
        return math.inf
    a, lam, m, L = i["a"], i["lam"], i["m"], i["L"]
    res = abs(a * math.sin(th) - m * lam) / (m * lam) + abs(y - L * math.tan(th)) / max(L, abs(y))
    res += abs(a * math.sin(Wa / 2) - lam) / lam
    res += abs(W - 2 * L * math.tan(Wa / 2)) / max(L, abs(W))
    return res + abs(dd - math.degrees(th)) / max(1.0, abs(dd))


def ref_single_slit(a, lam, m, L):
    """Zeros of the single-slit amplitude sin(beta)/beta, beta = pi a sin(theta)/lambda, located by scipy brentq
    between the neighbouring half-orders: the m-th dark fringe, and the first for the central maximum's width."""
    from scipy.optimize import brentq

    def zero(k):
        lo = math.asin((k - 0.5) * lam / a)
        hi = math.asin(min(1.0, (k + 0.5) * lam / a))
        return brentq(lambda t: math.sin(math.pi * a * math.sin(t) / lam), lo, hi, xtol=1e-18, rtol=8.9e-16, maxiter=500)

    th, th1 = zero(m), zero(1)
    return {"theta": th, "y": L * math.tan(th), "W": 2 * L * math.tan(th1), "W_angle": 2 * th1}


def malus_law(I0, theta, light="polarized") -> dict:
    _positive(I0, "the incident intensity I0")
    if not math.isfinite(theta):
        raise ValueError("the angle must be a finite number")
    c = _cos_snap(theta)
    k = 1.0 if light == "polarized" else 0.5
    I = k * I0 * c * c
    return {"I": _r(I), "fraction": _r(I / I0)}


def verify_malus(i, o) -> float:
    I, fr = o["I"], o["fraction"]
    if not (math.isfinite(I) and math.isfinite(fr)):
        return math.inf
    c = _cos_snap(i["theta"])
    light = i.get("light")
    k = 0.5 if light == "unpolarized" else 1.0
    return abs(I - k * i["I0"] * c * c) / i["I0"] + abs(fr - I / i["I0"])


def ref_malus(I0, theta, light="polarized"):
    """Mueller calculus (numpy): Stokes vectors through linear-polarizer Mueller matrices (unpolarized light first
    through a polarizer at 0, then the analyser at theta)."""
    import numpy as np

    def pol(a):
        c, s = math.cos(2 * a), math.sin(2 * a)
        return 0.5 * np.array([[1.0, c, s, 0.0], [c, c * c, c * s, 0.0], [s, c * s, s * s, 0.0], [0.0, 0.0, 0.0, 0.0]])

    if light == "polarized":
        S = np.array([I0, I0, 0.0, 0.0])
    else:
        S = pol(0.0) @ np.array([I0, 0.0, 0.0, 0.0])
    out = float((pol(theta) @ S)[0])
    return {"I": out, "fraction": out / I0}


# ================================================================================================ induction and AC
def faraday_emf(N, A1, B1, B2, dt, A2=None, theta=0.0) -> dict:
    if N < 1:
        raise ValueError("the number of turns N must be at least 1")
    _positive(dt, "the time interval dt")
    if A2 is None:
        A2 = A1
    if not (A1 >= 0 and A2 >= 0):
        raise ValueError("loop areas cannot be negative")
    c = _cos_snap(theta)
    phi1 = B1 * A1 * c
    phi2 = B2 * A2 * c
    dphi = phi2 - phi1
    emf_s = -N * dphi / dt
    return {"phi1": _r(phi1), "phi2": _r(phi2), "dphi": _r(dphi), "emf": _r(abs(emf_s)), "emf_signed": _r(emf_s)}


def verify_faraday(i, o) -> float:
    for k in ("phi1", "phi2", "dphi", "emf", "emf_signed"):
        if not math.isfinite(o[k]):
            return math.inf
    A1 = i["A1"]
    A2 = i["A2"] if i.get("A2") is not None else A1
    th = i.get("theta")
    if th is None:
        th = 0.0
    c = _cos_snap(th)
    p1, p2 = i["B1"] * A1 * c, i["B2"] * A2 * c
    s = max(abs(p1), abs(p2), 1e-300)
    res = abs(o["phi1"] - p1) / s + abs(o["phi2"] - p2) / s + abs(o["dphi"] - (o["phi2"] - o["phi1"])) / s
    N, dt = i["N"], i["dt"]
    e = N * abs(o["dphi"]) / dt
    scale = max(e, N * s * 1e-12 / dt, 1e-300)
    return res + abs(o["emf"] - e) / scale + abs(o["emf_signed"] + N * o["dphi"] / dt) / scale


def ref_faraday(N, A1, B1, B2, dt, A2=None, theta=0.0):
    """The instantaneous emf -N dPhi/dt of a field and an area changing linearly in time, integrated over the
    interval with scipy quad and divided by it (the average emf)."""
    from scipy.integrate import quad
    A2v = A1 if A2 is None else A2
    c = math.cos(theta)

    def rate(t):
        B = B1 + (B2 - B1) * t / dt
        A = A1 + (A2v - A1) * t / dt
        return ((B2 - B1) / dt * A + B * (A2v - A1) / dt) * c

    total, _ = quad(rate, 0.0, dt, epsabs=0, epsrel=1e-13)
    return {"dphi": total, "emf": abs(N * total / dt), "emf_signed": -N * total / dt}


def _wrong_faraday(N, A1, B1, B2, dt, A2=None, theta=0.0):
    """The number of turns left out."""
    o = faraday_emf(N, A1, B1, B2, dt, A2, theta)
    return {"phi1": o["phi1"], "phi2": o["phi2"], "dphi": o["dphi"], "emf": o["emf"] / N, "emf_signed": o["emf_signed"] / N}


def ideal_transformer(Np, Ns, Vp, Ip=None) -> dict:
    if Np < 1 or Ns < 1:
        raise ValueError("both windings need at least one turn")
    _positive(Vp, "the primary voltage Vp")
    if Ip is not None and not (math.isfinite(Ip) and Ip >= 0):
        raise ValueError("the primary current Ip cannot be negative")
    kind = "step-up" if Ns > Np else ("step-down" if Ns < Np else "isolation (1:1)")
    out = {"Vs": Vp * Ns / Np, "ratio": Ns / Np, "type": kind}
    if Ip is not None:
        out["Is"] = Ip * Np / Ns
    return out


def verify_transformer(i, o) -> float:
    Np, Ns, Vp = i["Np"], i["Ns"], i["Vp"]
    res = _rel(o["Vs"] * Np, Vp * Ns) + _rel(o["ratio"], Ns / Np)
    want = "step-up" if Ns > Np else ("step-down" if Ns < Np else "isolation (1:1)")
    if o["type"] != want:
        return math.inf
    if i.get("Ip") is not None:
        if "Is" not in o:
            return math.inf
        res += _rel(o["Is"] * Ns, i["Ip"] * Np)
    return res


def ref_transformer(Np, Ns, Vp, Ip=None):
    """Equal volts per turn solved as a linear system (numpy) for Vs; Is from the power balance Vp Ip = Vs Is of a
    loss-free transformer (not from the ampere-turn balance the procedure uses)."""
    import numpy as np
    Vs = float(np.linalg.solve(np.array([[1.0 / Ns]]), np.array([Vp / Np]))[0])
    out = {"Vs": Vs}
    if Ip is not None:
        out["Is"] = Vp * Ip / Vs
    return out


def series_rlc_ac(R, L, C, f, V) -> dict:
    _positive(R, "the resistance R")
    _positive(L, "the inductance L")
    _positive(C, "the capacitance C")
    _positive(f, "the frequency f")
    _positive(V, "the rms voltage V")
    w = 2 * math.pi * f
    XL = w * L
    XC = 1.0 / (w * C)
    X = XL - XC
    if abs(X) <= 1e-12 * max(XL, XC):                 # at resonance the reactance is exactly 0, not rounding residue
        X = 0.0
    Z = math.hypot(R, X)
    I = V / Z
    phi = math.atan2(X, R)
    return {"Z": _r(Z), "I": _r(I), "phi": _r(phi), "phi_deg": _r(math.degrees(phi)), "pf": _r(R / Z),
            "P": _r(I * I * R), "XL": XL, "XC": XC}


def verify_rlc(i, o) -> float:
    for k in ("Z", "I", "phi", "phi_deg", "pf", "P", "XL", "XC"):
        if not math.isfinite(o[k]):
            return math.inf
    R, L, C, f, V = i["R"], i["L"], i["C"], i["f"], i["V"]
    w = 2 * math.pi * f
    XL, XC = w * L, 1.0 / (w * C)
    X = XL - XC
    if abs(X) <= 1e-12 * max(XL, XC):
        X = 0.0
    Z = o["Z"]
    if Z <= 0:
        return math.inf
    res = _rel(o["XL"], XL) + _rel(o["XC"], XC) + abs(Z * Z - (R * R + X * X)) / (R * R + X * X)
    res += _rel(o["I"] * Z, V)
    res += abs(Z * math.cos(o["phi"]) - R) / Z + abs(Z * math.sin(o["phi"]) - X) / Z
    res += abs(o["pf"] - R / Z) + _rel(o["P"], V * o["I"] * o["pf"])
    return res + abs(o["phi_deg"] - math.degrees(o["phi"])) / 180


def ref_rlc(R, L, C, f, V):
    """Complex impedance from scipy.signal.freqs (Z(s) = (LCs^2 + RCs + 1)/(Cs) on the j-omega axis); the current
    phasor I = V/Z and the complex power S = V conj(I) give the power factor and the average power."""
    import numpy as np
    from scipy import signal
    w = 2 * np.pi * f
    _, h = signal.freqs([L * C, R * C, 1.0], [C, 0.0], worN=[w])
    Z = complex(h[0])
    I = V / Z
    S = V * I.conjugate()
    return {"Z": abs(Z), "I": abs(I), "phi": float(np.angle(Z)), "pf": S.real / abs(S), "P": S.real}


def _wrong_rlc(R, L, C, f, V):
    """The frequency used as if it were the angular frequency."""
    XL, XC = f * L, 1.0 / (f * C)
    X = XL - XC
    Z = math.hypot(R, X)
    I = V / Z
    phi = math.atan2(X, R)
    return {"Z": Z, "I": I, "phi": phi, "phi_deg": math.degrees(phi), "pf": R / Z, "P": I * I * R, "XL": XL, "XC": XC}


# ================================================================================================ DC sources
def terminal_voltage(emf, r, R) -> dict:
    _positive(emf, "the emf")
    if not (math.isfinite(r) and r >= 0 and math.isfinite(R) and R >= 0):
        raise ValueError("resistances cannot be negative")
    if R + r == 0:
        raise ValueError("a short circuit with no internal resistance would draw an unlimited current")
    I = emf / (R + r)
    return {"I": I, "V_t": I * R, "V_lost": I * r, "P_load": I * I * R, "efficiency": R / (R + r)}


def verify_terminal(i, o) -> float:
    for k in ("I", "V_t", "V_lost", "P_load", "efficiency"):
        if not math.isfinite(o[k]):
            return math.inf
    emf, r, R = i["emf"], i["r"], i["R"]
    I = o["I"]
    if I <= 0:
        return math.inf
    res = _rel(I * (R + r), emf)
    res += abs(o["V_t"] - I * R) / emf + abs(o["V_lost"] - I * r) / emf + abs(o["V_t"] + o["V_lost"] - emf) / emf
    res += abs(o["P_load"] - o["V_t"] * I) / (emf * I)
    return res + abs(o["efficiency"] - o["V_t"] / emf)


def ref_terminal(emf, r, R):
    """Nodal analysis (numpy): the terminal node voltage V solves (emf - V)/r = V/R; the current, lost volts, load
    power and efficiency follow from V."""
    import numpy as np
    if r == 0:
        V = emf
    else:
        g = 1.0 / r + (1.0 / R if R > 0 else 0.0)
        V = float(np.linalg.solve(np.array([[g]]), np.array([emf / r]))[0]) if R > 0 else 0.0
    I = (emf - V) / r if r > 0 else V / R
    return {"I": I, "V_t": V, "V_lost": emf - V, "P_load": V * I, "efficiency": V / emf}


def _wrong_terminal(emf, r, R):
    """The internal resistance ignored: I = emf / R."""
    if R == 0:
        return terminal_voltage(emf, r, R)
    I = emf / R
    return {"I": I, "V_t": I * R, "V_lost": I * r, "P_load": I * I * R, "efficiency": 1.0}


# ================================================================================================ friction
def _slips_level(mu_s, N, F):
    return F > mu_s * N * (1 + 1e-12)


def static_friction_limit(mu_s, N, F) -> dict:
    if not (math.isfinite(mu_s) and mu_s >= 0):
        raise ValueError("the coefficient of static friction cannot be negative")
    _positive(N, "the normal force N")
    if not (math.isfinite(F) and F >= 0):
        raise ValueError("the applied force F is a magnitude (zero or positive)")
    fm = mu_s * N
    return {"f_max": fm, "slips": _slips_level(mu_s, N, F), "margin": fm - F}


def verify_static_limit(i, o) -> float:
    mu, N, F = i["mu_s"], i["N"], i["F"]
    fm = o["f_max"]
    if not (math.isfinite(fm) and math.isfinite(o["margin"])):
        return math.inf
    if o["slips"] != _slips_level(mu, N, F):
        return math.inf
    return _rel(fm, mu * N) + abs(o["margin"] - (fm - F)) / max(abs(fm), F, 1e-300)


def ref_static_limit(mu_s, N, F):
    """Friction cone: the body holds while the angle of the needed contact force from the normal, atan(F/N), stays
    within the angle of friction atan(mu_s); the limiting force is N tan(atan(mu_s))."""
    phi = math.atan(mu_s)
    return {"f_max": N * math.tan(phi), "slips": math.atan2(F, N) > phi * (1 + 1e-12)}


def _wrong_static_limit(mu_s, N, F):
    """The normal force multiplied by g once more (taken as a mass)."""
    fm = mu_s * N * G0
    return {"f_max": fm, "slips": F > fm, "margin": fm - F}


def _slips_incline(mu_s, theta):
    return math.sin(theta) > mu_s * math.cos(theta) * (1 + 1e-12)


def static_friction_incline(mu_s, theta, m=None) -> dict:
    if not (math.isfinite(mu_s) and mu_s >= 0):
        raise ValueError("the coefficient of static friction cannot be negative")
    if not (0 <= theta < math.pi / 2):
        raise ValueError("the incline angle must lie between 0 and 90 degrees (90 excluded)")
    tc = math.atan(mu_s)
    out = {"theta_c": _r(tc), "theta_c_deg": _r(math.degrees(tc)), "slips": _slips_incline(mu_s, theta)}
    if m is not None:
        _positive(m, "the mass m")
        Nf = m * G0 * math.cos(theta)
        out["N"] = _r(Nf)
        out["f_needed"] = _r(m * G0 * math.sin(theta))
        out["f_max"] = _r(mu_s * Nf)
    return out


def verify_incline(i, o) -> float:
    mu, th = i["mu_s"], i["theta"]
    tc, dd = o["theta_c"], o["theta_c_deg"]
    if not (math.isfinite(tc) and math.isfinite(dd)) or tc < 0 or tc >= math.pi / 2:
        return math.inf
    if o["slips"] != _slips_incline(mu, th):
        return math.inf
    res = abs(math.tan(tc) - mu) / max(mu, 1e-300) + abs(dd - math.degrees(tc)) / max(1.0, abs(dd))
    m = i.get("m")
    if m is not None:
        if "N" not in o or "f_needed" not in o or "f_max" not in o:
            return math.inf
        Nf = m * G0 * math.cos(th)
        res += _rel(o["N"], Nf) + _rel(o["f_needed"], m * G0 * math.sin(th)) + _rel(o["f_max"], mu * Nf)
    return res


def ref_incline(mu_s, theta, m=None):
    """The critical angle where the weight's component along the slope equals the limiting friction,
    sin(t) = mu_s cos(t), found by scipy brentq; the forces from resolving the weight."""
    from scipy.optimize import brentq
    tc = 0.0 if mu_s == 0 else brentq(lambda t: math.sin(t) - mu_s * math.cos(t), 0.0, math.pi / 2,
                                      xtol=1e-18, rtol=8.9e-16, maxiter=500)
    out = {"theta_c": tc, "theta_c_deg": math.degrees(tc), "slips": math.tan(theta) > mu_s * (1 + 1e-12)}
    if m is not None:
        out["N"] = m * G0 * math.cos(theta)
        out["f_needed"] = m * G0 * math.sin(theta)
        out["f_max"] = mu_s * m * G0 * math.cos(theta)
    return out


def _wrong_incline(mu_s, theta, m=None):
    """The critical angle from tan(theta_c) = 1/mu_s (sine and cosine components swapped)."""
    o = static_friction_incline(mu_s, theta, m)
    if mu_s == 0:
        return o
    tc = math.atan(1 / mu_s)
    o["theta_c"], o["theta_c_deg"], o["slips"] = tc, math.degrees(tc), theta > tc
    return o


# ================================================================================================ elasticity
def young_deformation(F, A, L0, E) -> dict:
    _positive(A, "the cross-sectional area A")
    _positive(L0, "the original length L0")
    _positive(E, "Young's modulus E")
    stress = F / A
    strain = stress / E
    if not abs(strain) < 1:
        raise ValueError("a change of length as large as the original length is outside linear elasticity")
    return {"stress": stress, "strain": strain, "dL": strain * L0}


def verify_young(i, o) -> float:
    st, sn, dl = o["stress"], o["strain"], o["dL"]
    if not (math.isfinite(st) and math.isfinite(sn) and math.isfinite(dl)):
        return math.inf
    if i["F"] == 0:
        return abs(st) + abs(sn) + abs(dl)
    return _rel(st * i["A"], i["F"]) + _rel(sn * i["E"], st) + _rel(dl, sn * i["L0"])


def _ref_bar(force, length, modulus, area):
    """End displacement of a uniform bar fixed at one end: ten two-node elements of stiffness modulus * area / h
    assembled into a stiffness matrix and solved with numpy; the strain of the first element."""
    import numpy as np
    n = 10
    h = length / n
    k = modulus * area / h
    K = np.zeros((n, n))
    for e in range(n):
        if e == 0:
            K[0, 0] += k
        else:
            a, b = e - 1, e
            K[a, a] += k
            K[b, b] += k
            K[a, b] -= k
            K[b, a] -= k
    rhs = np.zeros(n)
    rhs[-1] = force
    u = np.linalg.solve(K, rhs)
    return float(u[-1]), float(u[0]) / h


def ref_young(F, A, L0, E):
    dL, strain = _ref_bar(F, L0, E, A)
    return {"dL": dL, "strain": strain, "stress": E * strain}


def shear_deformation(F, A, h, S) -> dict:
    _positive(A, "the area A of the sheared face")
    _positive(h, "the height h between the sheared faces")
    _positive(S, "the shear modulus S")
    stress = F / A
    strain = stress / S
    if not abs(strain) < 1:
        raise ValueError("a shear displacement as large as the height is outside linear elasticity")
    return {"stress": stress, "strain": strain, "dx": strain * h}


def verify_shear(i, o) -> float:
    st, sn, dx = o["stress"], o["strain"], o["dx"]
    if not (math.isfinite(st) and math.isfinite(sn) and math.isfinite(dx)):
        return math.inf
    if i["F"] == 0:
        return abs(st) + abs(sn) + abs(dx)
    return _rel(st * i["A"], i["F"]) + _rel(sn * i["S"], st) + _rel(dx, sn * i["h"])


def ref_shear(F, A, h, S):
    dx, strain = _ref_bar(F, h, S, A)
    return {"dx": dx, "strain": strain, "stress": S * strain}


def bulk_compression(dP, V0, B) -> dict:
    _positive(V0, "the original volume V0")
    _positive(B, "the bulk modulus B")
    if not math.isfinite(dP):
        raise ValueError("the pressure change must be a finite number")
    strain = -dP / B
    if not abs(strain) < 1:
        raise ValueError("a volume change as large as the volume itself is outside linear elasticity")
    dV = strain * V0
    return {"dV": dV, "strain": strain, "V": V0 + dV}


def verify_bulk(i, o) -> float:
    dV, sn, V = o["dV"], o["strain"], o["V"]
    if not (math.isfinite(dV) and math.isfinite(sn) and math.isfinite(V)):
        return math.inf
    return _rel(sn * i["B"], -i["dP"]) + _rel(dV, sn * i["V0"]) + _rel(V, i["V0"] + dV)


def ref_bulk(dP, V0, B):
    """Re-solve of the definition B = -dP / (dV / V0) for dV by scipy brentq."""
    from scipy.optimize import brentq
    span = 2 * abs(V0 * dP / B) + 1e-300
    dV = brentq(lambda x: -B * x / V0 - dP, -span, span, xtol=1e-300, rtol=8.9e-16, maxiter=500)
    return {"dV": dV, "strain": dV / V0, "V": V0 + dV}


# ================================================================================================ statics and systems
def beam_reactions(loads, positions, xA, xB) -> dict:
    n = len(loads)
    if n < 1 or len(positions) != n:
        raise ValueError("give one position for every load")
    if xA == xB:
        raise ValueError("the two supports must be at different points")
    total, moment, scale = 0.0, 0.0, 0.0
    for k in range(n):
        total += loads[k]
        moment += loads[k] * (positions[k] - xA)
        scale += abs(loads[k])
    RB = moment / (xB - xA)
    RA = total - RB
    if abs(RA) <= 1e-12 * scale:
        RA = 0.0
    if abs(RB) <= 1e-12 * scale:
        RB = 0.0
    return {"RA": RA + 0.0, "RB": RB + 0.0, "total": total}


def verify_beam(i, o) -> float:
    loads, xs, xA, xB = i["loads"], i["positions"], i["xA"], i["xB"]
    RA, RB = o["RA"], o["RB"]
    if not (math.isfinite(RA) and math.isfinite(RB) and math.isfinite(o["total"])):
        return math.inf
    s, tot, mom, span = 0.0, 0.0, 0.0, max(abs(xA), abs(xB))
    for k in range(len(loads)):
        s += abs(loads[k])
        tot += loads[k]
        mom += loads[k] * xs[k]
        span = max(span, abs(xs[k]))
    if s == 0:
        return abs(RA) + abs(RB) + abs(o["total"])
    res = abs(RA + RB - tot) / s + abs(RA * xA + RB * xB - mom) / (s * max(span, 1e-300))
    return res + abs(o["total"] - tot) / s


def ref_beam(loads, positions, xA, xB):
    """The two equilibrium conditions (forces, moments about the origin) solved as a linear system with numpy."""
    import numpy as np
    F = np.array(loads, dtype=float)
    x = np.array(positions, dtype=float)
    RA, RB = np.linalg.solve(np.array([[1.0, 1.0], [xA, xB]]), np.array([F.sum(), float(F @ x)]))
    return {"RA": float(RA), "RB": float(RB), "total": float(F.sum())}


def centre_of_mass(masses, x, y=None) -> dict:
    n = len(masses)
    if n < 1 or len(x) != n or (y is not None and len(y) != n):
        raise ValueError("give one coordinate for every mass")
    M, sx, sy = 0.0, 0.0, 0.0
    for k in range(n):
        if not (math.isfinite(masses[k]) and masses[k] > 0):
            raise ValueError("every mass must be positive")
        M += masses[k]
        sx += masses[k] * x[k]
        if y is not None:
            sy += masses[k] * y[k]
    out = {"M": M, "x_cm": sx / M + 0.0}
    if y is not None:
        out["y_cm"] = sy / M + 0.0
    return out


def verify_com(i, o) -> float:
    ms, xs, ys = i["masses"], i["x"], i.get("y")
    if not (math.isfinite(o["M"]) and math.isfinite(o["x_cm"])):
        return math.inf
    M, tx, sx = 0.0, 0.0, 0.0
    for k in range(len(ms)):
        M += ms[k]
        tx += ms[k] * (xs[k] - o["x_cm"])
        sx += ms[k] * abs(xs[k])
    res = _rel(o["M"], M) + abs(tx) / max(sx, M * abs(o["x_cm"]), 1e-300)
    if ys is not None:
        if "y_cm" not in o or not math.isfinite(o["y_cm"]):
            return math.inf
        ty, sy = 0.0, 0.0
        for k in range(len(ms)):
            ty += ms[k] * (ys[k] - o["y_cm"])
            sy += ms[k] * abs(ys[k])
        res += abs(ty) / max(sy, M * abs(o["y_cm"]), 1e-300)
    return res


def ref_com(masses, x, y=None):
    """The lever rule applied body by body: each new mass moves the running centre towards itself in proportion to
    its share of the combined mass."""
    M, cx, cy = masses[0], x[0], (y[0] if y is not None else 0.0)
    for k in range(1, len(masses)):
        m = masses[k]
        cx += (x[k] - cx) * m / (M + m)
        if y is not None:
            cy += (y[k] - cy) * m / (M + m)
        M += m
    out = {"M": M, "x_cm": cx}
    if y is not None:
        out["y_cm"] = cy
    return out


def vector_resultant(magnitudes, directions) -> dict:
    n = len(magnitudes)
    if n < 1 or len(directions) != n:
        raise ValueError("give one direction for every magnitude")
    xs, ys = [], []
    Rx, Ry, scale = 0.0, 0.0, 0.0
    for k in range(n):
        mag = magnitudes[k]
        if not (math.isfinite(mag) and mag >= 0):
            raise ValueError("magnitudes cannot be negative (reverse the direction instead)")
        c, s = _cos_sin_deg(directions[k])
        xs.append(_r(mag * c))
        ys.append(_r(mag * s))
        Rx += mag * c
        Ry += mag * s
        scale += mag
    if abs(Rx) <= 1e-12 * scale:
        Rx = 0.0
    if abs(Ry) <= 1e-12 * scale:
        Ry = 0.0
    R = math.hypot(Rx, Ry)
    direction = 0.0
    if R > 0:
        direction = math.degrees(math.atan2(Ry, Rx)) % 360.0
    return {"x_components": xs, "y_components": ys, "Rx": _r(Rx), "Ry": _r(Ry), "R": _r(R), "direction_deg": _r(direction)}


def verify_vectors(i, o) -> float:
    mags, dirs = i["magnitudes"], i["directions"]
    xs, ys = o["x_components"], o["y_components"]
    if len(xs) != len(mags) or len(ys) != len(mags):
        return math.inf
    Rx, Ry, R, d = o["Rx"], o["Ry"], o["R"], o["direction_deg"]
    if not (math.isfinite(Rx) and math.isfinite(Ry) and math.isfinite(R) and math.isfinite(d)):
        return math.inf
    scale = 0.0
    for k in range(len(mags)):
        scale += mags[k]
    if scale == 0:
        scale = 1.0
    res, sx, sy = 0.0, 0.0, 0.0
    for k in range(len(mags)):
        c, s = _cos_sin_deg(dirs[k])
        res += abs(xs[k] - mags[k] * c) / scale + abs(ys[k] - mags[k] * s) / scale
        sx += mags[k] * c
        sy += mags[k] * s
    res += abs(Rx - sx) / scale + abs(Ry - sy) / scale + abs(R - math.hypot(Rx, Ry)) / scale
    if R > 0:
        if d < 0 or d >= 360:
            return math.inf
        t = math.radians(d)
        res += abs(R * math.cos(t) - Rx) / scale + abs(R * math.sin(t) - Ry) / scale
    else:
        res += abs(d)
    return res


def ref_vectors(magnitudes, directions):
    """Complex numbers with numpy: the sum of magnitude * exp(i angle); components, modulus and argument."""
    import numpy as np
    z = np.array(magnitudes, dtype=float) * np.exp(1j * np.deg2rad(np.array(directions, dtype=float)))
    tot = complex(z.sum())
    return {"x_components": [float(v) for v in z.real], "y_components": [float(v) for v in z.imag],
            "Rx": tot.real, "Ry": tot.imag, "R": float(np.abs(tot)), "direction_deg": float(np.degrees(np.angle(tot))) % 360.0}


def _wrong_vectors(magnitudes, directions):
    """Angles measured from the y axis: sine and cosine swapped."""
    o = vector_resultant(magnitudes, [90.0 - d for d in directions])
    return o


# ================================================================================================ rotation
def rotational_second_law(tau, I) -> dict:
    if not (math.isfinite(I) and I > 0):
        raise ValueError("the moment of inertia must be positive")
    return {"alpha": tau / I + 0.0}


def verify_rotation(i, o) -> float:
    return _rel(o["alpha"] * i["I"], i["tau"])


def ref_rotation(tau, I):
    """Angular momentum form of the law, dL/dt = tau, integrated from rest over one second (scipy DOP853); the
    angular acceleration is the angular velocity L/I reached after that second."""
    from scipy.integrate import solve_ivp
    sol = solve_ivp(lambda t, y: [tau], (0.0, 1.0), [0.0], method="DOP853", rtol=1e-13, atol=1e-30)
    return {"alpha": float(sol.y[0, -1]) / I}


def angular_momentum_conservation(I1, omega1, I2) -> dict:
    _positive(I1, "the initial moment of inertia I1")
    _positive(I2, "the final moment of inertia I2")
    w2 = I1 * omega1 / I2
    K1 = I1 * omega1 * omega1 / 2
    K2 = I2 * w2 * w2 / 2
    return {"omega2": w2 + 0.0, "L": I1 * omega1 + 0.0, "K1": K1, "K2": K2, "dK": K2 - K1}


def verify_spin(i, o) -> float:
    for k in ("omega2", "L", "K1", "K2", "dK"):
        if not math.isfinite(o[k]):
            return math.inf
    I1, w1, I2 = i["I1"], i["omega1"], i["I2"]
    w2 = o["omega2"]
    res = _rel(I2 * w2, I1 * w1) + _rel(o["L"], I1 * w1)
    res += _rel(o["K1"], I1 * w1 * w1 / 2) + _rel(o["K2"], I2 * w2 * w2 / 2)
    return res + abs(o["dK"] - (o["K2"] - o["K1"])) / max(o["K1"], o["K2"], 1e-300)


def ref_spin(I1, omega1, I2):
    """The torque-free body changing its moment of inertia linearly in time: d(I omega)/dt = 0 written as
    d omega/dt = -omega I'(t)/I(t), integrated with scipy DOP853 from I1 to I2."""
    from scipy.integrate import solve_ivp
    sol = solve_ivp(lambda t, w: [-w[0] * (I2 - I1) / (I1 + (I2 - I1) * t)], (0.0, 1.0), [omega1], method="DOP853",
                    rtol=1e-13, atol=1e-15 * abs(omega1) + 1e-300)
    w2 = float(sol.y[0, -1])
    return {"omega2": w2, "L": I2 * w2, "K2": I2 * w2 * w2 / 2}


def _wrong_spin(I1, omega1, I2):
    """The ratio of moments of inertia inverted."""
    w2 = omega1 * I2 / I1
    K1 = I1 * omega1 * omega1 / 2
    K2 = I2 * w2 * w2 / 2
    return {"omega2": w2, "L": I1 * omega1, "K1": K1, "K2": K2, "dK": K2 - K1}


# ================================================================================================ sound and waves
def sound_intensity_level(I=None, beta=None) -> dict:
    if (I is None) == (beta is None):
        raise ValueError("give exactly one of the intensity I or the level beta")
    if I is not None:
        _positive(I, "the intensity I")
        return {"beta": _r(10 * math.log10(I / I0_HEAR)), "I": I}
    if not math.isfinite(beta):
        raise ValueError("the level must be a finite number of decibels")
    return {"beta": beta, "I": _r(I0_HEAR * 10 ** (beta / 10))}


def verify_level(i, o) -> float:
    b, I = o["beta"], o["I"]
    if not (math.isfinite(b) and math.isfinite(I)) or I <= 0:
        return math.inf
    res = abs(b - 10 * math.log10(I / I0_HEAR)) / max(1.0, abs(b))
    if i.get("I") is not None:
        res += _rel(I, i["I"])
    if i.get("beta") is not None:
        res += abs(b - i["beta"]) / max(1.0, abs(b))
    return res


def ref_level(I=None, beta=None):
    """Natural logarithms and exponentials instead of base-10 logarithms and powers."""
    if I is not None:
        return {"beta": 10 * (math.log(I) - math.log(I0_HEAR)) / math.log(10), "I": I}
    return {"beta": beta, "I": I0_HEAR * math.exp(beta * math.log(10) / 10)}


def _wrong_level(I=None, beta=None):
    """The amplitude (20 log10) form used for an intensity."""
    if I is not None:
        return {"beta": 20 * math.log10(I / I0_HEAR), "I": I}
    return {"beta": beta, "I": I0_HEAR * 10 ** (beta / 20)}


def sound_level_change(beta1=None, beta2=None, ratio=None) -> dict:
    if ratio is not None:
        if beta2 is not None:
            raise ValueError("give the intensity ratio or the second level, not both")
        if not (math.isfinite(ratio) and ratio > 0):
            raise ValueError("the intensity ratio I2/I1 must be positive")
        d = 10 * math.log10(ratio)
        out = {"d_beta": _r(d), "ratio": ratio}
        if beta1 is not None:
            out["beta2"] = _r(beta1 + d)
        return out
    if beta1 is None or beta2 is None:
        raise ValueError("give both levels beta1 and beta2, or the intensity ratio")
    d = beta2 - beta1
    return {"d_beta": _r(d), "ratio": _r(10 ** (d / 10)), "beta2": beta2}


def verify_level_change(i, o) -> float:
    d, ratio = o["d_beta"], o["ratio"]
    if not (math.isfinite(d) and math.isfinite(ratio)) or ratio <= 0:
        return math.inf
    res = abs(10 * math.log10(ratio) - d) / max(1.0, abs(d))
    if i.get("ratio") is not None:
        res += _rel(ratio, i["ratio"])
    if i.get("beta1") is not None:
        if "beta2" not in o or not math.isfinite(o["beta2"]):
            return math.inf
        res += abs(o["beta2"] - i["beta1"] - d) / max(1.0, abs(o["beta2"]))
    if i.get("beta2") is not None:
        res += abs(o["beta2"] - i["beta2"]) / max(1.0, abs(i["beta2"]))
    return res


def ref_level_change(beta1=None, beta2=None, ratio=None):
    """Through absolute intensities: each level converted to W/m^2 (natural exponential), the ratio of the two, and
    the level difference back through natural logarithms."""
    if ratio is not None:
        out = {"d_beta": 10 * math.log(ratio) / math.log(10), "ratio": ratio}
        if beta1 is not None:
            I2 = I0_HEAR * math.exp(beta1 * math.log(10) / 10) * ratio
            out["beta2"] = 10 * (math.log(I2) - math.log(I0_HEAR)) / math.log(10)
        return out
    I1 = I0_HEAR * math.exp(beta1 * math.log(10) / 10)
    I2 = I0_HEAR * math.exp(beta2 * math.log(10) / 10)
    return {"d_beta": 10 * math.log(I2 / I1) / math.log(10), "ratio": I2 / I1, "beta2": beta2}


def _wrong_level_change(beta1=None, beta2=None, ratio=None):
    """The amplitude (20 log10) form used for an intensity ratio."""
    if ratio is not None:
        d = 20 * math.log10(ratio)
        out = {"d_beta": d, "ratio": ratio}
        if beta1 is not None:
            out["beta2"] = beta1 + d
        return out
    d = beta2 - beta1
    return {"d_beta": d, "ratio": 10 ** (d / 20), "beta2": beta2}


def _pipe_length_factor(system):
    return 4.0 if system == "closed pipe" else 2.0


def standing_wave_harmonic(system, L, v, n) -> dict:
    _positive(L, "the length L")
    _positive(v, "the wave speed v")
    if n < 1:
        raise ValueError("the harmonic number n is a whole number 1, 2, 3, ...")
    if system == "closed pipe" and n % 2 == 0:
        raise ValueError("a pipe closed at one end has only odd harmonics (n = 1, 3, 5, ...)")
    k = _pipe_length_factor(system)
    f1 = v / (k * L)
    return {"f_n": n * f1, "lam_n": k * L / n, "f1": f1}


def verify_standing(i, o) -> float:
    fn, lam, f1 = o["f_n"], o["lam_n"], o["f1"]
    if not (math.isfinite(fn) and math.isfinite(lam) and math.isfinite(f1)):
        return math.inf
    k = _pipe_length_factor(i["system"])
    return _rel(fn * lam, i["v"]) + _rel(lam * i["n"], k * i["L"]) + _rel(f1 * k * i["L"], i["v"])


def ref_standing(system, L, v, n):
    """The n-th root of the boundary condition: sin(kL) = 0 for a string fixed at both ends or a pipe open at both
    ends, cos(kL) = 0 for a pipe closed at one end, found by scipy brentq between neighbouring half-orders;
    f = v k / (2 pi), lambda = 2 pi / k."""
    from scipy.optimize import brentq
    if system == "closed pipe":
        k = brentq(lambda x: math.cos(x * L), (n - 1) * math.pi / (2 * L), (n + 1) * math.pi / (2 * L),
                   xtol=1e-300, rtol=8.9e-16, maxiter=500)
        k1 = brentq(lambda x: math.cos(x * L), 0.0, math.pi / L, xtol=1e-300, rtol=8.9e-16, maxiter=500)
    else:
        k = brentq(lambda x: math.sin(x * L), (n - 0.5) * math.pi / L, (n + 0.5) * math.pi / L,
                   xtol=1e-300, rtol=8.9e-16, maxiter=500)
        k1 = brentq(lambda x: math.sin(x * L), 0.5 * math.pi / L, 1.5 * math.pi / L, xtol=1e-300, rtol=8.9e-16, maxiter=500)
    return {"f_n": v * k / (2 * math.pi), "lam_n": 2 * math.pi / k, "f1": v * k1 / (2 * math.pi)}


def _wrong_standing(system, L, v, n):
    """A whole wavelength fitted where a half (or a quarter) belongs: lambda_n = L / n."""
    standing_wave_harmonic(system, L, v, n)
    return {"f_n": n * v / L, "lam_n": L / n, "f1": v / L}


def string_wave_speed(T, mu) -> dict:
    _positive(T, "the tension T")
    _positive(mu, "the linear mass density mu")
    return {"v": math.sqrt(T / mu)}


def verify_string(i, o) -> float:
    v = o["v"]
    if not math.isfinite(v) or v <= 0:
        return math.inf
    return _rel(i["mu"] * v * v, i["T"])


def ref_string(T, mu):
    """Re-solve of the wave equation's speed condition mu v^2 = T for v by scipy brentq."""
    from scipy.optimize import brentq
    return {"v": brentq(lambda x: mu * x * x - T, 0.0, 1.0 + T / mu, xtol=1e-300, rtol=8.9e-16, maxiter=500)}


# ================================================================================================ random inputs
def _rand_coulomb(rng):
    return {"q1": _sign(rng) * _lu(rng, 1e-9, 1e-5), "q2": _sign(rng) * _lu(rng, 1e-9, 1e-5), "r": _lu(rng, 0.01, 2.0)}


def _rand_gravity(rng):
    return {"m1": _lu(rng, 1.0, 1e25), "m2": _lu(rng, 1.0, 1e23), "r": _lu(rng, 0.1, 1e9)}


def _rand_snell(rng):
    return {"n1": rng.uniform(1.0, 2.0), "n2": rng.uniform(1.0, 2.0), "theta1": rng.uniform(0.02, 1.5)}


def _rand_critical(rng):
    n1 = rng.uniform(1.2, 2.5)
    return {"n1": n1, "n2": rng.uniform(1.0, 0.9 * n1)}


def _rand_mag(rng):
    h = rng.uniform(0.005, 0.2)
    f = _sign(rng) * rng.uniform(0.05, 1.0)
    d_o = abs(f) * rng.choice([rng.uniform(0.1, 0.9), rng.uniform(1.1, 5.0)])
    if rng.random() < 0.7:
        return {"d_o": d_o, "h_o": h, "f": f}
    return {"d_o": d_o, "h_o": h, "d_i": _sign(rng) * d_o * rng.uniform(0.05, 0.8)}


def _rand_double(rng):
    return {"d": _lu(rng, 2e-5, 1e-3), "lam": rng.uniform(4e-7, 7e-7), "m": rng.randint(0, 5), "L": rng.uniform(0.5, 3.0)}


def _rand_grating(rng):
    out = {"lam": rng.uniform(4e-7, 7e-7), "m": rng.randint(1, 3)}
    if rng.random() < 0.7:
        out["N"] = rng.uniform(100.0, 1200.0)
    else:
        out["d"] = rng.uniform(8e-7, 1e-5)
    return out


def _rand_single(rng):
    return {"a": _lu(rng, 2e-6, 2e-4), "lam": rng.uniform(4e-7, 7e-7), "m": rng.randint(1, 3), "L": rng.uniform(0.5, 3.0)}


def _rand_malus(rng):
    th = rng.choice([rng.uniform(0.0, 1.4), rng.uniform(1.75, 3.1)])
    out = {"I0": rng.uniform(1.0, 1000.0), "theta": th}
    if rng.random() < 0.5:
        out["light"] = rng.choice(["polarized", "unpolarized"])
    return out


def _rand_faraday(rng):
    for _ in range(100):
        x = {"N": rng.randint(2, 500), "A1": _lu(rng, 1e-4, 0.1), "B1": rng.uniform(-1.0, 1.0), "B2": rng.uniform(-1.0, 1.0),
             "dt": _lu(rng, 0.01, 2.0)}
        if rng.random() < 0.5:
            x["A2"] = _lu(rng, 1e-4, 0.1)
        if rng.random() < 0.5:
            x["theta"] = rng.uniform(0.0, 1.2)
        A2 = x.get("A2", x["A1"])
        p1, p2 = x["B1"] * x["A1"], x["B2"] * A2
        if abs(p2 - p1) >= 0.05 * max(abs(p1), abs(p2)):
            return x
    return {"N": 100, "A1": 0.01, "B1": 0.0, "B2": 0.5, "dt": 0.1}


def _rand_transformer(rng):
    out = {"Np": rng.randint(10, 2000), "Ns": rng.randint(10, 2000), "Vp": _lu(rng, 5.0, 20000.0)}
    if rng.random() < 0.7:
        out["Ip"] = _lu(rng, 0.01, 50.0)
    return out


def _rand_rlc(rng):
    return {"R": _lu(rng, 1.0, 1000.0), "L": _lu(rng, 1e-4, 1.0), "C": _lu(rng, 1e-8, 1e-4), "f": _lu(rng, 10.0, 1e4),
            "V": rng.uniform(1.0, 240.0)}


def _rand_terminal(rng):
    return {"emf": rng.uniform(1.0, 24.0), "r": _lu(rng, 0.01, 5.0), "R": _lu(rng, 0.1, 100.0)}


def _rand_static_limit(rng):
    mu, N = rng.uniform(0.1, 1.0), _lu(rng, 1.0, 1000.0)
    return {"mu_s": mu, "N": N, "F": rng.uniform(0.0, 1.5) * mu * N}


def _rand_incline(rng):
    out = {"mu_s": rng.uniform(0.05, 1.2), "theta": rng.uniform(0.02, 1.4)}
    if rng.random() < 0.5:
        out["m"] = _lu(rng, 0.1, 500.0)
    return out


def _rand_young(rng):
    strain = _sign(rng) * _lu(rng, 1e-6, 1e-2)
    E, A = _lu(rng, 1e9, 4e11), _lu(rng, 1e-6, 1e-2)
    return {"F": strain * E * A, "A": A, "L0": _lu(rng, 0.1, 10.0), "E": E}


def _rand_shear(rng):
    strain = _sign(rng) * _lu(rng, 1e-6, 1e-2)
    S, A = _lu(rng, 1e9, 1e11), _lu(rng, 1e-5, 1e-1)
    return {"F": strain * S * A, "A": A, "h": _lu(rng, 0.01, 0.5), "S": S}


def _rand_bulk(rng):
    return {"dP": _sign(rng) * _lu(rng, 1e5, 1e8), "V0": _lu(rng, 1e-6, 10.0), "B": _lu(rng, 2e9, 3e11)}


def _rand_beam(rng):
    xA = rng.uniform(0.0, 1.0)
    xB = xA + rng.uniform(2.0, 8.0)
    n = rng.randint(1, 4)
    loads = [rng.uniform(10.0, 1000.0) for _ in range(n)]
    pos = [rng.uniform(xA - 0.5, xB + 0.5) for _ in range(n)]
    return {"loads": loads, "positions": pos, "xA": xA, "xB": xB}


def _rand_com(rng):
    n = rng.randint(2, 6)
    out = {"masses": [rng.uniform(0.1, 20.0) for _ in range(n)], "x": [rng.uniform(-5.0, 5.0) for _ in range(n)]}
    if rng.random() < 0.5:
        out["y"] = [rng.uniform(-5.0, 5.0) for _ in range(n)]
    return out


def _rand_vectors(rng):
    n = rng.randint(1, 5)
    return {"magnitudes": [rng.uniform(0.5, 50.0) for _ in range(n)], "directions": [rng.uniform(0.0, 360.0) for _ in range(n)]}


def _rand_rotation(rng):
    return {"tau": _sign(rng) * _lu(rng, 0.1, 500.0), "I": _lu(rng, 0.01, 50.0)}


def _rand_spin(rng):
    I1 = _lu(rng, 0.5, 10.0)
    return {"I1": I1, "omega1": _sign(rng) * _lu(rng, 0.5, 20.0), "I2": I1 * rng.choice([rng.uniform(0.2, 0.8), rng.uniform(1.25, 5.0)])}


def _rand_level(rng):
    if rng.random() < 0.5:
        return {"I": _lu(rng, 1e-10, 10.0)}
    return {"beta": rng.uniform(15.0, 130.0)}


def _rand_level_change(rng):
    b1 = rng.uniform(10.0, 120.0)
    if rng.random() < 0.5:
        return {"beta1": b1, "beta2": b1 + _sign(rng) * rng.uniform(1.0, 40.0)}
    out = {"ratio": _lu(rng, 0.001, 1000.0)}
    if abs(math.log10(out["ratio"])) < 0.05:
        out["ratio"] = 2.0
    if rng.random() < 0.5:
        out["beta1"] = b1
    return out


def _rand_standing(rng):
    system = rng.choice(["string", "open pipe", "closed pipe"])
    n = rng.choice([1, 3, 5, 7, 9]) if system == "closed pipe" else rng.randint(1, 8)
    return {"system": system, "L": rng.uniform(0.2, 3.0), "v": rng.uniform(100.0, 500.0), "n": n}


def _rand_string(rng):
    return {"T": _lu(rng, 1.0, 2000.0), "mu": _lu(rng, 1e-4, 0.1)}


# ================================================================================================ Gauss's law (core gap, 2026-11-12)
# Fields of symmetric charge distributions. Conventions: the charge Q carries its sign; E_radial is the component
# away from the line, the plane or the centre (positive for a positive charge outside); a line carries Q on a length L
# (lambda = Q/L) and is treated as infinitely long; a plane (a thin non-conducting sheet) carries Q on an area A
# (sigma = Q/A) and is treated as infinite, so r is not needed for it (when given it must be positive); "sphere" is a
# uniformly charged solid (insulating) ball of radius R, "shell" a thin spherical shell of radius R (also the field of
# a conducting sphere); on the shell itself (r = R) the field jumps and is refused. Q_enc is the charge inside the
# Gaussian surface through the point (a coaxial cylinder of length L, a pillbox through the area A, a concentric
# sphere of radius r) and flux = Q_enc/eps0 = 4 pi k Q_enc (N m^2/C).
_GAUSS = ("line", "plane", "sphere", "shell")


def gauss_field(geometry, Q, r=None, L=None, A=None, R=None) -> dict:
    if geometry not in _GAUSS:
        raise ValueError("geometry must be line, plane, sphere or shell")
    if not math.isfinite(Q):
        raise ValueError("the charge Q must be a finite number")
    if r is not None:
        _positive(r, "the distance r")
    if geometry == "line":
        if L is None or r is None:
            raise ValueError("a line of charge needs the length L that carries Q and the distance r from the line")
        if A is not None or R is not None:
            raise ValueError("a line of charge takes no area A or radius R")
        _positive(L, "the length L")
        dens = Q / L
        Er = 2.0 * KE * dens / r
        qenc = Q
    elif geometry == "plane":
        if A is None:
            raise ValueError("a plane of charge needs the area A that carries Q")
        if L is not None or R is not None:
            raise ValueError("a plane of charge takes no length L or radius R")
        _positive(A, "the area A")
        dens = Q / A
        Er = 2.0 * math.pi * KE * dens
        qenc = Q
    else:
        if R is None or r is None:
            raise ValueError("a charged sphere or shell needs its radius R and the distance r from its centre")
        if L is not None or A is not None:
            raise ValueError("a sphere or shell takes no length L or area A")
        _positive(R, "the radius R")
        if geometry == "shell" and r == R:
            raise ValueError("on the shell itself (r = R) the field is not defined: it jumps from 0 to kQ/R^2")
        if geometry == "sphere":
            dens = Q / (4.0 * math.pi * R ** 3 / 3.0)
        else:
            dens = Q / (4.0 * math.pi * R * R)
        if r >= R:
            qenc = Q
        elif geometry == "sphere":
            qenc = Q * (r / R) ** 3
        else:
            qenc = 0.0
        Er = KE * qenc / (r * r)
    return {"E": _r(abs(Er)), "E_radial": _r(Er), "Q_enc": _r(qenc), "flux": _r(4.0 * math.pi * KE * qenc),
            "density": _r(dens)}


def verify_gauss(i, o) -> float:
    """Gauss's law itself: E_radial times the Gaussian surface's area equals 4 pi k Q_enc; the enclosed charge, the
    density, the flux and |E| recomputed."""
    for k in ("E", "E_radial", "Q_enc", "flux", "density"):
        if k not in o or not math.isfinite(o[k]):
            return math.inf
    g, Q, r = i["geometry"], i["Q"], i.get("r")
    fk = 4.0 * math.pi * KE
    if g == "line":
        dens, qenc, area = Q / i["L"], Q, 2.0 * math.pi * r * i["L"]
    elif g == "plane":
        dens, qenc, area = Q / i["A"], Q, 2.0 * i["A"]
    else:
        R = i["R"]
        dens = Q / (4.0 * math.pi * R ** 3 / 3.0) if g == "sphere" else Q / (4.0 * math.pi * R * R)
        if r >= R:
            qenc = Q
        elif g == "sphere":
            qenc = Q * (r / R) ** 3
        else:
            qenc = 0.0
        area = 4.0 * math.pi * r * r
    s = abs(fk * Q)
    if s == 0:
        return abs(o["E"]) + abs(o["E_radial"]) + abs(o["Q_enc"]) + abs(o["flux"])
    return (abs(o["E_radial"] * area - fk * o["Q_enc"]) / s + _rel(o["Q_enc"], qenc) + _rel(o["flux"], fk * qenc)
            + _rel(o["E"], abs(o["E_radial"])) + _rel(o["density"], dens))


def ref_gauss(geometry, Q, r=None, L=None, A=None, R=None):
    """Coulomb's law summed over the charge distribution by quadrature (scipy quad), not Gauss's law: the infinite
    line as point charges along it, the infinite plane as concentric rings, the thin shell as rings (the shell-theorem
    integral; inside it the integral vanishes to below 1e-12 and is written as 0); the solid ball's enclosed charge by
    quadrature of its uniform density over the concentric sphere."""
    from scipy.integrate import quad
    fk = 4.0 * math.pi * KE
    inf = float("inf")
    if geometry == "line":
        lam = Q / L
        I, _ = quad(lambda t: (1.0 + t * t) ** -1.5, -inf, inf, epsabs=0, epsrel=1e-13, limit=200)
        Er, qenc, dens = KE * lam / r * I, Q, lam
    elif geometry == "plane":
        sig = Q / A
        I, _ = quad(lambda t: t * (1.0 + t * t) ** -1.5, 0.0, inf, epsabs=0, epsrel=1e-13, limit=200)
        Er, qenc, dens = 2.0 * math.pi * KE * sig * I, Q, sig
    elif geometry == "shell":
        x = R / r
        I, _ = quad(lambda t: (1.0 - x * math.cos(t)) / (1.0 + x * x - 2.0 * x * math.cos(t)) ** 1.5 * math.sin(t) / 2.0,
                    0.0, math.pi, epsabs=1e-15, epsrel=1e-13, limit=500)
        if abs(I) < 1e-12:
            I = 0.0
        qenc = Q * I
        Er, dens = KE * qenc / (r * r), Q / (4.0 * math.pi * R * R)
    else:
        frac, _ = quad(lambda s: 3.0 * s * s, 0.0, min(r / R, 1.0), epsabs=0, epsrel=1e-13)
        qenc = Q * frac
        Er, dens = KE * qenc / (r * r), Q / (4.0 / 3.0 * math.pi * R ** 3)
    return {"E": abs(Er), "E_radial": Er, "Q_enc": qenc, "flux": fk * qenc, "density": dens}


def _wrong_gauss(geometry, Q, r=None, L=None, A=None, R=None):
    """Line: k lambda / r (the factor 2 lost); plane: sigma/eps0 (the conductor's value); inside a sphere or shell:
    the whole charge treated as enclosed."""
    o = gauss_field(geometry, Q, r, L, A, R)
    if geometry == "line":
        Er = KE * o["density"] / r
    elif geometry == "plane":
        Er = 4.0 * math.pi * KE * o["density"]
    elif r < R:
        Er = KE * Q / (r * r)
    else:
        return o
    return dict(o, E=abs(Er), E_radial=Er)


def _rand_gauss(rng):
    g = rng.choice(list(_GAUSS))
    d = {"geometry": g, "Q": _sign(rng) * _lu(rng, 1e-10, 1e-5)}
    if g == "line":
        d["L"] = _lu(rng, 0.1, 10.0)
        d["r"] = _lu(rng, 1e-3, 1.0)
    elif g == "plane":
        d["A"] = _lu(rng, 1e-4, 10.0)
        if rng.random() < 0.5:
            d["r"] = _lu(rng, 1e-3, 1.0)
    else:
        R = _lu(rng, 0.01, 1.0)
        d["R"] = R
        d["r"] = R * (rng.uniform(0.1, 0.9) if rng.random() < 0.5 else rng.uniform(1.1, 5.0))
    return d


# ================================================================================================ question readers
_SYSTEMS = {"string": "string", "open pipe": "open pipe", "closed pipe": "closed pipe",
            "pipe open at both ends": "open pipe", "pipe closed at one end": "closed pipe"}


def parse_standing(q):
    if not isinstance(q, str):
        return None
    m = re.match(r"\s*standing[- ]wave harmonics?\s*(?:,\s*|\s+(?:on|in|of|for)\s+an?\s+|\s+)"
                 r"(string|open pipe|closed pipe|pipe open at both ends|pipe closed at one end)\s*:\s*", q, re.I)
    if not m:
        return None
    rest = re.sub(r"[.?!]\s*$", "", q[m.end():].strip()).strip()
    out = _fields(rest, {"l": "L", "v": "v", "n": "n"}, phys=("L", "v"), ints=("n",))
    if out is None:
        return None
    out["system"] = _SYSTEMS[m.group(1).lower()]
    return out


_APOS = r"['’]?s"


# ================================================================================================ the procedures
def _phys(sym, kind, meaning, required=True, **extra):
    return {"symbol": sym, "kind": kind, "meaning": meaning, "required": required, **extra}


def _numin(sym, meaning, required=True):
    return {"symbol": sym, "kind": "number", "meaning": meaning, "required": required}


def _intin(sym, meaning, lo=None, required=True):
    d = {"symbol": sym, "type": "integer", "meaning": meaning, "required": required}
    if lo is not None:
        d["min"] = lo
    return d


def _vecin(sym, meaning, required=True):
    return {"symbol": sym, "type": "vector", "meaning": meaning, "required": required, "min_length": 1}


def _choice(sym, choices, meaning, required=True):
    return {"symbol": sym, "type": "choice", "choices": list(choices), "meaning": meaning, "required": required}


def _o(sym, kind, meaning):
    return {"symbol": sym, "kind": kind, "meaning": meaning}


_SQ = 0.9975 ** 0.5      # sqrt(1 - 0.05^2), used by the single-slit worked example

PROCEDURES = {
    # ------------------------------------------------------------------------------------------ electrostatics
    "coulomb_force": {
        "name": "Coulomb's law: force between two point charges", "topic": "electrostatics / Coulomb's law",
        "purpose": "The electrostatic force between two point charges a distance r apart: its magnitude and its sign "
                   "along the line joining them (positive: repulsion, negative: attraction).",
        "inputs": [_phys("q1", "charge", "first charge (with its sign)"), _phys("q2", "charge", "second charge (with its sign)"),
                   _phys("r", "length", "distance between the charges", condition="r != 0")],
        "outputs": [_o("F", "force", "magnitude of the electrostatic force"),
                    _o("F_radial", "force", "force along the line joining the charges (positive: repulsion)")],
        "primary": "F", "conditions": ["r > 0", "point charges in vacuum (or air)"],
        "method": "F = k q1 q2 / r^2 with k = 8.9875517923e9 N m^2/C^2; the sign of q1 q2 tells repulsion or attraction",
        "latex": r"F = k\,\frac{|q_1 q_2|}{r^2}",
        "run": coulomb_force, "verify": verify_coulomb, "reference": ref_coulomb,
        "parse": lambda q: _pairs(q, r"coulomb" + _APOS + r" law(?: force)?|electrostatic force between two charges",
                                  {"q1": "q1", "q2": "q2", "r": "r"}, phys=("q1", "q2", "r")),
        "wrong": lambda q1, q2, r: {"F": abs(KE * q1 * q2 / r), "F_radial": KE * q1 * q2 / r},
        "random_inputs": _rand_coulomb,
        "templates": ["Coulomb's law: q1 = 2 uC, q2 = 3 uC, r = 10 cm"],
        "examples": [{"inputs": {"q1": 2e-6, "q2": 3e-6, "r": 0.1}, "outputs": {"F": 5.39253107538, "F_radial": 5.39253107538}},
                     {"inputs": {"q1": 1e-6, "q2": -4e-6, "r": 0.2}, "outputs": {"F": 0.89875517923, "F_radial": -0.89875517923}}],
        "questions": [{"q": "Coulomb's law: q1 = 2 uC, q2 = 3 uC, r = 10 cm", "outputs": {"F": 5.39253107538}},
                      {"q": "Coulomb's law: q1 = 1 uC, q2 = -4 uC, r = 20 cm", "outputs": {"F_radial": -0.89875517923}},
                      {"q": "Coulomb's law: q1 = 2 uC, q2 = 3 uC, r = 0 m", "outcome": "INVALID_INPUT"},
                      {"q": "Coulomb's law: q1 = 2 uC, q2 = 3 uC", "outcome": "NEEDS_INPUT"}]},
    # ------------------------------------------------------------------------------------------ gravitation
    "gravitational_force_two_masses": {
        "name": "Newton's law of universal gravitation: force between two masses", "topic": "gravitation / universal gravitation",
        "purpose": "The attractive gravitational force between two masses (point masses or spheres) whose centres are r apart.",
        "inputs": [_phys("m1", "mass", "first mass"), _phys("m2", "mass", "second mass"),
                   _phys("r", "length", "distance between the centres", condition="r != 0")],
        "outputs": [_o("F", "force", "magnitude of the gravitational attraction")],
        "primary": "F", "conditions": ["m1 > 0, m2 > 0, r > 0", "point masses or spherically symmetric bodies"],
        "method": "F = G m1 m2 / r^2 with G = 6.6743e-11 N m^2/kg^2", "latex": r"F = G\,\frac{m_1 m_2}{r^2}",
        "run": gravitational_force, "verify": verify_gravity, "reference": ref_gravity,
        "parse": lambda q: _pairs(q, r"gravitational force(?: between two (?:masses|bodies))?|newton" + _APOS
                                  + r" law of (?:universal )?gravitation", {"m1": "m1", "m2": "m2", "r": "r"}, phys=("m1", "m2", "r")),
        "wrong": lambda m1, m2, r: {"F": G_NEWTON * m1 * m2 / r},
        "random_inputs": _rand_gravity,
        "templates": ["Gravitational force: m1 = 100 kg, m2 = 50 kg, r = 2 m"],
        "examples": [{"inputs": {"m1": 100.0, "m2": 50.0, "r": 2.0}, "outputs": {"F": 8.342875e-8}},
                     {"inputs": {"m1": 5.972e24, "m2": 7.348e22, "r": 3.844e8}, "outputs": {"F": 6.6743e-11 * 5.972e24 * 7.348e22 / 3.844e8 ** 2}}],
        "questions": [{"q": "Gravitational force: m1 = 100 kg, m2 = 50 kg, r = 2 m", "outputs": {"F": 8.342875e-8}},
                      {"q": "Gravitational force: m1 = 100 kg, m2 = 50 kg, r = 0 m", "outcome": "INVALID_INPUT"},
                      {"q": "Gravitational force: m1 = 100 kg, r = 2 m", "outcome": "NEEDS_INPUT"}]},
    # ------------------------------------------------------------------------------------------ refraction
    "snell_refraction_angle": {
        "name": "Snell's law: angle of refraction", "topic": "geometric optics / refraction",
        "purpose": "The angle of refraction of a ray passing from a medium of index n1 into one of index n2; refused "
                   "with a reason when the ray is totally internally reflected.",
        "inputs": [_phys("n1", "refractive_index", "refractive index of the incident medium"),
                   _phys("n2", "refractive_index", "refractive index of the second medium"),
                   _phys("theta1", "angle", "angle of incidence from the normal (deg or rad)")],
        "outputs": [_o("theta2", "angle", "angle of refraction from the normal"),
                    _o("theta2_deg", "number", "angle of refraction in degrees")],
        "primary": "theta2", "conditions": ["n1, n2 >= 1", "0 <= theta1 <= 90 degrees", "n1 sin(theta1) <= n2 (no total internal reflection)"],
        "method": "sin(theta2) = n1 sin(theta1) / n2", "latex": r"n_1\sin\theta_1 = n_2\sin\theta_2",
        "run": snell_refraction, "verify": verify_snell, "reference": ref_snell,
        "parse": lambda q: _pairs(q, r"snell" + _APOS + r" law(?: refraction)?|angle of refraction",
                                  {"n1": "n1", "n2": "n2", "theta1": "theta1", "theta": "theta1"}, phys=("theta1",)),
        "wrong": lambda n1, n2, theta1: {"theta2": theta1 * n1 / n2, "theta2_deg": math.degrees(theta1 * n1 / n2)},
        "random_inputs": _rand_snell,
        "templates": ["Snell's law: n1 = 1, n2 = 1.5, theta1 = 30 deg"],
        "examples": [{"inputs": {"n1": 1.0, "n2": 1.5, "theta1": math.pi / 6}, "outputs": {"theta2": 0.33983690945412193, "theta2_deg": 19.47122063449069}},
                     {"inputs": {"n1": 1.33, "n2": 1.0, "theta1": math.pi / 6}, "outputs": {"theta2": math.asin(0.665)}}],
        "questions": [{"q": "Snell's law: n1 = 1, n2 = 1.5, theta1 = 30 deg", "outputs": {"theta2_deg": 19.47122063449069}},
                      {"q": "Snell's law: n1 = 1.5, n2 = 1, theta1 = 60 deg", "outcome": "INVALID_INPUT"},
                      {"q": "Snell's law: n1 = 1, n2 = 1.5, theta1 = 30", "outcome": "UNRECOGNIZED_FORMAT"},
                      {"q": "Snell's law: n1 = 1, n2 = 1.5", "outcome": "NEEDS_INPUT"}]},
    "critical_angle_total_internal_reflection": {
        "name": "Critical angle for total internal reflection", "topic": "geometric optics / total internal reflection",
        "purpose": "The smallest angle of incidence (from the normal) at which light in the denser medium is totally "
                   "reflected at the boundary with a less dense one.",
        "inputs": [_phys("n1", "refractive_index", "refractive index of the medium the light travels in (the denser one)"),
                   _phys("n2", "refractive_index", "refractive index of the medium beyond the boundary")],
        "outputs": [_o("theta_c", "angle", "critical angle from the normal"), _o("theta_c_deg", "number", "critical angle in degrees")],
        "primary": "theta_c", "conditions": ["n1 > n2 >= 1"],
        "method": "sin(theta_c) = n2 / n1", "latex": r"\sin\theta_c = \frac{n_2}{n_1}",
        "run": critical_angle, "verify": verify_critical, "reference": ref_critical,
        "parse": lambda q: _pairs(q, r"critical angle(?: for total internal reflection)?|total internal reflection",
                                  {"n1": "n1", "n2": "n2"}),
        "wrong": lambda n1, n2: {"theta_c": math.atan(n2 / n1), "theta_c_deg": math.degrees(math.atan(n2 / n1))},
        "random_inputs": _rand_critical,
        "templates": ["Critical angle: n1 = 1.5, n2 = 1"],
        "examples": [{"inputs": {"n1": 1.5, "n2": 1.0}, "outputs": {"theta_c": 0.7297276562269663, "theta_c_deg": 41.81031489577862}},
                     {"inputs": {"n1": 2.42, "n2": 1.0}, "outputs": {"theta_c": math.asin(1 / 2.42)}}],
        "questions": [{"q": "Critical angle: n1 = 1.5, n2 = 1", "outputs": {"theta_c_deg": 41.81031489577862}},
                      {"q": "Critical angle: n1 = 1, n2 = 1.5", "outcome": "INVALID_INPUT"},
                      {"q": "Critical angle: n1 = 1.5", "outcome": "NEEDS_INPUT"}]},
    # ------------------------------------------------------------------------------------------ images
    "lens_mirror_magnification": {
        "name": "Magnification and image height of a thin lens or a spherical mirror", "topic": "geometric optics / images",
        "purpose": "Magnification, image height, image distance and the nature of the image (real or virtual, upright "
                   "or inverted) from the object distance and either the image distance or the focal length.",
        "inputs": [_phys("d_o", "length", "object distance (positive)"), _phys("h_o", "length", "object height"),
                   _phys("d_i", "length", "image distance (positive: real image), if known", required=False),
                   _phys("f", "length", "focal length (positive: converging lens or concave mirror), if d_i is not given", required=False)],
        "outputs": [_o("m", "magnification", "magnification (negative: inverted image)"),
                    _o("h_i", "length", "image height (negative: inverted)"),
                    _o("d_i", "length", "image distance (positive: real, negative: virtual)"),
                    _o("image", "text", "real or virtual, upright or inverted")],
        "primary": "m", "conditions": ["d_o > 0, h_o > 0", "exactly one of d_i and f", "the object not at the focal point"],
        "method": "d_i = d_o f / (d_o - f) when f is given; m = -d_i / d_o; h_i = m h_o",
        "latex": r"m = \frac{h_i}{h_o} = -\frac{d_i}{d_o},\quad \frac{1}{d_o} + \frac{1}{d_i} = \frac{1}{f}",
        "run": lens_mirror_magnification, "verify": verify_magnification, "reference": ref_magnification,
        "parse": lambda q: _pairs(q, r"image magnification|(?:lens|mirror) magnification",
                                  {"d_o": "d_o", "do": "d_o", "d_i": "d_i", "di": "d_i", "f": "f", "h_o": "h_o", "ho": "h_o"},
                                  phys=("d_o", "d_i", "f", "h_o")),
        "wrong": lambda d_o, h_o, d_i=None, f=None: (lambda o: {**o, "m": -o["m"], "h_i": -o["h_i"]})(
            lens_mirror_magnification(d_o, h_o, d_i, f)),
        "random_inputs": _rand_mag,
        "templates": ["Image magnification: d_o = 30 cm, f = 10 cm, h_o = 2 cm"],
        "examples": [{"inputs": {"d_o": 0.3, "h_o": 0.02, "f": 0.1}, "outputs": {"d_i": 0.15, "m": -0.5, "h_i": -0.01, "image": "real, inverted"}},
                     {"inputs": {"d_o": 0.05, "h_o": 0.02, "f": 0.1}, "outputs": {"d_i": -0.1, "m": 2.0, "h_i": 0.04, "image": "virtual, upright"}},
                     {"inputs": {"d_o": 0.4, "h_o": 0.05, "d_i": 0.2}, "outputs": {"m": -0.5, "h_i": -0.025}}],
        "questions": [{"q": "Image magnification: d_o = 30 cm, f = 10 cm, h_o = 2 cm", "outputs": {"m": -0.5, "h_i": -0.01}},
                      {"q": "Image magnification: d_o = 5 cm, f = 10 cm, h_o = 2 cm", "outputs": {"m": 2.0, "image": "virtual, upright"}},
                      {"q": "Image magnification: d_o = 10 cm, f = 10 cm, h_o = 2 cm", "outcome": "INVALID_INPUT"},
                      {"q": "Image magnification: d_o = 30 cm, f = 10 cm, d_i = 15 cm, h_o = 2 cm", "outcome": "INVALID_INPUT"},
                      {"q": "Image magnification: d_o = 30 cm, f = 10 cm", "outcome": "NEEDS_INPUT"}]},
    # ------------------------------------------------------------------------------------------ wave optics
    "double_slit_bright_fringe": {
        "name": "Double-slit interference: angle and position of a bright fringe", "topic": "wave optics / interference",
        "purpose": "The angle and the position on the screen of the m-th bright fringe of a double slit, and the fringe "
                   "spacing near the centre.",
        "inputs": [_phys("d", "length", "slit separation"), _phys("lam", "length", "wavelength"),
                   _intin("m", "order of the bright fringe (0 for the central maximum)", lo=0),
                   _phys("L", "length", "distance from the slits to the screen")],
        "outputs": [_o("y", "length", "position of the m-th bright fringe on the screen, from the central maximum"),
                    _o("theta", "angle", "angle of the m-th bright fringe from the central axis"),
                    _o("theta_deg", "number", "that angle in degrees"),
                    _o("dy", "length", "fringe spacing near the centre (small-angle, lambda L / d)")],
        "primary": "y", "conditions": ["d, lam, L > 0", "m a whole number with m lambda < d"],
        "method": "d sin(theta) = m lambda; y = L tan(theta); spacing lambda L / d",
        "latex": r"d\sin\theta = m\lambda,\quad y = L\tan\theta,\quad \Delta y \approx \frac{\lambda L}{d}",
        "run": double_slit, "verify": verify_double_slit, "reference": ref_double_slit,
        "parse": lambda q: _pairs(q, r"double[- ]slit(?: interference| bright fringe)?|young" + _APOS + r" double[- ]slit",
                                  {"d": "d", "lam": "lam", "lambda": "lam", "m": "m", "l": "L"}, phys=("d", "lam", "L"), ints=("m",)),
        "wrong": _wrong_double_slit,
        "random_inputs": _rand_double,
        "templates": ["Double slit: d = 0.1 mm, lam = 600 nm, m = 2, L = 1.5 m"],
        "examples": [{"inputs": {"d": 1e-4, "lam": 6e-7, "m": 2, "L": 1.5},
                      "outputs": {"theta": math.asin(0.012), "y": 1.5 * 0.012 / math.sqrt(1 - 0.012 ** 2), "dy": 0.009}},
                     {"inputs": {"d": 5e-5, "lam": 5e-7, "m": 0, "L": 2.0}, "outputs": {"theta": 0.0, "y": 0.0, "dy": 0.02}}],
        "questions": [{"q": "Double slit: d = 0.1 mm, lam = 600 nm, m = 2, L = 1.5 m", "outputs": {"dy": 0.009}},
                      {"q": "Double slit: d = 1 um, lam = 600 nm, m = 2, L = 1 m", "outcome": "INVALID_INPUT"},
                      {"q": "Double slit: d = 0.1 mm, lam = 600 nm, m = 2.5, L = 1.5 m", "outcome": "INVALID_INPUT"},
                      {"q": "Double slit: d = 0.1 mm, lam = 600 nm, m = 2", "outcome": "NEEDS_INPUT"}]},
    "diffraction_grating_order": {
        "name": "Diffraction grating: angle of an order and the highest visible order", "topic": "wave optics / diffraction grating",
        "purpose": "The angle of the m-th order line of a diffraction grating, the line spacing and the highest order "
                   "that can be seen.",
        "inputs": [_phys("lam", "length", "wavelength"), _intin("m", "order of the line (1, 2, ...)", lo=1),
                   _phys("d", "length", "line spacing, if given instead of N", required=False),
                   _numin("N", "number of lines per millimetre, if given instead of d", required=False)],
        "outputs": [_o("theta", "angle", "angle of the m-th order from the central axis"),
                    _o("theta_deg", "number", "that angle in degrees"),
                    _o("m_max", "number", "highest order that can be seen"),
                    _o("d", "length", "line spacing")],
        "primary": "theta", "conditions": ["exactly one of d and N", "d > lambda", "1 <= m <= the highest order"],
        "method": "d = 1 mm / N; d sin(theta) = m lambda; highest order: largest m with m lambda < d",
        "latex": r"d\sin\theta = m\lambda,\quad m_{max} = \max\{m : m\lambda < d\}",
        "run": diffraction_grating, "verify": verify_grating, "reference": ref_grating,
        "parse": lambda q: _pairs(q, r"diffraction grating", {"lam": "lam", "lambda": "lam", "m": "m", "d": "d", "n": "N"},
                                  phys=("lam", "d"), ints=("m",), num_units={"N": ("lines/mm", "/mm", "lines/millimetre", "lines/millimeter")}),
        "wrong": _wrong_grating,
        "random_inputs": _rand_grating,
        "templates": ["Diffraction grating: N = 600 lines/mm, lam = 500 nm, m = 2"],
        "examples": [{"inputs": {"lam": 5e-7, "m": 2, "N": 600.0},
                      "outputs": {"theta": 0.6435011087932844, "theta_deg": 36.86989764584402, "m_max": 3}},
                     {"inputs": {"lam": 5e-7, "m": 1, "d": 2e-6},
                      "outputs": {"theta": 0.25268025514207865, "theta_deg": 14.477512185929925, "m_max": 3}}],
        "questions": [{"q": "Diffraction grating: N = 600 lines/mm, lam = 500 nm, m = 2", "outputs": {"theta_deg": 36.86989764584402, "m_max": 3}},
                      {"q": "Diffraction grating: d = 2 um, lam = 500 nm, m = 4", "outcome": "INVALID_INPUT"},
                      {"q": "Diffraction grating: N = 600 lines/mm, m = 2", "outcome": "NEEDS_INPUT"}]},
    "single_slit_minimum": {
        "name": "Single-slit diffraction: dark fringes and the width of the central maximum", "topic": "wave optics / diffraction",
        "purpose": "The angle and screen position of the m-th dark fringe of a single slit, and the angular and linear "
                   "width of the central bright maximum.",
        "inputs": [_phys("a", "length", "slit width"), _phys("lam", "length", "wavelength"),
                   _intin("m", "order of the dark fringe (1, 2, ...)", lo=1), _phys("L", "length", "distance from the slit to the screen")],
        "outputs": [_o("theta", "angle", "angle of the m-th dark fringe from the central axis"),
                    _o("theta_deg", "number", "that angle in degrees"),
                    _o("y", "length", "position of the m-th dark fringe on the screen, from the centre"),
                    _o("W", "length", "width of the central maximum on the screen"),
                    _o("W_angle", "angle", "angular width of the central maximum")],
        "primary": "theta", "conditions": ["a, lam, L > 0", "lambda < a", "m lambda < a"],
        "method": "a sin(theta) = m lambda; y = L tan(theta); central width 2 L tan(theta_1)",
        "latex": r"a\sin\theta = m\lambda,\quad W = 2L\tan\theta_1",
        "run": single_slit, "verify": verify_single_slit, "reference": ref_single_slit,
        "parse": lambda q: _pairs(q, r"single[- ]slit(?: diffraction| minimum)?",
                                  {"a": "a", "lam": "lam", "lambda": "lam", "m": "m", "l": "L"}, phys=("a", "lam", "L"), ints=("m",)),
        "wrong": lambda a, lam, m, L: (lambda o: {**o, "W": o["W"] / 2})(single_slit(a, lam, m, L)),
        "random_inputs": _rand_single,
        "templates": ["Single slit: a = 0.01 mm, lam = 500 nm, m = 1, L = 2 m"],
        "examples": [{"inputs": {"a": 1e-5, "lam": 5e-7, "m": 1, "L": 2.0},
                      "outputs": {"theta": math.asin(0.05), "W": 4 * 0.05 / _SQ, "W_angle": 2 * math.asin(0.05)}},
                     {"inputs": {"a": 1e-5, "lam": 5e-7, "m": 3, "L": 2.0}, "outputs": {"theta": math.asin(0.15)}}],
        "questions": [{"q": "Single slit: a = 0.01 mm, lam = 500 nm, m = 1, L = 2 m", "outputs": {"W": 4 * 0.05 / _SQ}},
                      {"q": "Single slit: a = 400 nm, lam = 500 nm, m = 1, L = 1 m", "outcome": "INVALID_INPUT"},
                      {"q": "Single slit: a = 0.01 mm, lam = 500 nm, m = 1", "outcome": "NEEDS_INPUT"}]},
    "malus_law_polarizer": {
        "name": "Malus's law: intensity through a polarizer", "topic": "wave optics / polarization",
        "purpose": "Intensity of light after an analyser whose transmission axis makes an angle theta with the light's "
                   "polarization (or, for unpolarized light, with a first polarizer it passed through).",
        "inputs": [_phys("I0", "intensity", "intensity of the incident light"),
                   _phys("theta", "angle", "angle between the polarization (or first polarizer) and the analyser's axis"),
                   _choice("light", ["polarized", "unpolarized"], "incident light (default polarized)", required=False)],
        "outputs": [_o("I", "intensity", "transmitted intensity"), _o("fraction", "number", "transmitted fraction I / I0")],
        "primary": "I", "conditions": ["I0 > 0", "ideal polarizers"],
        "method": "I = I0 cos^2(theta); unpolarized light first loses half at the first polarizer",
        "latex": r"I = I_0\cos^2\theta",
        "run": malus_law, "verify": verify_malus, "reference": ref_malus,
        "parse": lambda q: _pairs(q, r"malus" + _APOS + r" law", {"i0": "I0", "theta": "theta", "light": "light"},
                                  phys=("I0", "theta"), words=("light",)),
        "wrong": lambda I0, theta, light="polarized": {"I": (1.0 if light == "polarized" else 0.5) * I0 * math.cos(theta),
                                                      "fraction": (1.0 if light == "polarized" else 0.5) * math.cos(theta)},
        "random_inputs": _rand_malus,
        "templates": ["Malus's law: I0 = 100 W/m^2, theta = 60 deg"],
        "examples": [{"inputs": {"I0": 100.0, "theta": math.pi / 3}, "outputs": {"I": 25.0, "fraction": 0.25}},
                     {"inputs": {"I0": 100.0, "theta": math.pi / 4, "light": "unpolarized"}, "outputs": {"I": 25.0}},
                     {"inputs": {"I0": 80.0, "theta": math.pi / 2}, "outputs": {"I": 0.0}}],
        "questions": [{"q": "Malus's law: I0 = 100 W/m^2, theta = 60 deg", "outputs": {"I": 25.0}},
                      {"q": "Malus's law: I0 = 100 W/m^2, theta = 45 deg, light = unpolarized", "outputs": {"I": 25.0}},
                      {"q": "Malus's law: I0 = -5 W/m^2, theta = 30 deg", "outcome": "INVALID_INPUT"},
                      {"q": "Malus's law: I0 = 100 W/m^2", "outcome": "NEEDS_INPUT"}]},
    # ------------------------------------------------------------------------------------------ induction and AC
    "faraday_induced_emf": {
        "name": "Faraday's law: average emf induced by a change of magnetic flux", "topic": "electromagnetic induction / Faraday's law",
        "purpose": "The flux through a coil before and after a change of field or area, and the average emf induced in "
                   "its N turns during the change (with Lenz's sign).",
        "inputs": [_intin("N", "number of turns", lo=1), _phys("A1", "area", "initial area of the coil"),
                   _phys("B1", "magnetic_field", "initial magnetic field"), _phys("B2", "magnetic_field", "final magnetic field"),
                   _phys("dt", "time", "duration of the change", condition="dt != 0"),
                   _phys("A2", "area", "final area of the coil (default: unchanged)", required=False),
                   _phys("theta", "angle", "angle between the field and the coil's axis (default 0)", required=False)],
        "outputs": [_o("emf", "voltage", "magnitude of the average induced emf"),
                    _o("emf_signed", "voltage", "average induced emf with Lenz's sign, -N dPhi/dt"),
                    _o("phi1", "magnetic_flux", "initial flux through one turn"), _o("phi2", "magnetic_flux", "final flux through one turn"),
                    _o("dphi", "magnetic_flux", "change of flux through one turn")],
        "primary": "emf", "conditions": ["N >= 1", "dt > 0", "areas >= 0", "uniform field over the coil"],
        "method": "Phi = B A cos(theta); emf = -N (Phi2 - Phi1) / dt",
        "latex": r"\mathcal{E} = -N\,\frac{\Delta\Phi}{\Delta t},\quad \Phi = BA\cos\theta",
        "run": faraday_emf, "verify": verify_faraday, "reference": ref_faraday,
        "parse": lambda q: _pairs(q, r"faraday(?:" + _APOS + r" law)?(?: induced emf)?|induced emf",
                                  {"n": "N", "a1": "A1", "a": "A1", "a2": "A2", "b1": "B1", "b2": "B2", "dt": "dt", "theta": "theta"},
                                  phys=("A1", "A2", "B1", "B2", "dt", "theta"), ints=("N",), num_units={"N": ("turns",)}),
        "wrong": _wrong_faraday,
        "random_inputs": _rand_faraday,
        "templates": ["Faraday induced emf: N = 200, A1 = 100 cm^2, B1 = 0 T, B2 = 0.5 T, dt = 0.1 s"],
        "examples": [{"inputs": {"N": 200, "A1": 0.01, "B1": 0.0, "B2": 0.5, "dt": 0.1},
                      "outputs": {"phi2": 0.005, "dphi": 0.005, "emf": 10.0, "emf_signed": -10.0}},
                     {"inputs": {"N": 1, "A1": 0.02, "A2": 0.005, "B1": 0.4, "B2": 0.4, "dt": 0.25},
                      "outputs": {"phi1": 0.008, "phi2": 0.002, "dphi": -0.006, "emf": 0.024, "emf_signed": 0.024}},
                     {"inputs": {"N": 50, "A1": 0.04, "B1": 0.2, "B2": 0.0, "dt": 0.5, "theta": math.pi / 3},
                      "outputs": {"phi1": 0.004, "emf": 0.4}}],
        "questions": [{"q": "Faraday induced emf: N = 200, A1 = 100 cm^2, B1 = 0 T, B2 = 0.5 T, dt = 0.1 s", "outputs": {"emf": 10.0}},
                      {"q": "Faraday induced emf: N = 200, A1 = 100 cm^2, B1 = 0 T, B2 = 0.5 T, dt = 0 s", "outcome": "INVALID_INPUT"},
                      {"q": "Faraday induced emf: N = 200, A1 = 100 cm^2, B1 = 0 T, B2 = 0.5 T", "outcome": "NEEDS_INPUT"}]},
    "ideal_transformer_secondary": {
        "name": "Ideal transformer: secondary voltage and current", "topic": "electromagnetic induction / transformers",
        "purpose": "Secondary voltage (and, when the primary current is given, the secondary current) of an ideal "
                   "transformer from its turns, and whether it steps the voltage up or down.",
        "inputs": [_intin("Np", "number of turns of the primary", lo=1), _intin("Ns", "number of turns of the secondary", lo=1),
                   _phys("Vp", "voltage", "primary voltage (rms)"), _phys("Ip", "current", "primary current (rms), if known", required=False)],
        "outputs": [_o("Vs", "voltage", "secondary voltage"), _o("Is", "current", "secondary current (when Ip is given)"),
                    _o("ratio", "number", "turns ratio Ns / Np"), _o("type", "text", "step-up, step-down or isolation")],
        "primary": "Vs", "conditions": ["Np, Ns >= 1", "Vp > 0", "ideal: no losses, no magnetizing current"],
        "method": "Vs / Vp = Ns / Np; Np Ip = Ns Is", "latex": r"\frac{V_s}{V_p} = \frac{N_s}{N_p} = \frac{I_p}{I_s}",
        "run": ideal_transformer, "verify": verify_transformer, "reference": ref_transformer,
        "parse": lambda q: _pairs(q, r"ideal transformer", {"np": "Np", "ns": "Ns", "vp": "Vp", "ip": "Ip"},
                                  phys=("Vp", "Ip"), ints=("Np", "Ns"), num_units={"Np": ("turns",), "Ns": ("turns",)}),
        "wrong": lambda Np, Ns, Vp, Ip=None: (lambda o: {**o, "Vs": Vp * Np / Ns, **({"Is": Ip * Ns / Np} if Ip is not None else {})})(
            ideal_transformer(Np, Ns, Vp, Ip)),
        "random_inputs": _rand_transformer,
        "templates": ["Ideal transformer: Np = 500, Ns = 50, Vp = 230 V, Ip = 0.5 A"],
        "examples": [{"inputs": {"Np": 500, "Ns": 50, "Vp": 230.0, "Ip": 0.5}, "outputs": {"Vs": 23.0, "Is": 5.0, "ratio": 0.1, "type": "step-down"}},
                     {"inputs": {"Np": 100, "Ns": 2500, "Vp": 12.0}, "outputs": {"Vs": 300.0, "ratio": 25.0, "type": "step-up"}}],
        "questions": [{"q": "Ideal transformer: Np = 500, Ns = 50, Vp = 230 V, Ip = 0.5 A", "outputs": {"Vs": 23.0, "Is": 5.0}},
                      {"q": "Ideal transformer: Np = 100 turns, Ns = 2500 turns, Vp = 12 V", "outputs": {"Vs": 300.0}},
                      {"q": "Ideal transformer: Np = 500, Ns = 0, Vp = 230 V", "outcome": "INVALID_INPUT"},
                      {"q": "Ideal transformer: Np = 500, Vp = 230 V", "outcome": "NEEDS_INPUT"}]},
    "series_rlc_ac_circuit": {
        "name": "Series RLC circuit on an AC supply: impedance, current, phase and power", "topic": "AC circuits / series RLC",
        "purpose": "Reactances, impedance, rms current, phase angle, power factor and average power of a resistor, an "
                   "inductor and a capacitor in series across a sinusoidal supply.",
        "inputs": [_phys("R", "resistance", "resistance"), _phys("L", "inductance", "inductance"), _phys("C", "capacitance", "capacitance"),
                   _phys("f", "frequency", "supply frequency"), _phys("V", "voltage", "rms supply voltage")],
        "outputs": [_o("I", "current", "rms current"), _o("Z", "resistance", "impedance"),
                    _o("phi", "angle", "phase angle of the voltage ahead of the current (positive: inductive)"),
                    _o("phi_deg", "number", "phase angle in degrees"), _o("pf", "number", "power factor cos(phi)"),
                    _o("P", "power", "average power"), _o("XL", "resistance", "inductive reactance"), _o("XC", "resistance", "capacitive reactance")],
        "primary": "I", "conditions": ["R, L, C, f, V > 0", "sinusoidal steady state"],
        "method": "XL = 2 pi f L, XC = 1/(2 pi f C), Z = sqrt(R^2 + (XL - XC)^2), I = V/Z, tan(phi) = (XL - XC)/R, P = I^2 R",
        "latex": r"Z = \sqrt{R^2 + (X_L - X_C)^2},\quad I = \frac{V}{Z},\quad \tan\varphi = \frac{X_L - X_C}{R},\quad P = I^2R",
        "run": series_rlc_ac, "verify": verify_rlc, "reference": ref_rlc,
        "parse": lambda q: _pairs(q, r"series rlc(?: ac)?(?: circuit)?", {"r": "R", "l": "L", "c": "C", "f": "f", "v": "V", "vrms": "V"},
                                  phys=("R", "L", "C", "f", "V")),
        "wrong": _wrong_rlc,
        "random_inputs": _rand_rlc,
        "templates": ["Series RLC circuit: R = 30 ohm, L = 0.1 H, C = 10 uF, f = 50 Hz, V = 230 V"],
        "examples": [{"inputs": {"R": 30.0, "L": 0.1, "C": 1 / 60000, "f": 500 / math.pi, "V": 100.0},
                      "outputs": {"XL": 100.0, "XC": 60.0, "Z": 50.0, "I": 2.0, "pf": 0.6, "P": 120.0, "phi_deg": 53.13010235415598}},
                     {"inputs": {"R": 10.0, "L": 0.1, "C": 1e-5, "f": 1 / (2 * math.pi * math.sqrt(1e-6)), "V": 5.0},
                      "outputs": {"Z": 10.0, "I": 0.5, "pf": 1.0, "P": 2.5, "phi": 0.0}}],
        "questions": [{"q": "Series RLC circuit: R = 30 ohm, L = 0.1 H, C = 10 uF, f = 50 Hz, V = 230 V",
                       "outputs": {"XL": 2 * math.pi * 50 * 0.1, "XC": 1 / (2 * math.pi * 50 * 1e-5)}},
                      {"q": "Series RLC circuit: R = 30 ohm, L = 0.1 H, C = 0 uF, f = 50 Hz, V = 230 V", "outcome": "INVALID_INPUT"},
                      {"q": "Series RLC circuit: R = 30 ohm, L = 0.1 H, C = 10 uF, f = 50 Hz", "outcome": "NEEDS_INPUT"}]},
    "terminal_voltage_internal_resistance": {
        "name": "Terminal voltage of a source with internal resistance", "topic": "DC circuits / emf and internal resistance",
        "purpose": "Current, terminal voltage, lost volts, load power and efficiency of a source of emf with internal "
                   "resistance r driving a load resistance R.",
        "inputs": [_phys("emf", "voltage", "emf of the source"), _phys("r", "resistance", "internal resistance"),
                   _phys("R", "resistance", "load resistance")],
        "outputs": [_o("I", "current", "current in the circuit"), _o("V_t", "voltage", "terminal voltage (across the load)"),
                    _o("V_lost", "voltage", "voltage lost inside the source (I r)"), _o("P_load", "power", "power delivered to the load"),
                    _o("efficiency", "efficiency", "fraction of the source's power delivered to the load")],
        "primary": "I", "conditions": ["emf > 0", "r, R >= 0 and not both 0"],
        "method": "I = emf / (R + r); V_t = emf - I r = I R", "latex": r"I = \frac{\mathcal{E}}{R + r},\quad V_t = \mathcal{E} - Ir",
        "run": terminal_voltage, "verify": verify_terminal, "reference": ref_terminal,
        "parse": lambda q: _pairs(q, r"terminal voltage|internal resistance", {"emf": "emf", "E": "emf", "r": "r", "R": "R"},
                                  phys=("emf", "r", "R")),
        "wrong": _wrong_terminal,
        "random_inputs": _rand_terminal,
        "templates": ["Terminal voltage: emf = 12 V, r = 0.5 ohm, R = 5.5 ohm"],
        "examples": [{"inputs": {"emf": 12.0, "r": 0.5, "R": 5.5},
                      "outputs": {"I": 2.0, "V_t": 11.0, "V_lost": 1.0, "P_load": 22.0, "efficiency": 11 / 12}},
                     {"inputs": {"emf": 9.0, "r": 1.0, "R": 0.0}, "outputs": {"I": 9.0, "V_t": 0.0, "V_lost": 9.0, "P_load": 0.0}}],
        "questions": [{"q": "Terminal voltage: emf = 12 V, r = 0.5 ohm, R = 5.5 ohm", "outputs": {"I": 2.0, "V_t": 11.0}},
                      {"q": "Terminal voltage: emf = 12 V, r = 0 ohm, R = 0 ohm", "outcome": "INVALID_INPUT"},
                      {"q": "Terminal voltage: emf = 12 V, r = 0.5 ohm", "outcome": "NEEDS_INPUT"}]},
    # ------------------------------------------------------------------------------------------ friction
    "static_friction_limit": {
        "name": "Static friction: maximum friction force and whether a body slips", "topic": "forces / static friction",
        "purpose": "The largest static friction force mu_s N and whether an applied force parallel to the surface makes "
                   "the body slip.",
        "inputs": [_phys("mu_s", "friction_coefficient", "coefficient of static friction"), _phys("N", "force", "normal force"),
                   _phys("F", "force", "applied force parallel to the surface (magnitude)")],
        "outputs": [_o("f_max", "force", "maximum static friction force"), _o("slips", "boolean", "whether the body slips"),
                    _o("margin", "force", "f_max - F (positive: it holds)")],
        "primary": "f_max", "conditions": ["mu_s >= 0", "N > 0", "F >= 0"],
        "method": "f_max = mu_s N; it slips when F > f_max (a force equal to the limit holds it on the verge)",
        "latex": r"f_s \le \mu_s N",
        "run": static_friction_limit, "verify": verify_static_limit, "reference": ref_static_limit,
        "parse": lambda q: _pairs(q, r"static friction(?: limit)?", {"mu_s": "mu_s", "mu": "mu_s", "n": "N", "f": "F"},
                                  phys=("N", "F")),
        "wrong": _wrong_static_limit,
        "random_inputs": _rand_static_limit,
        "templates": ["Static friction: mu_s = 0.5, N = 100 N, F = 40 N"],
        "examples": [{"inputs": {"mu_s": 0.5, "N": 100.0, "F": 40.0}, "outputs": {"f_max": 50.0, "slips": False, "margin": 10.0}},
                     {"inputs": {"mu_s": 0.3, "N": 200.0, "F": 75.0}, "outputs": {"f_max": 60.0, "slips": True, "margin": -15.0}}],
        "questions": [{"q": "Static friction: mu_s = 0.5, N = 100 N, F = 40 N", "outputs": {"f_max": 50.0, "slips": False}},
                      {"q": "Static friction: mu_s = 0.3, N = 200 N, F = 75 N", "outputs": {"slips": True}},
                      {"q": "Static friction: mu_s = -0.2, N = 100 N, F = 40 N", "outcome": "INVALID_INPUT"},
                      {"q": "Static friction: mu_s = 0.5, N = 100 N", "outcome": "NEEDS_INPUT"}]},
    "static_friction_incline": {
        "name": "Static friction on an incline: critical angle and whether a body slides", "topic": "forces / static friction",
        "purpose": "The steepest incline on which a body can rest (tan(theta_c) = mu_s), whether it slides at the given "
                   "angle and, with its mass, the normal force, the friction needed and the friction available.",
        "inputs": [_phys("mu_s", "friction_coefficient", "coefficient of static friction"),
                   _phys("theta", "angle", "angle of the incline above the horizontal"),
                   _phys("m", "mass", "mass of the body, if the forces are wanted", required=False)],
        "outputs": [_o("theta_c", "angle", "critical (steepest holding) angle"), _o("theta_c_deg", "number", "critical angle in degrees"),
                    _o("slips", "boolean", "whether the body slides at the given angle"),
                    _o("N", "force", "normal force (when m is given)"), _o("f_needed", "force", "friction needed to hold it (when m is given)"),
                    _o("f_max", "force", "maximum static friction available (when m is given)")],
        "primary": "theta_c", "conditions": ["mu_s >= 0", "0 <= theta < 90 degrees", "g = 9.80665 m/s^2"],
        "method": "theta_c = atan(mu_s); slides when tan(theta) > mu_s; N = m g cos(theta), f_needed = m g sin(theta), f_max = mu_s N",
        "latex": r"\tan\theta_c = \mu_s",
        "run": static_friction_incline, "verify": verify_incline, "reference": ref_incline,
        "parse": lambda q: _pairs(q, r"static friction on an? (?:incline|slope|ramp)", {"mu_s": "mu_s", "mu": "mu_s", "theta": "theta", "m": "m"},
                                  phys=("theta", "m")),
        "wrong": _wrong_incline,
        "random_inputs": _rand_incline,
        "templates": ["Static friction on an incline: mu_s = 0.5, theta = 20 deg"],
        "examples": [{"inputs": {"mu_s": 0.5, "theta": math.radians(20)},
                      "outputs": {"theta_c": 0.4636476090008061, "theta_c_deg": 26.56505117707799, "slips": False}},
                     {"inputs": {"mu_s": 0.3, "theta": math.pi / 6, "m": 10.0},
                      "outputs": {"theta_c": 0.2914567944778671, "slips": True, "N": 98.0665 * math.cos(math.pi / 6),
                                  "f_needed": 98.0665 * 0.5, "f_max": 0.3 * 98.0665 * math.cos(math.pi / 6)}}],
        "questions": [{"q": "Static friction on an incline: mu_s = 0.5, theta = 20 deg", "outputs": {"theta_c_deg": 26.56505117707799, "slips": False}},
                      {"q": "Static friction on an incline: mu_s = 0.3, theta = 30 deg, m = 10 kg", "outputs": {"slips": True, "f_needed": 49.03325}},
                      {"q": "Static friction on an incline: mu_s = 0.5, theta = 95 deg", "outcome": "INVALID_INPUT"},
                      {"q": "Static friction on an incline: mu_s = 0.5", "outcome": "NEEDS_INPUT"}]},
    # ------------------------------------------------------------------------------------------ elasticity
    "youngs_modulus_deformation": {
        "name": "Tension or compression: stress, strain and change of length (Young's modulus)", "topic": "elasticity / Young's modulus",
        "purpose": "Stress, strain and change of length of a rod or wire of original length L0 and cross-section A "
                   "under an axial force F.",
        "inputs": [_phys("F", "force", "axial force (positive: tension, negative: compression)"), _phys("A", "area", "cross-sectional area"),
                   _phys("L0", "length", "original length"), _phys("E", "pressure", "Young's modulus")],
        "outputs": [_o("dL", "length", "change of length (positive: extension)"),
                    _o("stress", "pressure", "tensile stress F / A"), _o("strain", "number", "strain dL / L0")],
        "primary": "dL", "conditions": ["A, L0, E > 0", "linear elastic (|strain| < 1)"],
        "method": "stress = F/A; strain = stress/E; dL = strain L0", "latex": r"\Delta L = \frac{F L_0}{A E}",
        "run": young_deformation, "verify": verify_young, "reference": ref_young,
        "parse": lambda q: _pairs(q, r"young" + _APOS + r" modulus(?: deformation)?|tensile deformation",
                                  {"f": "F", "a": "A", "l0": "L0", "e": "E"}, phys=("F", "A", "L0", "E")),
        "wrong": lambda F, A, L0, E: (lambda o: {**o, "dL": F / (A * E)})(young_deformation(F, A, L0, E)),
        "random_inputs": _rand_young,
        "templates": ["Young's modulus deformation: F = 1000 N, A = 1 mm^2, L0 = 2 m, E = 200 GPa"],
        "examples": [{"inputs": {"F": 1000.0, "A": 1e-6, "L0": 2.0, "E": 2e11}, "outputs": {"stress": 1e9, "strain": 0.005, "dL": 0.01}},
                     {"inputs": {"F": -5e4, "A": 0.01, "L0": 3.0, "E": 2e10}, "outputs": {"stress": -5e6, "strain": -2.5e-4, "dL": -7.5e-4}}],
        "questions": [{"q": "Young's modulus deformation: F = 1000 N, A = 1 mm^2, L0 = 2 m, E = 200 GPa", "outputs": {"dL": 0.01}},
                      {"q": "Young's modulus deformation: F = 1000 N, A = 1 mm^2, L0 = 2 m, E = 0 GPa", "outcome": "INVALID_INPUT"},
                      {"q": "Young's modulus deformation: F = 1000 N, A = 1 mm^2, L0 = 2 m", "outcome": "NEEDS_INPUT"}]},
    "shear_modulus_deformation": {
        "name": "Shear: shear stress, shear strain and displacement (shear modulus)", "topic": "elasticity / shear modulus",
        "purpose": "Shear stress, shear strain and the sideways displacement of one face of a block of height h when a "
                   "force F acts parallel to a face of area A.",
        "inputs": [_phys("F", "force", "force parallel to the face"), _phys("A", "area", "area of the face the force acts on"),
                   _phys("h", "length", "distance between the sheared faces"), _phys("S", "pressure", "shear modulus")],
        "outputs": [_o("dx", "length", "sideways displacement of the face"),
                    _o("stress", "pressure", "shear stress F / A"), _o("strain", "number", "shear strain dx / h")],
        "primary": "dx", "conditions": ["A, h, S > 0", "linear elastic (|strain| < 1)"],
        "method": "stress = F/A; strain = stress/S; dx = strain h", "latex": r"\Delta x = \frac{F h}{A S}",
        "run": shear_deformation, "verify": verify_shear, "reference": ref_shear,
        "parse": lambda q: _pairs(q, r"shear modulus(?: deformation)?|shear deformation",
                                  {"f": "F", "a": "A", "h": "h", "s": "S", "g": "S"}, phys=("F", "A", "h", "S")),
        "wrong": lambda F, A, h, S: (lambda o: {**o, "dx": F / (A * S)})(shear_deformation(F, A, h, S)),
        "random_inputs": _rand_shear,
        "templates": ["Shear modulus deformation: F = 40 kN, A = 20 cm^2, h = 10 cm, S = 25 GPa"],
        "examples": [{"inputs": {"F": 4e4, "A": 0.002, "h": 0.1, "S": 2.5e10}, "outputs": {"stress": 2e7, "strain": 8e-4, "dx": 8e-5}},
                     {"inputs": {"F": 2000.0, "A": 0.01, "h": 0.05, "S": 8e10}, "outputs": {"stress": 2e5, "strain": 2.5e-6, "dx": 1.25e-7}}],
        "questions": [{"q": "Shear modulus deformation: F = 40 kN, A = 20 cm^2, h = 10 cm, S = 25 GPa", "outputs": {"dx": 8e-5}},
                      {"q": "Shear modulus deformation: F = 40 kN, A = 0 cm^2, h = 10 cm, S = 25 GPa", "outcome": "INVALID_INPUT"},
                      {"q": "Shear modulus deformation: F = 40 kN, A = 20 cm^2, h = 10 cm", "outcome": "NEEDS_INPUT"}]},
    "bulk_modulus_compression": {
        "name": "Bulk compression: volume change under a pressure change (bulk modulus)", "topic": "elasticity / bulk modulus",
        "purpose": "Volumetric strain, change of volume and final volume of a body of volume V0 when the pressure on it "
                   "rises by dP.",
        "inputs": [_phys("dP", "pressure", "increase of pressure (negative: decrease)"), _phys("V0", "volume", "original volume"),
                   _phys("B", "pressure", "bulk modulus")],
        "outputs": [_o("dV", "volume", "change of volume (negative: compression)"), _o("strain", "number", "volumetric strain dV / V0"),
                    _o("V", "volume", "final volume")],
        "primary": "dV", "conditions": ["V0, B > 0", "linear elastic (|dP / B| < 1)"],
        "method": "dV = -V0 dP / B", "latex": r"\Delta V = -\frac{V_0\,\Delta P}{B}",
        "run": bulk_compression, "verify": verify_bulk, "reference": ref_bulk,
        "parse": lambda q: _pairs(q, r"bulk modulus(?: compression)?", {"dp": "dP", "v0": "V0", "b": "B", "k": "B"},
                                  phys=("dP", "V0", "B")),
        "wrong": lambda dP, V0, B: {"dV": V0 * dP / B, "strain": dP / B, "V": V0 + V0 * dP / B},
        "random_inputs": _rand_bulk,
        "templates": ["Bulk modulus compression: dP = 20 MPa, V0 = 1 m^3, B = 2.2 GPa"],
        "examples": [{"inputs": {"dP": 2e7, "V0": 1.0, "B": 2.2e9}, "outputs": {"dV": -2e7 / 2.2e9, "V": 1.0 - 2e7 / 2.2e9}},
                     {"inputs": {"dP": 1e8, "V0": 0.002, "B": 1.6e11}, "outputs": {"dV": -1.25e-6, "strain": -6.25e-4}}],
        "questions": [{"q": "Bulk modulus compression: dP = 20 MPa, V0 = 1 m^3, B = 2.2 GPa", "outputs": {"dV": -2e7 / 2.2e9}},
                      {"q": "Bulk modulus compression: dP = 20 MPa, V0 = 1 m^3, B = 0 GPa", "outcome": "INVALID_INPUT"},
                      {"q": "Bulk modulus compression: dP = 20 MPa, V0 = 1 m^3", "outcome": "NEEDS_INPUT"}]},
    # ------------------------------------------------------------------------------------------ statics and systems
    "beam_support_reactions": {
        "name": "Static equilibrium of a beam on two supports: support reactions", "topic": "statics / equilibrium of a beam",
        "purpose": "The vertical reactions of the two supports of a beam carrying point loads, from the balance of "
                   "forces and of moments.",
        "inputs": [_vecin("loads", "point loads in newtons, downward positive (include the beam's weight at its centre)"),
                   _vecin("positions", "position of each load along the beam in metres"),
                   _phys("xA", "length", "position of support A"), _phys("xB", "length", "position of support B")],
        "outputs": [_o("RA", "force", "upward reaction of support A"), _o("RB", "force", "upward reaction of support B"),
                    _o("total", "force", "total load")],
        "primary": "RA", "conditions": ["one position per load", "xA != xB", "a rigid beam in equilibrium, vertical loads"],
        "method": "moments about A: RB (xB - xA) = sum F (x - xA); forces: RA = sum F - RB",
        "latex": r"\sum F = R_A + R_B,\quad R_B(x_B - x_A) = \sum_i F_i (x_i - x_A)",
        "run": beam_reactions, "verify": verify_beam, "reference": ref_beam,
        "parse": lambda q: _list_pairs(q, r"beam support reactions?|support reactions?(?: of a beam)?",
                                       {"loads": "loads", "positions": "positions", "xa": "xA", "xb": "xB"},
                                       ("loads", "positions"), {"loads": ("N",), "positions": ("m",)}, phys=("xA", "xB")),
        "wrong": lambda loads, positions, xA, xB: (lambda o: {"RA": o["RB"], "RB": o["RA"], "total": o["total"]})(
            beam_reactions(loads, positions, xA, xB)),
        "random_inputs": _rand_beam,
        "templates": ["Beam support reactions: loads = 200, 300 N; positions = 1, 3 m; xA = 0 m; xB = 4 m"],
        "examples": [{"inputs": {"loads": [200.0, 300.0], "positions": [1.0, 3.0], "xA": 0.0, "xB": 4.0},
                      "outputs": {"RA": 225.0, "RB": 275.0, "total": 500.0}},
                     {"inputs": {"loads": [500.0], "positions": [5.0], "xA": 0.0, "xB": 4.0}, "outputs": {"RA": -125.0, "RB": 625.0}}],
        "questions": [{"q": "Beam support reactions: loads = 200, 300 N; positions = 1, 3 m; xA = 0 m; xB = 4 m",
                       "outputs": {"RA": 225.0, "RB": 275.0}},
                      {"q": "Beam support reactions: loads = 200, 300 N; positions = 1, 3 m; xA = 2 m; xB = 2 m", "outcome": "INVALID_INPUT"},
                      {"q": "Beam support reactions: loads = 200, 300 N; positions = 1 m; xA = 0 m; xB = 4 m", "outcome": "INVALID_INPUT"},
                      {"q": "Beam support reactions: loads = 200, 300 N; positions = 1, 3 m; xA = 0 m", "outcome": "NEEDS_INPUT"}]},
    "centre_of_mass_point_masses": {
        "name": "Centre of mass of a set of point masses", "topic": "systems of particles / centre of mass",
        "purpose": "The coordinates of the centre of mass of point masses on a line or in a plane, and the total mass.",
        "inputs": [_vecin("masses", "masses in kilograms"), _vecin("x", "x coordinate of each mass in metres"),
                   _vecin("y", "y coordinate of each mass in metres (for masses in a plane)", required=False)],
        "outputs": [_o("x_cm", "length", "x coordinate of the centre of mass"), _o("y_cm", "length", "y coordinate of the centre of mass (when y is given)"),
                    _o("M", "mass", "total mass")],
        "primary": "x_cm", "conditions": ["every mass > 0", "one coordinate per mass"],
        "method": "x_cm = sum m x / sum m (and the same for y)", "latex": r"x_{cm} = \frac{\sum_i m_i x_i}{\sum_i m_i}",
        "run": centre_of_mass, "verify": verify_com, "reference": ref_com,
        "parse": lambda q: _list_pairs(q, r"cent(?:re|er) of mass", {"masses": "masses", "x": "x", "y": "y"}, ("masses", "x", "y"),
                                       {"masses": ("kg",), "x": ("m",), "y": ("m",)}),
        "wrong": lambda masses, x, y=None: (lambda o: {**o, "x_cm": sum(x) / len(x), **({"y_cm": sum(y) / len(y)} if y is not None else {})})(
            centre_of_mass(masses, x, y)),
        "random_inputs": _rand_com,
        "templates": ["Centre of mass: masses = 2, 3 kg; x = 0, 5 m"],
        "examples": [{"inputs": {"masses": [2.0, 3.0], "x": [0.0, 5.0]}, "outputs": {"x_cm": 3.0, "M": 5.0}},
                     {"inputs": {"masses": [1.0, 1.0, 2.0], "x": [0.0, 4.0, 0.0], "y": [0.0, 0.0, 3.0]}, "outputs": {"x_cm": 1.0, "y_cm": 1.5, "M": 4.0}}],
        "questions": [{"q": "Centre of mass: masses = 2, 3 kg; x = 0, 5 m", "outputs": {"x_cm": 3.0}},
                      {"q": "Center of mass: masses = 1, 1, 2 kg; x = 0, 4, 0 m; y = 0, 0, 3 m", "outputs": {"x_cm": 1.0, "y_cm": 1.5}},
                      {"q": "Centre of mass: masses = 2, -3 kg; x = 0, 5 m", "outcome": "INVALID_INPUT"},
                      {"q": "Centre of mass: masses = 2, 3 kg", "outcome": "NEEDS_INPUT"}]},
    "vector_components_resultant": {
        "name": "Vector components and the resultant of two-dimensional vectors", "topic": "vectors / two-dimensional vectors",
        "purpose": "The x and y components of each vector given by magnitude and direction, and the magnitude and "
                   "direction of their sum (forces, displacements, velocities: any one unit for all).",
        "inputs": [_vecin("magnitudes", "magnitude of each vector (all in one unit)"),
                   _vecin("directions", "direction of each vector in degrees, counterclockwise from +x")],
        "outputs": [_o("R", "number", "magnitude of the resultant"),
                    _o("direction_deg", "number", "direction of the resultant in degrees from +x (0 to 360)"),
                    _o("Rx", "number", "x component of the resultant"), _o("Ry", "number", "y component of the resultant"),
                    _o("x_components", "number", "x component of each vector"), _o("y_components", "number", "y component of each vector")],
        "primary": "R", "conditions": ["one direction per magnitude", "magnitudes >= 0"],
        "method": "components m cos(theta), m sin(theta); sums Rx, Ry; R = sqrt(Rx^2 + Ry^2); direction atan2(Ry, Rx)",
        "latex": r"R = \sqrt{R_x^2 + R_y^2},\quad \theta_R = \operatorname{atan2}(R_y, R_x)",
        "run": vector_resultant, "verify": verify_vectors, "reference": ref_vectors,
        "parse": lambda q: _list_pairs(q, r"vector resultant|resultant of vectors|vector components",
                                       {"magnitudes": "magnitudes", "directions": "directions", "angles": "directions"},
                                       ("magnitudes", "directions"), {"directions": ("deg", "°", "º", "degrees")}),
        "wrong": _wrong_vectors,
        "random_inputs": _rand_vectors,
        "templates": ["Vector resultant: magnitudes = 3, 4; directions = 0, 90 deg"],
        "examples": [{"inputs": {"magnitudes": [3.0, 4.0], "directions": [0.0, 90.0]},
                      "outputs": {"Rx": 3.0, "Ry": 4.0, "R": 5.0, "direction_deg": 53.13010235415598, "x_components": [3.0, 0.0], "y_components": [0.0, 4.0]}},
                     {"inputs": {"magnitudes": [10.0], "directions": [30.0]}, "outputs": {"Rx": 8.66025403784, "Ry": 5.0, "R": 10.0, "direction_deg": 30.0}},
                     {"inputs": {"magnitudes": [5.0, 5.0], "directions": [0.0, 180.0]}, "outputs": {"Rx": 0.0, "Ry": 0.0, "R": 0.0, "direction_deg": 0.0}}],
        "questions": [{"q": "Vector resultant: magnitudes = 3, 4; directions = 0, 90 deg", "outputs": {"R": 5.0, "direction_deg": 53.13010235415598}},
                      {"q": "Vector resultant: magnitudes = 3, -4; directions = 0, 90 deg", "outcome": "INVALID_INPUT"},
                      {"q": "Vector resultant: magnitudes = 3, 4; directions = 0 deg", "outcome": "INVALID_INPUT"},
                      {"q": "Vector resultant: magnitudes = 3, 4", "outcome": "NEEDS_INPUT"}]},
    # ------------------------------------------------------------------------------------------ rotation
    "rotational_second_law": {
        "name": "Newton's second law for rotation: angular acceleration from torque", "topic": "rotational dynamics / torque and angular acceleration",
        "purpose": "The angular acceleration of a rigid body about a fixed axis under a net torque.",
        "inputs": [_phys("tau", "torque", "net torque about the axis"),
                   _phys("I", "moment_of_inertia", "moment of inertia about the same axis", condition="I != 0")],
        "outputs": [_o("alpha", "angular_acceleration", "angular acceleration")],
        "primary": "alpha", "conditions": ["I > 0", "rigid body, fixed axis"],
        "method": "alpha = tau / I", "latex": r"\tau = I\alpha",
        "run": rotational_second_law, "verify": verify_rotation, "reference": ref_rotation,
        "parse": lambda q: _pairs(q, r"rotational second law|newton" + _APOS + r" second law for rotation",
                                  {"tau": "tau", "torque": "tau", "i": "I"}, phys=("tau", "I")),
        "wrong": lambda tau, I: {"alpha": tau * I},
        "random_inputs": _rand_rotation,
        "templates": ["Rotational second law: tau = 12 N*m, I = 3 kg*m^2"],
        "examples": [{"inputs": {"tau": 12.0, "I": 3.0}, "outputs": {"alpha": 4.0}},
                     {"inputs": {"tau": -2.0, "I": 0.5}, "outputs": {"alpha": -4.0}}],
        "questions": [{"q": "Rotational second law: tau = 12 N*m, I = 3 kg*m^2", "outputs": {"alpha": 4.0}},
                      {"q": "Rotational second law: tau = 12 N*m, I = 0 kg*m^2", "outcome": "INVALID_INPUT"},
                      {"q": "Rotational second law: tau = 12 N*m", "outcome": "NEEDS_INPUT"}]},
    "angular_momentum_conservation": {
        "name": "Conservation of angular momentum: change of spin rate", "topic": "rotational dynamics / conservation of angular momentum",
        "purpose": "The new angular velocity of a body whose moment of inertia changes with no external torque (a "
                   "spinning skater pulling in the arms), with its rotational kinetic energy before and after.",
        "inputs": [_phys("I1", "moment_of_inertia", "initial moment of inertia"), _phys("omega1", "angular_frequency", "initial angular velocity"),
                   _phys("I2", "moment_of_inertia", "final moment of inertia")],
        "outputs": [_o("omega2", "angular_frequency", "final angular velocity"), _o("L", "angular_momentum", "angular momentum (conserved)"),
                    _o("K1", "energy", "initial rotational kinetic energy"), _o("K2", "energy", "final rotational kinetic energy"),
                    _o("dK", "energy", "change of rotational kinetic energy")],
        "primary": "omega2", "conditions": ["I1, I2 > 0", "no external torque about the axis"],
        "method": "I1 omega1 = I2 omega2; K = I omega^2 / 2", "latex": r"I_1\omega_1 = I_2\omega_2",
        "run": angular_momentum_conservation, "verify": verify_spin, "reference": ref_spin,
        "parse": lambda q: _pairs(q, r"angular momentum conservation|conservation of angular momentum",
                                  {"i1": "I1", "omega1": "omega1", "w1": "omega1", "i2": "I2"}, phys=("I1", "omega1", "I2")),
        "wrong": _wrong_spin,
        "random_inputs": _rand_spin,
        "templates": ["Angular momentum conservation: I1 = 4 kg*m^2, omega1 = 2 rad/s, I2 = 1.6 kg*m^2"],
        "examples": [{"inputs": {"I1": 4.0, "omega1": 2.0, "I2": 1.6}, "outputs": {"omega2": 5.0, "L": 8.0, "K1": 8.0, "K2": 20.0, "dK": 12.0}},
                     {"inputs": {"I1": 2.0, "omega1": 6.0, "I2": 3.0}, "outputs": {"omega2": 4.0, "K1": 36.0, "K2": 24.0, "dK": -12.0}}],
        "questions": [{"q": "Angular momentum conservation: I1 = 4 kg*m^2, omega1 = 2 rad/s, I2 = 1.6 kg*m^2", "outputs": {"omega2": 5.0}},
                      {"q": "Angular momentum conservation: I1 = 4 kg*m^2, omega1 = 2 rad/s, I2 = 0 kg*m^2", "outcome": "INVALID_INPUT"},
                      {"q": "Angular momentum conservation: I1 = 4 kg*m^2, omega1 = 2 rad/s", "outcome": "NEEDS_INPUT"}]},
    # ------------------------------------------------------------------------------------------ sound and waves
    "sound_intensity_level": {
        "name": "Sound intensity level in decibels", "topic": "sound / intensity level",
        "purpose": "The intensity level in decibels of a sound of given intensity, or the intensity of a sound of given "
                   "level (reference: the threshold of hearing, 1e-12 W/m^2).",
        "inputs": [_phys("I", "intensity", "sound intensity, if known", required=False),
                   _numin("beta", "sound level in dB, if known", required=False)],
        "outputs": [_o("beta", "number", "intensity level in dB"), _o("I", "intensity", "sound intensity")],
        "primary": "beta", "conditions": ["exactly one of I and beta", "I > 0"],
        "method": "beta = 10 log10(I / I0); I = I0 10^(beta/10); I0 = 1e-12 W/m^2",
        "latex": r"\beta = 10\log_{10}\frac{I}{I_0}",
        "run": sound_intensity_level, "verify": verify_level, "reference": ref_level,
        "parse": lambda q: _pairs(q, r"sound intensity level", {"i": "I", "beta": "beta"}, phys=("I",), num_units={"beta": ("dB",)}),
        "wrong": _wrong_level,
        "random_inputs": _rand_level,
        "templates": ["Sound intensity level: I = 1e-5 W/m^2"],
        "examples": [{"inputs": {"I": 1e-5}, "outputs": {"beta": 70.0}},
                     {"inputs": {"beta": 90.0}, "outputs": {"I": 1e-3}},
                     {"inputs": {"I": 3.2e-6}, "outputs": {"beta": 10 * math.log10(3.2e6)}}],
        "questions": [{"q": "Sound intensity level: I = 1e-5 W/m^2", "outputs": {"beta": 70.0}},
                      {"q": "Sound intensity level: beta = 90 dB", "outputs": {"I": 1e-3}},
                      {"q": "Sound intensity level: I = 0 W/m^2", "outcome": "INVALID_INPUT"},
                      {"q": "Sound intensity level: I = 1e-5 W/m^2, beta = 70 dB", "outcome": "INVALID_INPUT"}]},
    "sound_level_change": {
        "name": "Change of sound level and the ratio of two intensities", "topic": "sound / intensity level",
        "purpose": "The ratio of the intensities of two sounds from their levels, or the change of level (and the new "
                   "level) for a given ratio of intensities.",
        "inputs": [_numin("beta1", "first sound level in dB", required=False), _numin("beta2", "second sound level in dB", required=False),
                   _numin("ratio", "intensity ratio I2 / I1, if given instead of the second level", required=False)],
        "outputs": [_o("d_beta", "number", "change of level beta2 - beta1 in dB"), _o("ratio", "number", "intensity ratio I2 / I1"),
                    _o("beta2", "number", "second level in dB (when the first is given)")],
        "primary": "d_beta", "conditions": ["both levels, or the ratio (with or without the first level)", "ratio > 0"],
        "method": "beta2 - beta1 = 10 log10(I2 / I1)", "latex": r"\beta_2 - \beta_1 = 10\log_{10}\frac{I_2}{I_1}",
        "run": sound_level_change, "verify": verify_level_change, "reference": ref_level_change,
        "parse": lambda q: _pairs(q, r"sound level (?:change|difference)", {"beta1": "beta1", "beta2": "beta2", "ratio": "ratio"},
                                  num_units={"beta1": ("dB",), "beta2": ("dB",)}),
        "wrong": _wrong_level_change,
        "random_inputs": _rand_level_change,
        "templates": ["Sound level change: beta1 = 60 dB, beta2 = 80 dB"],
        "examples": [{"inputs": {"beta1": 60.0, "beta2": 80.0}, "outputs": {"d_beta": 20.0, "ratio": 100.0, "beta2": 80.0}},
                     {"inputs": {"ratio": 100.0}, "outputs": {"d_beta": 20.0}},
                     {"inputs": {"beta1": 90.0, "beta2": 87.0}, "outputs": {"d_beta": -3.0, "ratio": 10 ** -0.3}}],
        "questions": [{"q": "Sound level change: beta1 = 60 dB, beta2 = 80 dB", "outputs": {"ratio": 100.0, "d_beta": 20.0}},
                      {"q": "Sound level change: beta1 = 60 dB, ratio = 2", "outputs": {"beta2": 60 + 10 * math.log10(2)}},
                      {"q": "Sound level change: ratio = 0", "outcome": "INVALID_INPUT"},
                      {"q": "Sound level change: beta1 = 60 dB", "outcome": "INVALID_INPUT"}]},
    "standing_wave_harmonics": {
        "name": "Standing waves: harmonics of a string and of open or closed pipes", "topic": "waves / standing waves and harmonics",
        "purpose": "The frequency and wavelength of the n-th harmonic, and the fundamental, of a string fixed at both "
                   "ends, a pipe open at both ends or a pipe closed at one end.",
        "inputs": [_choice("system", ["string", "open pipe", "closed pipe"], "string fixed at both ends, pipe open at both ends, or pipe closed at one end"),
                   _phys("L", "length", "length of the string or pipe"), _phys("v", "speed", "wave speed"),
                   _intin("n", "harmonic number (f_n = n f_1); odd only for a closed pipe", lo=1)],
        "outputs": [_o("f_n", "frequency", "frequency of the n-th harmonic"), _o("lam_n", "length", "wavelength of the n-th harmonic"),
                    _o("f1", "frequency", "fundamental frequency")],
        "primary": "f_n", "conditions": ["L, v > 0", "n >= 1; n odd for a pipe closed at one end", "no end correction"],
        "method": "string and open pipe: f_n = n v / (2L); closed pipe: f_n = n v / (4L), n odd; lambda_n = v / f_n",
        "latex": r"f_n = \frac{n v}{2L}\ \text{(string, open pipe)},\qquad f_n = \frac{n v}{4L},\ n\ \text{odd (closed pipe)}",
        "run": standing_wave_harmonic, "verify": verify_standing, "reference": ref_standing, "parse": parse_standing,
        "wrong": _wrong_standing,
        "random_inputs": _rand_standing,
        "templates": ["Standing wave harmonic, closed pipe: L = 25 cm, v = 340 m/s, n = 3"],
        "examples": [{"inputs": {"system": "string", "L": 0.65, "v": 286.0, "n": 1}, "outputs": {"f_n": 220.0, "lam_n": 1.3, "f1": 220.0}},
                     {"inputs": {"system": "open pipe", "L": 0.5, "v": 340.0, "n": 2}, "outputs": {"f_n": 680.0, "lam_n": 0.5, "f1": 340.0}},
                     {"inputs": {"system": "closed pipe", "L": 0.25, "v": 340.0, "n": 3}, "outputs": {"f_n": 1020.0, "lam_n": 1 / 3, "f1": 340.0}}],
        "questions": [{"q": "Standing wave harmonic, closed pipe: L = 25 cm, v = 340 m/s, n = 3", "outputs": {"f_n": 1020.0}},
                      {"q": "Standing wave harmonic, string: L = 65 cm, v = 286 m/s, n = 1", "outputs": {"f_n": 220.0}},
                      {"q": "Standing wave harmonic, closed pipe: L = 25 cm, v = 340 m/s, n = 2", "outcome": "INVALID_INPUT"},
                      {"q": "Standing wave harmonic, open pipe: L = 50 cm, n = 2", "outcome": "NEEDS_INPUT"}]},
    "string_wave_speed": {
        "name": "Wave speed on a stretched string", "topic": "waves / wave speed on a string",
        "purpose": "The speed of transverse waves on a string from its tension and its mass per unit length.",
        "inputs": [_phys("T", "force", "tension in the string"),
                   _phys("mu", "linear_mass_density", "mass per unit length of the string", condition="mu != 0")],
        "outputs": [_o("v", "speed", "speed of transverse waves")],
        "primary": "v", "conditions": ["T > 0, mu > 0", "a perfectly flexible string, small amplitudes"],
        "method": "v = sqrt(T / mu)", "latex": r"v = \sqrt{\frac{T}{\mu}}",
        "run": string_wave_speed, "verify": verify_string, "reference": ref_string,
        "parse": lambda q: _pairs(q, r"wave speed on a (?:stretched )?string|string wave speed", {"t": "T", "tension": "T", "mu": "mu"},
                                  phys=("T", "mu")),
        "wrong": lambda T, mu: {"v": T / mu},
        "random_inputs": _rand_string,
        "templates": ["Wave speed on a string: T = 100 N, mu = 0.01 kg/m"],
        "examples": [{"inputs": {"T": 100.0, "mu": 0.01}, "outputs": {"v": 100.0}},
                     {"inputs": {"T": 80.0, "mu": 0.005}, "outputs": {"v": 126.49110640673517}}],
        "questions": [{"q": "Wave speed on a string: T = 100 N, mu = 0.01 kg/m", "outputs": {"v": 100.0}},
                      {"q": "Wave speed on a string: T = 80 N, mu = 5 g/m", "outputs": {"v": 126.49110640673517}},
                      {"q": "Wave speed on a string: T = 100 N, mu = 0 kg/m", "outcome": "INVALID_INPUT"},
                      {"q": "Wave speed on a string: T = 100 N", "outcome": "NEEDS_INPUT"}]},
    # ------------------------------------------------------------------------------------------ Gauss's law (2026-11-12)
    "gauss_law_symmetric_field": {
        "name": "Gauss's law: electric field of a line, a plane, a charged sphere or a shell",
        "topic": "electrostatics / Gauss's law",
        "purpose": "The electric field of a symmetric charge distribution by Gauss's law: an infinite line (charge Q on a "
                   "length L), an infinite plane sheet (charge Q on an area A), a uniformly charged solid sphere or a thin "
                   "spherical shell of radius R, inside or outside; with the enclosed charge, the flux through the "
                   "Gaussian surface and the charge density.",
        "inputs": [_choice("geometry", list(_GAUSS), "line, plane, sphere (uniformly charged solid ball) or shell (thin shell, or a conducting sphere)"),
                   _phys("Q", "charge", "charge (with its sign): on the length L, the area A, or the whole sphere or shell"),
                   _phys("r", "length", "distance from the line, or from the centre (not needed for a plane)", required=False),
                   _phys("L", "length", "length of line that carries Q (line only; lambda = Q/L)", required=False),
                   _phys("A", "area", "area of plane that carries Q (plane only; sigma = Q/A)", required=False),
                   _phys("R", "length", "radius of the sphere or shell (sphere and shell only)", required=False)],
        "outputs": [_o("E", "electric_field", "magnitude of the electric field"),
                    _o("E_radial", "electric_field", "field component away from the line, plane or centre"),
                    _o("Q_enc", "charge", "charge enclosed by the Gaussian surface through the point"),
                    _o("flux", "number", "electric flux through the Gaussian surface, Q_enc/eps0 in N m^2/C"),
                    _o("density", "number", "charge density: lambda in C/m (line), sigma in C/m^2 (plane, shell), rho in C/m^3 (sphere)")],
        "primary": "E", "conditions": ["r > 0, L > 0, A > 0, R > 0", "line and plane treated as infinite",
                                       "uniform charge; on a shell (r = R) the field is refused", "charges in vacuum (or air)"],
        "method": "line E = lambda/(2 pi eps0 r); plane E = sigma/(2 eps0); sphere E = k Q r/R^3 inside, k Q/r^2 outside; "
                  "shell E = 0 inside, k Q/r^2 outside; flux = Q_enc/eps0",
        "latex": r"\oint \vec E\cdot d\vec A = \frac{Q_{enc}}{\varepsilon_0}:\quad E_{line} = \frac{\lambda}{2\pi\varepsilon_0 r},\ "
                 r"E_{plane} = \frac{\sigma}{2\varepsilon_0},\ E_{sphere} = \frac{kQr}{R^3}\ (r<R)",
        "run": gauss_field, "verify": verify_gauss, "reference": ref_gauss,
        "parse": lambda q: _pairs(q, r"gauss(?:['’]s)? law(?: field)?|electric field by gauss(?:['’]s)? law",
                                  {"geometry": "geometry", "q": "Q", "r": "r", "l": "L", "a": "A", "R": "R"},
                                  phys=("Q", "r", "L", "A", "R"), words=("geometry",)),
        "wrong": _wrong_gauss,
        "random_inputs": _rand_gauss,
        "templates": ["Gauss's law: geometry = line, Q = 2 uC, L = 1 m, r = 10 cm",
                      "Gauss's law: geometry = sphere, Q = 4 nC, R = 10 cm, r = 5 cm"],
        "examples": [{"inputs": {"geometry": "line", "Q": 2e-6, "L": 1.0, "r": 0.1},
                      "outputs": {"E": 2.0 * KE * 2e-6 / 0.1, "E_radial": 2.0 * KE * 2e-6 / 0.1, "Q_enc": 2e-6,
                                  "flux": 4.0 * math.pi * KE * 2e-6, "density": 2e-6}},
                     {"inputs": {"geometry": "plane", "Q": 5e-9, "A": 0.01},
                      "outputs": {"E": 2.0 * math.pi * KE * 5e-9 / 0.01, "Q_enc": 5e-9, "density": 5e-7}},
                     {"inputs": {"geometry": "sphere", "Q": 4e-9, "R": 0.1, "r": 0.05},
                      "outputs": {"E": 1797.51035846, "Q_enc": 5e-10, "flux": 4.0 * math.pi * KE * 5e-10}},
                     {"inputs": {"geometry": "shell", "Q": -3e-9, "R": 0.2, "r": 0.4},
                      "outputs": {"E": 168.516596106, "E_radial": -168.516596106, "Q_enc": -3e-9}}],
        "questions": [{"q": "Gauss's law: geometry = line, Q = 2 uC, L = 1 m, r = 10 cm", "outputs": {"E": 2.0 * KE * 2e-6 / 0.1}},
                      {"q": "Gauss's law: geometry = plane, Q = 5 nC, A = 0.01 m^2", "outputs": {"E": 2.0 * math.pi * KE * 5e-9 / 0.01}},
                      {"q": "Gauss's law: geometry = sphere, Q = 4 nC, R = 10 cm, r = 5 cm", "outputs": {"E": 1797.51035846}},
                      {"q": "Gauss law: geometry = shell, Q = -3 nC, R = 20 cm, r = 10 cm", "outputs": {"E": 0.0, "Q_enc": 0.0}},
                      {"q": "Gauss's law: geometry = shell, Q = -3 nC, R = 20 cm, r = 40 cm", "outputs": {"E_radial": -168.516596106}},
                      {"q": "Gauss's law: geometry = shell, Q = 3 nC, R = 20 cm, r = 20 cm", "outcome": "INVALID_INPUT"},
                      {"q": "Gauss's law: geometry = line, Q = 2 uC, L = 1 m, A = 1 m^2, r = 10 cm", "outcome": "INVALID_INPUT"},
                      {"q": "Gauss's law: geometry = line, L = 1 m, r = 10 cm", "outcome": "NEEDS_INPUT"}]},
}
