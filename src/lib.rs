extern crate murmur3;

use std::io::Cursor;
use wasm_bindgen::prelude::*;

fn str_as_cursor(string: &str) -> Cursor<&[u8]> {
    Cursor::new(string.as_bytes())
}

fn get_hash(s: &str) -> u32 {
    murmur3::murmur3_32(&mut str_as_cursor(&s), 0).unwrap()
}

fn divide_string(s: &str) -> [String; 3] {
    let len = s.len();
    let part_len = len / 3;

    let part1 = String::from(&s[..part_len]);
    let part2 = String::from(&s[part_len..2 * part_len]);
    let part3 = String::from(&s[2 * part_len..]);

    [part1, part2, part3]
}

fn hex_color(rgb: Vec<u32>) -> String {
    let hex_string: Vec<String> = rgb.iter().map(|part| format!("{:02x}", part)).collect();
    let hex_value = hex_string.join("");
    format!("#{}", hex_value)
}

fn rgb_to_hsl(rgb: (u8, u8, u8)) -> String {
    let (r, g, b) = (
        rgb.0 as f32 / 255.0,
        rgb.1 as f32 / 255.0,
        rgb.2 as f32 / 255.0,
    );

    let c_max = r.max(g).max(b);
    let c_min = r.min(g).min(b);
    let delta = c_max - c_min;

    let hue = if delta == 0.0 {
        0.0
    } else if c_max == r {
        60.0 * (((g - b) / delta).rem_euclid(6.0))
    } else if c_max == g {
        60.0 * (((b - r) / delta) + 2.0)
    } else {
        60.0 * (((r - g) / delta) + 4.0)
    };

    let lightness = (c_max + c_min) / 2.0;

    let saturation = if delta == 0.0 {
        0.0
    } else {
        delta / (1.0 - (2.0 * lightness - 1.0).abs())
    };

    format!(
        "hsl({}, {}%, {}%)",
        hue.trunc(),
        (saturation * 100.0).trunc(),
        (lightness * 100.0).trunc()
    )
}

fn rgb_to_cmyk(rgb: (u8, u8, u8)) -> String {
    let (r, g, b) = (
        rgb.0 as f32 / 255.0,
        rgb.1 as f32 / 255.0,
        rgb.2 as f32 / 255.0,
    );

    let black = 1.0 - r.max(g).max(b);
    let cyan = (1.0 - r - black) / (1.0 - black);
    let magenta = (1.0 - g - black) / (1.0 - black);
    let yellow = (1.0 - b - black) / (1.0 - black);

    format!(
        "cmyk({}%, {}%, {}%, {}%)",
        (cyan * 100.0).trunc(),
        (magenta * 100.0).trunc(),
        (yellow * 100.0).trunc(),
        (black * 100.0).trunc()
    )
}

fn rgb_to_hwb(rgb: (u8, u8, u8)) -> String {
    let (r, g, b) = (rgb.0 as f32, rgb.1 as f32, rgb.2 as f32);
    let whiteness = r.min(g).min(b);
    let v = r.max(g).max(b);
    let blackness = 1.0 - v / 255.0;
    let hue = if v == whiteness {
        0.0
    } else if v == r {
        (g - blackness) / (v - whiteness) % 6.0
    } else if v == g {
        (blackness - r) / (v - whiteness) + 2.0
    } else {
        (r - g) / (v - whiteness) + 4.0
    };
    let hue = hue * 60.0;
    let whiteness = whiteness / 255.0;

    format!(
        "hwb({}, {}%, {}%)",
        hue.trunc(),
        (whiteness * 100.0).trunc(),
        (blackness * 100.0).trunc()
    )
}

#[wasm_bindgen]
pub fn get_color(s: String, color_format: String) -> String {
    let rgb: Vec<u32> = divide_string(&s)
        .iter()
        .map(|part| (get_hash(part) % 255))
        .collect();
    if color_format == "rgb" {
        return format!("rgb({}, {}, {})", rgb[0], rgb[1], rgb[2]);
    }
    if color_format == "hsl" {
        return rgb_to_hsl((rgb[0] as u8, rgb[1] as u8, rgb[2] as u8));
    }
    if color_format == "hwb" {
        return rgb_to_hwb((rgb[0] as u8, rgb[1] as u8, rgb[2] as u8));
    }
    if color_format == "cmyk" {
        return rgb_to_cmyk((rgb[0] as u8, rgb[1] as u8, rgb[2] as u8));
    }
    hex_color(rgb)
}
