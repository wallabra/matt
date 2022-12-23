use crate::prelude::*;

pub struct Sampler<Resamp: Resampler> {
    playing: bool,
    last_rate: usize, // samplerate is always integer
    position: usize,
    reverse: bool,

    resampled: StereoBuffer,
    play_buffer: StereoBuffer,
    resampler: Resamp,
}

fn tuple_isize(tup: (usize, usize)) -> (isize, isize) {
    (tup.0 as isize, tup.1 as isize)
}

impl<Resamp> Sampler<Resamp>
where
    Resamp: Resampler,
{
    fn advance(&mut self, mut interval: usize, sample_data: &SampleData) {
        let loop_points: (isize, isize) = tuple_isize(
            sample_data
                .loop_points
                .unwrap_or((0, sample_data.length - 1)),
        );
        let mut next_position = self.position as isize;

        while interval > 0 {
            if self.reverse {
                next_position -= interval as isize;
            } else {
                next_position += interval as isize;
            }

            if next_position > loop_points.1 {
                match &sample_data.looper {
                    None => {
                        self.playing = false;
                        return;
                    }
                    Some(looper) => {
                        let offset = loop_points.1 - self.position as isize;
                        interval -= offset as usize;

                        next_position = match looper.dir {
                            LoopDirection::Normal => loop_points.0,
                            LoopDirection::PingPong => loop_points.1,
                        };
                    }
                }
            } else if next_position < loop_points.0 {
                if let Some(looper) = &sample_data.looper {
                    match looper.dir {
                        LoopDirection::Normal => {
                            self.playing = false;
                            return;
                        }
                        LoopDirection::PingPong => {
                            interval -= self.position - loop_points.0 as usize;
                            next_position = loop_points.0;
                        }
                    }
                }
            }
        }
    }

    fn check_resample(&mut self, samplerate: usize, sample_data: &SampleData) {
        if samplerate != self.last_rate || self.last_rate == 0 {
            self.resampled =
                StereoBuffer::new(sample_data.length * samplerate / sample_data.samplerate);
            StereoBuffer::from(&sample_data.data)
                .copy_to_and_resample((&mut self.resampled).into(), &self.resampler);
        }
    }

    fn render(&mut self, interval: usize, bufsize: usize, sample_data: &SampleData) {
        self.play_buffer = StereoBuffer::new(bufsize);

        //TODO
        todo!();
    }

    /**
     * Updates `interval` samples of this Sampler, using a SampleData as a reference.
     */
    pub fn tick(&mut self, samplerate: usize, bufsize: usize, sample_data: &SampleData) {
        self.check_resample(samplerate, sample_data);
        self.render(samplerate, bufsize, sample_data);
        self.advance(bufsize, sample_data);
    }
}

impl<Resamp> SourceObj for Sampler<Resamp>
where
    Resamp: Resampler,
{
    fn gush_audio(&self, to: &mut StereoBuffer) {
        self.play_buffer
            .copy_to_and_resample(to.into(), &NoResample);
    }
}

pub mod prelude {
    pub use super::Sampler;
}
