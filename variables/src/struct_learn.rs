
#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    // todo: if you want to make the method take ownership of the rectangle , use  &mut self
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

