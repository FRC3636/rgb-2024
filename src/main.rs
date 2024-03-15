pub mod nt;
pub mod shaders;

use std::error::Error;
use std::sync::{Arc, Mutex};

use nt::{nt_subscription_handler, setup_nt_client, NoteState};
use palette::{Clamp, IntoColor, LinSrgb};
use shaders::intake_indicator;
use shark::point::{primitives::line, Point};
use shark::shader::{FragThree, Shader};

fn points() -> impl Iterator<Item = Point> {
    line(
        Point::new(0.0, 0.0, 0.0),
        Point::new(1.0, 0.0, 0.0),
        STRIP_LENGTH as _,
    )
}

const STRIP_PORT: i32 = 10;
const STRIP_LENGTH: i32 = 100;
const LEDS_PER_METER: i32 = 144;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (_client, subscription) = setup_nt_client().await.unwrap();

    let note_state = Arc::new(Mutex::new(NoteState::None));

    tokio::spawn(nt_subscription_handler(subscription, note_state.clone()));

    let mut strip = rs_ws281x::ControllerBuilder::new()
        .channel(
            0,
            rs_ws281x::ChannelBuilder::new()
                .pin(STRIP_PORT)
                .count(STRIP_LENGTH)
                .strip_type(rs_ws281x::StripType::Ws2812)
                .brightness(255)
                .build(),
        )
        .build()
        .unwrap();

    let shader = intake_indicator(note_state);
    let start = std::time::Instant::now();
    loop {
        let time = start.elapsed();
        let colors = render(&shader, points(), time.as_secs_f64()).collect::<Vec<[u8; 4]>>();
        let leds = strip.leds_mut(0);
        leds.copy_from_slice(colors.as_slice());
        strip.render().unwrap();
    }
}

fn render<'a>(
    shader: &'a impl Shader<FragThree>,
    points: impl Iterator<Item = Point> + 'a,
    time: f64,
) -> impl Iterator<Item = [u8; 4]> + 'a {
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
            [
                (c.blue * 255.0) as u8,
                (c.green * 255.0) as u8,
                (c.red * 255.0) as u8,
                0,
            ]
        })
}
