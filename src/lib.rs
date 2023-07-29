//! A Rust library for generating fast, high-quality profile avatars.
//! 
//! 
//! ## Example
//! 
//! ```
//! use amaryllis::Avatar;
//! 
//! // Simple
//! Avatar::new(200, 200, None, None).simple([255, 255, 255, 255])
//!     .save("simple_avatar.webp").unwrap();
//! 
//! // Simple with text
//! Avatar::new(
//!     200, 200,
//!     Option::Some("John Middlename Doe"),
//!     Option::Some([0, 0, 0, 255])
//! ).simple([255, 255, 255, 255]).save(
//!     "simple_avatar_text.webp"
//! ).unwrap();
//! 
//! // Gradient
//! Avatar::new(200, 200, None, None).gradient(0.0025, colorgrad::reds())
//!     .save("gradient_avatar.webp").unwrap();
//!
//! // Gradient with text
//! Avatar::new(
//!     200, 200,
//!     Option::Some("John Middlename Doe"),
//!     Option::Some([0, 0, 0, 255])
//! ).gradient(
//!     0.0025,
//!     CustomGradient::new().html_colors(&["deeppink", "cyan"]).build().unwrap()
//! )
//! .save("gradient_avatar_text.webp").unwrap();
//! ```


pub use colorgrad;
use image::{ImageBuffer, Rgba, RgbaImage};
use imageproc::drawing::{draw_text_mut, text_size};
use rusttype::{Font, Scale};
use noise::NoiseFn;
use rand::Rng;
use crate::utils::remap;

mod utils;

/// Struct representing an avatar
#[allow(dead_code)]
pub struct Avatar {
    /// Image width
    pub width: i32,
    pub height: i32,
    /// Initials for a name, if given
    pub initials: Option<String>,
    /// Array of four unsigned 8-bit integers, representing an RGBA value, if given
    pub text_color: Option<[u8; 4]>
}

#[allow(dead_code)]
impl Avatar {
    /// Returns a new Avatar object with specificied parameters
    /// 
    /// # Arguments
    /// 
    /// * `width` - Image width as a 32-bit integer
    /// * `height` - Image height as a 32-bit integer
    /// * `name_string` - The optional name or username of the owner of the avatar. If `None`, it will be ignored
    /// * `text_rgba` - An optional array of four unsigned 8-bit integers, representing an RGBA value. If `name_string` is `Some`, this must be `Some`
    pub fn new(width: i32, height: i32, name_string: Option<&str>, text_rgba: Option<[u8; 4]>) -> Self {
        return match name_string {
            Some(name) =>  {
                if text_rgba.is_none() {
                    return Avatar {
                        width,
                        height,
                        initials: None,
                        text_color: None
                    }
                } else {
                    let mut initials: String = "".to_string();

                    let name_split = &name.split(" ");
                    for word in name_split.clone() {
                        let pos = name_split.clone().position(|x| x == word).unwrap();
                        if pos == 0 || pos == name_split.clone().count() - 1 {
                            initials.push(word.chars().nth(0).unwrap())
                        }
                    }


                    Avatar {
                        width,
                        height,
                        initials: Option::Some(initials),
                        text_color: text_rgba
                    }
                }
            },
            None => {
                Avatar {
                    width,
                    height,
                    initials: None,
                    text_color: None
                }
            }
        };
    }

    /// Returns an RGBA Image (`RgbaImage`) of a solid color (and initials of the user, if given)
    /// 
    /// # Arguments
    /// 
    /// * `color_rgba` - An array of four unsigned 8-bit integers, representing an RGBA value.
    /// 
    /// # Example
    /// ```
    /// use amaryllis::Avatar;
    /// 
    /// let avatar = Avatar::new(200, 200, None, None).simple([255, 255, 255, 255]);
    /// 
    /// avatar.save("cool_avatar.png");
    /// ```
    #[allow(dead_code)]
    pub fn simple(&self, color_rgba: [u8; 4]) -> RgbaImage {
        let mut image_buf: RgbaImage = ImageBuffer::new(
            u32::try_from(self.width).unwrap(),
            u32::try_from(self.height).unwrap()
        );

        for (_, _, pixel) in image_buf.enumerate_pixels_mut() {
            *pixel = Rgba(color_rgba);
        }

        match &self.initials {
            Some(initials) => {
                match self.text_color {
                    Some(text_color) => {
                        let font = Vec::from(include_bytes!("../assets/Roboto.ttf") as &[u8]);
                        let font = Font::try_from_vec(font).unwrap();

                        let font_scale = Scale {
                            x: self.width as f32 / 2.0,
                            y: self.height as f32 / 2.0
                        };

                        let (text_width, text_height) = text_size(font_scale, &font, &initials);

                        draw_text_mut(
                            &mut image_buf,
                            Rgba(text_color),
                            self.width / 2 - text_width / 2,
                            self.height / 2 - text_height / 2 - 6,
                            font_scale, &font, 
                            &initials
                        )
                    },
                    None => ()
                }
            },
            None => ()
        }

        image_buf
    }


    /// Returns an RGBA Image (`RgbaImage`) of a noisy gradient (and initials of the user, if given)
    /// 
    /// # Arguments
    /// 
    /// * `noise_scale` - A 64-bit float value representing noise scale. Higher values are more noisy
    /// * `gradient` - A `Gradient` value to use as a background
    /// 
    /// # Example
    /// ```
    /// use amaryllis::Avatar;
    /// use colorgrad::CustomGradient;
    /// 
    /// let avatar = Avatar::new(
    ///     200, 
    ///     200, 
    ///     Option::Some("John Middlename Doe"),
    ///     Option::Some([0, 0, 0, 255])
    /// ).gradient(
    ///     0.0025,
    ///     CustomGradient::new().html_colors(&["deeppink", "cyan"]).build().unwrap()
    /// );
    /// 
    /// avatar.save("cool_avatar.png");
    /// ```
    #[allow(dead_code)]
    pub fn gradient(&self, noise_scale: f64, gradient: colorgrad::Gradient) -> RgbaImage {
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

        match &self.initials {
            Some(initials) => {
                match self.text_color {
                    Some(text_color) => {
                        let font = Vec::from(include_bytes!("../assets/Roboto.ttf") as &[u8]);
                        let font = Font::try_from_vec(font).unwrap();

                        let font_scale = Scale {
                            x: self.width as f32 / 2.0,
                            y: self.height as f32 / 2.0
                        };

                        let (text_width, text_height) = text_size(font_scale, &font, &initials);

                        draw_text_mut(
                            &mut image_buf,
                            Rgba(text_color),
                            self.width / 2 - text_width / 2,
                            self.height / 2 - text_height / 2 - 6,
                            font_scale, &font, 
                            &initials
                        )
                    },
                    None => ()
                }
            },
            None => ()
        }

        image_buf
    }


}


#[cfg(test)]
mod tests {
    use crate::Avatar;

    use std::fs;
    use chrono::{DateTime, Utc};
    use colorgrad::CustomGradient;

    #[test]
    fn main() {
        //
        let _ = fs::create_dir_all("test_results/simple");
        let _ = fs::create_dir_all("test_results/simple/text");
        let _ = fs::create_dir_all("test_results/gradient");
        let _ = fs::create_dir_all("test_results/gradient/text");

        let timestamp = DateTime::timestamp_millis(&Utc::now());


        // Simple
        Avatar::new(
            200,
            200,
            None,
            None
        ).simple(
            [255, 255, 255, 255],
        ).save(format!("test_results/simple/{timestamp}.webp")).unwrap();


        // Simple with text
        Avatar::new(
            200,
            200,
            Option::Some("John Middlename Doe"),
            Option::Some([0, 0, 0, 255])
        ).simple(
            [255, 255, 255, 255],
        ).save(format!("test_results/simple/text/{timestamp}.webp")).unwrap();
        

        // Gradient
        Avatar::new(
            200,
            200,
            None,
            None
        ).gradient(
            0.0025,
            colorgrad::reds()
        )
        .save(format!("test_results/gradient/{timestamp}.webp")).unwrap();

        
        // Gradient with text
        Avatar::new(
            200,
            200,
            Option::Some("John Middlename Doe"),
            Option::Some([0, 0, 0, 255])
        ).gradient(
            0.0025,
            CustomGradient::new().html_colors(&["deeppink", "cyan"]).build().unwrap()
        )
        .save(format!("test_results/gradient/text/{timestamp}.webp")).unwrap();
    }
}

