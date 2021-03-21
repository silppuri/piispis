use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys;
use rand::{thread_rng, Rng};
use std::thread;

const CANVAS_WIDTH: i32 = 800;
const CANVAS_HEIGHT: i32 = 600;
const PIISPIS_WIDTH: i32 = 80;
const PIISPIS_HEIGHT: i32 = 80;

const ANIM_DELAY: f64 = 0.016; // 16 FPS

const INITIAL_VELOCITY_X: i32 = 5;
const INITIAL_VELOCITY_Y: i32 = 15;

const ACCELERATION_Y: i32 = -1;

struct Piispis {
    pos_x: i32,
    pos_y: i32,
    vel_x: i32,
    vel_y: i32,

    html: Option<web_sys::HtmlElement>
}

macro_rules! console_log {
    ($($t:tt)*) => (web_sys::console::log_1(&JsValue::from(format_args!($($t)*).to_string())))
}

impl Piispis {
    pub fn new(px: i32, py: i32) {
        if !Piispis::is_in_canvas(px, py) {
            return;
        }

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

#[wasm_bindgen]
pub fn main() -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let body = document.body().unwrap();

    let canvas = document.create_element("div").unwrap();
    canvas.set_id("canvas");
    body.append_child(&canvas).unwrap();

    let callback = Closure::wrap(Box::new(|event: web_sys::MouseEvent| {

        for _n in 0..5 {
            Piispis::new(
                event.offset_x(),
                CANVAS_HEIGHT-event.offset_y(),
            );
        }
    }) as Box<dyn Fn(_)>);

    canvas.add_event_listener_with_callback(
        "mouseup",
        callback.as_ref().unchecked_ref(),
    ).unwrap();

    callback.forget();
    Ok(())
}
