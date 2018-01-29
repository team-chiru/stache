pub trait Writer<Output> where Output: Default {
    fn new() -> Self;
    fn reset(&mut self);
    fn write(&mut self, new: &Output);
}