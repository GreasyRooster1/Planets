mod lib;

use blue_engine::{header::{Engine, ObjectSettings}, ObjectStorage, primitive_shapes::triangle, Renderer, ShaderSettings, StringBuffer, Vertex, wgpu};
use blue_engine::glm::sqrt;
use blue_engine::primitive_shapes::uv_sphere;
use lib::Position;

fn main() {
    // initialize the engine
    let mut engine = Engine::new().expect("engine couldn't be initialized");

    // create a triangle
    ico_sphere("ico",0, &mut engine.renderer, &mut engine.objects);

    let radius = 6f32;
    let start = std::time::SystemTime::now();

    // run the engine
    engine
        .update_loop(move |_, _, _, _, camera, _| {
            let camx = start.elapsed().unwrap().as_secs_f32().sin() * radius;
            let camz = start.elapsed().unwrap().as_secs_f32().cos() * radius;
            camera
                .set_position(camx, 0.0, camz)
                .expect("Couldn't update the camera eye");
        })
        .expect("Error during update loop");
}

fn ico_sphere(name: impl StringBuffer, subs:i32, renderer: &mut Renderer, objects: &mut ObjectStorage){
    let t = (1.0 + f32::sqrt(5.0))/2.;
    let mut vertices: Vec<Vertex> = vec![];
    let raw_vertices:Vec<[f32;3]>=vec![
        [-1.,  t,  0.], [1.,  t,  0.], [-1., -t,  0.], [1., -t,  0.],
        [0., -1.,  t], [0.,  1.,  t], [0., -1., -t], [0.,  1., -t],
        [t,  0., -1.], [t,  0.,  1.], [-t,  0., -1.], [-t,  0.,  1.]
    ];

    for raw_vert in raw_vertices {
        vertices.append(&mut vec![
            Vertex {
                position: raw_vert,
                uv: [0., 0.],
                normal: [0., 1., 0.],
            }
        ]);
    }
    let indices =vec![
        0, 11, 5, 0, 5, 1, 0, 1, 7, 0, 7, 10, 0, 10, 11,
        1, 5, 9, 5, 11, 4, 11, 10, 2, 10, 7, 6, 7, 1, 8,
        3, 9, 4, 3, 4, 2, 3, 2, 6, 3, 6, 8, 3, 8, 9,
        4, 9, 5, 2, 4, 11, 6, 2, 10, 8, 6, 7, 9, 8, 1
    ];

    objects.new_object(
        name.clone(),
        vertices,
        indices,
        ObjectSettings{
            shader_settings: ShaderSettings {
                polygon_mode: wgpu::PolygonMode::Line,
                ..Default::default()
            },
            ..Default::default()
        },
        renderer,
    ).unwrap();
}