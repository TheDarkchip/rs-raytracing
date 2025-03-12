use std::io::Write;

use crate::vec3::Vector3;

pub type Color = Vector3;
pub fn write_color<W: Write>(out: &mut W, pixel_color: Color) -> std::io::Result<()> {
    let Vector3 { x: r, y: g, z: b } = pixel_color;

    let r_byte = (255.999 * r) as u8;
    let g_byte = (255.999 * g) as u8;
    let b_byte = (255.999 * b) as u8;

    writeln!(out, "{} {} {}", r_byte, g_byte, b_byte)?;
    Ok(())
}
