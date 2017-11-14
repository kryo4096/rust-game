use std::collections::*;
use math;

struct Axis {
    negative_key : u32,
    positive_key : u32,
    gravity      : f32,
    value        : f32,
}

pub struct InputManager<'a> {
    keys          : HashSet<u32>,
    keys_pressed  : HashSet<u32>,
    keys_released : HashSet<u32>,
    axes          : HashMap<&'a str, Axis>,
}

impl<'a> InputManager<'a> {

    pub fn new() -> Self {
        let keys = HashSet::new();
        let keys_pressed = HashSet::new();
        let keys_released = HashSet::new();
        let axes = HashMap::new();

        Self {keys, keys_pressed, keys_released, axes}
    }

    pub fn press(&mut self, scancode: u32) {
        self.keys_pressed.insert(scancode);
        self.keys.insert(scancode);
    }

    pub fn release(&mut self, scancode: u32) {
        self.keys_released.insert(scancode);
        self.keys.remove(&scancode);
    }

    pub fn update(&mut self, delta: f32) {

        let keys = &self.keys;

        for (_, mut axis) in self.axes.iter_mut() {

            let mut target = 0.;

            if keys.contains(&axis.positive_key) {
                target += 1.;
            }

            if keys.contains(&axis.negative_key) {
                target -= 1.;
            }

            axis.value = math::lerp(axis.value, target, axis.gravity * delta);
        }

        self.keys_pressed.clear();
        self.keys_released.clear();
    }

    pub fn get_key(&self, scancode: u32) -> bool {
        self.keys.contains(&scancode)
    }

    pub fn get_key_pressed(&self, scancode: u32) -> bool {
        self.keys_pressed.contains(&scancode)
    }

    pub fn get_key_released(&self, scancode: u32) -> bool {
        self.keys_released.contains(&scancode)
    }

    pub fn register_axis(&mut self, name: &'a str, negative_key: u32, positive_key: u32, gravity: f32) {
        self.axes.insert(name, Axis {
            negative_key,
            positive_key,
            gravity,
            value: 0.,
        });
    }

    pub fn remove_axis(&mut self, name: &'a str) {
        self.axes.remove(name);
    }

    pub fn axis(&self, name: &'a str) -> f32 {
        self.axes.get(name).unwrap().value
    }
}
