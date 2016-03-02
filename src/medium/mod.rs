use basics::*;

#[derive(Clone, Copy, Debug, Default)]
pub struct Medium
{
    pub emission: Option<Colour>,
    pub absorption: Option<Colour>,
}

impl Medium
{
    pub fn new_emission(emission: Colour) -> Medium
    {
        Medium {
            emission: Some(emission),
            absorption: None,
        }
    }

    pub fn new_absorption(absorption: Colour) -> Medium
    {
        Medium {
            emission: None,
            absorption: Some(absorption),
        }
    }
}
