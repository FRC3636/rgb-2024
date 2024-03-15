use std::sync::{Arc, Mutex};

use crate::nt::NoteState;
use palette::Srgb;
use shark::shader::{FragOne, FragThree, IntoShader, Shader, ShaderExt};

// This is cursed and bad because of how networktables works but it's the only part of the intake indicator that will be infected by it
pub fn intake_indicator(note_state: Arc<Mutex<NoteState>>) -> impl Shader<FragThree> {
    (move |_frag: FragOne| match *note_state.lock().unwrap() {
        NoteState::None => Srgb::new(0.0, 0.0, 0.0),
        NoteState::Handoff => Srgb::new(1.0, 0.0, 0.0),
        NoteState::Shooter => Srgb::new(0.0, 1.0, 0.0),
    })
    .into_shader()
    .extrude()
    .extrude()
}
