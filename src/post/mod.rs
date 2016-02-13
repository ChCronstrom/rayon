use image::{Rgb, RgbImage};
use basics::HdrImage;

pub struct PostProcessor;

fn f32_to_u8(float: f32) -> u8
{
    let clamped = if float > 1.0 { 1.0 }
             else if float < 0.0 { 0.0 }
             else { float };

    (255.0 * clamped).round() as u8
}

fn post_process_pixel(pixel: Rgb<f32>) -> Rgb<u8>
{
    Rgb { data: [ f32_to_u8(pixel.data[0]),
                   f32_to_u8(pixel.data[1]),
                   f32_to_u8(pixel.data[2]), ] }
}

impl PostProcessor
{
    pub fn new() -> PostProcessor
    {
        PostProcessor
    }

    pub fn process(&self, hdr_image: &HdrImage) -> RgbImage
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
