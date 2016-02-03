use intersectable::plane::Plane;

pub struct Scene;

pub fn example_scene() -> Scene
{
	let mut scene = Scene::new();
	//let plane = Plane::new();
	//scene.add_intersectable(plane);
	return scene;
}

impl Scene
{
	pub fn new() -> Scene
	{
		Scene
	}
}
