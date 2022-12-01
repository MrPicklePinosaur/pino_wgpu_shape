//! A primitive shape rendering library

pub struct Point2D {
    pub x: f32,
    pub y: f32
}

pub struct Quad(pub Point2D, pub Point2D, pub Point2D, pub Point2D);

pub struct Line(pub Point2D, pub Point2D);

