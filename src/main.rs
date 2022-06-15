use std::thread::sleep;
use std::time::{Instant, Duration};
use std::{io::Read, env};
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::keyboard::Keycode;

use rb_core::*;

const SCREEN_WIDTH: usize = CPU::getRESW();
const SCREEN_HEIGHT: usize = CPU::getRESH();
const SCALE: u32 = 4;
const WINDOW_WIDTH: u32 = (SCREEN_WIDTH as u32 + 10) * SCALE;
const WINDOW_HEIGHT: u32 = (SCREEN_HEIGHT as u32 + 10) * SCALE;
const TICKS_PER_FRAME: usize = 70224;

const ONE_SECOND_IN_MICROS: usize = 1000000000;
const ONE_SECOND_IN_CYCLES: usize = 4190000;
const NUMBER_OF_PIXELS: usize = 23040;

fn buffer_from_file(path: &str) -> Vec<u8> {
    let mut file = std::fs::File::open(path).expect("File not there");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Could not read file");
    // println!("{:?}", buffer);
    buffer
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let bootrombuffer = buffer_from_file(&args[1]);
    let rombuffer = buffer_from_file(&args[2]);

    // Setup SDL
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
    .window("GameBoy Emulator", WINDOW_WIDTH, WINDOW_HEIGHT)
    .position_centered()
    .opengl()
    .build()
    .unwrap();
    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut cpu = CPU::new(Some(bootrombuffer), rombuffer);
    
    let mut cycles_elapsed_in_frame = 0usize;
    let mut now = Instant::now();
    'running: loop {
        for evt in event_pump.poll_iter() {
            match evt {
                Event::Quit{..}  | Event::KeyDown{keycode: Some(Keycode::Escape), ..} => { 
                    break 'running;
                },
                _ => ()
            }
        }
        let time_delta = now.elapsed().subsec_nanos();
        now = Instant::now();
        let delta = time_delta as f64 / ONE_SECOND_IN_MICROS as f64;
        let cycles_to_run = delta * ONE_SECOND_IN_CYCLES as f64;

        let mut cycles_elapsed = 0;
        while cycles_elapsed <= cycles_to_run as usize {
            cycles_elapsed += 1;
            cpu.step();
            // sleep(Duration::from_millis(100));
        }
        draw_screen(&cpu, &mut canvas);

        cycles_elapsed_in_frame += cycles_elapsed;

        // TODO: Consider updating buffer after every line is rendered.
        if cycles_elapsed_in_frame >= TICKS_PER_FRAME {
            draw_screen(&cpu, &mut canvas);
        }

    }
}

fn draw_screen(cpu: &CPU, canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    // canvas.clear();
    // let screen_buf = cpu.bus.gpu.canvas_buffer;
    // Now set draw color to white, iterate through each point and see if it should be drawn
    // canvas.set_draw_color(Color::RGB(255, 255, 255));

    for (i, pixel) in cpu.bus.gpu.canvas_buffer.chunks(4).enumerate() {
        // Convert our 1D array's index into a 2D (x,y) position
        let x = (i % SCREEN_WIDTH) as u32;
        let y = (i / SCREEN_WIDTH) as u32;

        // buffer[i] = (pixel[3] as u32) << 24
        //     | (pixel[2] as u32) << 16
        //     | (pixel[1] as u32) << 8
        //     | (pixel[0] as u32);

        canvas.set_draw_color(Color::RGBA(pixel[0], pixel[1], pixel[2], pixel[3]));

        // Draw a rectangle at (x,y), scaled up by our SCALE value
        let rect = Rect::new((x * SCALE) as i32, (y * SCALE) as i32, SCALE, SCALE);
        canvas.fill_rect(rect).unwrap();
    }
    canvas.present();
}
