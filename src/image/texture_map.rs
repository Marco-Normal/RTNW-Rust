use std::io::{Error, ErrorKind};

use image::{GenericImageView, ImageError, ImageReader};

use crate::textures::ImageTexture;

pub fn read_image(path: String) -> Result<ImageTexture, ImageError> {
    let image = image::open(path);
    match image {
        Ok(img) => {
            let img = img.into_rgb8();
            let (ux, uy) = img.dimensions();
            return Ok(ImageTexture::new(img.into_raw(), ux, uy));
        }
        Err(e) => return Err(e),
    };
}
