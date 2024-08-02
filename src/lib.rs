
pub struct Position{
    pub x:f32,
    pub y:f32,
    pub z:f32,
}

impl Position{
    fn xyz(x:f32,y:f32,z:f32)->Position{
        Position{
            x,
            y,
            z
        }
    }
}