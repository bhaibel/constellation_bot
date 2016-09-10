extern crate svg;

use self::svg::Document;
use self::svg::node::element::{Circle, Rectangle};
use std::vec::Vec;

use super::constellation::star_field::Star;
use super::constellation::Constellation;
use super::bounding_box::BoundingBox;

pub fn svg(constellation: Constellation, bounds: BoundingBox) -> String {
    let mut document = Document::new()
        .set("viewBox", (bounds.origin_x, bounds.origin_y, bounds.width, bounds.height))
        .set("width", bounds.width)
        .set("height", bounds.height)
        .set("viewport-fill", "#143166");

    let background = Rectangle::new()
        .set("x", bounds.origin_x)
        .set("y", bounds.origin_y)
        .set("width", bounds.width)
        .set("height", bounds.height)
        .set("fill", "#143166");

    document = document.add(background);

    for point in points(constellation.stars) {
        document = document.add(point);
    }

    document
        .to_string() 
}

fn points(stars: Vec<Star>) -> Vec<Circle> {
    stars.iter().map ( |star|
        Circle::new()
            .set("cx", star.x)
            .set("cy", star.y)
            .set("r",  star.size)
            .set("fill", "#FFFFFF")
    ).collect::<Vec<Circle>>()
}