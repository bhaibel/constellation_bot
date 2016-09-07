extern crate svg;

use self::svg::Document;
use std::vec::Vec;

use super::star_field::Star;   

pub fn svg(star_field: Vec<Star>) -> String {
    Document::new()
        .set("viewBox", (0, 0, 100, 100))
        .set("width", 100)
        .set("height", 100)
        .to_string() 
}