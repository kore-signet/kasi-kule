//! generates an sRGB -> linear lookup table
//! it's very cursed

use kasi_kule::utils::linearize_channel;

fn float_to_bits(v: f32) -> u32 {
    v.to_bits().to_be()
}

fn main() {
    let mut vals: Vec<String> = Vec::new();

    for i in 0..=255 {
        vals.push(format!(
            "float_from_bits({})",
            float_to_bits(linearize_channel(i))
        ));
    }

    println!(
        "pub const sRGB_LOOKUP: [f32; 256] = unsafe {{ [{}] }};",
        vals.join(",")
    );
}
