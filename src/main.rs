
//
// image src
//http://yann.lecun.com/exdb/mnist/


mod input;
mod render;
mod net;
mod activation;

use render::Renderer;
use input::InputHandler;
use vecm::vec::Vec2u;


use std::{path::Path, time::Instant, fs::File, io::{BufReader, Read}};

use sdl2::{image::InitFlag, render::Canvas, pixels::Color, rect::Rect};

use crate::input::Control;

pub const IMAGE_HEIGHT: usize = 28;
pub const IMAGE_WIDTH: usize = 28;
pub const DRAWING_INCREMENT: u8 = 5;

struct Images {
    pub amount: u32,
    pub width: u32,
    pub height: u32,
    buffer: Vec<u8>,
    idents: Vec<u8>
}


impl Images {
    pub fn new(images: &Path, labels: &Path) -> Self {
        let images = File::open(images).unwrap();
        let mut image_reader = BufReader::new(images);
        let mut image_buffer = Vec::new();
        image_reader.read_to_end(&mut image_buffer).unwrap();
        let amount = as_u32_be(&image_buffer[4..8]) as u32;
        let height = as_u32_be(&image_buffer[8..12]) as u32;
        let width = as_u32_be(&image_buffer[12..16]) as u32;

        let labels = File::open(labels).unwrap();
        let mut label_reader = BufReader::new(labels);
        let mut label_buffer = Vec::new();
        label_reader.read_to_end(&mut label_buffer).unwrap();

        Self {
            amount,
            width,
            height,
            buffer: image_buffer,
            idents: label_buffer
        }
    }

    //returns images & label
    pub fn get(&self, id: usize) -> ([u8; IMAGE_HEIGHT * IMAGE_WIDTH], u8) {
        let buffer_start = 16 + IMAGE_HEIGHT * IMAGE_WIDTH * id;
        let buffer_end = buffer_start + IMAGE_HEIGHT * IMAGE_WIDTH;
        let mut image = [0; IMAGE_HEIGHT * IMAGE_WIDTH];
        image.copy_from_slice(&self.buffer[buffer_start..buffer_end]);
        return (image, self.idents[8 + id])
    }
}

fn main() -> Result<(), String> {
    let mut rng = rand::thread_rng();
    let font_path = &Path::new("../../res/times-new-roman.ttf");
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;
    let mut window_size = Vec2u::new(560, 660);
    let drawing_space = Vec2u::fill(560);
    let window = video_subsystem.window("ML-Digit-Recognizer", window_size.x, window_size.y)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut renderer = Renderer::new(window);
    let mut event_pump = sdl_context.event_pump()?;
    let mut inputs = InputHandler::new();

    let mut last_frame_time = Instant::now();

    let images = Images::new(&Path::new("./res/minst/train-images-idx3-ubyte"), &Path::new("./res/minst/train-labels-idx1-ubyte"));

    let tile_size = window_size.x / images.width as u32;

    let mut cnt: u32 = 0;
    let mut cooldown: f32 = 0.0;

    let mut drawing_mode = false;
    let mut drawing = [0; IMAGE_WIDTH * IMAGE_HEIGHT];

    'running: loop {
        let dt = determine_deltatime(&mut last_frame_time);
        inputs.handle_events(&mut event_pump);
        if inputs.quit { break 'running; }
        cooldown = (cooldown - dt).max(0.0);
        if inputs.pressed(Control::Down) && cooldown == 0.0 { 
            drawing_mode = !drawing_mode; 
            cooldown = 0.2;
        }

        if inputs.pressed(Control::Color) && cooldown == 0.0 {
            drawing = [0; IMAGE_WIDTH * IMAGE_HEIGHT];
        }

        if drawing_mode {
            draw(&mut drawing, tile_size, drawing_space, &inputs) 
        }

        renderer.start_frame();
        if drawing_mode {
            renderer.draw_digit(&drawing);
        } else {
            renderer.draw_digit(&images.get(cnt as usize).0);
        }
        renderer.draw_border();
        renderer.end_frame();

        if !drawing_mode && cooldown == 0.0 {
            if inputs.pressed(Control::Left) && cnt > 0 {
                cnt -= 1;
                cooldown = 0.2;
            } else if inputs.pressed(Control::Right) && cnt < images.amount {
                cnt += 1;
                cooldown = 0.2;
            }
        }

        //use sdl2::mouse::MouseButton::*;
        //inputs.mouse_up(Left);
        //inputs.mouse_up(Right);
    }
    Ok(())
}

fn draw(drawing: &mut[u8], tile_size: u32, drawing_space: Vec2u, inputs: &InputHandler) {
    if inputs.mouse_pos.x > drawing_space.x || inputs.mouse_pos.y > drawing_space.y {
        return
    }
    let pos = inputs.mouse_pos / Vec2u::fill(tile_size);
    let tile = drawing.get_mut(pos.x as usize + pos.y as usize * IMAGE_WIDTH).unwrap();
    if inputs.left_click && *tile <= 255 - DRAWING_INCREMENT {
                *tile += 5;
    } else if inputs.right_click && *tile >= DRAWING_INCREMENT {
                *tile -= 5;
    }
}


fn determine_deltatime(last_frame_time: &mut Instant) -> f32 {
    let current_frame_time = Instant::now();
    let dt = (current_frame_time - *last_frame_time).as_secs_f32();
    *last_frame_time = current_frame_time;
    return dt;
}


fn as_u32_be(array: &[u8]) -> i32 {
    ((array[0] as i32) << 24) +
    ((array[1] as i32) << 16) +
    ((array[2] as i32) <<  8) +
    ((array[3] as i32) <<  0)
}
