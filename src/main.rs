use image::{RgbImage, Rgb};



// Structure pour représenter un rectangle/carré
struct Rect {
    x: i32,
    y: i32,
    size: i32,
}

// Fonction pour dessiner un rectangle sur l'image
fn draw_rect(img: &mut RgbImage, rect: Rect, color: Rgb<u8>) {
    // Récupérer les dimensions de l'image
    let img_w = img.width() as i32;
    let img_h = img.height() as i32;
    
    // Calculer les coordonnées du rectangle en tenant compte des limites
    // On utilise max(0) pour éviter les coordonnées négatives
    let x0 = rect.x.max(0);
    let y0 = rect.y.max(0);
    
    // On utilise min() pour éviter de dépasser les limites de l'image
    let x1 = (rect.x + rect.size).min(img_w);
    let y1 = (rect.y + rect.size).min(img_h);
    
    // Si le rectangle est complètement hors écran, on ne fait rien
    if x0 >= x1 || y0 >= y1 {
        return;
    }
    
    // Dessiner pixel par pixel
    for y in y0..y1 {
        for x in x0..x1 {
            img.put_pixel(x as u32, y as u32, color);
        }
    }
}

// Fonction récursive pour générer le carré de Cantor
fn generate_cantor(img: &mut RgbImage, rect: Rect, iter: u32, color: Rgb<u8>) {
    // Condition d'arrêt 1 : taille invalide
    if rect.size <= 0 {
        return;
    }
    
    // Condition d'arrêt 2 : plus d'itérations ou taille trop petite
    if iter == 0 || rect.size < 1 {
        draw_rect(img, rect, color);
        return;
    }
    
    // Calculer la taille des sous-carrés (division par 3)
    let s = rect.size / 3;
    
    // Si la taille devient 0, on dessine le rectangle actuel
    if s == 0 {
        draw_rect(img, rect, color);
        return;
    }
    
    // Les 4 coins de la grille 3x3 (on saute le centre)
    // Positions relatives : (0,0), (2,0), (0,2), (2,2)
    let offsets = [(0, 0), (2, 0), (0, 2), (2, 2)];
    
    // Pour chaque coin, créer un nouveau rectangle et appeler récursivement
    for (ox, oy) in offsets {
        let nx = rect.x + ox * s;
        let ny = rect.y + oy * s;
        
        let new_rect = Rect {
            x: nx,
            y: ny,
            size: s,
        };
        
        // Appel récursif avec une itération de moins
        generate_cantor(img, new_rect, iter - 1, color);
    }
}

fn main() {
    // Récupérer les arguments de la ligne de commande
    let args: Vec<String> = std::env::args().collect();
    
    // Vérifier qu'on a au moins 3 arguments : nom_programme, largeur, iterations
    if args.len() < 3 {
        eprintln!("Usage: {} <largeur> <iterations> [fichier_sortie]", args[0]);
        eprintln!("Exemple: {} 512 4", args[0]);
        std::process::exit(1);
    }
    
    // Parser les arguments
    let width: u32 = args[1].parse().expect("La largeur doit être un nombre");
    let height = width; // Pour simplifier, on fait un carré
    let iterations: u32 = args[2].parse().expect("Le nombre d'itérations doit être un nombre");
    let output_file = args.get(3).map(|s| s.as_str()).unwrap_or("cantor.png");
    
    println!("Génération d'un carré de Cantor {}x{} avec {} itérations...", width, height, iterations);
    
    // Créer une image blanche
    let mut img = RgbImage::new(width, height);
    
    // Remplir l'image en blanc
    for pixel in img.pixels_mut() {
        *pixel = Rgb([255, 255, 255]);
    }
    
    // Définir le rectangle initial (couvre toute l'image)
    let initial_rect = Rect {
        x: 0,
        y: 0,
        size: width.min(height) as i32, // On prend le minimum pour être sûr
    };
    
    // Couleur noire pour le fractal
    let color = Rgb([0, 0, 0]);
    
    // Générer le fractal
    generate_cantor(&mut img, initial_rect, iterations, color);
    
    // Sauvegarder l'image
    img.save(output_file).expect("Erreur lors de la sauvegarde");
    println!("Image sauvegardée dans {}", output_file);
}
