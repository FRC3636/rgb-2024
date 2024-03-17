mod nt;
mod shaders;
mod spi;

use std::error::Error;
use std::sync::{Arc, Mutex};

use nt::{nt_subscription_handler, setup_nt_client, NoteState};
use palette::{Clamp, IntoColor, LinSrgb};
use shaders::intake_indicator;
use shark::point::{primitives::line, Point};
use shark::shader::{FragThree, Shader};
use smart_leds::{SmartLedsWrite, RGB8};

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

    // let mut strip = rs_ws281x::ControllerBuilder::new()
    //     .channel(
    //         0,
    //         rs_ws281x::ChannelBuilder::new()
    //             .pin(STRIP_PORT)
    //             .count(STRIP_LENGTH)
    //             .strip_type(rs_ws281x::StripType::Ws2812)
    //             .brightness(255)
    //             .build(),
    //     )
    //     .build()
    //     .unwrap();
    let spi = spi::SpiDevice::open("/dev/spidev0.0").unwrap();
    let mut strip = ws2812_spi::Ws2812::new(spi);

    let shader = intake_indicator(note_state);
    let start = std::time::Instant::now();
    loop {
        let time = start.elapsed();
        let colors = render(&shader, points(), time.as_secs_f64());
        strip.write(colors).unwrap();
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
                (c.blue * 256.0) as u8,
                (c.green * 256.0) as u8,
                (c.red * 256.0) as u8,
            )
        })
}
