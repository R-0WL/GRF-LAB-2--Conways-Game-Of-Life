use minifb::{Key, KeyRepeat, Window, WindowOptions};
use std::thread::sleep;
use std::time::Duration;

const FB_WIDTH: usize = 100;
const FB_HEIGHT: usize = 100;
const SCALE: usize = 8;

const WIN_WIDTH: usize = FB_WIDTH * SCALE;
const WIN_HEIGHT: usize = FB_HEIGHT * SCALE;

const BLACK: u32 = 0x000000;
const WHITE: u32 = 0xFFFFFF;

struct Renderer {
    framebuffer: Vec<u32>,
    generation: u64,
}
const PESCADOR: &str = include_str!("../pescador_100x100.txt");

fn load_pescador(renderer: &mut Renderer) {
    for (y, line) in PESCADOR.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '#' {
                renderer.point(x, y, WHITE);
            }
        }
    }
}

impl Renderer {

    fn new() -> Self {

        Self {
            framebuffer: vec![BLACK; FB_WIDTH * FB_HEIGHT],
            generation: 0,
        }

    }

    fn clear(&mut self) {

        for pixel in self.framebuffer.iter_mut() {
            *pixel = BLACK;
        }

    }

    fn point(&mut self, x: usize, y: usize, color: u32) {

        if x < FB_WIDTH && y < FB_HEIGHT {

            self.framebuffer[y * FB_WIDTH + x] = color;

        }

    }

    fn get_color(&self, x: usize, y: usize) -> u32 {

        if x < FB_WIDTH && y < FB_HEIGHT {

            self.framebuffer[y * FB_WIDTH + x]

        } else {

            BLACK

        }

    }

    fn load_pattern(&mut self, filename: &str) {

        self.clear();

        let text = std::fs::read_to_string(filename)
            .expect("No se pudo abrir el patrón");

        for (y, line) in text.lines().enumerate() {

            if y >= FB_HEIGHT {
                break;
            }

            for (x, ch) in line.chars().enumerate() {

                if x >= FB_WIDTH {
                    break;
                }

                if ch == '#' {
                    self.point(x, y, WHITE);
                }

            }

        }

        self.generation = 0;

    }

    fn alive_neighbors(&self, x: usize, y: usize) -> usize {

        let mut count = 0;

        for dy in -1i32..=1 {

            for dx in -1i32..=1 {

                if dx == 0 && dy == 0 {
                    continue;
                }

                let nx = x as i32 + dx;
                let ny = y as i32 + dy;

                if nx >= 0 &&
                   ny >= 0 &&
                   nx < FB_WIDTH as i32 &&
                   ny < FB_HEIGHT as i32 {

                    if self.get_color(nx as usize, ny as usize) == WHITE {

                        count += 1;

                    }

                }

            }

        }

        count

    }

        fn render(&mut self) {

        let mut next = self.framebuffer.clone();

        for y in 0..FB_HEIGHT {

            for x in 0..FB_WIDTH {

                let alive = self.get_color(x, y) == WHITE;
                let neighbors = self.alive_neighbors(x, y);

                let index = y * FB_WIDTH + x;

                next[index] = match (alive, neighbors) {

                    (true, 2) | (true, 3) => WHITE,

                    (true, _) => BLACK,

                    (false, 3) => WHITE,

                    _ => BLACK,

                };

            }

        }

        self.framebuffer = next;

        self.generation += 1;

    }

    fn draw(&self, window: &mut Window) {

        let mut screen = vec![BLACK; WIN_WIDTH * WIN_HEIGHT];

        for y in 0..FB_HEIGHT {

            for x in 0..FB_WIDTH {

                let color = self.get_color(x, y);

                for sy in 0..SCALE {

                    for sx in 0..SCALE {

                        let px = x * SCALE + sx;
                        let py = y * SCALE + sy;

                        screen[py * WIN_WIDTH + px] = color;

                    }

                }

            }

        }

        window
            .update_with_buffer(
                &screen,
                WIN_WIDTH,
                WIN_HEIGHT,
            )
            .unwrap();

    }

}




fn main() {

    let mut renderer = Renderer::new();
//mi patrón inicial :)
    renderer.load_pattern("pescador_100x100.txt");

    let mut window = Window::new(
        "Conway's Game of Life",
        WIN_WIDTH,
        WIN_HEIGHT,
        WindowOptions::default(),
    )
    .unwrap();
    renderer.draw(&mut window);
   
    sleep(Duration::from_secs(2));
    let mut paused = false;

    while window.is_open() && !window.is_key_down(Key::Escape) {

        if window.is_key_pressed(Key::Space, KeyRepeat::No) {

            paused = !paused;

        }

        if window.is_key_pressed(Key::R, KeyRepeat::No) {

           load_pescador(&mut renderer);

        }

        if !paused {

            renderer.render();

        }

        window.set_title(
            &format!(
                "Conway's Game of Life | Generación {}{}",
                renderer.generation,
                if paused { " (PAUSA)" } else { "" }
            )
        );

        renderer.draw(&mut window);

        sleep(Duration::from_millis(100));

    }

}