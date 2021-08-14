use sdl2::TimerSubsystem;

pub struct FpsCounter {
    timer_subsystem: TimerSubsystem,
    interval: u32,
    last_tick: u32,
    last_second: u32,
    fps: u16,
    pub show_fps: bool,
}

impl FpsCounter {
    pub fn new(target_fps: u32, mut timer_subsystem: TimerSubsystem) -> FpsCounter {
        let now = timer_subsystem.ticks();

        FpsCounter {
            timer_subsystem,
            interval: 1000 / target_fps,
            last_tick: now,
            last_second: now,
            fps: 0,
            show_fps: false,
        }
    }

    pub fn tick(&mut self) -> (bool, f64, Option<u16>) {
        let now = self.timer_subsystem.ticks();
        let dt = now - self.last_tick;
        let elapsed = dt as f64 / 1000.0;
        let mut show_fps = None;
        if dt < self.interval {
            self.timer_subsystem.delay(self.interval - dt);
            return (false, 0f64, show_fps);
        }
        self.last_tick = now;

        if self.show_fps {
            self.fps += 1;
            if now - self.last_second > 1000 {
                // println!("FPS: {}", self.fps);
                show_fps = Some(self.fps);
                self.last_second = now;
                self.fps = 0;
            }
        }
        return (true, elapsed, show_fps);
    }
}
