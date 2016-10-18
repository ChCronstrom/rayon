use basics::*;
use texture::{Texture, LightInteraction};

use na::{Norm};
use rand::{Rand};

#[derive(Clone, Copy, Debug)]
pub struct Glass
{
    index_of_refraction: Float,
}

impl Glass
{
    pub fn new(index_of_refraction: Float) -> Glass
    {
        Glass {
            index_of_refraction: index_of_refraction,
        }
    }
}

impl Texture for Glass
{
    fn evaluate_texture(&self, rng: &mut RandomSource, location: Point, incidence: Vector, normal: Vector) -> LightInteraction
    {
        let _ = location;

        // TODO: Ray should carry information about the medium it is travelling through, to allow for
        // nested media, with two differing indices of refraction.

        // First we determine if we're entering or leaving the material.
        let incidence = incidence.normalize();

        // These will be useful later
        let (dot_product, projection, rejection) = calculate_projection_rejection(incidence, normal);

        // The ratio of indices of refraction;
        let n;

        // The cosine of the incident ray
        let cosine_incident;

        let sign;

        // These depend on whether we are entering or leaving the material.
        if dot_product < 0.0 // Entering
        {
            n = 1.0 / self.index_of_refraction;
            cosine_incident = -dot_product;
            sign = -1.0;
        }
        else // Leaving
        {
            n = self.index_of_refraction;
            cosine_incident = dot_product;
            sign = 1.0;
        }

        // Snell's law:
        // sin (incident) / sin(refracted) = n_inside / n_outside
        // So: sin (refracted) = (n_outside / n_inside) * sin (incident)
        let sine_incident = (1.0 - cosine_incident * cosine_incident).sqrt();
        let sine_refracted = n * sine_incident;
        let cosine_refracted_squared = 1.0 - sine_refracted * sine_refracted;

        // The direction in which the ray will reflect
        let reflected_direction = rejection - projection;

        // If this value is negative then there is no cos² (refracted), and we have total internal
        // reflection. We use !(x > 0) rather than x <= 0 to catch the NaNs that may occur in stupid
        // corner cases (e.g. if cos (incident) is slightly more than 1.0).
        if !(cosine_refracted_squared > 0.0)
        {
            return LightInteraction::new_uncoloured(reflected_direction);
        }

        let cosine_refracted = cosine_refracted_squared.sqrt();

        // Calculate the reflectivity of the interface
        let reflectance = fresnel_equations(n, cosine_incident, cosine_refracted);

        // Decide whether to reflect or refract
        if Float::rand(rng) < reflectance
        {
            return LightInteraction::new_uncoloured(reflected_direction);
        }
        else
        {
            let refracted_direction = rejection.normalize() * sine_refracted + normal * (sign * cosine_refracted);
            return LightInteraction::new_uncoloured(refracted_direction);
        }
    }
}

fn fresnel_equations(index_of_refraction: Float, cosine_incident: Float, cosine_refracted: Float) -> Float
{
    let reflectance_s_sqrt = (cosine_incident - index_of_refraction * cosine_refracted) /
                             (cosine_incident + index_of_refraction * cosine_refracted);

    let reflectance_p_sqrt = (cosine_refracted - index_of_refraction * cosine_incident) /
                             (cosine_refracted + index_of_refraction * cosine_incident);

    0.5 * (reflectance_p_sqrt * reflectance_p_sqrt + reflectance_s_sqrt * reflectance_s_sqrt)
}
