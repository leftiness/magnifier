use magnifier::slide;

pub trait Slide {
    fn slide(&mut self, dx: i32, dy: i32) -> Result<(), slide::Error>;
}
