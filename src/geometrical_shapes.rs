// // use rand::Rng;
// use raster::{Image, Color};
// #[derive(Debug, PartialEq, Clone)]
// pub struct Point {
//     pub x: i32,
//     pub y: i32,
// }
// // ******************************Point******************************************
// // ******************************Point******************************************
// // ******************************Point******************************************
// // ******************************Point******************************************


// // ******************************Line******************************************
// // ******************************Line******************************************
// // ******************************Line******************************************
// // ******************************Line******************************************


// pub struct Line {
//     start: Point,
//     end: Point,
// }

// impl Line {
//     pub fn new(start: Point, end: Point) -> Self {
//         Line { start, end }
//     }

//     pub fn draw(&self, image: &mut Image, color: Color) {
//         // Utilisation de l'algorithme de Bresenham pour tracer la ligne entre les deux points
//         let mut x0 = self.start.x;
//         let mut y0 = self.start.y;
//         let x1 = self.end.x;
//         let y1 = self.end.y;

//         let dx = (x1 - x0).abs();
//         let sx = if x0 < x1 { 1 } else { -1 };
//         let dy = -(y1 - y0).abs();
//         let sy = if y0 < y1 { 1 } else { -1 };
//         let mut err = dx + dy;

//         loop {
//             let _ = image.set_pixel(x0, y0, color.clone());
//             if x0 == x1 && y0 == y1 {
//                 break;
//             }
//             let e2 = 2 * err;
//             if e2 >= dy {
//                 err += dy;
//                 x0 += sx;
//             }
//             if e2 <= dx {
//                 err += dx;
//                 y0 += sy;
//             }
//         }
//     }
// }

// // fn main() {
// //     let mut image = Image::blank(800, 600);
// //     // Remplir l'image avec une couleur de fond
// //     for x in 0..800 {
// //         for y in 0..600 {
// //             let _ = image.set_pixel(x, y, Color::white());
// //         }
// //     }

// //     // Définir les points de début et de fin de la ligne
// //     let start = Point { x: 100, y: 100 };
// //     let end = Point { x: 700, y: 500 };

// //     // Créer la ligne
// //     let line = Line::new(start, end);

// //     // Dessiner la ligne sur l'image avec une couleur noire
// //     line.draw(&mut image, Color::black());

// //     // Sauvegarder l'image sur le disque
// //     let _ = raster::save(&image, "line.png");
// // }

// // ******************************Line******************************************
// // ******************************Line******************************************
// // ******************************Line******************************************
// // ******************************Line******************************************



// // ******************Triangle************************
// // ******************Triangle************************
// // ******************Triangle************************
// // ******************Triangle************************

// pub struct Triangle {
//     point_a: Point,
//     point_b: Point,
//     point_c: Point,
// }
// impl Triangle {
//     pub fn new(point_a: Point, point_b: Point, point_c: Point) -> Self {
//         Triangle { point_a, point_b, point_c }
//     }
//     pub fn draw(&self, image: &mut Image, color: Color) {
//         // Utilisation de l'algorithme de tracé de ligne pour dessiner les côtés du triangle
//         self.draw_line(&self.point_a, &self.point_b, image, color.clone());
//         self.draw_line(&self.point_b, &self.point_c, image, color.clone());
//         self.draw_line(&self.point_c, &self.point_a, image, color.clone());
//     }
//     // Fonction interne pour tracer une ligne entre deux points
//     fn draw_line(&self, start: &Point, end: &Point, image: &mut Image, color: Color) {
//         let line = Line::new(start.clone(), end.clone());
//         line.draw(image, color);
//     }
// }

// // fn main() {
// //     let mut image = Image::blank(800, 600);
// //     // Remplir l'image avec une couleur de fond
// //     for x in 0..800 {
// //         for y in 0..600 {
// //             let _ = image.set_pixel(x, y, Color::white());
// //         }
// //     }

// //     // Définir les sommets du triangle
// //     let point_a = Point { x: 400, y: 100 };
// //     let point_b = Point { x: 100, y: 500 };
// //     let point_c = Point { x: 700, y: 500 };

// //     // Créer le triangle
// //     let triangle = Triangle::new(point_a, point_b, point_c);

// //     // Dessiner le triangle sur l'image avec une couleur noire
// //     triangle.draw(&mut image, Color::black());

// //     // Sauvegarder l'image sur le disque
// //     let _ = raster::save(&image, "triangle.png");
// // }


// // ******************Triangle************************
// // ******************Triangle************************
// // ******************Triangle************************
// // ******************Triangle************************


// // ******************Rectangle************************
// // ******************Rectangle************************
// // ******************Rectangle************************
// // ******************Rectangle************************


// pub struct Rectangle {
//     ref_1: Point,
//     ref_2: Point,
// }
// impl Rectangle {
//     pub fn new(ref_1: Point, ref_2: Point) -> Self {
//         Rectangle { ref_1, ref_2 }
//     }
//     pub fn draw(&self, image: &mut Image, color: Color) {
//         // Dessiner les quatre côtés du rectangle en utilisant la méthode draw_line
//         self.draw_line(&self.ref_1, &Point { x: self.ref_1.x, y: self.ref_2.y }, image, color.clone());
//         self.draw_line(&Point { x: self.ref_1.x, y: self.ref_2.y }, &self.ref_2, image, color.clone());
//         self.draw_line(&self.ref_2, &Point { x: self.ref_2.x, y: self.ref_1.y }, image, color.clone());
//         self.draw_line(&Point { x: self.ref_2.x, y: self.ref_1.y }, &self.ref_1, image, color.clone());
//     }
//     // Fonction interne pour dessiner une ligne entre deux points
//     fn draw_line(&self, start: &Point, end: &Point, image: &mut Image, color: Color) {
//         let line = Line::new(start.clone(), end.clone());
//         line.draw(image, color);
//     }
// }

// // fn main() {
// //     let mut image = Image::blank(800, 600);
// //     // Remplir l'image avec une couleur de fond
// //     for x in 0..800 {
// //         for y in 0..600 {
// //             let _ = image.set_pixel(x, y, Color::white());
// //         }
// //     }

// //     // Définir les coins du rectangle
// //     let ref_1 = Point { x: 200, y: 200 };
// //     let ref_2 = Point { x: 600, y: 400 };

// //     // Créer le rectangle
// //     let rectangle = Rectangle::new(ref_1, ref_2);

// //     // Dessiner le rectangle sur l'image avec une couleur noire
// //     rectangle.draw(&mut image, Color::black());

// //     // Sauvegarder l'image sur le disque
// //     let _ = raster::save(&image, "rectangle.png");
// // }


// // ******************Rectangle************************
// // ******************Rectangle************************
// // ******************Rectangle************************
// // ******************Rectangle************************


// // ******************Circle************************
// // ******************Circle************************
// // ******************Circle************************
// // ******************Circle************************


// pub struct Circle {
//     center: Point,
//     radius: i32,
// }
// impl Circle {
//     pub fn new(center: Point, radius: i32) -> Self {
//         Circle { center, radius }
//     }
//     pub fn draw(&self, image: &mut Image, color: Color) {
//         let mut x = self.center.x - self.radius;
//         while x <= self.center.x + self.radius {
//             let mut y = self.center.y - self.radius;
//             while y <= self.center.y + self.radius {
//                 let distance_squared = (x - self.center.x).pow(2) + (y - self.center.y).pow(2);
//                 let distance = f64::from(distance_squared).sqrt();
//                 if distance >= f64::from(self.radius - 1) && distance <= f64::from(self.radius) {
//                     let _ = image.set_pixel(x, y, color.clone());
//                 }
//                 y += 1;
//             }
//             x += 1;
//         }
//     }
// }



// // fn main() {
// //     let mut image = Image::blank(800, 600);
// //     // Fill the image with white color
// //     for x in 0..800 {
// //         for y in 0..600 {
// //             let _ = image.set_pixel(x, y, Color::white());
// //         }
// //     }

// //     let center = Point { x: 400, y: 300 };
// //     let radius = 300;
// //     let circle = Circle::new(center, radius);
// //     circle.draw(&mut image, Color::black());
    
// //     // Save the image to disk
// //     let _ = raster::save(&image, "drawing.png");
// // }

// // ******************Circle************************
// // ******************Circle************************
// // ******************Circle************************
// // ******************Circle************************

// fn main() {
//     // Créer une nouvelle image
//     let mut image = Image::blank(800, 600);

//     // Dessiner une ligne
//     let start_line = Point { x: 1800, y: 200 };
//     let end_line = Point { x: 350, y: 400 };
//     let line = Line::new(start_line, end_line);
//     line.draw(&mut image, Color::black());

//     // Dessiner un cercle
//     let center_circle = Point { x: 400, y: 300 };
//     let radius_circle = 100;
//     let circle = Circle::new(center_circle, radius_circle);
//     circle.draw(&mut image, Color::red());

//     // Dessiner un triangle
//     let point_a = Point { x: 250, y: 100 };
//     let point_b = Point { x: 300, y: 200 };
//     let point_c = Point { x: 100, y: 500 };
//     let triangle = Triangle::new(point_a, point_b, point_c);
//     triangle.draw(&mut image, Color::blue());

//     // Dessiner un rectangle
//     let ref_1_rect = Point { x: 20, y: 50 };
//     let ref_2_rect = Point { x: 500, y: 100 };
//     let rectangle = Rectangle::new(ref_1_rect, ref_2_rect);
//     rectangle.draw(&mut image, Color::green());

//     // Sauvegarder l'image sur le disque
//     let _ = raster::save(&image, "figures.png");
// }




// ********************************************************************************************
// ********************************************************************************************
// ********************************************************************************************
// ********************************************************************************************
// ********************************************************************************************
// ********************************************************************************************
// ********************************************************************************************

use rand::Rng;
use raster::{Color, Image};
//Drawable trait
pub trait Drawable {
    fn draw(&self, _arg: &mut Image){}
    fn color(&self) -> Color{
        Color::rgb(rand::thread_rng().gen_range(0, 255),
        rand::thread_rng().gen_range(0, 255),
        rand::thread_rng().gen_range(0, 255)
        )
    }
}
//Displayable trait
pub trait Displayable{
    fn display(&mut self, x: i32, y: i32, color: Color);
}


// *************************************************************************


pub struct  Point{
    pub x: i32,
    pub y: i32
}
impl Point{
    pub fn new(x_n: i32, y_n: i32) -> Self{
        Self{
            x: x_n,
            y: y_n
        }
    }
    pub fn random(r_x: i32, r_y: i32) -> Self{
        Self { 
            x: rand::thread_rng().gen_range(0, r_x),
            y: rand::thread_rng().gen_range(0, r_y)
        }
    } 
}
impl Drawable for Point{
    fn draw(&self, img: &mut Image) {
       img.display(self.x, self.y,  self.color())
    }
    
}



// *************************************************************************


pub struct Line{
    start: Point,
    end: Point
} 
impl Line{
    pub fn new(start_n: &Point, end_n: &Point) -> Self{
        Self{
            start: Point::new(start_n.x, start_n.y),
            end: Point::new(end_n.x, end_n.y)
        }
    }
    pub fn random(x_r:i32, y_r: i32) -> Self{
        Self{
            start: Point::random(x_r, y_r),
            end : Point::random(x_r, y_r)
        }
    }
    
}
impl Drawable for Line{
    fn draw(&self, img: &mut Image) {
        let diff_x = (self.end.x - self.start.x).abs();
        let diff_y = -(self.end.y - self.start.y).abs();
        
        let sens_x = if self.start.x < self.end.x { 1 } else { -1 };
        let sens_y = if self.start.y < self.end.y { 1 } else { -1 };
        let mut err = diff_x + diff_y;
        let mut x = self.start.x;
        let mut y = self.start.y;
        let color = self.color();
        while x != self.end.x || y != self.end.y {
            img.display(x, y, color.clone());
            let e2 = 2 * err;
            if e2 >= diff_y { err += diff_y; x += sens_x; }
            if e2 <= diff_x { err += diff_x; y += sens_y; }
        }
    
    }
}



// *************************************************************************


pub struct Triangle{
    point_a: Point,
    point_b: Point,
    point_c: Point
}
impl Triangle{
    pub fn new(a: &Point, b: &Point, c: &Point) -> Self{
        Self{
            point_a: Point::new(a.x, a.y),
            point_b: Point::new(b.x, b.y),
            point_c: Point::new(c.x, c.y)
        }
    }
}
impl Drawable for Triangle{
    fn draw(&self, img: &mut Image) {
        let line1 = Line::new(&self.point_a, &self.point_b);
        let line2 = Line::new(&self.point_b, &self.point_c);
        let line3 = Line::new(&self.point_c, &self.point_a);
        line1.draw(img);
        line2.draw(img);
        line3.draw(img);
    }
}


// *************************************************************************


pub struct Rectangle {
    ref_1: Point,
    ref_2: Point,
}

impl Rectangle {
    pub fn new(ref_1: &Point, ref_2: &Point) -> Self {
        Self {
            ref_1: Point::new(ref_1.x, ref_1.y),
            ref_2: Point::new(ref_2.x, ref_2.y),
        }
    }
}

impl Drawable for Rectangle {
    fn draw(&self, img: &mut Image) {
        let line1 = Line::new(&self.ref_1, &Point::new(self.ref_1.x, self.ref_2.y));
        let line2 = Line::new(&Point::new(self.ref_1.x, self.ref_2.y), &self.ref_2);
        let line3 = Line::new(&self.ref_2, &Point::new(self.ref_2.x, self.ref_1.y));
        let line4 = Line::new(&Point::new(self.ref_2.x, self.ref_1.y), &self.ref_1);
        
        line1.draw(img);
        line2.draw(img);
        line3.draw(img);
        line4.draw(img);
    }
}



// *************************************************************************

pub struct Circle{
    center: Point,
    radius: i32
}
impl Circle{
    pub fn random(max_x: i32, max_y: i32) -> Self {
        let center = Point::random(max_x as i32, max_y as i32);
        let radius = rand::thread_rng().gen_range(1, 101); 
        Circle { center, radius }
    }
}
impl Drawable for Circle {
    fn draw(&self, img: &mut Image) {
        let mut x = 0;
        let mut y = self.radius;
        let mut err = 0;
        while x <= y {
            img.display(self.center.x + x, self.center.y + y, self.color());
            img.display(self.center.x + y, self.center.y + x, self.color());
            img.display(self.center.x - x, self.center.y + y, self.color());
            img.display(self.center.x - y, self.center.y + x, self.color());
            img.display(self.center.x + x, self.center.y - y, self.color());
            img.display(self.center.x + y, self.center.y - x, self.color());
            img.display(self.center.x - x, self.center.y - y, self.color());
            img.display(self.center.x - y, self.center.y - x, self.color());
            if err <= 0 {
                y -= 1;
                err += 2 * (y as i32) + 1;
            }
            if err > 0 {
                x += 1;
                err -= 2 * (x as i32) + 1;
            }
        }
    }
}

// *************************************************************************

impl Displayable for Image {
    fn display(&mut self, x: i32, y: i32, color: Color) {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            self.set_pixel(x, y, color).unwrap();
        }
    }
}

// *************************************************************************

pub struct Pentagon {
    center: Point,
    radius: i32,
}

impl Pentagon {
    pub fn new(center: Point, radius: i32) -> Self {
        Pentagon { center, radius }
    }
}

impl Drawable for Pentagon {
    fn draw(&self, img: &mut Image) {
        // Calcul des coordonnées des sommets du pentagone
        let mut points = Vec::new();
        let angle_degrees:f32 = 360.0 / 5.0;
        let angle_radians = angle_degrees.to_radians();
        let mut current_angle: f32 = 0.0;

        for _ in 0..5 {
            let x: f32 = self.center.x as f32 + (self.radius as f32 * current_angle.cos());
            let y = self.center.y as f32 + (self.radius as f32 * current_angle.sin());
            points.push(Point::new(x as i32, y as i32));
            current_angle += angle_radians;
        }

        // Dessin des lignes reliant les sommets du pentagone
        for i in 0..5 {
            let start = &points[i];
            let end = if i == 4 { &points[0] } else { &points[i + 1] };
            let line = Line::new(&start, &end);
            line.draw(img);
        }
    }
}

// **************************************************************************************

pub struct Cube {
    center: Point,
    size: i32,
}

impl Cube {
    pub fn new(center: Point, size: i32) -> Self {
        Cube { center, size }
    }
}

impl Drawable for Cube {
    fn draw(&self, img: &mut Image) {
        // Calcul des coordonnées des sommets du cube
        let half_size = self.size / 2;
        let mut points = Vec::new();
        points.push(Point::new(self.center.x - half_size, self.center.y - half_size));
        points.push(Point::new(self.center.x + half_size, self.center.y - half_size));
        points.push(Point::new(self.center.x + half_size, self.center.y + half_size));
        points.push(Point::new(self.center.x - half_size, self.center.y + half_size));

        // Dessin des lignes reliant les sommets du cube
        let lines = vec![
            (0, 1), (1, 2), (2, 3), (3, 0), // Base du cube
            (0, 4), (1, 5), (2, 6), (3, 7), // Arêtes verticales
            (4, 5), (5, 6), (6, 7), (7, 4), // Sommets opposés
        ];

        for (start_idx, end_idx) in lines {
            let start = &points[start_idx];
            let end = &points[end_idx];
            let line = Line::new(start, end);
            line.draw(img);
        }
    }
}
