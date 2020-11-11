mod utils;
mod pedestrian;

use wasm_bindgen::prelude::*;

use crate::pedestrian::Pedestrian;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, hello-wasm!");
}

#[wasm_bindgen]
pub fn simulation() {
    const STEPS: i32 = 35;

    let initial_states = vec![
        [0.0, 0.0, 1.0, 0.0, 0.0, 10.0],
        [-0.3, 10.0, -1.0, 0.0, -0.3, 0.0],
    ];

    let mut pedestrians: Vec<Pedestrian> = initial_states
        .iter()
        .zip(1..initial_states.len() + 1)
        .map(|(states, uid)| Pedestrian::new(uid, states))
        .collect();

    for i in 0..STEPS {
        const DELTA: f64 = 1.0;
        pedestrians.iter_mut().for_each(|ped| {
            ped.step(DELTA);
        });

        let filename = &["step", i.to_string().as_str()].concat();
        alert(filename);
    }
}