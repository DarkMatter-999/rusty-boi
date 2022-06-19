use std::thread::sleep;
use std::time::{Instant, Duration};
use std::{io::Read, env};
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::ttf::Font;
use sdl2::video::Window;
use sdl2::keyboard::Keycode;
use sdl2::render::TextureQuery;

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

    // // Debug
    // let window2 = video_subsystem.window("MemView", 500, 500).position_centered()
    // .opengl()
    // .build()
    // .unwrap();

    // let ttf_context = sdl2::ttf::init().unwrap();
    // let mut canvas2 = window2.into_canvas().build().unwrap();
    // let texture_creator = canvas2.texture_creator();
    // let mut font = ttf_context.load_font(&args[3], 128).unwrap();
    // font.set_style(sdl2::ttf::FontStyle::BOLD);

    
    // let mut ins: [u8; 10] = [0; 10];

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
            // ins = get_mem(&cpu);
            // draw_debug(&mut canvas2, &mut font, ins);
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

fn draw_debug(canvas2: &mut Canvas<Window>, font: &mut Font, ins: [u8; 10]) {
    let strs: Vec<String> = ins.iter()
                               .map(|b| format!("0x{:x}", b))
                               .collect();
    let strs = strs.join("\n");

    let texture_creator = canvas2.texture_creator();
    
    font.set_style(sdl2::ttf::FontStyle::BOLD);

    // render a surface, and convert it to a texture bound to the canvas
    let mut surface = font
        .render(&strs)
        .blended(Color::RGBA(255, 0, 0, 255))
        .unwrap();
    let mut texture = texture_creator
        .create_texture_from_surface(&surface)
        .unwrap();

    canvas2.set_draw_color(Color::RGBA(195, 217, 255, 255));
    canvas2.clear();

    let TextureQuery { width, height, .. } = texture.query();

    canvas2.copy(&texture, None, Some(Rect::new(1, 1, 500, 50))).unwrap();
    canvas2.present();
}
fn get_mem(cpu: &CPU) -> [u8; 10] {
    let mut data: [u8; 10] = [0; 10];

    for x in 0..10 {
        data[x] = cpu.bus.read_byte(cpu.pc + (x as u16));
    }

    data
}