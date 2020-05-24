use sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

pub struct Game {
    pub sdl_context: sdl2::Sdl,
    pub canvas: sdl2::render::Canvas<sdl2::video::Window>,
    pub running: bool,
}

impl Game {
    pub fn new()-> Game {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = &sdl_context.video().unwrap();
        let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
            .position_centered()
            .build()
            .unwrap();
        let canvas = window.into_canvas().build().unwrap();
        let running = true;
        Game {
            sdl_context,
            canvas,
            running
        }
    }

    pub fn event(&mut self) {
        let event_pump = &mut self.sdl_context.event_pump().unwrap();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                     self.running = false;
                },
                _ => {}
            }
        }
    }

    pub fn update(&mut self) {
        println!("updating");
    }

    pub fn render(&mut self) {
        self.canvas.set_draw_color(Color::RGB(125, 125, 125));
        self.canvas.clear();
        self.canvas.present();
    }
}