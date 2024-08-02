mod lib;

use blue_engine::{header::{Engine, ObjectSettings}, ObjectStorage, primitive_shapes::triangle, Renderer, ShaderSettings, StringBuffer, Vertex, wgpu};
use blue_engine::glm::sqrt;
use blue_engine::primitive_shapes::uv_sphere;
use lib::Position;

fn main() {
    // initialize the engine
    let mut engine = Engine::new().expect("engine couldn't be initialized");

    // create a triangle
    ico_sphere("ico",3, &mut engine.renderer, &mut engine.objects,
    ObjectSettings{
       shader_settings: ShaderSettings {
           polygon_mode: wgpu::PolygonMode::Line,
           ..Default::default()
       },
       ..Default::default()
   });

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

fn ico_sphere(name: impl StringBuffer, subs:i32, renderer: &mut Renderer, objects: &mut ObjectStorage, settings:ObjectSettings){
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
    let mut indices:Vec<u16> =vec![
        0, 11, 5, 0, 5, 1, 0, 1, 7, 0, 7, 10, 0, 10, 11,
        1, 5, 9, 5, 11, 4, 11, 10, 2, 10, 7, 6, 7, 1, 8,
        3, 9, 4, 3, 4, 2, 3, 2, 6, 3, 6, 8, 3, 8, 9,
        4, 9, 5, 2, 4, 11, 6, 2, 10, 8, 6, 7, 9, 8, 1
    ];

    for i in 0..subs{
        let mut new_vertices = vec![];
        let mut new_indices = vec![];
        for j in (0..indices.len()).step_by(3){
            let tri_v1 = vertices[indices[j]as usize];
            let tri_v2 = vertices[indices[j+1]as usize];
            let tri_v3 = vertices[indices[j+2]as usize];

            let mid_v1 = get_middle_point(tri_v1, tri_v2);
            let mid_v2 = get_middle_point(tri_v2, tri_v3);
            let mid_v3 = get_middle_point(tri_v3, tri_v1);

            add_tri(tri_v1, mid_v1, mid_v3, &mut new_vertices, &mut new_indices);
            add_tri(tri_v2, mid_v2, mid_v1, &mut new_vertices, &mut new_indices);
            add_tri(tri_v3, mid_v3, mid_v2, &mut new_vertices, &mut new_indices);
            add_tri(mid_v1, mid_v2, mid_v3, &mut new_vertices, &mut new_indices);
        }
        vertices = new_vertices.clone();
        indices = new_indices.clone();
    }

    objects.new_object(
        name.clone(),
        vertices,
        indices,
        settings,
        renderer,
    ).unwrap();
}

fn add_tri(v1:Vertex, v2:Vertex, v3:Vertex, vertices: &mut Vec<Vertex>, indices: &mut Vec<u16>){
    vertices.append(&mut vec![v1]);
    vertices.append(&mut vec![v2]);
    vertices.append(&mut vec![v3]);
    vertices.append(&mut vec![v1]);

    indices.append(&mut vec![(vertices.len() - 3) as u16]);
    indices.append(&mut vec![(vertices.len() - 2) as u16]);
    indices.append(&mut vec![(vertices.len()-1) as u16]);
}

fn get_middle_point(v1:Vertex,v2:Vertex)->Vertex{
    Vertex{
        position: [
            (v1.position[0]+v2.position[0])/2.,
            (v1.position[1]+v2.position[1])/2.,
            (v1.position[2]+v2.position[2])/2.,
        ],
        uv: [
            (v1.uv[0]+v2.uv[0])/2.,
            (v1.uv[1]+v2.uv[1])/2.,
        ],
        normal: [
            (v1.normal[0]+v2.normal[0])/2.,
            (v1.normal[1]+v2.normal[1])/2.,
            (v1.normal[2]+v2.normal[2])/2.,
        ],
    }
}