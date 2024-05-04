use bevy::math::bounding::*;
use bevy::prelude::*;

#[derive(Component)]
pub enum Shape {
    Rectangle(Rectangle),
    Circle(Circle),
    Triangle(Triangle2d),
    Line(Segment2d),
    Capsule(Capsule2d),
    Polygon(RegularPolygon),
}

#[derive(Component)]
pub enum DesiredVolume {
    Aabb,
    Circle,
}

#[derive(Component)]
pub struct Spin;

#[derive(Component, Deref, DerefMut, Default)]
pub struct Intersects(bool);

#[derive(Component, Debug)]
pub enum CurrentVolume {
    Aabb(Aabb2d),
    Circle(BoundingCircle),
}
