use std::sync::{Arc, Mutex};

use crate::nt::NoteState;
use palette::{IntoColor, LinSrgb, Srgb};
use shark::shader::{
    primitives::{checkerboard, color},
    FragOne, FragThree, IntoShader, Shader, ShaderExt,
};

pub fn slide_over_time<S: Shader<FragOne>>(shader: S) -> impl Shader<FragOne> {
    (move |frag: FragOne| {
        let new_pos = frag.pos + frag.time;
        shader.shade(FragOne {
            pos: new_pos,
            ..frag
        })
    })
    .into_shader()
}

// This is cursed and bad because of how networktables works but it's the only part of the intake indicator that will be infected by it
pub fn intake_indicator(note_state: Arc<Mutex<NoteState>>) -> impl Shader<FragThree> {
    (move |frag: FragOne| {
        // println!("{:?}", frag.pos);
        match *note_state.lock().unwrap() {
            NoteState::None => LinSrgb::new(0.0, 0.0, 0.0),
            NoteState::Handoff => slide_over_time(checkerboard(
                color(Srgb::new(1.0, 0.35, 0.0)),
                color(Srgb::new(1.0, 1.0, 1.0)),
                0.03429,
            ))
            .scale_time(0.8)
            .shade(frag)
            .into_color(),
            // NoteState::Handoff => LinSrgb::new(frag.pos, 0.0, 0.0),
            NoteState::Shooter => LinSrgb::new(0.0, 1.0, 0.0),
        }
    })
    .into_shader()
    .extrude()
    .extrude()
}
