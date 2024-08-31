pub mod images;

pub use crate::error::Error;
pub use image::RgbImage;
pub use images::*;

use std::fmt;

#[derive(Clone, Default, PartialEq)]
pub struct Image {
    pub image_encoded: Vec<u8>,
    pub view_id: u32,
}

impl Image {
    pub fn decode_rgb(&self) -> Result<image::RgbImage, Error> {
        Ok(image::load_from_memory(&self.image_encoded)?.into())
    }
}

impl fmt::Debug for Image {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        f.debug_struct("Image")
            .field("image_encoded.len()", &self.image_encoded.len())
            .field("view_id", &self.view_id)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn decode_rgb() {
        use super::*;

        let image = Image {
            image_encoded: vec![
                0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a, 0x00, 0x00,
                0x00, 0x0d, 0x49, 0x48, 0x44, 0x52, 0x00, 0x00, 0x00, 0x01,
                0x00, 0x00, 0x00, 0x01, 0x08, 0x06, 0x00, 0x00, 0x00, 0x1f,
                0x15, 0xc4, 0x89, 0x00, 0x00, 0x00, 0x01, 0x73, 0x52, 0x47,
                0x42, 0x00, 0xae, 0xce, 0x1c, 0xe9, 0x00, 0x00, 0x00, 0x44,
                0x65, 0x58, 0x49, 0x66, 0x4d, 0x4d, 0x00, 0x2a, 0x00, 0x00,
                0x00, 0x08, 0x00, 0x01, 0x87, 0x69, 0x00, 0x04, 0x00, 0x00,
                0x00, 0x01, 0x00, 0x00, 0x00, 0x1a, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x03, 0xa0, 0x01, 0x00, 0x03, 0x00, 0x00, 0x00, 0x01,
                0x00, 0x01, 0x00, 0x00, 0xa0, 0x02, 0x00, 0x04, 0x00, 0x00,
                0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0xa0, 0x03, 0x00, 0x04,
                0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00,
                0x00, 0x00, 0xf9, 0x22, 0x9d, 0xfe, 0x00, 0x00, 0x00, 0x0d,
                0x49, 0x44, 0x41, 0x54, 0x08, 0x1d, 0x63, 0xf8, 0xcf, 0x60,
                0xdb, 0x0d, 0x00, 0x05, 0x06, 0x01, 0xc8, 0x5d, 0xd6, 0x92,
                0xd1, 0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4e, 0x44, 0xae,
                0x42, 0x60, 0x82,
            ],
            view_id: Default::default(),
        };

        // It should be idempotent
        for _ in 0..3 {
            let image = image.decode_rgb();
            assert!(image.is_ok(), "{}", image.unwrap_err());

            let image = image.unwrap();
            assert_eq!(image.height(), 1);
            assert_eq!(image.width(), 1);
            assert_eq!(image.get_pixel(0, 0).0, [0xff, 0x00, 0x3d]);
        }
    }
}
