extern crate svg;

use self::svg::Document;
use self::svg::node::element::{Circle, Rectangle, LinearGradient, Stop};
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
            .add(self.background_gradient())
            .add(self.background())
            .add(self.frame())
            .add(self.star_field())
            .to_string() 
    }


    fn background_gradient(&self) -> LinearGradient {
        LinearGradient::new()
            .set("id", "bg_gradient")
            .set("x1", ".2")
            .set("y1", "0")
            .set("x2", ".8")
            .set("y2", "1")
            .add(Stop::new()
                .set("offset", "0%")
                .set("stop-color", "#143166")
            )
            .add(Stop::new()
                .set("offset", "100%")
                .set("stop-color", "#041A42")
            )
    }

    fn background(&self) -> Rectangle {
        Rectangle::new()
            .set("x", self.bounds.origin_x)
            .set("y", self.bounds.origin_y)
            .set("width", self.bounds.width)
            .set("height", self.bounds.height)
            .set("fill", "url(#bg_gradient)")
    }

    fn star_field(&self) -> Document {
        let inset = self.star_bounds().inset();
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

    // once you can figure out how to autoderive vector implementations
    // on a vector-based newtype, this goes on StarField
    fn star_bounds(&self) -> BoundingBox {
        let xs: Vec<i32> = self.constellation.stars.iter().map( |star| star.x).collect();
        let ys: Vec<i32> = self.constellation.stars.iter().map( |star| star.y).collect();

        BoundingBox {
            origin_x: xs.iter().min().unwrap().clone(),
            origin_y: ys.iter().min().unwrap().clone(),
            width: xs.iter().max().unwrap().clone(),
            height: ys.iter().max().unwrap().clone()
        }
    }

    fn points(&self) -> Vec<Circle> {
        self.constellation.stars.iter().map ( |star|
            Circle::new()
                .set("cx", star.x)
                .set("cy", star.y)
                .set("r",  star.size.scaled(1.5, 4.0))
                .set("fill", star.color.as_hex())
        ).collect::<Vec<Circle>>()
    }

    fn frame(&self) -> Rectangle {
        let offset_width = self.bounds.width / 16;
        let offset_height = self.bounds.width / 16;
        Rectangle::new()
            .set("x", self.bounds.origin_x + offset_width)
            .set("y", self.bounds.origin_y + offset_height)
            .set("width", self.bounds.width - (offset_width * 2))
            .set("height", self.bounds.height - (offset_height * 2))
            .set("stroke", "#FFFFFF")
            .set("stroke-width", "1")
            .set("stroke-opacity", ".7")
            .set("fill", "#000000")
            .set("fill-opacity", ".2")
    }
}
