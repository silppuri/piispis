use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen::prelude::*;
use web_sys;
use rand::{thread_rng, Rng};
use std::thread;

const CANVAS_WIDTH: i32 = 800;
const CANVAS_HEIGHT: i32 = 600;
const PIISPIS_WIDTH: i32 = 58;
const PIISPIS_HEIGHT: i32 = 37;

const ANIM_DELAY: f64 = 0.016; // 16 FPS

const INITIAL_VELOCITY_X: i32 = 5;
const INITIAL_VELOCITY_Y: i32 = 15;

const ACCELERATION_Y: i32 = -1;

macro_rules! console_log {
    ($($t:tt)*) => (web_sys::console::log_1(&JsValue::from(format_args!($($t)*).to_string())))
}

struct Arena {
    width: f64,
    height: f64,
}

impl Arena {
    pub fn new(height: f64, width: f64) {
        let mut arena = Arena {
            height: height,
            width: width,
        };
        let callback = Closure::wrap(Box::new(move || {
            arena.update();
        }) as Box<dyn FnMut()>);

        web_sys::window().unwrap().set_onresize(
            Some(callback.as_ref().unchecked_ref())
        );
        callback.forget();
    }
    pub fn update(self: &mut Arena) {
        let window = web_sys::window().unwrap();
        self.width = window.inner_width().unwrap().as_f64().unwrap();
        self.height = window.inner_height().unwrap().as_f64().unwrap();
    }
}

#[wasm_bindgen]
pub fn main() -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();
    let arena_html = document.create_element("div").unwrap();
    arena_html.set_attribute("class", "arena").unwrap();
    body.append_child(&arena_html).unwrap();

    let inner_width = window.inner_width().unwrap().as_f64().unwrap();
    let inner_height = window.inner_height().unwrap().as_f64().unwrap();
    Arena::new(inner_width, inner_height);

    Ok(())
}


struct Piispis {
    pos_x: i32,
    pos_y: i32,
    vel_x: i32,
    vel_y: i32,

    html: Option<web_sys::HtmlElement>
}
impl Piispis {
    pub fn new(px: i32, py: i32) {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let html = document
            .create_element("div").unwrap()
            .dyn_into::<web_sys::HtmlElement>().unwrap();
        html.set_attribute("class", "piispis").unwrap();

        document.get_element_by_id("canvas").unwrap()
            .append_child(&html).unwrap();

        let mut rng = thread_rng();
        let x: f64 = rng.gen();
        let direction: i32 = match x {
            x if x < 0.5 => -1,
            x if x >= 0.5 => 1,
            _ => 1
        };

        let mut piispis = Piispis {
            pos_x: px,
            pos_y: py,
            vel_x: direction * INITIAL_VELOCITY_X + rng.gen_range(0,5),
            vel_y: INITIAL_VELOCITY_Y + rng.gen_range(0,7),
            html: Some(html),
        };

        piispis.update();

        let callback = Closure::wrap(Box::new(move || {
            piispis.update();
        }) as Box<dyn FnMut()>);

        window.set_interval_with_callback_and_timeout_and_arguments_0(
            callback.as_ref().unchecked_ref(),
            (ANIM_DELAY*1000.0) as i32,
        ).unwrap();

        callback.forget();
    }

    pub fn is_in_canvas(pos_x: i32, pos_y: i32) -> bool {
        (pos_x - PIISPIS_WIDTH/2 > 0) && (pos_x + PIISPIS_WIDTH/2 < CANVAS_WIDTH) &&
        (pos_y - PIISPIS_HEIGHT/2 > 0)
    }

    pub fn update(self: &mut Piispis) {
        let html = match &self.html {
            None => return,
            Some(html) => html,
        };

        self.vel_y = self.vel_y + ACCELERATION_Y;
        self.pos_x = self.pos_x + self.vel_x;
        self.pos_y = self.pos_y + self.vel_y;

        if !Piispis::is_in_canvas(self.pos_x, self.pos_y) {
            html.remove();
            self.html = None;
            drop(self);
            return;
        }

        html.style().set_property(
            "top",
            &format!("{:}px", (CANVAS_HEIGHT - self.pos_y) - PIISPIS_HEIGHT/2),
        ).unwrap();

        html.style().set_property(
            "left",
            &format!("{:}px", self.pos_x-PIISPIS_WIDTH/2),
        ).unwrap();
    }
}
