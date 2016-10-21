use basics::*;
use camera::Camera;
use functions::{Chequered, ClosureFunction, Function};
use functions::noise::{VectorNoise};
use intersectable::*;
use medium::Medium;
use texture::*;

pub struct Scene
{
    pub objects: Union,
    pub camera: Camera,
}

/*pub fn example_scene() -> Scene
{
    let mut scene = Scene::new();
    scene.camera = Camera::from_position(Point::new(0.1, -4.0, 0.9), Point::new(0.0, 0.0, 1.0));

    let gray_texture = Lambertian { pigment: Colour::new(1.0, 1.0, 1.0) };
    let plane = Textured::new(Plane, gray_texture);
    scene.objects.subobjects.push(Box::new(plane));

    let emissive_texture = Emissive { colour: Colour::new(1.0, 1.0, 1.0) };
    let sphere = Textured::new(Sphere, emissive_texture);
    scene.objects.subobjects.push(Box::new(sphere));

    let red_texture = Lambertian::new(Colour::new(0.8, 0.0, 0.2));
    let transformed = Transformed::new(Textured::new(Sphere, red_texture), Trans::new_translation(0.0, 0.0, 2.0));
    scene.objects.subobjects.push(Box::new(transformed));

    return scene;
}*/

pub fn example_scene() -> Scene
{
    let mut scene = Scene::new();
    scene.camera = Camera::from_position(Point::new(0.1, -4.0, 1.2), Point::new(0.0, 0.0, 1.0));

    //let gray_texture = Lambertian::new(Chequered::new(Colour::new(1.0, 1.0, 1.0), Colour::new(0.5, 0.5, 0.6)));
    let random_texture = VectorNoise::new(0, 0.5);
    let test_texture = Lambertian::new(ClosureFunction::new(move |p| random_texture.evaluate(p) * 0.5 + 0.5));
    let glass_texture = Glass::new(1.5);
    let emissive_texture = Emissive { colour: Colour::new(22.0, 20.0, 20.0) };

    let plane = Textured::new(Plane, test_texture);
    scene.objects.subobjects.push(Box::new(plane));

    let sphere = Transformed::new(Textured::new(Sphere, emissive_texture), Trans::new_translation(9.0, 2.0, 9.0));
    scene.objects.subobjects.push(Box::new(sphere));

    let transformed = Transformed::new(Textured::new_with_media(Sphere, glass_texture, Medium::new_absorption(Colour::new(-0.3, -0.3, -0.01)), Default::default()), Trans::new_translation(0.0, 0.0, 1.0));
    //let transformed = Textured::new(Sphere, glass_texture);
    scene.objects.subobjects.push(Box::new(transformed));

    return scene;
}


impl Scene
{
    pub fn new() -> Scene
    {
        Scene
        {
            objects: Union::new(),
            camera: Default::default(),
        }
    }
}
