pub mod nt;

use std::error::Error;

use nt::setup_nt_client;
use palette::{Clamp, IntoColor, LinSrgb, Srgb};
use shark::point::{primitives::line, Point};
use shark::shader::{
    primitives::{checkerboard, time_rainbow},
    FragOne, FragThree, Fragment, IntoShader, Shader, ShaderExt,
};

fn shader() -> impl Shader<FragThree> {
    let toggle = (|frag: FragOne| {
        if frag.time() % 2.0 > 1.0 {
            Srgb::new(1.0, 1.0, 1.0)
        } else {
            Srgb::new(0.0, 0.0, 0.0)
        }
    })
    .into_shader();
    let checkerboard = checkerboard(toggle, time_rainbow().scale_time(40.0), 0.2001);
    checkerboard.extrude().extrude()
}

fn points() -> impl Iterator<Item = Point> {
    line(
        Point::new(0.0, 0.0, 0.0),
        Point::new((STRIP_LENGTH / LEDS_PER_METER) as _, 0.0, 0.0),
        STRIP_LENGTH as _,
    )
}

const STRIP_PORT: i32 = 10;
const STRIP_LENGTH: i32 = 100;
const LEDS_PER_METER: i32 = 144;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (client, subscription) = setup_nt_client().await.unwrap();

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
    let shader = shader();
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
                (c.red * 255.0) as u8,
                (c.green * 255.0) as u8,
                (c.blue * 255.0) as u8,
                0,
            ]
        })
}
