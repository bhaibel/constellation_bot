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
}
