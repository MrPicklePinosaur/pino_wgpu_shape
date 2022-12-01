use wgpu::SurfaceError;
use winit::{event_loop::{EventLoop, ControlFlow}, window::WindowBuilder, event::{Event, WindowEvent}};

fn main() {

    // bunch of setup boilerplate
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("highgui")
        .build(&event_loop)
        .unwrap();

    let instance = wgpu::Instance::new(wgpu::Backends::all());
    let surface = unsafe { instance.create_surface(&window) };

    let (device, queue) = futures::executor::block_on(async {
	
	let adapter = instance
	    .request_adapter(&wgpu::RequestAdapterOptions {
		power_preference: wgpu::PowerPreference::default(),
		compatible_surface: Some(&surface),
		force_fallback_adapter: false,
	    })
	    .await
	    .unwrap();

	adapter
	    .request_device(
		&wgpu::DeviceDescriptor {
		    features: wgpu::Features::empty(),
		    limits: if cfg!(target_arch = "wasm32") {
			wgpu::Limits::downlevel_webgl2_defaults()
		    } else {
			wgpu::Limits::default()
		    },
		    label: None,
		},
		None,
	    )
	    .await
	    .unwrap()
    });


    let render_format = wgpu::TextureFormat::Bgra8UnormSrgb;
    let mut size = window.inner_size();
    let mut config = wgpu::SurfaceConfiguration {
	usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
	format: render_format,
	width: size.width,
	height: size.height,
	present_mode: wgpu::PresentMode::Fifo,
	alpha_mode: wgpu::CompositeAlphaMode::Auto,
    };

    surface.configure(&device, &config);

    // run event loop
    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } => {
            if window_id == window.id() {
                match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(physical_size) => {
			let new_size = *physical_size;
			if new_size.width > 0 && new_size.height > 0 {
			    config.width = new_size.width;
			    config.height = new_size.height;
			    surface.configure(&device, &config);
			}
                    }
                    _ => {}
                }
            }
        },
        Event::RedrawRequested(window_id) => {

	    let output = surface.get_current_texture().expect("could not get texture");

	    let view = output
		.texture
		.create_view(&wgpu::TextureViewDescriptor::default());
	    let mut encoder = device
		.create_command_encoder(&wgpu::CommandEncoderDescriptor {
		    label: Some("Render Encoder"),
	    });

	    let render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
		label: Some("Render Pass"),
		color_attachments: &[Some(wgpu::RenderPassColorAttachment {
		    view: &view,
		    resolve_target: None,
		    ops: wgpu::Operations {
			load: wgpu::LoadOp::Clear(wgpu::Color {
			    r: 0.0,
			    g: 0.0,
			    b: 0.0,
			    a: 1.0,
			}),
			store: true,
		    },
		})],
		depth_stencil_attachment: None,
	    });
	    drop(render_pass);

	    queue.submit(std::iter::once(encoder.finish()));
	    output.present()

        }

	_ => {}
    });
}

