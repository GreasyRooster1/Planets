use blue_engine::{
    header::{ Engine, ObjectSettings },
    primitive_shapes::triangle
};

fn main() {
    // initialize the engine
    let mut engine = Engine::new().expect("engine couldn't be initialized");

    // create a triangle
    triangle("my triangle", ObjectSettings::default(), &mut engine.renderer, &mut engine.objects).unwrap();

    // run the engine
    engine
        .update_loop(move |_, _, _, _, _, _| {})
        .expect("Error during update loop");
}