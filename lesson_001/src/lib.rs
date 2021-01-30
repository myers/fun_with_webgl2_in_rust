mod gl_wrapper;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console;
use web_sys::WebGl2RenderingContext;

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

    // Set Up Data Buffers
    let vertices: [f32; 3] = [0.0, 0.0, 0.0];

    let buffer = gl
        .context
        .create_buffer()
        .ok_or("failed to create buffer")?;
    gl.context
        .bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));

    unsafe {
        let vert_array = js_sys::Float32Array::view(&vertices);

        gl.context.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &vert_array,
            WebGl2RenderingContext::STATIC_DRAW,
        );
    }

    //Set Up For Drawing
    gl.context.uniform1f(u_point_size_loc.as_ref(), 75.0);

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

    //Draw the points
    gl.context.draw_arrays(WebGl2RenderingContext::POINTS, 0, 1);

    Ok(())
}
