// all the easing equations are from http://robertpenner.com/easing/

use std::f64::consts::PI;
use libm::*;

#[derive(Debug, Clone, Copy)]
pub struct Linear;

#[derive(Debug, Clone, Copy)]
pub struct Sine;

#[derive(Debug, Clone, Copy)]
pub struct Quintuple;

#[derive(Debug, Clone, Copy)]
pub struct Quartic;

#[derive(Debug, Clone, Copy)]
pub struct Quadratic;

#[derive(Debug, Clone, Copy)]
pub struct Exponential;

#[derive(Debug, Clone, Copy)]
pub struct Elastic;

#[derive(Debug, Clone, Copy)]
pub struct Cubic;

#[derive(Debug, Clone, Copy)]
pub struct Circular;

#[derive(Debug, Clone, Copy)]
pub struct Bounce;

#[derive(Debug, Clone, Copy)]
pub struct Back;

#[derive(Debug, Clone, Copy)]
pub struct Spring;

// t: current time, b: begInnIng value, c: change In value, d: duration

impl Linear {
    pub fn ease_in(t: f64, b: f64, c: f64, d: f64) -> f64 {
        c * t / d + b
    }
}

impl Sine {
    pub fn ease_in(t: f64, b: f64, c: f64, d: f64) -> f64 {
        -c * cos(t / d * (PI / 2.0)) + c + b
    }

    pub fn ease_out(t: f64, b: f64, c: f64, d: f64) -> f64 {
        c * sin(t / d * (PI / 2.0)) + b
    }

    pub fn ease_in_out(t: f64, b: f64, c: f64, d: f64) -> f64 {
        -c / 2.0 * (cos(PI * t / d) - 1.0) + b
    }

    pub fn ease_out_in(t: f64, b: f64, c: f64, d: f64) -> f64 {
        let h = c / 2.0;
        if t < d / 2.0 {
            Sine::ease_out(t * 2.0, b, h, d)
        } else {
            //                   0           0.5      0.5 2
            Sine::ease_in((t * 2.0) - d, b + h, h, d)
        }
    }
}

impl Quintuple {
    pub fn ease_in(t: f64, b: f64, c: f64, d: f64) -> f64 {
        c * pow(t / d, 5.0) + b
    }

    pub fn ease_out(t: f64, b: f64, c: f64, d: f64) -> f64 {
        c * (pow(t / d - 1.0, 5.0) + 1.0) + b
    }

    pub fn ease_in_out(t: f64, b: f64, c: f64, d: f64) -> f64 {
        let t = t / d * 2.0;

        if t < 1.0 {
            c / 2.0 * pow(t, 5.0) + b
        } else {
            c / 2.0 * (pow(t - 2.0, 5.0) + 2.0) + b
        }
    }

    pub fn ease_out_in(t: f64, b: f64, c: f64, d: f64) -> f64 {
        if t < d / 2.0 {
            Quintuple::ease_out(t * 2.0, b, c / 2.0, d)
        } else {
            Quintuple::ease_in((t * 2.0) - d, b + c / 2.0, c / 2.0, d)
        }
    }
}

impl Quartic {
    pub fn ease_in(t: f64, b: f64, c: f64, d: f64) -> f64 {
        c * pow(t / d, 4.0) + b
    }

    pub fn ease_out(t: f64, b: f64, c: f64, d: f64) -> f64 {
        -c * (pow(t / d - 1.0, 4.0) - 1.0) + b
    }

    pub fn ease_in_out(t: f64, b: f64, c: f64, d: f64) -> f64 {
        let t = t / d * 2.0;

        if t < 1.0 {
            c / 2.0 * pow(t, 4.0) + b
        } else {
            -c / 2.0 * (pow(t - 2.0, 4.0) - 2.0) + b
        }
    }

    pub fn ease_out_in(t: f64, b: f64, c: f64, d: f64) -> f64 {
        if t < d / 2.0 {
            Quartic::ease_out(t * 2.0, b, c / 2.0, d)
        } else {
            Quartic::ease_in((t * 2.0) - d, b + c / 2.0, c / 2.0, d)
        }
    }
}

impl Quadratic {
    pub fn ease_in(t: f64, b: f64, c: f64, d: f64) -> f64 {
        c * pow(t / d, 2.0) + b
    }

    pub fn ease_out(t: f64, b: f64, c: f64, d: f64) -> f64 {
        -c * (t / d) * (t / d - 2.0) + b
    }

    pub fn ease_in_out(t: f64, b: f64, c: f64, d: f64) -> f64 {
        let t = t / d * 2.0;

        if t < 1.0 {
            c / 2.0 * t * t + b
        } else {
            -c / 2.0 * ((t - 1.0) * (t - 3.0) - 1.0) + b
        }
    }

    pub fn ease_out_in(t: f64, b: f64, c: f64, d: f64) -> f64 {
        if t < d / 2.0 {
            Quadratic::ease_out(t * 2.0, b, c / 2.0, d)
        } else {
            Quadratic::ease_in((t * 2.0) - d, b + c / 2.0, c / 2.0, d)
        }
    }
}

impl Exponential {
    pub fn ease_in(t: f64, b: f64, c: f64, d: f64) -> f64 {
        if t == 0.0 {
            b
        } else {
            c * pow(2.0, 10.0 * (t / d - 1.0)) + b - c * 0.001
        }
    }

    pub fn ease_out(t: f64, b: f64, c: f64, d: f64) -> f64 {
        if t == d {
            b + c
        } else {
            c * 1.001 * (-pow(2.0, -10.0 * t / d) + 1.0) + b
        }
    }

    pub fn ease_in_out(t: f64, b: f64, c: f64, d: f64) -> f64 {
        if t == 0.0 {
            b
        } else if t == d {
            b + c
        } else {
            let t = t / d * 2.0;

            if t < 1.0 {
                c / 2.0 * pow(2.0, 10.0 * (t - 1.0)) + b - c * 0.0005
            } else {
                c / 2.0 * 1.0005 * (-pow(2.0, -10.0 * (t - 1.0)) + 2.0) + b
            }
        }
    }

    pub fn ease_out_in(t: f64, b: f64, c: f64, d: f64) -> f64 {
        if t < d / 2.0 {
            Exponential::ease_out(t * 2.0, b, c / 2.0, d)
        } else {
            Exponential::ease_in((t * 2.0) - d, b + c / 2.0, c / 2.0, d)
        }
    }
}

impl Elastic{
    pub fn ease_in(t: f64, b: f64, c: f64, d: f64) -> f64 {
        if t == 0.0 {
            b
        } else if t == d {
            b + c
        } else {
            let t = (t / d) - 1.0;
            let p = d * 0.3;
            let a = c * pow(2.0, 10.0 * t);
            let s = p / 4.0;

            -(a * sin((t * d - s) * (2.0 * PI) / p)) + b
        }
    }

    pub fn ease_out(t: f64, b: f64, c: f64, d: f64) -> f64 {
        if t == 0.0 {
            b
        } else if t == d {
            b + c
        } else {
            let t = t / d;
            let p = d * 0.3;
            let s = p / 4.0;

            c * pow(2.0, -10.0 * t) * sin((t * d - s) * (2.0 * PI) / p) + c + b
        }
    }

    pub fn ease_in_out(t: f64, b: f64, c: f64, d: f64) -> f64 {
        if t == 0.0 {
            return b;
        }

        let mut t = t / (d / 2.0);
        if t == 2.0 {
            return b + c;
        }

        let p = d * (0.3 * 1.5);
        let a = c;
        let s = p / 4.0;

        t = t - 1.0;
        if t < 0.0 {
            -0.5 * (a * pow(2.0, 10.0 * t) * sin((t * d - s) * (2.0 * PI) / p)) + b
        } else {
            a * pow(2.0, -10.0 * t) * sin((t * d - s) * (2.0 * PI) / p) * 0.5 + c + b
        }
    }

    pub fn ease_out_in(t: f64, b: f64, c: f64, d: f64) -> f64 {
        if t < d / 2.0 {
            Elastic::ease_out(t * 2.0, b, c / 2.0, d)
        } else {
            Elastic::ease_in((t * 2.0) - d, b + c / 2.0, c / 2.0, d)
        }
    }
}

impl Cubic {
    pub fn ease_in(t: f64, b: f64, c: f64, d: f64) -> f64 {
        let t = t / d;
        c * t * t * t + b
    }

    pub fn ease_out(t: f64, b: f64, c: f64, d: f64) -> f64 {
        let t = t / d - 1.0;
        c * (t * t * t + 1.0) + b
    }

    pub fn ease_in_out(t: f64, b: f64, c: f64, d: f64) -> f64 {
        let t = t / (d / 2.0);

        if t < 1.0 {
            c / 2.0 * t * t * t + b
        } else {
            let t = t - 2.0;
            c / 2.0 * (t * t * t + 2.0) + b
        }
    }

    pub fn ease_out_in(t: f64, b: f64, c: f64, d: f64) -> f64 {
        if t < d / 2.0 {
            Cubic::ease_out(t * 2.0, b, c / 2.0, d)
        } else {
            Cubic::ease_in((t * 2.0) - d, b + c / 2.0, c / 2.0, d)
        }
    }
}

impl Circular {
    pub fn ease_in(t: f64, b: f64, c: f64, d: f64) -> f64 {
        let t = t / d;
        -c * (sqrt(1.0 - t * t) - 1.0) + b
    }

    pub fn ease_out(t: f64, b: f64, c: f64, d: f64) -> f64 {
        let t = t / d - 1.0;
        c * sqrt(1.0 - t * t) + b
    }

    pub fn ease_in_out(t: f64, b: f64, c: f64, d: f64) -> f64 {
        let t = t / (d / 2.0);

        if t < 1.0 {
            -c / 2.0 * (sqrt(1.0 - t * t) - 1.0) + b
        } else {
            let t = t - 2.0;
            c / 2.0 * (sqrt(1.0 - t * t) + 1.0) + b
        }
    }

    pub fn ease_out_in(t: f64, b: f64, c: f64, d: f64) -> f64 {
        if t < d / 2.0 {
            Circular::ease_out(t * 2.0, b, c / 2.0, d)
        } else {
            Circular::ease_in((t * 2.0) - d, b + c / 2.0, c / 2.0, d)
        }
    }
}

impl Bounce {
    pub fn ease_out(t: f64, b: f64, c: f64, d: f64) -> f64 {
        let t = t / d;

        if t < 1.0 / 2.75 {
            c * (7.5625 * t * t) + b
        } else if t < 2.0 / 2.75 {
            let t = t - (1.5 / 2.75);
            c * (7.5625 * t * t + 0.75) + b
        } else if t < 2.5 / 2.75 {
            let t = t - (2.25 / 2.75);
            c * (7.5625 * t * t + 0.9375) + b
        } else {
            let t = t - (2.625 / 2.75);
            c * (7.5625 * t * t + 0.984375) + b
        }
    }

    pub fn ease_in(t: f64, b: f64, c: f64, d: f64) -> f64 {
        c - Bounce::ease_out(d - t, 0.0, c, d) + b
    }

    pub fn ease_in_out(t: f64, b: f64, c: f64, d: f64) -> f64 {
        let h = c / 2.0;
        if t < d / 2.0 {
            Bounce::ease_in(t * 2.0, b, h, d)
        } else {
            Bounce::ease_out(t * 2.0 - d, b + h, h, d)
        }
    }

    pub fn ease_out_in(t: f64, b: f64, c: f64, d: f64) -> f64 {
        let h = c / 2.0;
        if t < d / 2.0 {
            Bounce::ease_out(t * 2.0, b, h, d)
        } else {
            Bounce::ease_in(t * 2.0 - d, b + h, h, d)
        }
    }
}

impl Back {
    pub fn ease_in(t: f64, b: f64, c: f64, d: f64) -> f64 {
        let s = 1.70158;
        let t = t / d;
        c * t * t * ((s + 1.0) * t - s) + b
    }

    pub fn ease_out(t: f64, b: f64, c: f64, d: f64) -> f64 {
        let s = 1.70158;
        let t = t / d - 1.0;
        c * (t * t * ((s + 1.0) * t + s) + 1.0) + b
    }

    pub fn ease_in_out(t: f64, b: f64, c: f64, d: f64) -> f64 {
        let s = 1.70158 * 1.525;
        let t = t / (d / 2.0);

        if t < 1.0 {
            c / 2.0 * (t * t * ((s + 1.0) * t - s)) + b
        } else {
            let t = t - 2.0;
            c / 2.0 * (t * t * ((s + 1.0) * t + s) + 2.0) + b
        }
    }

    pub fn ease_out_in(t: f64, b: f64, c: f64, d: f64) -> f64 {
        let h = c / 2.0;
        if t < d / 2.0 {
            Back::ease_out(t * 2.0, b, h, d)
        } else {
            Back::ease_in(t * 2.0 - d, b + h, h, d)
        }
    }
}

impl Spring {
    pub fn ease_out(t: f64, b: f64, c: f64, d: f64) -> f64 {
        let t = t / d;
        let s = 1.0 - t;
        let t = (sin(t * PI * (0.2 + 2.5 * t * t * t)) * pow(s, 2.2) + t) * (1.0 + (1.2 * s));
        c * t + b
    }

    pub fn ease_in(t: f64, b: f64, c: f64, d: f64) -> f64 {
        c - Spring::ease_out(d - t, 0.0, c, d) + b
    }

    pub fn ease_in_out(t: f64, b: f64, c: f64, d: f64) -> f64 {
        let h = c / 2.0;
        if t < d / 2.0 {
            Spring::ease_in(t * 2.0, b, h, d)
        } else {
            Spring::ease_out(t * 2.0 - d, b + h, h, d)
        }
    }

    pub fn ease_out_in(t: f64, b: f64, c: f64, d: f64) -> f64 {
        let h = c / 2.0;
        if t < d / 2.0 {
            Spring::ease_out(t * 2.0, b, h, d)
        } else {
            Spring::ease_in(t * 2.0 - d, b + h, h, d)
        }
    }
}
