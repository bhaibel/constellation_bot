extern crate svg;

use self::svg::Document;
use self::svg::node::element::Circle;
use std::vec::Vec;

use super::star_field::Star;
use super::BoundingBox;

pub fn svg(star_field: Vec<Star>, bounds: BoundingBox) -> String {
    let mut document = Document::new()
        .set("viewBox", (bounds.origin_x, bounds.origin_y, bounds.width, bounds.height))
        .set("width", bounds.width)
        .set("height", bounds.height);

    for point in points(star_field) {
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
            .set("r",  5)
    ).collect::<Vec<Circle>>()
}