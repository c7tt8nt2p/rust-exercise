#![allow(unused_variables, dead_code)]

use std::f64::consts::PI;
use std::ops::Add;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn magnitude(&self) -> f64 {
        let x = self.x;
        let y = self.y;
        ((x.pow(2) + y.pow(2)) as f64).sqrt()
    }

    fn dist(&self, other: &Point) -> f64 {
        let (x1, y1, x2, y2) = (self.x, self.y, other.x, other.y);
        (((x2 - x1).pow(2) + (y2 - y1).pow(2)) as f64).sqrt()
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Clone)]
pub struct Polygon {
    point_list: Vec<Point>,
}

impl Polygon {
    fn new() -> Self {
        Polygon {
            point_list: Vec::new(),
        }
    }
    fn add_point(&mut self, point: Point) {
        self.point_list.push(point);
    }

    fn left_most_point(&self) -> Option<&Point> {
        self.point_list.get(0)
    }
}

pub struct Circle {
    point: Point,
    radius: u8,
}

impl Circle {
    fn new(point: Point, radius: u8) -> Self {
        Self { point, radius }
    }
}

pub enum Shape {
    Polygon(Polygon),
    Circle(Circle),
}

impl Shape {
    fn perimeter(&self) -> f64 {
        match self {
            Shape::Polygon(p) => {
                let mut windows = p.point_list.windows(2);
                let mut perimeter = 0.0;

                let perimeter_calc = |x1: i32, y1: i32, x2: i32, y2: i32| {
                    ((((x1 - x2).pow(2)) + ((y1 - y2).pow(2))) as f64).sqrt()
                };

                while let Some([a, b]) = windows.next() {
                    let (x1, y1, x2, y2) = (a.x, a.y, b.x, b.y);
                    perimeter += perimeter_calc(x1, y1, x2, y2);
                }
                if let (Some(last), Some(first)) = (p.point_list.last(), p.point_list.first()) {
                    let (x1, y1, x2, y2) = (last.x, last.y, first.x, first.y);
                    perimeter += perimeter_calc(x1, y1, x2, y2);
                }
                perimeter
            }
            Shape::Circle(c) => 2f64 * PI * c.radius as f64,
        }
    }
}

impl From<Polygon> for Shape {
    fn from(value: Polygon) -> Self {
        Shape::Polygon(value)
    }
}

impl From<Circle> for Shape {
    fn from(value: Circle) -> Self {
        Shape::Circle(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn round_two_digits(x: f64) -> f64 {
        (x * 100.0).round() / 100.0
    }

    #[test]
    fn test_point_magnitude() {
        let p1 = Point::new(12, 13);
        assert_eq!(round_two_digits(p1.magnitude()), 17.69);
    }

    #[test]
    fn test_point_dist() {
        let p1 = Point::new(10, 10);
        let p2 = Point::new(14, 13);
        assert_eq!(round_two_digits(p1.dist(&p2)), 5.00);
    }

    #[test]
    fn test_point_add() {
        let p1 = Point::new(16, 16);
        let p2 = p1 + Point::new(-4, 3);
        assert_eq!(p2, Point::new(12, 19));
    }

    #[test]
    fn test_polygon_left_most_point() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);

        let mut poly = Polygon::new();
        poly.add_point(p1.clone());
        poly.add_point(p2);
        assert_eq!(poly.left_most_point().cloned(), Some(p1));
    }

    #[test]
    fn test_polygon_iter() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);

        let mut poly = Polygon::new();
        poly.add_point(p1);
        poly.add_point(p2);

        let points = poly.point_list.iter().cloned().collect::<Vec<_>>();
        assert_eq!(points, vec![Point::new(12, 13), Point::new(16, 16)]);
    }

    #[test]
    fn test_shape_perimeters() {
        let mut poly = Polygon::new();
        poly.add_point(Point::new(12, 13));
        poly.add_point(Point::new(17, 11));
        poly.add_point(Point::new(16, 16));
        // sqrt( ((12-17)^2) + ((13-11)^2) ) = 5.3851648071
        // sqrt( ((17-16)^2) + ((11-16)^2) ) = 5.09901951359
        // sqrt( ((16-12)^2) + ((16-13)^2) ) = 5

        let shapes = vec![
            Shape::from(poly),
            Shape::from(Circle::new(Point::new(10, 20), 5)),
        ];
        let perimeters = shapes
            .iter()
            .map(Shape::perimeter)
            .map(round_two_digits)
            .collect::<Vec<_>>();
        assert_eq!(perimeters, vec![15.48, 31.42]);
    }
}

#[allow(dead_code)]
fn main() {}
