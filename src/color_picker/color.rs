#[derive(Clone)]
pub struct RGBA {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl Default for RGBA {
    fn default() -> Self {
        Self {
            red: Default::default(),
            green: Default::default(),
            blue: Default::default(),
            alpha: u8::MAX,
        }
    }
}

impl RGBA {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            red: r,
            green: g,
            blue: b,
            alpha: a,
        }
    }

    pub fn new_rgb(r: u8, g: u8, b: u8) -> Self {
        Self {
            red: r,
            green: g,
            blue: b,
            alpha: u8::MAX,
        }
    }

    pub fn to_hex_string(&self) -> String {
        if self.alpha == u8::MAX {
            format!("#{:02X}{:02X}{:02X}", self.red, self.green, self.blue)
        } else {
            format!(
                "#{:02X}{:02X}{:02X}{:02X}",
                self.red, self.green, self.blue, self.alpha
            )
        }
    }
}

impl From<HSV> for RGBA {
    fn from(value: HSV) -> Self {
        let HSV {
            hue: h,
            saturation: s,
            value: v,
            alpha,
        } = value;
        let h = f64::from(h);

        let c = v * s;
        let x = c * (1.0 - f64::abs(((h / 60.0) % 2.0) - 1.0));
        let m = v - c;

        let (r, g, b) = if (0.0..60.0).contains(&h) {
            (c, x, 0.0)
        } else if (60.0..120.0).contains(&h) {
            (x, c, 0.0)
        } else if (120.0..180.0).contains(&h) {
            (0.0, c, x)
        } else if (180.0..240.0).contains(&h) {
            (0.0, x, c)
        } else if (240.0..300.0).contains(&h) {
            (x, 0.0, c)
        } else if (300.0..360.0).contains(&h) {
            (c, 0.0, x)
        } else {
            (c, x, 0.0)
        };

        let (r, g, b) = (
            ((r + m) * 255.0) as u8,
            ((g + m) * 255.0) as u8,
            ((b + m) * 255.0) as u8,
        );

        RGBA::new(r, g, b, alpha)
    }
}

#[derive(Clone)]
pub struct HSV {
    pub hue: u16,
    pub saturation: f64,
    pub value: f64,
    pub alpha: u8,
}

impl HSV {
    pub fn new(hue: u16, saturation: f64, value: f64) -> Self {
        Self {
            hue,
            saturation,
            value,
            alpha: u8::MAX,
        }
    }

    pub fn new_alpha(hue: u16, saturation: f64, value: f64, alpha: u8) -> Self {
        Self {
            hue,
            saturation,
            value,
            alpha,
        }
    }
}

impl From<RGBA> for HSV {
    fn from(value: RGBA) -> Self {
        let RGBA {
            red: r,
            green: g,
            blue: b,
            alpha,
        } = value;

        let (r, g, b) = (r as f64 / 255.0, g as f64 / 255.0, b as f64 / 255.0);

        let c_max = f64::max(r, f64::max(g, b));
        let c_min = f64::min(r, f64::min(g, b));
        let delta = c_max - c_min;

        let hue = if delta == 0.0 {
            0.0
        } else if c_max == r {
            60.0 * (((g - b) / delta) % 6.0)
        } else if c_max == g {
            60.0 * (((b - r) / delta) + 2.0)
        } else if c_max == b {
            60.0 * (((r - g) / delta) + 4.0)
        } else {
            unreachable!()
        };

        let saturation = match c_max == 0.0 {
            true => 0.0,
            false => delta / c_max,
        };

        let value = c_max;

        HSV {
            hue: hue.to_string().parse().unwrap(),
            saturation,
            value,
            alpha,
        }
    }
}
