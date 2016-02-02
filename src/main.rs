extern crate image;

mod scene;
mod renderer;
mod post;

use renderer::{Renderer};
use post::{PostProcessor};

pub type HdrImage = image::ImageBuffer<image::Rgb<f32>, Vec<f32>>;

fn main()
{
    println!("Making a scene ...");
    let scene = scene::example_scene();
    let	renderer = Renderer::new();
    let postprocessor = PostProcessor::new();

    println!("Rendering ...");
    let image = renderer.render(&scene);

    println!("Post-processing ...");
    let u8image = postprocessor.process(&image);

    println!("Saving ...");
    u8image.save("testimage.png").unwrap();
}
