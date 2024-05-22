pub trait Location {
    fn get_location(&self) -> (u32, u32);
    fn set_location(&mut self, x: u32, y: u32);
}
