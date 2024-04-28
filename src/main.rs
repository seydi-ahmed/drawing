mod geometrical_shapes;

use geometrical_shapes as gs;
use gs::*;
use raster::*;
use rand::Rng;


// fn main() {
//     let mut image = Image::blank(1000, 1000);

//     gs::Line::random(image.width, image.height).draw(&mut image);

//     gs::Point::random(image.width, image.height).draw(&mut image);

//     let rectangle = gs::Rectangle::new(&gs::Point::new(150, 150), &gs::Point::new(50, 50));
//     rectangle.draw(&mut image);

//     let triangle = gs::Triangle::new (
//             &gs::Point::new(500, 500),
//             &gs::Point::new(250, 700),
//             &gs::Point::new(700, 800),
//     );
//     triangle.draw(&mut image);

//     for _ in 1..50 {
//         gs::Circle::random(image.width, image.height).draw(&mut image);
//     }

//     raster::save(&image, "image.png").unwrap();
// }

fn main() {
    // Création d'une image
    let mut image = Image::blank(800, 600);

    // Dessin de formes aléatoires sur l'image
    for _ in 0..10 {
        // Choix aléatoire d'une position pour le centre du pentagone
        let center = Point::new(rand::thread_rng().gen_range(0, 800), rand::thread_rng().gen_range(0, 600));
        // Choix aléatoire du rayon du pentagone
        let radius = rand::thread_rng().gen_range(50, 200);

        // Création d'un pentagone avec le centre et le rayon choisis
        let pentagon = Pentagon::new(center, radius);

        // Dessin du pentagone sur l'image
        pentagon.draw(&mut image);
    }

    // Sauvegarde de l'image sur le disque
    raster::save(&image, "shapes.png").expect("Failed to save image");
}