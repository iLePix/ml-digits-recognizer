use sdl2::{render::Canvas, video::Window, pixels::Color, rect::Rect};

use crate::{IMAGE_HEIGHT, IMAGE_WIDTH};


pub struct Renderer {
    canvas: Canvas<sdl2::video::Window>,
}

impl Renderer {
    pub fn new(window: Window) -> Self {
        let mut canvas: Canvas<sdl2::video::Window> = window.into_canvas().build()
            .expect("could not create a canvas");
        canvas.set_blend_mode(sdl2::render::BlendMode::Blend);
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        Self {
            canvas
        }
    }

    pub fn draw_digit(&mut self, image: &[u8; IMAGE_HEIGHT * IMAGE_WIDTH]) {
        let tile_size: u32 = 20;
        for y in 0..IMAGE_HEIGHT {
            for x in 0..IMAGE_WIDTH {
                let luma = image[x + y * IMAGE_WIDTH] as u8;
                self.canvas.set_draw_color(Color::RGB(luma, luma, luma));
                self.canvas.fill_rect(Rect::new((x as u32 * tile_size) as i32, (y as u32 * tile_size) as i32, tile_size, tile_size)).unwrap();
            }
        }
    }

    pub fn draw_border(&mut self) {
        self.canvas.set_draw_color(Color::RED);
        self.canvas.fill_rect(Rect::new(0, 561, 560, 3)).unwrap();
    } 

    pub fn start_frame(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
    }

    pub fn end_frame(&mut self) {
        self.canvas.present();
    }
}
