use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console;
use web_sys::WebGl2RenderingContext;

mod gl_wrapper;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    // console::log_1(&JsValue::from_str("Hello world!"));

    let mut gl = gl_wrapper::GlWrapper::new("main")?;

    gl.set_size(500, 500)?;
    gl.clear();
    let prog = gl.build_program(
        include_str!("vertex.vshader"),
        include_str!("fragment.fshader"),
    )?;

    // 4. Get Location of Uniforms and Attributes.
    gl.context.use_program(Some(&prog));

    let a_position_loc: u32 = gl.context.get_attrib_location(&prog, "a_position") as u32;
    let u_point_size_loc = gl.context.get_uniform_location(&prog, "uPointSize");
    let u_angle_loc = gl.context.get_uniform_location(&prog, "uAngle");

    // Set Up Data Buffers
    let vertices: [f32; 3] = [0.0, 0.0, 0.0];
    let vertex_count: i32 = (vertices.len() / 3) as i32;

    let buffer = gl.create_array_buffer(&vertices)?;

    //Tell gl which buffer we want to use at the moment
    gl.context
        .bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));

    //Enable the position attribute in the shader
    gl.context.enable_vertex_attrib_array(a_position_loc);

    //Set which buffer the attribute will pull its data from
    gl.context.vertex_attrib_pointer_with_i32(
        a_position_loc,
        3,
        WebGl2RenderingContext::FLOAT,
        false,
        0,
        0,
    );

    //Done setting up the buffer
    gl.context
        .bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, None);

    let mut point_size = 0.0;
    let point_size_step = 3.0;
    let mut angle = 0.0;
    let angle_step = (std::f32::consts::PI / 180.0) * 90.0;

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let performance = window().performance().unwrap();

    let mut previous_render: f32 = performance.now() as f32;

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        let now = performance.now() as f32;
        let delta_time = (now - previous_render) / 1000.0;
        previous_render = now;

        point_size += delta_time * point_size_step;
        let psize = (point_size.sin() * 10.0) + 30.0;
        gl.context.uniform1f(u_point_size_loc.as_ref(), psize);

        angle += delta_time * angle_step;
        gl.context.uniform1f(u_angle_loc.as_ref(), angle);

        gl.clear();
        gl.context
            .draw_arrays(WebGl2RenderingContext::POINTS, 0, vertex_count);

        // Schedule ourself for another requestAnimationFrame callback.
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());

    Ok(())
}

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}
