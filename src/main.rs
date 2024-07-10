extern crate sdl2;
mod roads;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::image::LoadTexture;
use smart_road::vehicle::{Direction, Vehicle, Virage, remove_vehicles_out_of_bounds};
use smart_road::statistics::Statistics;
use std::time::Duration;
use roads::draw_roads;

fn main() {
    let sdl_context = sdl2::init().expect("Could not initialize SDL2");
    let video_subsystem = sdl_context.video().expect("Could not initialize video subsystem");
    let collision_distance: u32 = 50;
    let mut id: usize = 0;
    let mut stats = Statistics::new();

    let window = video_subsystem
        .window("Smart_road Simulation", 800, 800)
        .position_centered()
        .build()
        .expect("Could not create window");

    let mut canvas = window.into_canvas().build().expect("Could not create canvas");

    let texture_creator = canvas.texture_creator();
    let point_texture = texture_creator.load_texture("assets/point.png").expect("Could not load point texture");
    let vehicle_texture = texture_creator.load_texture("assets/car-down.png").expect("Could not load vehicle texture");

    let mut vehicles: Vec<Vehicle> = Vec::new();
    let mut event_pump = sdl_context.event_pump().expect("Could not get event pump");

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    stats.display_statistics(&sdl_context, &mut event_pump);
                    break 'running;
                },
                Event::KeyDown { keycode, .. } => {
                    if let Some(direction) = map_key_to_direction(keycode) {
                        if Vehicle::is_safe_position(&vehicles) {
                            id += 1;
                            let vehicle = Vehicle::new(id,  2, direction, Virage::new());
                            stats.track_vehicle_entry(&vehicle);
                            vehicles.push(vehicle);
                        }
                    }
                },
                _ => {}
            }
        }

        // Clear the canvas
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // Draw roads
        draw_roads(&mut canvas);

        // First iteration: Collect updates for vehicles without mutable borrow
        let vehicle_actions: Vec<(usize, Vec<VehicleAction>)> = vehicles.iter().enumerate().map(|(i, vehicle)| {
            let mut actions = Vec::new();

            if let Some(pivot) = &vehicle.turning_point  {
                if vehicle.virage == Virage::R{
                    actions.push(VehicleAction::Accelerate);
                }
            }

            if vehicle.detect_vehicle_front(&vehicles, collision_distance) {
                actions.push(VehicleAction::Stop);
            } else if vehicle.velocity == 0 {
                actions.push(VehicleAction::Start);
            }

            if vehicle.let_priority_left(&vehicles) {
                stats.increment_close_calls();
                actions.push(VehicleAction::Start);
            }

            (i, actions)
        }).collect();

        // Second iteration: Apply updates to vehicles with mutable borrow
        for (i, actions) in vehicle_actions {
            let vehicle = &mut vehicles[i];
            for action in actions {
                match action {
                    VehicleAction::Stop => vehicle.mut_velocity(0),
                    VehicleAction::Start => vehicle.mut_velocity(2),
                    VehicleAction::Accelerate => vehicle.mut_velocity(3),
                }
            }

            vehicle.render(&mut canvas, Color::RGB(255, 0, 0), &vehicle_texture).unwrap();
            vehicle.draw_position_vehicule(&mut canvas, &point_texture).unwrap();
            vehicle.drive();
        }

        // Remove vehicles out of bounds
        let vehicles_out_of_bounds = remove_vehicles_out_of_bounds(&mut vehicles, 800, 800);
        for vehicle in vehicles_out_of_bounds {
            stats.track_vehicle_exit(&vehicle);
        }
        canvas.present();

        // Control frame rate
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

/// Maps a key code to a vehicle direction.
fn map_key_to_direction(keycode: Option<Keycode>) -> Option<Direction> {
    match keycode {
        Some(Keycode::Up) => Some(Direction::North),
        Some(Keycode::Down) => Some(Direction::South),
        Some(Keycode::Right) => Some(Direction::East),
        Some(Keycode::Left) => Some(Direction::West),
        Some(Keycode::R) => Some(Direction::random()),
        _ => None,
    }
}

enum VehicleAction {
    Stop,
    Start,
    Accelerate,
}
