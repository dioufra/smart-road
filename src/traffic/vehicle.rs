use sdl2::{pixels::Color, rect::Rect, render::{Texture, WindowCanvas}};
use rand::Rng;

#[derive(Debug, Clone)]
pub struct Vehicle {
    pub id: usize,
    pub origin: Position,
    pub position: Position,
    pub direction: Direction,
    pub virage: Virage,
    pub turning_point: Option<Pivot>,
    pub velocity: i32,
    pub rect: Rect,
    pub angle: f64,
    pub position_middle_sensor: Position,
    pub position_front_sensor: Position,
    pub entry_time: std::time::Instant,
}

pub const CAR_WIDTH: u32 = 25;
pub const CAR_HEIGHT: u32 = 45;
pub const STEP: u32 = 50;
pub const SAFETY_DISTANCE: u32 = 100;

impl Vehicle {
    pub fn new(id: usize, velocity: i32, direction: Direction, virage: Virage) -> Self {
        let angle = match direction {
            Direction::North => 180.0,
            Direction::South => 0.0,
            Direction::East => -90.0,
            Direction::West => 90.0,
        };

        let position = Position::new(&direction, &virage);
        let turning_point = Pivot::new(&direction, &virage, &position);
        let rect = Rect::new(position.x, position.y, CAR_WIDTH, CAR_HEIGHT);

        let position_middle_sensor = Position::set_position(&direction, position.x, position.y, "middle");
        let position_front_sensor = Position::set_position(&direction, position.x, position.y, "front");
        let entry_time = std::time::Instant::now();

        Self {
            id,
            position,
            origin: position.clone(),
            direction,
            virage,
            turning_point,
            velocity,
            rect,
            angle,
            position_middle_sensor,
            position_front_sensor,
            entry_time,
        }
    }

    pub fn render(&mut self, canvas: &mut WindowCanvas, _color: Color, texture: &Texture) -> Result<(), String> {
        canvas.copy_ex(texture, None, self.rect, self.angle, None, false, false)?;
        Ok(())
    }

    pub fn draw_position_vehicule(&self, canvas: &mut WindowCanvas, texture: &Texture) -> Result<(), String> {
        let rect_middle = Rect::new(self.position_middle_sensor.x, self.position_middle_sensor.y, 20, 20);
        canvas.copy_ex(texture, None, rect_middle, self.angle, None, false, false)?;
        let rect_front = Rect::new(self.position_front_sensor.x, self.position_front_sensor.y, 20, 20);
        canvas.copy_ex(texture, None, rect_front, self.angle, None, false, false)?;
        Ok(())
    }

    pub fn drive(&mut self) {
        let n = match self.virage {
            Virage::L => -1,
            Virage::R | Virage::S => 1,
        };
        match self.direction {
            Direction::North => {
                if let Some(pivot) = &self.turning_point {
                    if self.rect.y <= pivot.position.y {
                        self.angle = -90.0 * n as f64;

                        self.rect.set_x(self.rect.x + (n * self.velocity) as i32);
                        self.rect.set_y(pivot.position.y + 2);

                        self.position.x = self.rect.x;
                        self.position.y = self.rect.y;

                        self.turning_point = None;
                        if self.angle > 0.0 {
                            self.direction = Direction::West;
                        } else {
                            self.direction = Direction::East;
                        }
                    } else {
                        self.rect.set_y(self.rect.y - self.velocity as i32);
                        self.position.y = self.rect.y;
                    }
                } else {
                    self.rect.set_y(self.rect.y - self.velocity as i32);
                    self.position.y = self.rect.y;
                }
            }
            Direction::South => {
                if let Some(pivot) = &self.turning_point {
                    if self.rect.y >= pivot.position.y {
                        self.angle = 90.0 * n as f64;
                        self.rect.set_x(self.rect.x - (n * self.velocity) as i32);
                        self.rect.set_y(pivot.position.y + 2);

                        self.position.x = self.rect.x;
                        self.position.y = self.rect.y;

                        self.turning_point = None;
                        if self.angle > 0.0 {
                            self.direction = Direction::West;
                        } else {
                            self.direction = Direction::East;
                        }
                    } else {
                        self.rect.set_y(self.rect.y + self.velocity as i32);
                        self.position.y = self.rect.y;
                    }
                } else {
                    self.rect.set_y(self.rect.y + self.velocity as i32);
                    self.position.y = self.rect.y;
                }
            }
            Direction::West => {
                if let Some(pivot) = &self.turning_point {
                    if self.rect.x <= pivot.position.x {
                        self.angle = if n > 0 { 180.0 } else { 0.0 };
                        self.rect.set_x(pivot.position.x + 2);
                        self.rect.set_y(self.rect.y - (n * self.velocity) as i32);

                        self.position.x = self.rect.x;
                        self.position.y = self.rect.y;

                        self.turning_point = None;
                        if self.angle == 180.0 {
                            self.direction = Direction::North;
                        } else {
                            self.direction = Direction::South;
                        }
                    } else {
                        self.rect.set_x(self.rect.x - self.velocity as i32);
                        self.position.x = self.rect.x;
                    }
                } else {
                    self.rect.set_x(self.rect.x - self.velocity as i32);
                    self.position.x = self.rect.x;
                }
            }
            Direction::East => {
                if let Some(pivot) = &self.turning_point {
                    if self.rect.x >= pivot.position.x {
                        self.angle = if n > 0 { 0.0 } else { 180.0 };

                        self.rect.set_x(pivot.position.x + 2);
                        self.rect.set_y(self.rect.y + (n * self.velocity) as i32);

                        self.position.x = self.rect.x;
                        self.position.y = self.rect.y;

                        self.turning_point = None;
                        if self.angle == 180.0 {
                            self.direction = Direction::North;
                        } else {
                            self.direction = Direction::South;
                        }
                    } else {
                        self.rect.set_x(self.rect.x + self.velocity as i32);
                        self.position.x = self.rect.x;
                    }
                } else {
                    self.rect.set_x(self.rect.x + self.velocity as i32);
                    self.position.x = self.rect.x;
                }
            }
        }

        self.position_middle_sensor = Position::set_position(&self.direction, self.position.x, self.position.y, "middle");
        self.position_front_sensor = Position::set_position(&self.direction, self.position.x, self.position.y, "front");
    }

    pub fn is_out_of_bounds(&self, map_width: i32, map_height: i32) -> bool {
        self.position.x < 0 || self.position.x > map_width || self.position.y < 0 || self.position.y > map_height
    }

    pub fn let_priority_left(&self, others: &Vec<Vehicle>) -> bool {
        for other in others {
            if self.velocity == 0 && other.velocity == 0 && self.id != other.id {
                match (self.direction, other.direction) {
                    (Direction::East, Direction::North) | (Direction::North, Direction::West) | (Direction::South, Direction::East) | (Direction::West, Direction::South) => {
                        return true;
                    }
                    _ => {}
                }
            }
        }
        false
    }

    pub fn is_safe_position(vehicles: &Vec<Vehicle>) -> bool {
        if vehicles.is_empty() {
            return true;
        }
        let last = vehicles.last().unwrap();
        let distance = (((last.position.x - last.origin.x).pow(2) + (last.position.y - last.origin.y).pow(2)) as f32).sqrt() as i32;

        // return distance >=  SAFETY_DISTANCE as i32  || last.direction != *direction
        return distance >= SAFETY_DISTANCE as i32;
    }

    pub fn detect_vehicle_front(&self, vehicles: &Vec<Vehicle>, detection_distance: u32) -> bool {
        if self.virage == Virage::R {
            return false;
        }

        for vehicle in vehicles {
            if self.id != vehicle.id {
                let distance = (((self.position_front_sensor.x - vehicle.position_middle_sensor.x).pow(2) + (self.position_front_sensor.y - vehicle.position_middle_sensor.y).pow(2)) as f32).sqrt();

                match self.direction {
                    Direction::North | Direction::South => {
                        match vehicle.direction {
                            Direction::East | Direction::West => {
                                if distance as u32 <= detection_distance && (vehicle.position_middle_sensor.x >= self.position_middle_sensor.x - 49 && vehicle.position_middle_sensor.x <= self.position_middle_sensor.x + 49) {
                                    return true;
                                }
                            }
                            Direction::North => {
                                if distance as u32 <= detection_distance && vehicle.position_middle_sensor.x == self.position_middle_sensor.x && vehicle.position_middle_sensor.y <= self.position_middle_sensor.y && self.direction == Direction::North {
                                    return true;
                                }
                            }
                            Direction::South => {
                                if distance as u32 <= detection_distance && vehicle.position_middle_sensor.x == self.position_middle_sensor.x && vehicle.position_middle_sensor.y >= self.position_middle_sensor.y && self.direction == Direction::South {
                                    return true;
                                }
                            }
                        }
                    }

                    Direction::East | Direction::West => {
                        match vehicle.direction {
                            Direction::North | Direction::South => {
                                if distance as u32 <= detection_distance && (vehicle.position_middle_sensor.y >= self.position_middle_sensor.y - 49 && vehicle.position_middle_sensor.y <= self.position_middle_sensor.y + 49) {
                                    return true;
                                }
                            }
                            Direction::East => {
                                if distance as u32 <= detection_distance && vehicle.position_middle_sensor.y == self.position_middle_sensor.y && vehicle.position_middle_sensor.x >= self.position_middle_sensor.x && self.direction == Direction::East {
                                    return true;
                                }
                            }
                            Direction::West => {
                                if distance as u32 <= detection_distance && vehicle.position_middle_sensor.y == self.position_middle_sensor.y && vehicle.position_middle_sensor.x <= self.position_middle_sensor.x && self.direction == Direction::West {
                                    return true;
                                }
                            }
                        }
                    }
                }
            }
        }
        false
    }

    pub fn mut_velocity(&mut self, speed: i32) {
        self.velocity = speed;
    }
}

pub fn remove_vehicles_out_of_bounds(vehicles: &mut Vec<Vehicle>, map_width: i32, map_height: i32) -> Vec<Vehicle> {
    let mut out_of_bounds_vehicles = Vec::new();
    vehicles.retain(|vehicle| {
        if vehicle.is_out_of_bounds(map_width, map_height) {
            out_of_bounds_vehicles.push(vehicle.clone());
            false
        } else {
            true
        }
    });
    out_of_bounds_vehicles
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..4) {
            0 => Direction::North,
            1 => Direction::South,
            2 => Direction::West,
            _ => Direction::East,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(direction: &Direction, virage: &Virage) -> Self {
        let step = 50;

        let n = match virage {
            Virage::L => 50,
            Virage::S => 100,
            Virage::R => 150,
        };

        let pos = match direction {
            Direction::North => (400 + (n - step) + ((step - CAR_WIDTH) / 2), 800),
            Direction::South => (400 - n + ((step - CAR_WIDTH) / 2), 0),
            Direction::West => (800, 400 - n + (step - CAR_HEIGHT) / 2),
            Direction::East => (0, 400 + (n - step) + (step - CAR_HEIGHT) / 2),
        };

        Self {
            x: pos.0 as i32,
            y: pos.1 as i32,
        }
    }

    pub fn set_position(direction: &Direction, x: i32, y: i32, sensor: &str) -> Self {
        let (x, y) = match sensor {
            "middle" => {
                match direction {
                    Direction::North => (x + 3, y + 13),
                    Direction::South => (x + 3, y + 13),
                    Direction::East => (x + 3, y + 13),
                    Direction::West => (x + 3, y + 13),
                }
            }
            _ => {
                match direction {
                    Direction::North => (x + 3, y - 40 as i32),
                    Direction::South => (x + 3, y + 35 * 2 as i32),
                    Direction::East => (x + 50 as i32, y + 15),
                    Direction::West => (x - 50 as i32, y + 15),
                }
            }
        };

        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Virage {
    L,
    R,
    S,
}

impl Virage {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..3) {
            0 => Virage::L,
            1 => Virage::R,
            _ => Virage::S,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Pivot {
    pub position: Position,
}

impl Pivot {
    pub fn new(direction: &Direction, virage: &Virage, position: &Position) -> Option<Pivot> {
        let step = 50;

        let n = match virage {
            Virage::L => 50,
            Virage::S => return None,
            Virage::R => 150,
        };

        let mut offset_x = position.x;
        let mut offset_y = position.y;

        match (direction, virage) {
            (Direction::North, Virage::L) => offset_y = 400 - n,
            (Direction::North, Virage::R) => offset_y = 400 + n - step,
            (Direction::South, Virage::L) => offset_y = 400,
            (Direction::South, Virage::R) => offset_y = 400 - n,
            (Direction::West, Virage::L) => offset_x = 400 - n + 10,
            (Direction::West, Virage::R) => offset_x = 400 + n - step + 10,
            (Direction::East, Virage::L) => offset_x = 400 + 10,
            (Direction::East, Virage::R) => offset_x = 400 - n + 10,
            _ => return None,
        };

        Some(Self {
            position: Position {
                x: offset_x as i32,
                y: offset_y as i32,
            },
        })
    }
}
