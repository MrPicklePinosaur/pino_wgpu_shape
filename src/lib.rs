//! A primitive shape rendering library

use wgpu::include_wgsl;

const VERTEX_ENTRY_POINT: &'static str = "vs_main";
const FRAGMENT_ENTRY_POINT: &'static str = "fs_main";

pub struct Point2D {
    pub x: f32,
    pub y: f32
}

pub struct Quad(pub Point2D, pub Point2D, pub Point2D, pub Point2D);

pub struct Line(pub Point2D, pub Point2D);

pub struct ShapeRenderer {
    pipeline: wgpu::RenderPipeline,
}

impl ShapeRenderer {

    pub fn new(device: &wgpu::Device, render_format: wgpu::TextureFormat) -> Self {

	let shader = device.create_shader_module(include_wgsl!("shaders/shader.wgsl"));

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
		buffers: &[]
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
	}
    }

    pub fn draw(&self, encoder: &mut wgpu::CommandEncoder, view: &wgpu::TextureView) {
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
    }

    pub fn queue(&mut self) {
	
    }
}
