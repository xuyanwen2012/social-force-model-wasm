extern crate nalgebra;

use nalgebra::Vector2;
use std::fmt;

#[derive(Copy, Clone)]
pub struct Pedestrian {
    pub position: Vector2<f64>,
    pub velocity: Vector2<f64>,
    pub destination: Vector2<f64>,
    uid: usize,
    tau: f64,
    max_speed: f64,
}

impl Pedestrian {
    pub fn new(uid: usize, states: &[f64; 6]) -> Self {
        let velocity = Vector2::new(states[2], states[3]);
        Self {
            position: Vector2::new(states[0], states[1]),
            velocity,
            destination: Vector2::new(states[4], states[5]),
            uid,
            tau: 0.5,
            max_speed: 1.3 * velocity.magnitude(),
        }
    }

    pub fn desired_direction(&self) -> Vector2<f64> {
        (self.destination - self.position).normalize()
    }

    pub fn speed(&self) -> f64 {
        self.velocity.magnitude()
    }

    pub fn step(&mut self, delta: f64) {
        let desired_dir = self.desired_direction();

        // Compute acceleration to desired velocity
        let f0 = (desired_dir * self.speed() - self.velocity) * (1.0 / self.tau);

        // Social force
        let f = f0;

        self.velocity = self.capped_velocity(delta * f);

        self.position += self.velocity * delta;
        // self.position.add_scalar_mut(1.0);

        if self.uid == 2 {
            println!("{}", self.position)
        };
    }

    fn capped_velocity(&self, velocity: Vector2<f64>) -> Vector2<f64> {
        let speed = velocity.magnitude();

        // let factor = cmp::min(1.0, self.max_speed / speed);
        let factor = if self.max_speed / speed > 1.0 {
            1.0
        } else {
            self.max_speed / speed
        };

        velocity * factor
    }
}

impl fmt::Display for Pedestrian {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "uid: {} pos: ({}, {}), vel: ({}, {}), des: ({}, {})",
            self.uid,
            self.position.x,
            self.position.y,
            self.velocity.x,
            self.velocity.y,
            self.destination.x,
            self.destination.y
        )
    }
}