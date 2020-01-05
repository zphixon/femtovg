
use std::time::Instant;

use glutin::event::{Event, WindowEvent, ElementState, KeyboardInput, VirtualKeyCode};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;

use rscanvas::{Canvas, Color, Paint, LineCap, LineJoin, Winding, renderer::GlRenderer, math};

fn main() {
    let el = EventLoop::new();
    //let wb = WindowBuilder::new().with_inner_size((800.0, 600.0).into()).with_title("A fantastic window!");
    let wb = WindowBuilder::new().with_title("A fantastic window!");

    let windowed_context = ContextBuilder::new().with_vsync(true).build_windowed(wb, &el).unwrap();
    let windowed_context = unsafe { windowed_context.make_current().unwrap() };

    let renderer = GlRenderer::new(|s| windowed_context.get_proc_address(s) as *const _).expect("Cannot create renderer");
    let mut canvas = Canvas::new(renderer);
    
    //canvas.add_font("../rust-engine/game/assets/fonts/Roboto-Regular.ttf");
    //canvas.add_font("/home/ptodorov/Workspace/harfbuzz-example/fonts/amiri-regular.ttf");
    //canvas.add_font(String::from("/usr/share/fonts/droid/DroidSerif-Regular.ttf"));
    canvas.add_font("/usr/share/fonts/noto/NotoSans-Regular.ttf");
    //canvas.add_font("/usr/share/fonts/noto/NotoSansDevanagari-Regular.ttf");
    
    //canvas.set_font(font_id);
    
    //let image_id = canvas.create_image("/home/ptodorov/Pictures/EGzVIXuXkAIyHhw.jpg", ImageFlags::empty()).expect("Cannot create image");
    //let image_id = canvas.create_image("/home/ptodorov/Downloads/645565.jpg", ImageFlags::PREMULTIPLIED).expect("Cannot create image");
    
    //dbg!(canvas.text_bounds(15.0, 300.0, "Hello World"));
    
    let mut x: f32 = 0.0;
    let mut y: f32 = 0.0;
    let mut rot = 0.0;
    
    let start = Instant::now();
    
    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        
        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::Resized(logical_size) => {
                    let dpi_factor = windowed_context.window().hidpi_factor();
                    windowed_context.resize(logical_size.to_physical(dpi_factor));
                }
                WindowEvent::KeyboardInput { input: KeyboardInput { virtual_keycode: Some(VirtualKeyCode::Left), state: ElementState::Pressed, .. }, .. } => {
					x -= 1.2;
				}
				WindowEvent::KeyboardInput { input: KeyboardInput { virtual_keycode: Some(VirtualKeyCode::Right), state: ElementState::Pressed, .. }, .. } => {
					x += 1.2;
				}
				WindowEvent::KeyboardInput { input: KeyboardInput { virtual_keycode: Some(VirtualKeyCode::Up), state: ElementState::Pressed, .. }, .. } => {
					y -= 1.2;
				}
				WindowEvent::KeyboardInput { input: KeyboardInput { virtual_keycode: Some(VirtualKeyCode::Down), state: ElementState::Pressed, .. }, .. } => {
					y += 1.2;
				}
				WindowEvent::KeyboardInput { input: KeyboardInput { virtual_keycode: Some(VirtualKeyCode::Q), state: ElementState::Pressed, .. }, .. } => {
					rot -= 0.5;
				}
				WindowEvent::KeyboardInput { input: KeyboardInput { virtual_keycode: Some(VirtualKeyCode::E), state: ElementState::Pressed, .. }, .. } => {
					rot += 0.5;
				}
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit
                }
                _ => (),
            }
            Event::RedrawRequested(_) => {
				let dpi_factor = windowed_context.window().hidpi_factor();
					
				let size = windowed_context.window().inner_size().to_physical(dpi_factor);
				
				//x += 0.1;
				
				let t = start.elapsed().as_secs_f32();
				
				canvas.begin_frame(size.width as f32, size.height as f32, dpi_factor as f32);
				
				neumorphism(&mut canvas, size.width as f32, size.height as f32);
				
				/*
				draw_spinner(&mut canvas, 15.0, 285.0, 10.0, t);
				
				canvas.set_fill_color(Color::hex("#70c1b3"));
				canvas.set_stroke_color(Color::hex("#247ba0"));
				
				draw_rects(&mut canvas, 15.0, 15.0);
				
				draw_caps(&mut canvas, 15.0, 110.0);
				
				draw_joins(&mut canvas, 110.0, 110.0);
				
				draw_lines(&mut canvas, 205.0, 110.0);
				*/
				
				if false {
					let text = "Thanks for using our products and services (“Services”). If you’re based in the European Economic Area or Switzerland";
					let bounds = canvas.text_bounds(15.0, 300.0, text);
					
					//dbg!(bounds);
					//canvas.rotate(math::Deg(rot));
					canvas.begin_path();
					canvas.set_stroke_width(1.0);
					canvas.rect(bounds[0], bounds[1], bounds[2] - bounds[0], bounds[3] - bounds[1]);
					//canvas.stroke();
					
					canvas.set_fill_color(Color::hex("#454545"));
					canvas.set_font_size(16);
					canvas.text(15.0 + x, 230.0 + y, text);
					canvas.set_font_blur(1.0);
					canvas.set_fill_color(Color::rgbaf(0.0, 0.0, 0.0, 0.3));
					//canvas.text(15.0 + x, 80.0 + y, text);
					
					//canvas.text(15.0 + x, 30.0 + bounds[3] - bounds[1], "qpowieqpwoei");
				}
				 
				
				/*
				canvas.set_line_cap(LineCap::Butt);
				canvas.set_line_join(LineJoin::Miter);
				canvas.begin_path();
				canvas.move_to(200.0, 100.0);
				canvas.line_to(350.0, 100.0);
				canvas.line_to(500.0, 100.0 + (x.sin() * 50.0));
				canvas.stroke();
				
				canvas.set_line_cap(LineCap::Round);
				canvas.set_line_join(LineJoin::Round);
				canvas.begin_path();
				canvas.move_to(200.0, 200.0);
				canvas.line_to(350.0, 200.0);
				canvas.line_to(500.0, 200.0 + (x.sin() * 50.0));
				canvas.stroke();
				
				canvas.set_line_cap(LineCap::Square);
				canvas.set_line_join(LineJoin::Bevel);
				canvas.begin_path();
				canvas.move_to(200.0, 300.0);
				canvas.line_to(350.0, 300.0);
				canvas.line_to(500.0, 300.0 + (x.sin() * 50.0));
				canvas.stroke();
				*/
				
				/*
				canvas.begin_path();
				canvas.rect(10.0, 20.0, 200.0, 100.0);
				canvas.set_stroke_width(10.0);
				canvas.stroke();
				*/
				
				//canvas.rounded_rect(10.0, 20.0, 200.0, 100.0, 10.0);
				//canvas.rounded_rect_varying(10.0, 20.0, 200.0, 100.0, 5.0, 10.0, 15.0, 20.0);
				//canvas.rounded_rect_varying(20.0 + (x.sin() * 10.0), 20.0, 200.0, 100.0, 5.0, 10.0, 15.0, 20.0);
				//canvas.rect(50.0, 50.0, 300.0, 400.0);
				
				/*
				canvas.begin_path();
				canvas.move_to(100.0, 100.0);
				canvas.line_to(300.0, 150.0);
				canvas.line_to(300.0, 350.0);
				canvas.line_to(100.0, 300.0);
				canvas.fill();
				*/
				
				//canvas.rotate(math::Deg(x));
				//canvas.translate(10.0, 20.0);
				//canvas.skew_x(math::Deg(10.0));
				
				// Gradients
				if false {
					canvas.set_fill_paint(Paint::linear_gradient(50.0, 50.0, 150.0, 150.0, Color::rgb(0, 0, 0), Color::rgb(255, 0, 0)));
					canvas.set_stroke_paint(Paint::linear_gradient(50.0, 50.0, 150.0, 150.0, Color::rgb(255, 0, 0), Color::rgb(0, 0, 0)));
					canvas.begin_path();
					canvas.rounded_rect(50.0, 50.0, 100.0, 100.0, 10.0);
					canvas.fill();
					canvas.stroke();
					
					canvas.save();
					canvas.translate(170.0, 50.0);
					canvas.set_fill_paint(Paint::box_gradient(0.0, 0.0, 100.0, 100.0, 0.0, 20.0, Color::rgba(0, 0, 0, 128), Color::rgba(0, 0, 0, 0)));
					canvas.begin_path();
					canvas.rect(0.0, 0.0, 100.0, 100.0);
					canvas.fill();
					canvas.restore();
					
					canvas.save();
					canvas.translate(290.0, 50.0);
					canvas.set_fill_paint(Paint::radial_gradient(50.0, 50.0, 0.0, 50.0, Color::rgb(0, 0, 0), Color::rgb(255, 255, 255)));
					canvas.begin_path();
					canvas.rect(0.0, 0.0, 100.0, 100.0);
					canvas.fill();
					canvas.restore();
				}
				
				// arc_to test
				if false {
					canvas.save();
					canvas.translate(0.0, 140.0);
					canvas.begin_path();
					canvas.move_to(20.0, 20.0);
					canvas.line_to(100.0, 20.0);
					canvas.arc_to(150.0, 20.0, 150.0, 70.0, 50.0);
					canvas.line_to(150.0, 120.0);
					canvas.stroke();
					canvas.restore();
				}
				
				// Image
				/*
				if false {
					
					canvas.save();
					//canvas.translate(170.0, 170.0);
					
					canvas.set_fill_color(Color::hex("#70c1b3"));
					canvas.begin_path();
					canvas.rect(0.0, 0.0, 512.0, 512.0);
					canvas.fill();
					
					let paint = Paint::image(graphics::ImageId(0), 0.0, 0.0, 512.0, 512.0, math::Rad(0.0), 1.0);
					canvas.set_fill_paint(paint);
					
					canvas.begin_path();
					canvas.rect(0.0, 0.0, 512.0, 512.0);
					canvas.fill();
					canvas.restore();
				}*/
				
				canvas.end_frame();
				
				windowed_context.swap_buffers().unwrap();
			}
            Event::MainEventsCleared => {
				windowed_context.window().request_redraw()
			}
            _ => (),
        }
    });
}

fn neumorphism(canvas: &mut Canvas, w: f32, h: f32) {
	canvas.save();
	
	canvas.begin_path();
	canvas.set_fill_color(Color::hex("#efeeee"));
	canvas.rect(0.0, 0.0, w, h);
	canvas.fill();
	
	let rect_w = 350.0;
	let rect_h = 220.0;
	let x = (w - rect_w) / 2.0;
	let y = (h - rect_h) / 2.0;
	
	let shadow = Paint::box_gradient(x, y, rect_w, rect_h, 12.0, 16.0, Color::rgba(0, 0, 0, 128), Color::rgba(0, 0, 0, 0));
	canvas.begin_path();
	canvas.rounded_rect(x + 6.0, y + 6.0, rect_w, rect_h, 12.0);
	canvas.set_fill_paint(shadow);
	canvas.fill();
	
	let shadow = Paint::box_gradient(x, y, rect_w, rect_h, 12.0, 26.0, Color::rgba(255, 255, 255, 211), Color::rgba(0, 0, 0, 0));
	canvas.begin_path();
	canvas.rounded_rect(x - 6.0, y - 6.0, rect_w, rect_h, 12.0);
	canvas.set_fill_paint(shadow);
	canvas.fill();
	
	canvas.begin_path();
	canvas.set_fill_color(Color::hex("#efeeee"));
	canvas.set_stroke_width(1.0);
	canvas.set_stroke_color(Color::rgbaf(1.0, 1.0, 1.0, 0.2));
	canvas.rounded_rect(x, y, rect_w, rect_h, 12.0);
	canvas.fill();
	canvas.stroke();
	
	canvas.restore();
}

fn draw_joins(canvas: &mut Canvas, x: f32, y: f32) {
    canvas.save();
    canvas.translate(x, y);
    
    let w = 50.0;
    
    canvas.set_stroke_color(Color::hex("#247ba0"));
    
    canvas.begin_path();
    canvas.rect(0.0, 0.0, 80.0, 80.0);
    canvas.stroke();
    
    canvas.scissor(0.0, 0.0, 80.0, 80.0);
    
    canvas.set_stroke_color(Color::hex("#70c1b3"));
    canvas.set_stroke_width(10.0);
    canvas.set_line_cap(LineCap::Butt);
    
    /* TODO: this panics with "attempt to subtract with overflow"
    canvas.set_line_join(LineJoin::Miter);
    canvas.begin_path();
    canvas.move_to(0.0, 40.0);
    canvas.line_to(w/2.0, 10.0);
    canvas.move_to(w, 40.0);
    canvas.stroke();
    */
    
    canvas.translate(15.0, 0.0);
    
    canvas.set_line_join(LineJoin::Miter);
    canvas.begin_path();
    canvas.move_to(0.0, 40.0);
    canvas.line_to(w/2.0, 10.0);
    canvas.line_to(w, 40.0);
    canvas.stroke();
    
    canvas.translate(0.0, 25.0);
    
    canvas.set_line_join(LineJoin::Bevel);
    canvas.begin_path();
    canvas.move_to(0.0, 40.0);
    canvas.line_to(w/2.0, 10.0);
    canvas.line_to(w, 40.0);
    canvas.stroke();
    
    canvas.translate(0.0, 25.0);
    
    canvas.set_line_join(LineJoin::Round);
    canvas.begin_path();
    canvas.move_to(0.0, 40.0);
    canvas.line_to(w/2.0, 10.0);
    canvas.line_to(w, 40.0);
    canvas.stroke();
    
    canvas.restore();
}

fn draw_caps(canvas: &mut Canvas, x: f32, y: f32) {
    canvas.save();
    canvas.translate(x, y);
    
    let w = 80.0;
    
    canvas.set_stroke_color(Color::hex("#247ba0"));
    
    canvas.begin_path();
    canvas.rect(0.0, 0.0, 80.0, 80.0);
    canvas.stroke();
    
    canvas.set_stroke_color(Color::hex("#70c1b3"));
    
    canvas.set_stroke_width(12.0);
    
    canvas.set_line_cap(LineCap::Butt);
    canvas.begin_path();
    canvas.move_to(20.0, 15.0);
    canvas.line_to(60.0, 15.0);
    canvas.stroke();
    
    canvas.set_line_cap(LineCap::Square);
    canvas.begin_path();
    canvas.move_to(20.0, 40.0);
    canvas.line_to(60.0, 40.0);
    canvas.stroke();
    
    canvas.set_line_cap(LineCap::Round);
    canvas.begin_path();
    canvas.move_to(20.0, 65.0);
    canvas.line_to(60.0, 65.0);
    canvas.stroke();
    
    canvas.restore();
}

fn draw_lines(canvas: &mut Canvas, x: f32, y: f32) {
    canvas.save();
    canvas.translate(x, y);
    
    let w = 80.0;
    
    for i in 0..8 {
        canvas.set_stroke_width(i as f32);
        
        canvas.begin_path();
        canvas.move_to(0.0, i as f32 * 10.0);
        canvas.line_to(w, 10.0 + i as f32 * 10.0);
        canvas.stroke();
    }
    
    canvas.set_shape_anti_alias(false);
    
    canvas.translate(95.0, 0.0);
    
    for i in 0..8 {
        canvas.set_stroke_width(i as f32);
        
        canvas.begin_path();
        canvas.move_to(0.0, i as f32 * 10.0);
        canvas.line_to(w, 10.0 + i as f32 * 10.0);
        canvas.stroke();
    }
    
    canvas.restore();
}

fn draw_rects(canvas: &mut Canvas, x: f32, y: f32) {
    canvas.set_stroke_width(2.0);
    
    canvas.save();
    canvas.translate(x, y);
    
    canvas.begin_path();
    canvas.rect(0.0, 0.0, 80.0, 80.0);
    canvas.fill();
    
    canvas.translate(95.0, 0.0);
    canvas.begin_path();
    canvas.rect(0.0, 0.0, 80.0, 80.0);
    canvas.stroke();
    
    canvas.translate(95.0, 0.0);
    canvas.begin_path();
    canvas.rounded_rect(0.0, 0.0, 80.0, 80.0, 10.0);
    canvas.fill();
    
    canvas.translate(95.0, 0.0);
    canvas.begin_path();
    canvas.rounded_rect(0.0, 0.0, 80.0, 80.0, 10.0);
    canvas.stroke();
    
    canvas.translate(95.0, 0.0);
    canvas.begin_path();
    canvas.rounded_rect_varying(0.0, 0.0, 80.0, 80.0, 20.0, 20.0, 5.0, 5.0);
    canvas.fill();
    canvas.stroke();
    
    // TODO: Instead of save/restore pairs try doing something with scopes or closures
    // Or use temp var and use drop to restore state
    canvas.translate(95.0, 0.0);
    
    canvas.save();
    canvas.translate(40.0, 0.0);
    canvas.rotate(math::Deg(45.0));
    canvas.begin_path();
    canvas.rounded_rect(0.0, 0.0, 55.0, 55.0, 5.0);
    canvas.stroke();
    canvas.restore();
    
    canvas.translate(95.0, 0.0);
    canvas.save();
    canvas.skew_x(math::Deg(-10.0));
    canvas.begin_path();
    canvas.rect(0.0, 0.0, 80.0, 80.0);
    canvas.stroke();
    canvas.restore();
    
    canvas.translate(95.0, 0.0);
    canvas.begin_path();
    canvas.circle(40.0, 40.0, 40.0);
    canvas.fill();
    canvas.stroke();
    
    canvas.translate(95.0, 0.0);
    canvas.begin_path();
    canvas.ellipse(40.0, 40.0, 30.0, 40.0);
    canvas.fill();
    canvas.stroke();
    
    canvas.translate(95.0, 0.0);
    draw_star(canvas, 0.0, 0.0, 80.0);
    
    canvas.restore();
}

fn draw_star(canvas: &mut Canvas, cx: f32, cy: f32, scale: f32) {
    canvas.save();
    
    let r = 0.45 * scale;
    let tau = 6.2831853;
    
    canvas.translate(cx + scale*0.5, cy + scale*0.5);
    canvas.begin_path();
    canvas.move_to(r, 0.0);
    
    for i in 0..7 {
        let theta = 3.0 * i as f32 * tau / 7.0;
        canvas.line_to(theta.cos() * r, theta.sin() * r);
    }
    
    canvas.translate(scale * 0.5, scale * 0.5);
    canvas.close_path();
    // canvas.fill(); TODO: Why is this not filling ok
    canvas.stroke();
    
    canvas.restore();
}

fn draw_spinner(canvas: &mut Canvas, cx: f32, cy: f32, r: f32, t: f32) {
	let a0 = 0.0 + t * 6.0;
	let a1 = std::f32::consts::PI + t * 6.0;
	let r0 = r;
	let r1 = r * 0.75;

	canvas.save();

	canvas.begin_path();
	canvas.arc(cx,cy, r0, a0, a1, Winding::CW);
	canvas.arc(cx,cy, r1, a1, a0, Winding::CCW);
	canvas.close_path();
	
	let ax = cx + a0.cos() * (r0+r1)*0.5;
	let ay = cy + a0.sin() * (r0+r1)*0.5;
	let bx = cx + a1.cos() * (r0+r1)*0.5;
	let by = cy + a1.sin() * (r0+r1)*0.5;
	
	let paint = Paint::linear_gradient(ax, ay, bx, by, Color::rgba(0, 0, 0, 0), Color::rgba(0, 0, 0, 128));
	canvas.set_fill_paint(paint);
	canvas.fill();

	canvas.restore();
}