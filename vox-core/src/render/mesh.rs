use wgpu::util::DeviceExt;

use super::vertex::Vertex;

pub trait AsMesh {
    fn to_mesh(&self, material_id: Option<usize>, device: &wgpu::Device) -> Mesh;
}

pub struct Mesh {
    index_buffer: wgpu::Buffer,
    vertex_buffer: wgpu::Buffer,
    num_indices: u32,
    // the material assigned to this mesh from the materials
    // to be used with models
    material_id: Option<usize>, 
}

impl Mesh {
    pub fn new(device: &wgpu::Device,
        vertices: &[Vertex],
        indices: &[u32],
        material_id: Option<usize>,
        name: &str,
    ) -> Self {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(name),
            usage: wgpu::BufferUsages::VERTEX,
            contents: bytemuck::cast_slice(vertices),
        });
        
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(name),
            usage: wgpu::BufferUsages::INDEX,
            contents: bytemuck::cast_slice(indices),
        });

        let num_indices = indices.len() as u32;
        
        Mesh {
            index_buffer,
            vertex_buffer,
            material_id,
            num_indices,
        }
    }

    pub fn index_buffer(&self) -> &wgpu::Buffer {
        &self.index_buffer
    }

    pub fn vertex_buffer(&self) -> &wgpu::Buffer {
        &self.vertex_buffer
    }

    pub fn material_id(&self) -> Option<usize> {
        self.material_id
    }

    pub fn num_indices(&self) -> u32 {
        self.num_indices
    }
}

