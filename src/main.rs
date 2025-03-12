use std::io::Write;

use indicatif::ProgressIterator;

mod vec3;

mod color;
use color::{Color, write_color};
use ray::Point3;
use ray::Ray;
use vec3::Vector3;

mod ray;

fn ray_color(r: Ray) -> Color {
    let unit_direction = r.direction.unit();
    let a = 0.5 * (unit_direction.y + 1.0);
    (1.0 - a) * Color::from((1.0, 1.0, 1.0)) + a * Color::from((0.5, 0.7, 1.0))
}

fn main() -> std::io::Result<()> {
    // IMAGE
    let aspect_ratio = 16.0 / 9.0;
    let img_width = 400;

    let img_height = (img_width as f64 / aspect_ratio) as i32;
    let img_height = if img_height < 1 { 1 } else { img_height };

    // CAMERA
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (img_width as f64 / img_height as f64);
    let camera_center = Point3::from((0.0, 0.0, 0.0));

    // Calculate the vectors across the horizontal and down the vertical viewport edges
    let viewport_u = Vector3::from((viewport_width, 0.0, 0.0));
    let viewport_v = Vector3::from((0.0, -viewport_height, 0.0));

    // Calculate the horizontal and vertical delta vectors from pixel to pixel
    let pixel_delta_u = viewport_u / img_width.into();
    let pixel_delta_v = viewport_v / img_height.into();

    // Calculate the location of the upper left pixel
    let viewport_upper_left = camera_center
        - Vector3::from((0.0, 0.0, focal_length))
        - viewport_u / 2.0
        - viewport_v / 2.0;

    let pixel_0_0_location = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // STDOUT STUFF
    let out = std::io::stdout();
    let mut handle = out.lock();

    // PPM Meta String
    writeln!(handle, "P3\n{} {}\n255", img_width, img_height)?;

    for j in (0..img_height).progress() {
        for i in 0..img_width {
            let pixel_center =
                pixel_0_0_location + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);

            let ray_direction = pixel_center - camera_center;
            let r = Ray {
                origin: camera_center,
                direction: ray_direction,
            };

            let pixel_color: Color = ray_color(r);

            write_color(&mut handle, pixel_color)?;
        }
    }

    Ok(())
}
