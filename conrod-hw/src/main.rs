#[macro_use]
extern crate conrod;
extern crate rand;
extern crate ttf_noto_sans;

// --- std ---
use std::{
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
};

// --- external ---
use conrod::{
    Borderable,
    Colorable,
    Labelable,
    Positionable,
    Sizeable,
    Widget,
    backend::glium::glium::{self, Surface, glutin},
    text::FontCollection,
    widget,
};

struct EventLoop {
    last_update: Instant,
    ui_needs_update: bool,
}

impl EventLoop {
    fn new() -> EventLoop {
        EventLoop {
            last_update: Instant::now(),
            ui_needs_update: true,
        }
    }

    fn next(&mut self, events_loop: &mut glutin::EventsLoop) -> Vec<glutin::Event> {
        let last_update = self.last_update;
        let sixteen_ms = Duration::from_millis(16);
        let duration_since_last_update = Instant::now().duration_since(last_update);
        if duration_since_last_update < sixteen_ms { thread::sleep(sixteen_ms - duration_since_last_update); }

        let mut events = vec![];
        events_loop.poll_events(|event| events.push(event));
//        if events.is_empty() && !self.ui_needs_update {
//            events_loop.run_forever(|event| {
//                events.push(event);
//                glutin::ControlFlow::Break
//            });
//        }

        self.ui_needs_update = false;
        self.last_update = Instant::now();

        events
    }

    fn needs_update(&mut self) { self.ui_needs_update = true; }
}

fn main() {
    const WIDTH: u32 = 400;
    const HEIGHT: u32 = 200;

    let mut events_loop = glutin::EventsLoop::new();
    let window_builder = glutin::WindowBuilder::new()
        .with_title("AMD YES!")
        .with_dimensions(WIDTH, HEIGHT);
    let context_builder = glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(4);
    let display = glium::Display::new(window_builder, context_builder, &events_loop).unwrap();

    let mut ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();

    ui.fonts.insert(
        FontCollection::from_bytes(ttf_noto_sans::REGULAR)
            .into_font()
            .unwrap()
    );

    widget_ids!(struct Ids { text, btn, auto_btn });
    let text_i = Arc::new(Mutex::new(1u32));
    let text_color = Arc::new(Mutex::new(conrod::color::BLACK));
    let mut btn_color = conrod::color::BLACK;
    let ids = Ids::new(ui.widget_id_generator());

    let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();
    let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();

    let mut event_loop = EventLoop::new();
    'main: loop {
        for event in event_loop.next(&mut events_loop) {
            if let Some(event) = conrod::backend::winit::convert_event(event.clone(), &display) {
                ui.handle_event(event);
                event_loop.needs_update();
            }

            match event {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::KeyboardInput {
                        input: glutin::KeyboardInput {
                            virtual_keycode: Some(glutin::VirtualKeyCode::Escape),
                            ..
                        },
                        ..
                    } => break 'main,
                    _ => ()
                }
                _ => ()
            }
        }

        {
            let ui = &mut ui.set_widgets();

            widget::Text::new(&format!("AMD YES! +{}", text_i.lock().unwrap()))
                .mid_bottom_with_margin_on(ids.btn, 40.)
                .color(*text_color.lock().unwrap())
                .font_size(32)
                .set(ids.text, ui);

            if widget::Button::new()
                .label("YES!")
                .label_font_size(32)
                .label_y(conrod::position::Relative::Scalar(2.5))
                .w(60.)
                .h(20.)
                .middle_of(ui.window)
                .border(0.)
                .color(conrod::color::WHITE)
                .label_color(btn_color)
                .press_color(conrod::color::WHITE)
                .hover_color(conrod::color::WHITE)
                .set(ids.btn, ui)
                .was_clicked() {
                *text_i.lock().unwrap() += 1;
                *text_color.lock().unwrap() = conrod::color::rgb(rand::random(), rand::random(), rand::random());
                btn_color = conrod::color::rgb(rand::random(), rand::random(), rand::random());
            }

            if widget::Button::new()
                .label("AUTO YES!")
                .label_font_size(32)
                .label_y(conrod::position::Relative::Scalar(2.5))
                .w(60.)
                .h(20.)
                .mid_top_with_margin_on(ids.btn, 40.)
                .border(0.)
                .color(conrod::color::WHITE)
                .label_color(btn_color)
                .press_color(conrod::color::WHITE)
                .hover_color(conrod::color::WHITE)
                .set(ids.auto_btn, ui)
                .was_clicked() {
                let text_i = Arc::clone(&text_i);
                let text_color = Arc::clone(&text_color);
                thread::spawn(move || {
                    loop {
                        *text_i.lock().unwrap() += 1;
                        *text_color.lock().unwrap() = conrod::color::rgb(rand::random(), rand::random(), rand::random());
                        thread::sleep(Duration::from_millis(100));
                    }
                });
            }
        }

        if let Some(primitives) = ui.draw_if_changed() {
            renderer.fill(&display, primitives, &image_map);
            let mut target = display.draw();
            target.clear_color(1., 1., 1., 1.);
            renderer.draw(&display, &mut target, &image_map).unwrap();
            target.finish().unwrap();
        }
    }
}
