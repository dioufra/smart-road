use sdl2::{Sdl, pixels::Color, rect::Rect, render::TextureQuery, ttf::Font};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use crate::vehicle::Vehicle;
use std::time::Duration;

pub struct Statistics {
    max_velocity: i32,
    min_velocity: i32,
    max_time: Duration,
    min_time: Duration,
    close_calls: u32,
    total_vehicles: usize,
    vehicles_passed: usize,
}

impl Statistics {
    pub fn new() -> Self {
        Self {
            max_velocity: 0,
            min_velocity: 0,
            max_time: Duration::new(0, 0),
            min_time: Duration::new(u64::MAX, 0),
            close_calls: 0,
            total_vehicles: 0,
            vehicles_passed: 0,
        }
    }

    pub fn track_vehicle_entry(&mut self, _vehicle: &Vehicle) {
        self.total_vehicles += 1;
    }

    pub fn track_vehicle_exit(&mut self, vehicle: &Vehicle) {
        self.vehicles_passed += 1;
        let elapsed_time = vehicle.entry_time.elapsed();
        if vehicle.velocity > self.max_velocity{
            self.max_velocity = vehicle.velocity;
        }
        if elapsed_time > self.max_time {
            self.max_time = elapsed_time;
        }
        if elapsed_time < self.min_time {
            self.min_time = elapsed_time;
        }
    }

    pub fn increment_close_calls(&mut self) {
        self.close_calls += 1;
    }

    fn format_duration(duration: &Duration) -> String {
        let secs = duration.as_secs();
        let millis = duration.subsec_millis();
        format!("{}.{:03} seconds", secs, millis)
    }

    pub fn display_statistics(&self, sdl_context: &Sdl, event_pump: &mut sdl2::EventPump) {
        let video_subsystem = sdl_context.video().expect("Could not initialize video subsystem");
        let window = video_subsystem
            .window("Simulation Statistics", 400, 400)
            .position_centered()
            .build()
            .expect("Could not create window");

        let mut canvas = window.into_canvas().build().expect("Could not create canvas");

        let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();
        let font = ttf_context.load_font("assets/DejaVuSans.ttf", 28).expect("Could not load font");

        let texture_creator = canvas.texture_creator();

        let render_text = |text: &str, font: &Font, color: Color| {
            let surface = font.render(text).blended(color).map_err(|e| e.to_string()).unwrap();
            let texture = texture_creator.create_texture_from_surface(&surface).map_err(|e| e.to_string()).unwrap();
            let TextureQuery { width, height, .. } = texture.query();
            let target = Rect::new(0, 0, width, height);
            (texture, target)
        };

        let stats = vec![
            format!("Vehicles passed: {}", self.vehicles_passed),
            format!("Max velocity: {}", self.max_velocity),
            format!("Min velocity: {}", self.min_velocity),
            format!("Max time: {}", Statistics::format_duration(&self.max_time)),
            format!("Min time: {}", Statistics::format_duration(&self.min_time)),
            format!("Close calls: {}", self.close_calls),
        ];

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        for (i, stat) in stats.iter().enumerate() {
            let (texture, target) = render_text(stat, &font, Color::RGB(255, 255, 255));
            canvas.copy(&texture, None, Some(Rect::new(10, 10 + (i as i32) * 40, target.width(), target.height()))).unwrap();
        }

        canvas.present();

        'stats: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'stats;
                    },
                    _ => {}
                }
            }
        }
    }
}
