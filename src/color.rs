pub struct Rgb {
    r: u8,
    g: u8,
    b: u8
}

pub struct Hsl {
    pub h: u16,
    pub s: u8,
    pub l: u8
}

impl Rgb {
    pub fn as_hex(&self) -> String {
        format!("#{:x}{:x}{:x}", self.r, self.g, self.b)
    } 
}

impl Hsl {
    pub fn as_rgb(&self) -> Rgb {
        // add special case later for 0-saturation

        let h = self.h as f32 / 360.0;
        let s = self.s as f32 / 100.0;
        let l = self.l as f32 / 100.0;

        let q = if l < 0.5 { l * (1.0 + s) } else { l + s - l * s };
        let p = 2.0 * l - q;

        let r = hue_to_rgb(p, q, h + 1.0 / 3.0);
        let g = hue_to_rgb(p, q, h);
        let b = hue_to_rgb(p, q, h - 1.0 / 3.0);

        Rgb {
            r: (r * 255.0).round() as u8,
            g: (g * 255.0).round() as u8,
            b: (b * 255.0).round() as u8
        }
    }
}

fn hue_to_rgb(p: f32, q: f32, t: f32) -> f32 {
    let t = if t < 0.0 { t + 1.0 } else { t };
    let t = if t > 1.0 { t - 1.0 } else { t };

    match t {
        _ if t < 1.0 / 6.0 => p + (q - p) * 6.0 * t,
        _ if t < 1.0 / 2.0 => q,
        _ if t < 2.0 / 3.0 => p + (q - p) * (2.0 / 3.0 - t) * 6.0,
        _                  => p
    }
}
