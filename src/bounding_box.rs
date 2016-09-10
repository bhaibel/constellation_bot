pub struct BoundingBox {
    pub origin_x: i32,
    pub origin_y: i32,
    pub width: i32,
    pub height: i32   
}

impl BoundingBox {
    pub fn default() -> BoundingBox {
        BoundingBox {
            origin_x: 0,
            origin_y: 0,
            width: 480,
            height: 320
        }
    }

    pub fn inset(&self) -> BoundingBox {
        BoundingBox {
            origin_x: self.origin_x - 10,
            origin_y: self.origin_y - 10,
            width: self.width + 10,
            height: self.height + 10
        }
    }
}
