// Imports essentiels : génération GIF, manipulation d'images RGB, E/S fichier.
use gif::{Encoder, Frame, Repeat};
use image::{Rgb, RgbImage};
use std::fs::File;
use std::io::BufWriter;

// Paramètres d'animation : nombre de frames, progression du zoom et délai.
const FRAME_COUNT: u32 = 40;
const FRAME_DELAY_CS: u16 = 5; // 5 centi-secondes ≈ 50 ms

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
    // Construit la fenêtre à partir de la taille écran et du facteur de zoom.
    fn new(width: u32, height: u32, zoom: f64) -> Self {
        let view_size = 1.0 / zoom;
    
        // ✅ ZOOM VERS LE COIN HAUT-GAUCHE (0,0)
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
    

    // Convertit une abscisse normalisée en pixel horizontal.
    fn map_x(&self, value: f64) -> f64 {
        ((value - self.vx) / self.view_size) * self.width
    }

    // Convertit une ordonnée normalisée en pixel vertical.
    fn map_y(&self, value: f64) -> f64 {
        ((value - self.vy) / self.view_size) * self.height
    }
}

// Remplit un rectangle projeté dans l'image en respectant le viewport courant.
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

// Génère récursivement les sous-carrés du Cantor et les dessine si visibles.
fn generate_cantor(
    img: &mut RgbImage,
    rect: Rect,
    iter: u32,
    max_iter: u32,
    view: &Viewport
)
 {
    if rect.size <= 0.0 {
        return;
    }

    let projected_size = (rect.size / view.view_size) * view.width;
    if iter == 0 || projected_size < 1.0 {
        let ratio = iter as f64 / max_iter as f64;
    
        // 🎨 Dégradé FIXE, parfait sur fond blanc
        let r = (50.0 + 205.0 * ratio) as u8;
        let g = (80.0 + 120.0 * (1.0 - ratio)) as u8;
        let b = (180.0 + 50.0 * ratio) as u8;
    
        let color = Rgb([r, g, b]);
        draw_rect(img, rect, color, view);
        return;
    }
    

    let s = rect.size / 3.0;
    if s <= 0.0 {
        let ratio = iter as f64 / max_iter as f64;

        let r = (50.0 + 205.0 * ratio) as u8;
        let g = (80.0 + 120.0 * (1.0 - ratio)) as u8;
        let b = (180.0 + 50.0 * ratio) as u8;

        let color = Rgb([r, g, b]);
        draw_rect(img, rect, color, view);
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

// Fabrique une image correspondant à un zoom donné et renvoie le buffer brut.
fn render_frame(width: u32, height: u32, iterations: u32, zoom: f64) -> Vec<u8> {
    let mut img = RgbImage::new(width, height);
    for pixel in img.pixels_mut() {
        *pixel = Rgb([255, 255, 255]);
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

fn main() {
    // Récupère et valide les arguments CLI (largeur, itérations, sortie optionnelle).
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <largeur> <iterations> [fichier_sortie]", args[0]);
        eprintln!("Exemple: {} 512 4", args[0]);
        std::process::exit(1);
    }

    let width: u32 = args[1].parse().expect("La largeur doit être un nombre");
    let height = width;
    let iterations: u32 = args[2].parse().expect("Le nombre d'itérations doit être un nombre");
    let output_file = args.get(3).map(|s| s.as_str()).unwrap_or("cantor.gif");

    println!(
        "Génération d'un carré de Cantor {}x{} avec {} itérations...",
        width, height, iterations
    );

    // Prépare le fichier GIF et l'encodeur avec répétition infinie.
    let file = File::create(output_file).expect("Impossible de créer le fichier GIF");
    let writer = BufWriter::new(file);
    let mut encoder = Encoder::new(writer, width as u16, height as u16, &[])
        .expect("Impossible d'initialiser l'encodeur GIF");
    encoder
        .set_repeat(Repeat::Infinite)
        .expect("Impossible de configurer la répétition du GIF");

    // Boucle d'animation : calcule le zoom, rend l'image et ajoute la frame GIF.
    for frame_idx in 0..FRAME_COUNT {
        let t = frame_idx as f64 / FRAME_COUNT as f64;
    
        // ✅ zoom autosimilaire parfaitement loopable
        let zoom = 3.0_f64.powf(t);
    
        let frame_pixels = render_frame(width, height, iterations, zoom);
    
        let mut frame = Frame::from_rgb(width as u16, height as u16, &frame_pixels);
        frame.delay = FRAME_DELAY_CS;
    
        encoder.write_frame(&frame)
            .expect("Impossible d'ajouter une frame au GIF");
    
        println!("Frame {} générée avec zoom {:.3}", frame_idx + 1, zoom);
    }
    
    // Message de fin confirmant la création du GIF animé.
    println!("GIF sauvegardé dans {}", output_file);
}
