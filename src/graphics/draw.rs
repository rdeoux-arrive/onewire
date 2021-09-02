pub trait Draw {
    fn draw(&self, frame: &mut [u8], width: u32);
}
