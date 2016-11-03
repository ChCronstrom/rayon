mod worker;

use std::iter::Iterator;
use std::sync::Mutex;

use basics::*;
use scene::Scene;
use self::worker::RenderTask;

use scoped_threadpool::Pool;

#[derive(Clone, Copy)]
pub struct RenderSettings<'a>
{
    width: u32,
    height: u32,
    supersamples: u32,
    threads: u32,
    scene: &'a Scene,
}

impl<'a> RenderSettings<'a>
{
    pub fn new(scene: &Scene, (width, height): (u32, u32), supersamples: u32, threads: u32) -> RenderSettings
    {
        RenderSettings {
            scene: scene,
            width: width,
            height: height,
            supersamples: supersamples,
            threads: threads,
        }
    }

    pub fn render(self) -> HdrImage
    {
        let final_image = Mutex::new(HdrImage::new(self.width, self.height));
        {
            let final_image_ref = &final_image;

            let mut pool = Pool::new(self.threads);

            pool.scoped(|scope| {
                let blocks = self.subdivide_render();

                for (i, block) in blocks.into_iter().enumerate()
                {
                    scope.execute(move || {
                        RenderTask::new(self, block, i as u64).render(final_image_ref);
                    });
                }
            });
        }
        final_image.into_inner().expect("Mutex was poisoned.")
    }

    fn subdivide_render(&self) -> Vec<RenderBlock>
    {
        let step = 64;
        let mut result = Vec::with_capacity(((1 + self.width / step) * (1 + self.height / step)) as usize);

        let xsteps = {
            let mut xsteps: Vec<_> = (0..(1 + self.width / step)).map(|i| i * step).collect();
            if xsteps[xsteps.len()-1] != self.width {
                xsteps.push(self.width);
            }
            xsteps
        };

        let ysteps = {
            let mut ysteps: Vec<_> = (0..(1 + self.height / step)).map(|i| i * step).collect();
            if ysteps[ysteps.len() - 1] != self.height {
                ysteps.push(self.height);
            }
            ysteps
        };

        for yrange in (&ysteps).windows(2)
        {
            //let &[ystart, ystop] = yrange;
            let ystart = yrange[0];
            let ystop = yrange[1];

            for xrange in (&xsteps).windows(2)
            {
                //let &[xstart, xstop] = xrange;
                let xstart = xrange[0];
                let xstop = xrange[1];

                result.push(RenderBlock((xstart, xstop - xstart), (ystart, ystop - ystart)));
            }
        }

        result
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct RenderBlock((u32, u32), (u32, u32));
