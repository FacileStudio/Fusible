//! IPC-2152 trace heating, as the Brooks and Adam fit used by KiCad:
//! `dT = K * I^a * W^b * H^c`, with `W` in mils, `H` in mils of copper and `I` in amps.
//!
//! Source: https://docs.kicad.org/doxygen/tracks__width__versus__current__formula_8h.html

/// Mils of copper thickness per ounce per square foot.
const MIL_PER_OZ: f64 = 1.378;
const MIL_PER_MM: f64 = 1.0 / 0.0254;

struct Fit {
    k: f64,
    a: f64,
    b: f64,
    c: f64,
}

const EXTERNAL: Fit = Fit {
    k: 215.3,
    a: 2.0,
    b: -1.15,
    c: -1.0,
};

const INTERNAL: [(f64, Fit); 4] = [
    (
        0.5,
        Fit {
            k: 120.0,
            a: 2.0,
            b: -1.10,
            c: -1.52,
        },
    ),
    (
        1.0,
        Fit {
            k: 200.0,
            a: 1.9,
            b: -1.10,
            c: -1.52,
        },
    ),
    (
        2.0,
        Fit {
            k: 300.0,
            a: 2.0,
            b: -1.15,
            c: -1.52,
        },
    ),
    (
        3.0,
        Fit {
            k: 262.5,
            a: 1.9,
            b: -1.15,
            c: -1.52,
        },
    ),
];

fn fit(internal: bool, oz: f64) -> &'static Fit {
    if !internal {
        return &EXTERNAL;
    }
    let nearest = INTERNAL
        .iter()
        .min_by(|x, y| (x.0 - oz).abs().total_cmp(&(y.0 - oz).abs()));
    nearest.map_or(&INTERNAL[0].1, |(_, fit)| fit)
}

/// Temperature rise in degrees Celsius of a trace `width_mm` wide carrying `amps`.
pub(super) fn rise(amps: f64, width_mm: f64, internal: bool, oz: f64) -> f64 {
    let Fit { k, a, b, c } = fit(internal, oz);
    k * amps.powf(*a) * (width_mm * MIL_PER_MM).powf(*b) * (oz * MIL_PER_OZ).powf(*c)
}

/// Narrowest width in millimetres, rounded up to 0.01 mm, that keeps the rise
/// within `max_rise`. `None` when even 20 mm is not enough.
pub(super) fn min_width(amps: f64, max_rise: f64, internal: bool, oz: f64) -> Option<f64> {
    let (mut low, mut high) = (0.01, 20.0);
    if rise(amps, high, internal, oz) > max_rise {
        return None;
    }
    for _ in 0..60 {
        let mid = (low + high) / 2.0;
        match rise(amps, mid, internal, oz) > max_rise {
            true => low = mid,
            false => high = mid,
        }
    }
    Some((high * 100.0).ceil() / 100.0)
}
