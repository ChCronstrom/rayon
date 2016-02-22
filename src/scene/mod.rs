use basics::*;
use camera::Camera;
use intersectable::*;
use texture::*;

use na::Pnt3;

pub struct Scene
{
    pub objects: Union,
    pub camera: Camera,
}

/*pub fn example_scene() -> Scene
{
    let mut scene = Scene::new();
    scene.camera = Camera::from_position(Pnt3::new(0.1, -4.0, 0.9), Pnt3::new(0.0, 0.0, 1.0));

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
    scene.camera = Camera::from_position(Pnt3::new(0.1, -4.0, 0.9), Pnt3::new(0.0, 0.0, 1.0));

    let gray_texture = Lambertian::new(Colour::new(1.0, 1.0, 1.0));
    let glass_texture = Glass::new(1.5);
    let emissive_texture = Emissive { colour: Colour::new(10.0, 10.0, 10.0) };

    let plane = Textured::new(Plane, gray_texture);
    scene.objects.subobjects.push(Box::new(plane));

    let sphere = Transformed::new(Textured::new(Sphere, emissive_texture), Trans::new_translation(9.0, 2.0, 9.0));
    scene.objects.subobjects.push(Box::new(sphere));

    let transformed = Transformed::new(Textured::new(Sphere, glass_texture), Trans::new_translation(0.0, 0.0, 0.9));
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
