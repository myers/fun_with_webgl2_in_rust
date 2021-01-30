// convert https://raw.githubusercontent.com/sketchpunk/FunWithWebGL2/master/lesson_002/gl.js to rust

use js_sys;
use std::convert::TryInto;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext, WebGlProgram, WebGlShader};

pub struct GlWrapper {
    pub canvas: HtmlCanvasElement,
    pub context: WebGl2RenderingContext,
}

impl GlWrapper {
    pub fn new(canvas_id: &str) -> Result<Self, JsValue> {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let canvas = document.get_element_by_id(canvas_id).unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;
        let context = canvas
            .get_context("webgl2")?
            .unwrap()
            .dyn_into::<WebGl2RenderingContext>()?;

        context.clear_color(1.0, 1.0, 1.0, 1.0);

        Ok(GlWrapper {
            canvas: canvas,
            context: context,
        })
    }

    pub fn clear(&mut self) {
        self.context.clear(
            WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT,
        );
    }

    pub fn set_size(&mut self, w: u32, h: u32) -> Result<(), JsValue> {
        let canvas_style = self.canvas.style();
        canvas_style.set_property("width", &*format!("{}px", w))?;
        canvas_style.set_property("height", &*format!("{}px", h))?;
        self.canvas.set_width(w);
        self.canvas.set_height(h);
        self.context
            .viewport(0, 0, w.try_into().unwrap(), h.try_into().unwrap());
        Ok(())
    }

    pub fn compile_shader(
        &mut self,
        shader_type: u32,
        source: &str,
    ) -> Result<WebGlShader, String> {
        let shader = self
            .context
            .create_shader(shader_type)
            .ok_or_else(|| String::from("Unable to create shader object"))?;
        self.context.shader_source(&shader, source);
        self.context.compile_shader(&shader);

        if self
            .context
            .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(shader)
        } else {
            Err(self
                .context
                .get_shader_info_log(&shader)
                .unwrap_or_else(|| String::from("Unknown error creating shader")))
        }
    }

    pub fn link_program(
        &mut self,
        vert_shader: &WebGlShader,
        frag_shader: &WebGlShader,
    ) -> Result<WebGlProgram, String> {
        let program = self
            .context
            .create_program()
            .ok_or_else(|| String::from("Unable to create shader object"))?;

        self.context.attach_shader(&program, vert_shader);
        self.context.attach_shader(&program, frag_shader);
        self.context.link_program(&program);

        if self
            .context
            .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(program)
        } else {
            Err(self
                .context
                .get_program_info_log(&program)
                .unwrap_or_else(|| String::from("Unknown error creating program object")))
        }
    }

    pub fn build_program(
        &mut self,
        vertex_shader_src: &str,
        fragment_shader_src: &str,
    ) -> Result<WebGlProgram, String> {
        let vert_shader =
            self.compile_shader(WebGl2RenderingContext::VERTEX_SHADER, vertex_shader_src)?;
        let frag_shader =
            self.compile_shader(WebGl2RenderingContext::FRAGMENT_SHADER, fragment_shader_src)?;
        let prog = self.link_program(&vert_shader, &frag_shader)?;
        Ok(prog)
    }
}
