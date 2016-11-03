extern crate rayon;

use rayon::*;
use rayon::functions::*;
use rayon::intersectable::*;
use rayon::texture::*;

pub fn example_scene() -> Scene
{
    let mut scene = Scene::new();
    //scene.camera = Camera::from_position(Point::new(0.1, -4.0, 1.2), Point::new(0.0, 0.0, 1.0));
    scene.camera = Camera::from_position(Point::new(0.0, -4.0, 1.2), Point::new(0.0, 0.0, 1.0));

    let gray_texture = Lambertian::new(Chequered::new(Colour::new(1.0, 1.0, 1.0), Colour::new(0.5, 0.5, 0.6)));
    let random_texture = VectorNoise::new(0, 0.5);
    let test_texture = Lambertian::new(ClosureFunction::new(move |p| random_texture.evaluate(p) * 0.5 + 0.5));
    let glass_texture = Glass::new(1.5);
    let emissive_texture = Emissive { colour: Colour::new(22.0, 20.0, 20.0) };

    //let plane = Textured::new(Plane, test_texture);
    //scene.objects.subobjects.push(Box::new(plane));

    let sphere = Transformed::new(Textured::new(Sphere, emissive_texture), Trans::new_translation(9.0, 2.0, 9.0));
    scene.objects.subobjects.push(Box::new(sphere));

    //let transformed = Textured::new(Sphere, glass_texture);
    //let transformed = Transformed::new(Textured::new_with_media(Sphere, glass_texture, Medium::new_absorption(Colour::new(-0.3, -0.3, -0.01)), Default::default()), Trans::new_translation(0.0, 0.0, 1.0));
    let transformed = Transformed::new(
        //Transformed::new(CrossSection { subobjects: vec![Box::new(Transformed::new(Plane, Trans::new(((-0.0000, 0.0000, -1.0000), (0.0000, 1.0000, 0.0000), (1.0000, 0.0000, -0.0000)), (1.0000, 0.0000, 0.0000)))), Box::new(Transformed::new(Plane, Trans::new(((-0.0000, 0.0000, 1.0000), (0.0000, 1.0000, 0.0000), (-1.0000, 0.0000, -0.0000)), (-1.0000, 0.0000, 0.0000)))), Box::new(Transformed::new(Plane, Trans::new(((1.0000, 0.0000, 0.0000), (0.0000, -0.0000, -1.0000), (0.0000, 1.0000, -0.0000)), (0.0000, 1.0000, 0.0000)))), Box::new(Transformed::new(Plane, Trans::new(((1.0000, 0.0000, 0.0000), (0.0000, -0.0000, 1.0000), (0.0000, -1.0000, -0.0000)), (0.0000, -1.0000, 0.0000)))), Box::new(Transformed::new(Plane, Trans::new(((1.0000, 0.0000, 0.0000), (0.0000, 1.0000, 0.0000), (0.0000, 0.0000, 1.0000)), (0.0000, 0.0000, 1.0000)))), Box::new(Transformed::new(Plane, Trans::new(((1.0000, 0.0000, 0.0000), (0.0000, -1.0000, -0.0000), (0.0000, 0.0000, -1.0000)), (0.0000, 0.0000, -1.0000)))), ] }, Trans::new(((0.7071, 0.5000, 0.5000), (-0.7071, 0.5000, 0.5000), (0.0000, -0.7071, 0.7071)), (0.0000, 0.0000, 0.0000))),
        Textured::new(Plane, gray_texture),
        Trans::new(((0.9950, 0.0998, 0.0000), (-0.0998, 0.9950, 0.0000), (0.0000, 0.0000, 1.0000)), (0.0000, 0.0000, 0.0000)));
    scene.objects.subobjects.push(Box::new(transformed));

    return scene;
}

fn main()
{
    let scene = example_scene();
    println!("Making a scene ...");
    let renderer = RenderSettings::new(&scene, (1920, 1080), 50, 4);
    let postprocessor = PostProcessor::new();

    println!("Rendering ...");
    let image = renderer.render();

    println!("Post-processing ...");
    let u8image = postprocessor.process(&image);

    println!("Saving ...");
    u8image.save("testimage.png").unwrap();
}
