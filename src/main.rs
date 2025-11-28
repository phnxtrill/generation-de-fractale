use image::{Rgb, RgbImage};
use minifb::{Key, Window, WindowOptions};
use std::time::Duration;

// ============================
// STRUCTURES
// ============================

#[derive(Clone, Copy)]
struct Rect {
    x: f64,
    y: f64,
    size: f64,
}

struct Viewport {
    vx: f64,
    vy: f64,
    view_size: f64,
    width: f64,
    height: f64,
}

impl Viewport {
    fn new(width: u32, height: u32, zoom: f64, cx: f64, cy: f64) -> Self {
        let view_size = 1.0 / zoom;

        let vx = cx - view_size / 2.0;
        let vy = cy - view_size / 2.0;

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

// ============================
// DESSIN
// ============================

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

// ============================
// FRACTALE 
// ============================

fn generate_cantor(
    img: &mut RgbImage,
    rect: Rect,
    iter: u32,
    _max_iter: u32, // plus utilisé → underscore
    view: &Viewport,
) {
    // Culling hors écran (optimisation)
    if rect.x + rect.size < view.vx
        || rect.x > view.vx + view.view_size
        || rect.y + rect.size < view.vy
        || rect.y > view.vy + view.view_size
    {
        return;
    }

    let projected_size = (rect.size / view.view_size) * view.width;

    // Couleur des carrés
    let color = Rgb([80, 200, 255]);

    // Condition d'arrêt
    if iter == 0 || projected_size < 1.0 {
        draw_rect(img, rect, color, view);
        return;
    }

    let s = rect.size / 3.0;

    let offsets = [(0.0, 0.0), (2.0, 0.0), (0.0, 2.0), (2.0, 2.0)];
    for (ox, oy) in offsets {
        let new_rect = Rect {
            x: rect.x + ox * s,
            y: rect.y + oy * s,
            size: s,
        };
        generate_cantor(img, new_rect, iter - 1, iter, view);
    }
}


// ============================
// RENDER
// ============================

fn render_frame(
    width: u32,
    height: u32,
    iterations: u32,
    zoom: f64,
    cam_x: f64,
    cam_y: f64,
) -> Vec<u8> {
    let mut img = RgbImage::new(width, height);

    // Couleur du fond
    for pixel in img.pixels_mut() {
        *pixel = Rgb([12, 15, 25]);
    }

    let viewport = Viewport::new(width, height, zoom, cam_x, cam_y);

    let initial_rect = Rect {
        x: 0.0,
        y: 0.0,
        size: 1.0,
    };

    generate_cantor(&mut img, initial_rect, iterations, iterations, &viewport);
    img.into_raw()
}

// ============================
// MAIN GRAPHIQUE
// ============================

fn main() {
    let width = 800;
    let height = 800;

    let mut zoom:f64 = 1.0;
    let mut cam_x = 0.5;
    let mut cam_y = 0.5;
    let mut target_zoom = zoom; // zoom cible pour le lissage

    let base_iter: i32 = 4;
    let max_iter: i32 = 50;
    let mut iterations: u32;
    let mut window = Window::new(
        "Carré de Cantor - Fractale Temps Réel",
        width,
        height,
        WindowOptions::default(),
    )
    .unwrap();

    let mut buffer = vec![0u32; width * height];

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Zoom souris 
        if let Some((_, scroll)) = window.get_scroll_wheel() {
            if scroll > 0.0 {
                target_zoom *= 1.15;
            } else if scroll < 0.0 {
                target_zoom *= 0.87;
            }
        }

        // Lissage du zoom (interpolation)
        zoom += (target_zoom - zoom) * 0.15;

        // Sécurité anti valeurs extrêmes
        zoom = zoom.clamp(0.2, 99999999999999999999999999.0);
        target_zoom = target_zoom.clamp(0.2, 99999999999999999999999999.0);


        // Déplacement de la caméra au clavier
        let speed = 0.01 / zoom; // vitesse adaptative au zoom

        if window.is_key_down(Key::Up) || window.is_key_down(Key::W) {
            cam_y -= speed;
        }
        if window.is_key_down(Key::Down) || window.is_key_down(Key::S) {
            cam_y += speed;
        }
        if window.is_key_down(Key::Left) || window.is_key_down(Key::A) {
            cam_x -= speed;
        }
        if window.is_key_down(Key::Right) || window.is_key_down(Key::D) {
            cam_x += speed;
        }

        // Empêche la caméra de sortir du carré [0,1]
        cam_x = cam_x.clamp(0.0, 1.0);
        cam_y = cam_y.clamp(0.0, 1.0);

        // Itérations dynamiques 
        let zoom_log2 = zoom.log2();
        let mut bonus = zoom_log2.floor() as i32;
        if bonus < 0 {
            bonus = 0;
        }

        let target_iter = (base_iter + bonus).min(max_iter);
        iterations = target_iter as u32;

        // Réinitialisation Caméra + Zoom
        if window.is_key_pressed(Key::R, minifb::KeyRepeat::No) {
            cam_x = 0.5;
            cam_y = 0.5;
            zoom = 1.0;
            target_zoom = 1.0;
            iterations = base_iter as u32;
        }

        let frame = render_frame(width as u32, height as u32, iterations, zoom, cam_x, cam_y);

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
