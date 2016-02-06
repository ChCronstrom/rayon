use camera::Camera;
use intersectable::union::Union;
use intersectable::plane::Plane;

use na::Pnt3;

pub struct Scene
{
	pub objects: Union,
	pub camera: Camera,
}

pub fn example_scene() -> Scene
{
	let mut scene = Scene::new();
	scene.camera = Camera::from_position(Pnt3::new(0.0, -4.0, 2.0), Pnt3::new(0.0, 0.0, 1.0));
	let plane = Plane;
	scene.objects.subobjects.push(Box::new(plane));
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
