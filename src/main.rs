use raster::{Image, Color};
#[derive(Debug, PartialEq, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

// ******************************Line******************************************
// ******************************Line******************************************
// ******************************Line******************************************
// ******************************Line******************************************


pub struct Line {
    start: Point,
    end: Point,
}

impl Line {
    pub fn new(start: Point, end: Point) -> Self {
        Line { start, end }
    }

    pub fn draw(&self, image: &mut Image, color: Color) {
        // Utilisation de l'algorithme de Bresenham pour tracer la ligne entre les deux points
        let mut x0 = self.start.x;
        let mut y0 = self.start.y;
        let x1 = self.end.x;
        let y1 = self.end.y;

        let dx = (x1 - x0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let dy = -(y1 - y0).abs();
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;

        loop {
            let _ = image.set_pixel(x0, y0, color.clone());
            if x0 == x1 && y0 == y1 {
                break;
            }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x0 += sx;
            }
            if e2 <= dx {
                err += dx;
                y0 += sy;
            }
        }
    }
}

// fn main() {
//     let mut image = Image::blank(800, 600);
//     // Remplir l'image avec une couleur de fond
//     for x in 0..800 {
//         for y in 0..600 {
//             let _ = image.set_pixel(x, y, Color::white());
//         }
//     }

//     // Définir les points de début et de fin de la ligne
//     let start = Point { x: 100, y: 100 };
//     let end = Point { x: 700, y: 500 };

//     // Créer la ligne
//     let line = Line::new(start, end);

//     // Dessiner la ligne sur l'image avec une couleur noire
//     line.draw(&mut image, Color::black());

//     // Sauvegarder l'image sur le disque
//     let _ = raster::save(&image, "line.png");
// }

// ******************************Line******************************************
// ******************************Line******************************************
// ******************************Line******************************************
// ******************************Line******************************************



// ******************Triangle************************
// ******************Triangle************************
// ******************Triangle************************
// ******************Triangle************************

pub struct Triangle {
    point_a: Point,
    point_b: Point,
    point_c: Point,
}


impl Triangle {
    pub fn new(point_a: Point, point_b: Point, point_c: Point) -> Self {
        Triangle { point_a, point_b, point_c }
    }

    pub fn draw(&self, image: &mut Image, color: Color) {
        // Utilisation de l'algorithme de tracé de ligne pour dessiner les côtés du triangle
        self.draw_line(&self.point_a, &self.point_b, image, color.clone());
        self.draw_line(&self.point_b, &self.point_c, image, color.clone());
        self.draw_line(&self.point_c, &self.point_a, image, color.clone());
    }

    // Fonction interne pour tracer une ligne entre deux points
    fn draw_line(&self, start: &Point, end: &Point, image: &mut Image, color: Color) {
        let line = Line::new(start.clone(), end.clone());
        line.draw(image, color);
    }
}

// fn main() {
//     let mut image = Image::blank(800, 600);
//     // Remplir l'image avec une couleur de fond
//     for x in 0..800 {
//         for y in 0..600 {
//             let _ = image.set_pixel(x, y, Color::white());
//         }
//     }

//     // Définir les sommets du triangle
//     let point_a = Point { x: 400, y: 100 };
//     let point_b = Point { x: 100, y: 500 };
//     let point_c = Point { x: 700, y: 500 };

//     // Créer le triangle
//     let triangle = Triangle::new(point_a, point_b, point_c);

//     // Dessiner le triangle sur l'image avec une couleur noire
//     triangle.draw(&mut image, Color::black());

//     // Sauvegarder l'image sur le disque
//     let _ = raster::save(&image, "triangle.png");
// }


// ******************Triangle************************
// ******************Triangle************************
// ******************Triangle************************
// ******************Triangle************************


// ******************Rectangle************************
// ******************Rectangle************************
// ******************Rectangle************************
// ******************Rectangle************************


pub struct Rectangle {
    ref_1: Point,
    ref_2: Point,
}

impl Rectangle {
    pub fn new(ref_1: Point, ref_2: Point) -> Self {
        Rectangle { ref_1, ref_2 }
    }

    pub fn draw(&self, image: &mut Image, color: Color) {
        // Dessiner les quatre côtés du rectangle en utilisant la méthode draw_line
        self.draw_line(&self.ref_1, &Point { x: self.ref_1.x, y: self.ref_2.y }, image, color.clone());
        self.draw_line(&Point { x: self.ref_1.x, y: self.ref_2.y }, &self.ref_2, image, color.clone());
        self.draw_line(&self.ref_2, &Point { x: self.ref_2.x, y: self.ref_1.y }, image, color.clone());
        self.draw_line(&Point { x: self.ref_2.x, y: self.ref_1.y }, &self.ref_1, image, color.clone());
    }

    // Fonction interne pour dessiner une ligne entre deux points
    fn draw_line(&self, start: &Point, end: &Point, image: &mut Image, color: Color) {
        let line = Line::new(start.clone(), end.clone());
        line.draw(image, color);
    }
}

// fn main() {
//     let mut image = Image::blank(800, 600);
//     // Remplir l'image avec une couleur de fond
//     for x in 0..800 {
//         for y in 0..600 {
//             let _ = image.set_pixel(x, y, Color::white());
//         }
//     }

//     // Définir les coins du rectangle
//     let ref_1 = Point { x: 200, y: 200 };
//     let ref_2 = Point { x: 600, y: 400 };

//     // Créer le rectangle
//     let rectangle = Rectangle::new(ref_1, ref_2);

//     // Dessiner le rectangle sur l'image avec une couleur noire
//     rectangle.draw(&mut image, Color::black());

//     // Sauvegarder l'image sur le disque
//     let _ = raster::save(&image, "rectangle.png");
// }


// ******************Rectangle************************
// ******************Rectangle************************
// ******************Rectangle************************
// ******************Rectangle************************


// ******************Circle************************
// ******************Circle************************
// ******************Circle************************
// ******************Circle************************


pub struct Circle {
    center: Point,
    radius: i32,
}

impl Circle {
    pub fn new(center: Point, radius: i32) -> Self {
        Circle { center, radius }
    }

    pub fn draw(&self, image: &mut Image, color: Color) {
        let mut x = self.center.x - self.radius;
        while x <= self.center.x + self.radius {
            let mut y = self.center.y - self.radius;
            while y <= self.center.y + self.radius {
                let distance_squared = (x - self.center.x).pow(2) + (y - self.center.y).pow(2);
                let distance = f64::from(distance_squared).sqrt();
                if distance >= f64::from(self.radius - 1) && distance <= f64::from(self.radius) {
                    let _ = image.set_pixel(x, y, color.clone());
                }
                y += 1;
            }
            x += 1;
        }
    }
}



// fn main() {
//     let mut image = Image::blank(800, 600);
//     // Fill the image with white color
//     for x in 0..800 {
//         for y in 0..600 {
//             let _ = image.set_pixel(x, y, Color::white());
//         }
//     }

//     let center = Point { x: 400, y: 300 };
//     let radius = 300;
//     let circle = Circle::new(center, radius);
//     circle.draw(&mut image, Color::black());
    
//     // Save the image to disk
//     let _ = raster::save(&image, "drawing.png");
// }

// ******************Circle************************
// ******************Circle************************
// ******************Circle************************
// ******************Circle************************

fn main() {
    // Créer une nouvelle image
    let mut image = Image::blank(800, 600);

    // Dessiner une ligne
    let start_line = Point { x: 100, y: 100 };
    let end_line = Point { x: 700, y: 500 };
    let line = Line::new(start_line, end_line);
    line.draw(&mut image, Color::black());

    // Dessiner un cercle
    let center_circle = Point { x: 400, y: 300 };
    let radius_circle = 100;
    let circle = Circle::new(center_circle, radius_circle);
    circle.draw(&mut image, Color::red());

    // Dessiner un triangle
    let point_a = Point { x: 200, y: 200 };
    let point_b = Point { x: 100, y: 500 };
    let point_c = Point { x: 700, y: 500 };
    let triangle = Triangle::new(point_a, point_b, point_c);
    triangle.draw(&mut image, Color::blue());

    // Dessiner un rectangle
    let ref_1_rect = Point { x: 200, y: 200 };
    let ref_2_rect = Point { x: 600, y: 400 };
    let rectangle = Rectangle::new(ref_1_rect, ref_2_rect);
    rectangle.draw(&mut image, Color::green());

    // Sauvegarder l'image sur le disque
    let _ = raster::save(&image, "figures.png");
}