use basics::*;
use camera::Camera;
use intersectable::union::Union;
use intersectable::plane::Plane;
use intersectable::sphere::Sphere;
use intersectable::transformed::Transformed;
use texture::*;

use na::Pnt3;

pub struct Scene
{
    pub objects: Union,
    pub camera: Camera,
}

pub fn example_scene() -> Scene
{
    let mut scene = Scene::new();
    scene.camera = Camera::from_position(Pnt3::new(0.1, -4.0, 0.9), Pnt3::new(0.0, 0.0, 1.0));

    let gray_texture = Lambertian { pigment: Colour::new(0.9, 0.9, 0.99) };
    let plane = Plane { texture: Box::new(gray_texture) };
    scene.objects.subobjects.push(Box::new(plane));

    let emissive_texture = Emissive { colour: Colour::new(1.0, 1.0, 1.0) };
    let sphere = Sphere { texture: Box::new(emissive_texture) };
    scene.objects.subobjects.push(Box::new(sphere));

    let red_texture = Lambertian::new(Colour::new(0.8, 0.0, 0.2));
    let transformed = Transformed::new(Sphere { texture: Box::new(red_texture) }, Trans::new_translation(0.0, 0.0, 2.0));
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
