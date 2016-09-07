extern crate svg;

use self::svg::Document;
use self::svg::node::element::Circle;
use std::vec::Vec;

use super::star_field::Star;   

pub fn svg(star_field: Vec<Star>) -> String {
    let mut document = Document::new()
        .set("viewBox", (0, 0, 100, 100))
        .set("width", 100)
        .set("height", 100);

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