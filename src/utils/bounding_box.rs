#[derive(Debug, Clone, Copy)]
pub struct BoundingBox {
    x1: f32,
    x2: f32,
    y1: f32,
    y2: f32,
}

impl BoundingBox {
    pub fn new(x1: f32, y1: f32, x2: f32, y2: f32) -> Self {
        return BoundingBox { x1, y1, x2, y2 };
    }

    pub fn intersects(&self, other: &BoundingBox) -> bool {
        return self.x1 <= other.x2
            && self.x2 >= other.x1
            && self.y1 <= other.y2
            && self.y2 >= other.y1;
    }
}
