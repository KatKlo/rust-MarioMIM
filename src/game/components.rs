use bevy::prelude::*;
use rand::{Rng, SeedableRng, thread_rng};
use rand::distributions::Alphanumeric;
use rand_pcg::Pcg64;
use rand_seeder::Seeder;

#[derive(Component, Copy, Clone)]
pub enum GameDirection {
    Left,
    Right,
}

#[derive(Component)]
pub struct FinishLine;

#[derive(Component)]
pub struct Wall;

#[derive(Component)]
pub struct WeakBullet;

#[derive(Component)]
pub struct StrongBullet;

#[derive(Component)]
pub enum Weapon {
    WeakBullet,
    StrongBullet,
}

#[derive(Component)]
pub struct Bullet;

#[derive(Component)]
pub struct Jumper {
    pub jump_impulse: f32,
    pub is_jumping: bool,
}

impl Default for Jumper {
    fn default() -> Self {
        Jumper {
            jump_impulse: 13.0,
            is_jumping: false,
        }
    }
}

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct Powerup;

#[derive(Component)]
pub struct Coffee;

#[derive(Component)]
pub struct Rust;

#[derive(Component)]
pub struct Bug {
    pub speed: f32,
    pub facing_direction: GameDirection,
}

impl Default for Bug {
    fn default() -> Self {
        Bug {
            speed: 2.0,
            facing_direction: GameDirection::Right,
        }
    }
}

#[derive(Component)]
pub struct Enemy;

#[derive(Debug, Component, PartialEq, Eq, Clone)]
pub struct Random {
    pub generator: Pcg64,
    pub seed: String,
    pub can_change: bool,
}

impl Random {
    pub fn new() -> Self {
        Random {
            generator: Pcg64::from_entropy(),
            seed: String::new(),
            can_change: false,
        }
    }

    fn generate_random_seed() -> String {
        thread_rng()
            .sample_iter(&Alphanumeric)
            .take(8)
            .map(char::from)
            .collect()
    }

    pub fn add_char(&mut self, c: char) {
        if self.can_change {
            self.seed.push(c);
        }
    }

    pub fn delete_last(&mut self) {
        if self.can_change {
            self.seed.pop();
        }
    }

    pub fn new_random_seed(&mut self) {
        self.seed = Random::generate_random_seed();
    }

    pub fn reset(&mut self) {
        self.generator = Seeder::from(&self.seed).make_rng();
    }
}
#[derive(Component, Default)]
pub struct LivingBeing;

#[derive(Component)]
pub struct PhantomEntity;

