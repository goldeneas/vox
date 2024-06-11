pub trait Vertex {
    // Self: Sized <=> https://doc.rust-lang.org/reference/items/traits.html#object-safety
    fn desc() -> wgpu::VertexBufferLayout<'static> where Self: Sized;
}
