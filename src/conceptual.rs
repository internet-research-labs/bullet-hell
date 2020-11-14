pub trait World {
    fn update(&self, _: String);
    fn tick(&mut self);
    fn reset(&mut self);
    fn to_string(&self) -> String;
}

pub trait WorldState {
    fn local_state();
}
