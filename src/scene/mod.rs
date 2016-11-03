use camera::Camera;
use intersectable::Union;

pub struct Scene
{
    pub objects: Union,
    pub camera: Camera,
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
