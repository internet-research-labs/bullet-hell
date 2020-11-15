pub trait World {
    fn update(&self, _: String);
    fn tick(&mut self);
    fn reset(&mut self);
    fn to_string(&self) -> String;
}

pub trait WorldState {
    fn local_state();
}

#[derive(Clone)]
pub struct UpdateReq {
    pub id: i64,
    pub msg: String,
}

pub enum PlayerReq {
    Nothing,
    Fetch,
    Update(UpdateReq),
}
