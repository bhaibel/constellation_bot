extern crate svg;

use self::svg::Document;
use self::svg::node::element::{Circle, Rectangle};
use std::vec::Vec;

use super::constellation::Constellation;
use super::bounding_box::BoundingBox;

pub fn svg(constellation: Constellation, bounds: BoundingBox) -> String {
    let formatter = Formatter {
        bounds: bounds,
        constellation: constellation
    };
    formatter.svg()
}

struct Formatter {
    bounds: BoundingBox,
    constellation: Constellation
}

impl Formatter {
    fn svg(&self) -> String {
        Document::new()
            .set("viewBox", (
                self.bounds.origin_x,
                self.bounds.origin_y,
                self.bounds.width,
                self.bounds.height)
            )
            .set("width", self.bounds.width)
            .set("height", self.bounds.height)
            .set("viewport-fill", "#143166")
            .add(self.background())
            .add(self.star_field())
            .to_string() 
    }

    fn background(&self) -> Rectangle {
        Rectangle::new()
            .set("x", self.bounds.origin_x)
            .set("y", self.bounds.origin_y)
            .set("width", self.bounds.width)
            .set("height", self.bounds.height)
            .set("fill", "#143166")
    }

    fn star_field(&self) -> Document {
        let inset = self.bounds.inset();
        let mut star_field = Document::new()
            .set("viewBox", (
                inset.origin_x,
                inset.origin_y,
                inset.width,
                inset.height
            ))
            .set("width", self.bounds.width * 3 / 4)
            .set("height", self.bounds.height * 3 / 4)
            .set("x", self.bounds.width / 8)
            .set("y", self.bounds.height / 8);

        for point in self.points() {
            star_field = star_field.add(point);
        }

        star_field
    }

    fn points(&self) -> Vec<Circle> {
        self.constellation.stars.iter().map ( |star|
            Circle::new()
                .set("cx", star.x)
                .set("cy", star.y)
                .set("r",  star.size)
                .set("fill", "#FFFFFF")
        ).collect::<Vec<Circle>>()
    }
}