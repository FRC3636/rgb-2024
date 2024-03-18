mod nt;
mod shaders;
mod spi;
mod strips;

use std::error::Error;
use std::sync::{Arc, Mutex};

use nt::{nt_subscription_handler, setup_nt_client, NoteState};
use palette::{Clamp, IntoColor, LinSrgb};
use shaders::intake_indicator;
use shark::point::Point;
use shark::shader::{FragThree, Shader};
use smart_leds::{SmartLedsWrite, RGB8};

const TARGET_FPS: f64 = 144.0;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (_client, subscription) = setup_nt_client().await?;

    let note_state = Arc::new(Mutex::new(NoteState::None));

    tokio::spawn(nt_subscription_handler(subscription, note_state.clone()));

    let mut intake_indicator_left = strips::gpio_10()?;
    let mut intake_indicator_right = strips::gpio_18()?;

    let indicator_shader = intake_indicator(note_state);

    let target_frame_time = std::time::Duration::from_secs_f64(1.0 / TARGET_FPS);

    let loop_start = std::time::Instant::now();
    loop {
        let frame_start = std::time::Instant::now();
        let time = loop_start.elapsed();

        let colors = render(
            &indicator_shader,
            strips::intake_indicator(),
            time.as_secs_f64(),
        );
        intake_indicator_left.write(colors).unwrap();
        let colors = render(
            &indicator_shader,
            strips::intake_indicator(),
            time.as_secs_f64(),
        );
        intake_indicator_right.write(colors).unwrap();

        if let Some(remaining) = target_frame_time.checked_sub(frame_start.elapsed()) {
            tokio::time::sleep(remaining).await;
        }
    }
}

fn render<'a>(
    shader: &'a impl Shader<FragThree>,
    points: impl Iterator<Item = Point> + 'a,
    time: f64,
) -> impl Iterator<Item = RGB8> + 'a {
    points
        .map(move |point| {
            shader.shade(FragThree {
                pos: [point.x, point.y, point.z],
                time,
            })
        })
        .map(|c| {
            let c: LinSrgb<f64> = c.into_color();
            c.clamp()
        })
        .map(|c| {
            RGB8::new(
                (c.red * 256.0) as u8,
                (c.green * 256.0) as u8,
                (c.blue * 256.0) as u8,
            )
        })
}
