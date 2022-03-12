#[derive(Clone, Copy, Debug)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

impl Vector2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn len(self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn norm(self) -> Self {
        let len = self.len();
        Self::new(self.x / len, self.y / len)
    }
}
