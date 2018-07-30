#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
}

implement_vertex!(Vertex, position);

#[derive(Copy, Clone)]
struct Instance {
    color: [f32; 3],
    world_position: [f32; 3],
}

implement_vertex!(Instance, color, world_position);
