use std::time::{Duration, Instant};

pub struct FPSCounter {
    interval: Duration,
    last_sampled_instant: Instant,
    frames: u32,
    fps: f32,
    ms_per_frame: f32,
}

impl Default for FPSCounter {
    fn default() -> Self {
        Self::new(500)
    }
}

impl FPSCounter {
    pub fn new(interval_ms: u64) -> Self {
        Self {
            interval: Duration::from_millis(interval_ms),
            last_sampled_instant: Instant::now(),
            frames: 0,
            fps: 0.0,
            ms_per_frame: 0.0,
        }
    }

    pub fn update(&mut self) {
        self.frames += 1;
        let elapsed = self.last_sampled_instant.elapsed();

        if elapsed >= self.interval {
            self.fps = self.frames as f32 / elapsed.as_secs_f32();
            self.ms_per_frame = elapsed.as_secs_f32() * 1000.0 / self.frames as f32;

            self.print_stats();

            self.frames = 0;
            self.last_sampled_instant = Instant::now();
        }
    }

    fn print_stats(&self) {
        print!(
            "\rFPS: {:>6.1} | FrameTime: {:>6.2}ms",
            self.fps, self.ms_per_frame
        );
        use std::io::{self, Write};
        io::stdout().flush().unwrap();
    }
}
