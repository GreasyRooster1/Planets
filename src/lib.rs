use blue_engine::{CameraContainer, Vertex};

pub struct Position{
    pub x:f32,
    pub y:f32,
    pub z:f32,
}
pub struct MeshData{
    pub vertices:Vec<Vertex>,
    pub indices:Vec<u16>,
}

pub struct Tri{
    pub v1:Vertex,
    pub v2:Vertex,
    pub v3:Vertex,
}

impl Position{
    pub(crate) fn xyz(x:f32, y:f32, z:f32) ->Position{
        Position{
            x,
            y,
            z
        }
    }
}

impl Clone for Position{
    fn clone(&self) -> Self {
        Position{
            x:self.x.clone(),
            y:self.y.clone(),
            z:self.z.clone(),
        }
    }
}


