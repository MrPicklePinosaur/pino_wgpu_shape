//! A primitive shape rendering library

use std::mem;

use wgpu::{include_wgsl, util::DeviceExt};
use cgmath::{prelude, num_traits::Pow, Vector3};

const VERTEX_ENTRY_POINT: &'static str = "vs_main";
const FRAGMENT_ENTRY_POINT: &'static str = "fs_main";

pub struct Point2D {
    pub x: f32,
    pub y: f32
}

pub enum Shape {
    Triangle(Triangle),
    Rect(Rect),
    Quad(Quad),
    Line(Line),
}

pub struct Triangle(pub Point2D, pub Point2D, pub Point2D);

pub struct Quad(pub Point2D, pub Point2D, pub Point2D, pub Point2D);

// top left, width, height
pub struct Rect(pub Point2D, pub usize, pub usize);

pub struct Line(pub Point2D, pub Point2D);

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

impl Vertex {

    const ATTRIBS: [wgpu::VertexAttribute; 2] = wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];

    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

struct Instance {
    position: Vector3<f32>,
    scale: Vector3<f32>,
}

impl Instance {
    fn to_raw(&self) -> InstanceRaw {
	InstanceRaw {
	    model: (cgmath::Matrix4::from_translation(self.position) * cgmath::Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z)).into()
	}
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct InstanceRaw {
    model: [[f32; 4]; 4]
}

impl InstanceRaw {

    const ATTRIBS: [wgpu::VertexAttribute; 4] = wgpu::vertex_attr_array![5 => Float32x4, 6 => Float32x4, 7 => Float32x4, 8 => Float32x4];

    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &Self::ATTRIBS,
        }
    }
}

// this is a square
const VERTICES: &[Vertex] = &[
    Vertex { position: [-0.5, 0.5, 0.0], color: [1.0, 0.0, 0.0] },
    Vertex { position: [-0.5, -0.5, 0.0], color: [0.0, 1.0, 0.0] },
    Vertex { position: [0.5, -0.5, 0.0], color: [0.0, 0.0, 1.0] },

    Vertex { position: [-0.5, 0.5, 0.0], color: [1.0, 0.0, 0.0] },
    Vertex { position: [0.5, -0.5, 0.0], color: [0.0, 0.0, 1.0] },
    Vertex { position: [0.5, 0.5, 0.0], color: [0.0, 1.0, 0.0] },
];
const INSTANCE_BUFFER_INIT_SIZE: usize = 64;

pub struct ShapeRenderer {
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    queue: Vec<Shape>,
    instances: Vec<Instance>,
    instance_buffer: wgpu::Buffer,
}

impl ShapeRenderer {

    pub fn new(device: &wgpu::Device, render_format: wgpu::TextureFormat) -> Self {

	let shader = device.create_shader_module(include_wgsl!("shaders/shader.wgsl"));

	// create vertex buffer
	let vertex_buffer = device.create_buffer_init(
	    &wgpu::util::BufferInitDescriptor {
		label: Some("Vertex Buffer"),
		contents: bytemuck::cast_slice(VERTICES),
		usage: wgpu::BufferUsages::VERTEX,
	    }
	);

	// create instances (REMOVE THIS LATER)
	let instances = vec![
	    Instance {position: Vector3::new(0., 0., 0.), scale: Vector3::new(1., 1., 1.)},
	    Instance {position: Vector3::new(1., 1., 1.), scale: Vector3::new(1., 1., 1.)}
	];
	let instance_data = instances.iter().map(Instance::to_raw).collect::<Vec<_>>();

	// create instance buffer
	let instance_buffer = device.create_buffer_init(
	    &wgpu::util::BufferInitDescriptor{
		label: Some("Instance Buffer"),
		contents: bytemuck::cast_slice(&instance_data),
		usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
	    }
	);
	/*
	let instance_buffer = device.create_buffer(
	    &wgpu::BufferDescriptor {
		label: Some("Instance Buffer"),
		size: mem::size_of::<Instance>() as u64 * INSTANCE_BUFFER_INIT_SIZE as u64,
		usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
		mapped_at_creation: false
	    }
	);
	*/

	// create render pipeline
	let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
	    label: Some("Render Pipeline Layout"),
	    bind_group_layouts: &[],
	    push_constant_ranges: &[],
	});

	let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
	    label: Some("Render Pipeline"),
	    layout: Some(&render_pipeline_layout),
	    vertex: wgpu::VertexState {
		module: &shader,
		entry_point: VERTEX_ENTRY_POINT,
		buffers: &[
		    Vertex::desc(),
		    InstanceRaw::desc(),
		]
	    },
	    fragment: Some(wgpu::FragmentState {
		module: &shader,
		entry_point: FRAGMENT_ENTRY_POINT,
		targets: &[Some(wgpu::ColorTargetState {
		    format: render_format,
		    blend: Some(wgpu::BlendState::REPLACE),
		    write_mask: wgpu::ColorWrites::ALL,
		})],
	    }),
	    primitive: wgpu::PrimitiveState {
		topology: wgpu::PrimitiveTopology::TriangleList,
		strip_index_format: None,
		front_face: wgpu::FrontFace::Ccw,
		cull_mode: Some(wgpu::Face::Back),
		polygon_mode: wgpu::PolygonMode::Fill,
		unclipped_depth: false,
		conservative: false,
	    },
	    depth_stencil: None,
	    multisample: wgpu::MultisampleState {
		count: 1,
		mask: !0,
		alpha_to_coverage_enabled: false
	    },
	    multiview: None
	});

	ShapeRenderer {
	    pipeline: render_pipeline,
	    vertex_buffer,
	    queue: vec![],
	    instances,
	    instance_buffer,
	}
    }

    pub fn draw(&mut self, encoder: &mut wgpu::CommandEncoder, view: &wgpu::TextureView) {
	let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
	    label: Some("Render Pass"),
	    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
		view: &view,
		resolve_target: None,
		ops: wgpu::Operations {
		    load: wgpu::LoadOp::Clear(wgpu::Color {
			r: 1.0,
			g: 1.0,
			b: 0.3,
			a: 1.0,
		    }),
		    store: true,
		},
	    })],
	    depth_stencil_attachment: None,
	});
	render_pass.set_pipeline(&self.pipeline);
	render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
	render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
	render_pass.draw(0..(VERTICES.len() as u32), 0..self.instances.len() as _);

	self.queue.clear();
    }

    pub fn queue(&mut self, shape: Shape) {
	self.queue.push(shape);
    }
}
