use colorgrad::Gradient;
use image::{ImageBuffer, Rgba, RgbaImage};
use imageproc::drawing::{draw_text_mut, text_size};
use rusttype::{Font, Scale};
use noise::NoiseFn;
use rand::Rng;
use crate::utils::remap;

mod utils;


#[allow(dead_code)]
struct Avatar {
    width: i32,
    height: i32
}

#[allow(dead_code)]
impl Avatar {
    fn new(width: i32, height: i32) -> Self {
        Avatar {
            width, height
        }
    }

    #[allow(dead_code)]
    fn gradient(&self, noise_scale: f64, gradient: Gradient, text_rgba: [u8; 4], name_string: Option<String>) -> RgbaImage {
        let noise = noise::OpenSimplex::new(rand::thread_rng().gen_range(0..4294967295));

        let mut image_buf: RgbaImage = ImageBuffer::new(
            u32::try_from(self.width).unwrap(),
            u32::try_from(self.height).unwrap()
        );

        for (x, y, pixel) in image_buf.enumerate_pixels_mut() {
            let noise_t = noise.get([x as f64 * noise_scale, y as f64 * noise_scale]);
            let rgba = gradient.at(remap(noise_t, -1.0, 1.0, 0.0, 1.0)).to_rgba8();
            *pixel = Rgba(rgba);
        }

        match name_string {
            Some(name) => {
                let font = Vec::from(include_bytes!("../Roboto.ttf") as &[u8]);
                let font = Font::try_from_vec(font).unwrap();

                let font_scale = Scale {
                    x: self.width as f32 / 2.0,
                    y: self.height as f32 / 2.0
                };

                let (text_width, text_height) = text_size(font_scale, &font, &name);

                draw_text_mut(
                    &mut image_buf,
                    Rgba(text_rgba),
                    self.width / 2 - text_width / 2,
                    self.height / 2 - text_height / 2 - 6,
                    font_scale, &font, 
                    &name
                )
            },
            None => ()
        }

        image_buf
    }



}


#[cfg(test)]
mod tests {
    use crate::{Avatar};

    use std::fs;
    use chrono::{DateTime, Utc};
    use colorgrad::CustomGradient;

    #[test]
    fn main() {
        //
        let _ = fs::create_dir_all("test_results/gradient");
        let _ = fs::create_dir_all("test_results/gradient_text");

        let timestamp = DateTime::timestamp_millis(&Utc::now());

        
        Avatar::new(200, 200)
            .gradient(
                0.0025,
                colorgrad::reds(),
                [255, 255, 255, 255],
                None
            )
            .save(format!("test_results/gradient/{timestamp}.webp")).unwrap();

        Avatar::new(200, 200)
            .gradient(
                0.0025,
                CustomGradient::new().html_colors(&["deeppink", "cyan"]).build().unwrap(),
                [255, 255, 255, 255],
                Option::Some("JLN".to_string())
            )
            .save(format!("test_results/gradient_text/{timestamp}.webp")).unwrap();
    }
}

