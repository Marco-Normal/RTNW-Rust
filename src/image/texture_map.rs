use image::ImageReader;

fn read_image(path: String) -> Option<[u8], Error> {
    let raw_image = ImageReader::open(path);
    const BYTES_PER_PIXEL: usize = 3;
    if let Ok(img) = raw_image {
        let Ok((width, height)) = img.into_dimensions();
        let bytes_per_scanline = width * BYTES_PER_PIXEL as u32;
        let total_bytes = width * height * BYTES_PER_PIXEL as u32;
    }
}

fn float_to_byte(value: u32) -> u8 {
    match value {
        1.. => return 255,
        0..1 => return value as u8,
        _ => return 0,
    }
}
