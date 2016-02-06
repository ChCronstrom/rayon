extern crate nalgebra as na;
extern crate image;

mod basics;
mod camera;
mod intersectable;
mod post;
mod renderer;
mod scene;

use renderer::{Renderer};
use post::{PostProcessor};

fn main()
{
    println!("Making a scene ...");
    let scene = scene::example_scene();
    let	renderer = Renderer::new(&scene);
    let postprocessor = PostProcessor::new();

    println!("Rendering ...");
    let image = renderer.render();

    println!("Post-processing ...");
    let u8image = postprocessor.process(&image);

    println!("Saving ...");
    u8image.save("testimage.png").unwrap();
}
