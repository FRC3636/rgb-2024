use std::sync::{Arc, Mutex};

use crate::nt::NoteState;
use noise::NoiseFn;
use palette::{IntoColor, LinSrgb, Okhsl, Srgb};
use shark::shader::{
    primitives::{checkerboard, color},
    FragOne, FragThree, Fragment, IntoShader, Shader, ShaderExt,
};

fn noise(generator: impl NoiseFn<f64, 2>) -> impl Shader<FragOne> {
    (move |frag: FragOne| {
        let val = generator.get([frag.pos * 25.0, frag.time()]);
        Okhsl::new(0.0, 0.0, val.abs())
    })
    .into_shader()
}

fn slide_over_time<S: Shader<FragOne>>(shader: S) -> impl Shader<FragOne> {
    (move |frag: FragOne| {
        let new_pos = frag.pos + frag.time;
        shader.shade(FragOne {
            pos: new_pos,
            ..frag
        })
    })
    .into_shader()
}

fn conveyor<S1: Shader<FragOne>, S2: Shader<FragOne>>(
    shader1: S1,
    shader2: S2,
    section_len: f64,
    speed: f64,
) -> impl Shader<FragOne> {
    slide_over_time(checkerboard(shader1, shader2, section_len)).scale_time(speed)
}

fn boil<'a>(
    perlin: &'a noise::Perlin,
    mul: impl Shader<FragOne> + 'a,
) -> impl Shader<FragOne> + 'a {
    noise(perlin)
        .add(color(Okhsl::new(0.0, 0.0, 0.35)))
        .multiply(mul)
}

// This is cursed and bad because of how networktables works but it's the only part of the intake indicator that will be infected by it
pub fn intake_indicator(note_state: Arc<Mutex<NoteState>>) -> impl Shader<FragThree> {
    let perlin = noise::Perlin::new(0);
    (move |frag: FragOne| {
        // println!("{:?}", frag.pos);
        match *note_state.lock().unwrap() {
            NoteState::None => boil(&perlin, color(LinSrgb::new(0.0, 0.0, 0.8)))
                .shade(frag)
                .into_color(),
            NoteState::Handoff => conveyor(
                color(Srgb::new(1.0, 0.35, 0.0)),
                color(Srgb::new(1.0, 1.0, 1.0)),
                0.1,
                0.5,
            )
            .shade(frag)
            .into_color(),
            // NoteState::Handoff => LinSrgb::new(frag.pos, 0.0, 0.0),
            NoteState::Shooter => boil(&perlin, color(LinSrgb::new(0.0, 1.0, 0.0)))
                .shade(frag)
                .into_color(),
        }
    })
    .into_shader()
    .extrude()
    .extrude()
}
