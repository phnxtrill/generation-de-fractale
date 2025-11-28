use image::{Rgb, RgbImage};
use minifb::{Key, Window, WindowOptions};
use std::time::Duration;

// Carré élémentaire du Cantor exprimé dans l'espace normalisé [0, 1].
#[derive(Clone, Copy)]
struct Rect {
    x: f64,
    y: f64,
    size: f64,
}

// Viewport décrit la fenêtre d'observation (coordonnées normalisées -> pixels).
struct Viewport {
    vx: f64,
    vy: f64,
    view_size: f64,
    width: f64,
    height: f64,
}

impl Viewport {
    fn new(width: u32, height: u32, zoom: f64) -> Self {
        let view_size = 1.0 / zoom;

        // ✅ Zoom vers le coin haut-gauche
        let vx = 0.0;
        let vy = 0.0;

        Self {
            vx,
            vy,
            view_size,
            width: width as f64,
            height: height as f64,
        }
    }

    fn map_x(&self, value: f64) -> f64 {
        ((value - self.vx) / self.view_size) * self.width
    }

    fn map_y(&self, value: f64) -> f64 {
        ((value - self.vy) / self.view_size) * self.height
    }
}

// Remplit un rectangle projeté dans l'image
fn draw_rect(img: &mut RgbImage, rect: Rect, color: Rgb<u8>, view: &Viewport) {
    let img_w = img.width() as i32;
    let img_h = img.height() as i32;

    let x0 = view.map_x(rect.x).floor() as i32;
    let y0 = view.map_y(rect.y).floor() as i32;
    let x1 = view.map_x(rect.x + rect.size).ceil() as i32;
    let y1 = view.map_y(rect.y + rect.size).ceil() as i32;

    let x0 = x0.max(0);
    let y0 = y0.max(0);
    let x1 = x1.min(img_w);
    let y1 = y1.min(img_h);

    if x0 >= x1 || y0 >= y1 {
        return;
    }

    for y in y0..y1 {
        for x in x0..x1 {
            img.put_pixel(x as u32, y as u32, color);
        }
    }
}

// Génère récursivement les sous-carrés du Cantor
fn generate_cantor(
    img: &mut RgbImage,
    rect: Rect,
    iter: u32,
    max_iter: u32,
    view: &Viewport,
) {
    if rect.size <= 0.0 {
        return;
    }

    let projected_size = (rect.size / view.view_size) * view.width;
    if iter == 0 || projected_size < 1.0 {
        let ratio = iter as f64 / max_iter as f64;

        let r = (50.0 + 205.0 * ratio) as u8;
        let g = (80.0 + 120.0 * (1.0 - ratio)) as u8;
        let b = (180.0 + 50.0 * ratio) as u8;

        let color = Rgb([r, g, b]);
        draw_rect(img, rect, color, view);
        return;
    }

    let s = rect.size / 3.0;
    if s <= 0.0 {
        return;
    }

    let offsets = [(0.0, 0.0), (2.0, 0.0), (0.0, 2.0), (2.0, 2.0)];
    for (ox, oy) in offsets {
        let new_rect = Rect {
            x: rect.x + ox * s,
            y: rect.y + oy * s,
            size: s,
        };
        generate_cantor(img, new_rect, iter - 1, max_iter, view);
    }
}

// Fabrique une image correspondant à un zoom donné
fn render_frame(width: u32, height: u32, iterations: u32, zoom: f64) -> Vec<u8> {
    let mut img = RgbImage::new(width, height);

    // ✅ Fond bleu nuit
    for pixel in img.pixels_mut() {
        *pixel = Rgb([12, 15, 25]);
    }

    let viewport = Viewport::new(width, height, zoom);

    let initial_rect = Rect {
        x: 0.0,
        y: 0.0,
        size: 1.0,
    };

    generate_cantor(&mut img, initial_rect, iterations, iterations, &viewport);
    img.into_raw()
}

// =====================
// ✅ MAIN GRAPHIQUE ICI
// =====================
fn main() {
    let width = 800;
    let height = 800;
    let iterations = 6;
    let mut zoom = 1.0;

    let mut window = Window::new(
        "Carré de Cantor - Temps Réel",
        width,
        height,
        WindowOptions::default(),
    )
    .unwrap();

    let mut buffer = vec![0u32; width * height];

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // ✅ Zoom fluide infini
        zoom *= 1.01;

        let frame = render_frame(width as u32, height as u32, iterations, zoom);

        for i in 0..buffer.len() {
            let r = frame[i * 3] as u32;
            let g = frame[i * 3 + 1] as u32;
            let b = frame[i * 3 + 2] as u32;
            buffer[i] = (r << 16) | (g << 8) | b;
        }

        window
            .update_with_buffer(&buffer, width, height)
            .unwrap();

        std::thread::sleep(Duration::from_millis(16)); // ~60 FPS
    }
}
