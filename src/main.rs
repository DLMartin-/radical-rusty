extern crate sdl2;

use sdl2::event::Event;
use sdl2::pixels::Color;
use std::time::Duration;

pub fn main() {
    let _ = game_main();
}

#[derive(Debug)]
enum GameError {
    SdlInitialization,
    WindowCreation,
    CanvasCreation,
    EventPumpInitialization,
}

type GameResult = Result<(), GameError>;

fn game_main() -> GameResult {
    let sdl_context = init_sdl()?;
    let mut graphics_context = init_graphics(&sdl_context)?;
    let mut event_pump = init_event_pump(&sdl_context)?;

    game_loop(&mut graphics_context, &mut event_pump)
}

fn init_sdl() -> Result<sdl2::Sdl, GameError> {
    match sdl2::init() {
        Ok(sdl) => Ok(sdl),
        Err(_) => Err(GameError::SdlInitialization),
    }
}

fn init_event_pump(sdl_context: &sdl2::Sdl) -> Result<sdl2::EventPump, GameError> {
    match sdl_context.event_pump() {
        Ok(event_pump) => Ok(event_pump),
        Err(_) => Err(GameError::EventPumpInitialization),
    }
}

struct GraphicsContext {
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
}

fn init_graphics(sdl_context: &sdl2::Sdl) -> Result<GraphicsContext, GameError> {
    let video_subsystem = sdl_context.video().unwrap();

    let window = match video_subsystem
        .window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
    {
        Ok(window) => window,
        Err(_) => return Err(GameError::WindowCreation),
    };

    let canvas = match window.into_canvas().build() {
        Ok(canvas) => canvas,
        Err(_) => return Err(GameError::CanvasCreation),
    };

    Ok(GraphicsContext { canvas })
}

fn game_loop(
    graphics_context: &mut GraphicsContext,
    event_pump: &mut sdl2::EventPump,
) -> Result<(), GameError> {
    let canvas = &mut graphics_context.canvas;
    let mut i: u8 = 155;
    'game_loop: loop {
        i = i.wrapping_add(1);
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'game_loop,
                _ => {}
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
