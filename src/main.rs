mod lib;
mod input;

use std::time::SystemTime;
use blue_engine::{CameraContainer, header::{Engine, ObjectSettings}, ObjectStorage, primitive_shapes::triangle, Renderer, ShaderSettings, StringBuffer, Vertex, wgpu};
use blue_engine::glm::{lerp, lerp_scalar};
use blue_engine_utilities::egui;
use blue_engine_utilities::egui::egui as gui;
use blue_engine_utilities::egui::egui::Slider;
use lib::Position;
use Planets::MeshData;
use crate::input::is_key_pressed;

fn main() {
    // initialize the engine
    let mut engine = Engine::new().expect("engine couldn't be initialized");

    // create a triangle
    ico_sphere("ico",2, &mut engine.renderer, &mut engine.objects,
    ObjectSettings{
       shader_settings: ShaderSettings {
           polygon_mode: wgpu::PolygonMode::Line,
           ..Default::default()
       },
       ..Default::default()
   });

    // Start the egui context
    let gui_context = egui::EGUI::new(&mut engine.renderer, &engine.window);

    // We add the gui as plugin, which runs once before everything else to fetch events, and once during render times for rendering and other stuff
    engine.signals.add_signal("egui", Box::new(gui_context));

    engine.camera
        .set_target(0.,0.,0.)
        .expect("Couldn't update the camera eye");

    let mut wireframe = false;
    let mut normalization_factor = 0.;
    let mut subs = 0;

    let mut radius = 2f32;
    let mut angle = 0f32;

    let timer = SystemTime::now();
    let mut frame_timer = SystemTime::now();
    let mut fps = 0;
    let mut elapsed_frame_time = 0;

    // run the engine
    engine.update_loop(move |renderer, window, objects, input, camera, signals|
    {

        frame_timer = SystemTime::now();

        let egui_plugin = signals
            .get_signal::<egui::EGUI>("egui")
            .expect("Plugin not found")
            .expect("Plugin type mismatch");
        egui_plugin.ui(
            |ctx| {
                gui::Window::new("Planets").show(ctx, |ui| {
                    ui.checkbox(&mut wireframe,"Wireframe");
                    ui.add(Slider::new(&mut subs, 0..=4).text("subs"));
                    ui.add(Slider::new(&mut normalization_factor, 0.0..=1.0).text("norm"));
                });

                gui::Window::new("Stats").show(ctx, |ui| {
                    ui.label(format!("FPS: {0}",fps))
                });

                let ico = objects.get_mut("ico").unwrap();

                ico.shader_settings = ShaderSettings{
                    polygon_mode: if wireframe {wgpu::PolygonMode::Line}else{wgpu::PolygonMode::Fill},
                    ..Default::default()
                };

                let new_mesh = get_ico_mesh(subs,normalization_factor);
                ico.vertices = new_mesh.vertices;
                ico.indices = new_mesh.indices;
                ico.update(renderer).unwrap()
            },
            window,
        );

        if is_key_pressed(38)&&radius> 1.1 {
            radius -= 0.001;
        }
        if is_key_pressed(40){
            radius += 0.001;
        }

        if is_key_pressed(39){
            angle += 0.001;
        }
        if is_key_pressed(37){
            angle -= 0.001;
        }
        let camx = angle.sin() * radius;
        let camz = angle.cos() * radius;
        camera
            .set_position(camx, 0.0, camz)
            .expect("Couldn't update the camera eye");

        elapsed_frame_time = frame_timer.elapsed().unwrap().as_millis();
        fps = (60_000/elapsed_frame_time)/1000
    })
    .expect("Error during update loop");
}

fn ico_sphere(name: impl StringBuffer, subs:i32, renderer: &mut Renderer, objects: &mut ObjectStorage, settings:ObjectSettings){
    let mesh = get_ico_mesh(subs, 1.);
    objects.new_object(
        name.clone(),
        mesh.vertices,
        mesh.indices,
        settings,
        renderer,
    ).unwrap();
}

fn get_ico_mesh(subs:i32, normalization_factor: f64) ->MeshData{
    let t = (1.0 + f32::sqrt(5.0))/2.;
    let mut vertices: Vec<Vertex> = vec![];
    let raw_vertices:Vec<[f32;3]>=vec![
        [-1.,  t,  0.], [1.,  t,  0.], [-1., -t,  0.], [1., -t,  0.],
        [0., -1.,  t], [0.,  1.,  t], [0., -1., -t], [0.,  1., -t],
        [t,  0., -1.], [t,  0.,  1.], [-t,  0., -1.], [-t,  0.,  1.]
    ];

    for raw_vert in raw_vertices {
        let pos = normalize_position(Position::xyz(raw_vert[0],raw_vert[1],raw_vert[2]));
        vertices.append(&mut vec![
            Vertex {
                position: [pos.x,pos.y,pos.z],
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

            let mid_v1 = get_middle_point(tri_v1, tri_v2, normalization_factor);
            let mid_v2 = get_middle_point(tri_v2, tri_v3, normalization_factor);
            let mid_v3 = get_middle_point(tri_v3, tri_v1, normalization_factor);

            add_tri(tri_v1, mid_v1, mid_v3, &mut new_vertices, &mut new_indices);
            add_tri(tri_v2, mid_v2, mid_v1, &mut new_vertices, &mut new_indices);
            add_tri(tri_v3, mid_v3, mid_v2, &mut new_vertices, &mut new_indices);
            add_tri(mid_v1, mid_v2, mid_v3, &mut new_vertices, &mut new_indices);
        }
        vertices = new_vertices.clone();
        indices = new_indices.clone();
    }
    MeshData{
        vertices,
        indices,
    }
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

fn get_middle_point(v1:Vertex, v2:Vertex, normalization_factor: f64) ->Vertex{
    let pos_v1:Position = Position::xyz(v1.position[0],v1.position[1],v1.position[2]);
    let pos_v2:Position = Position::xyz(v2.position[0],v2.position[1],v2.position[2]);

    let pos_avg:Position = Position::xyz((pos_v1.x+pos_v2.x)/2.,(pos_v1.y+pos_v2.y)/2.,(pos_v1.z+pos_v2.z)/2.);

    let pos_normal = normalize_position(pos_avg.clone());

    let pos_final = Position::xyz(
        lerp_scalar(pos_avg.x, pos_normal.x, normalization_factor as f32),
        lerp_scalar(pos_avg.y, pos_normal.y, normalization_factor as f32),
        lerp_scalar(pos_avg.z, pos_normal.z, normalization_factor as f32)
    );

    Vertex{
        position: [
            pos_final.x,
            pos_final.y,
            pos_final.z,
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

fn normalize_position(position: Position)->Position{
    let len = f32::sqrt(position.x.powi(2)+position.y.powi(2)+position.z.powi(2));

    Position::xyz(position.x/len,position.y/len,position.z/len)
}