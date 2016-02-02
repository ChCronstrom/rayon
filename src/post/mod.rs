use image::{Rgb, RgbImage};
use super::HdrImage;

pub struct PostProcessor;

fn post_process_pixel(pixel : Rgb<f32>) -> Rgb<u8>
{
    Rgb { data : [ (255.0 * pixel.data[0]).round() as u8,
                   (255.0 * pixel.data[1]).round() as u8,
                   (255.0 * pixel.data[2]).round() as u8, ] }
}

impl PostProcessor
{
    pub fn new() -> PostProcessor
    {
        PostProcessor
    }

    pub fn process(&self, hdr_image : &HdrImage) -> RgbImage
    {
        let (w, h) = (hdr_image.width(), hdr_image.height());
        let mut result = RgbImage::new(w, h);

        for y in 0..h
        {
            for x in 0..w
            {
                result.put_pixel(x, y, post_process_pixel(*hdr_image.get_pixel(x, y)));
            }
        }

        result
    }
}
