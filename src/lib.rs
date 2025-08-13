use rand::{Rng, thread_rng};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys;

const CANVAS_HEIGHT: i32 = 600;
const PIISPIS_WIDTH: i32 = 58;
const PIISPIS_HEIGHT: i32 = 37;

const ANIM_DELAY_MS: i32 = 16; // 16ms for ~60 FPS
const INITIAL_VELOCITY_X: i32 = 5;
const INITIAL_VELOCITY_Y: i32 = 15;
const ACCELERATION_Y: i32 = -1;

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Velocity {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Piispis {
    position: Position,
    velocity: Velocity,
    html_element: Option<web_sys::HtmlElement>,
}

macro_rules! console_log {
    ($($t:tt)*) => (web_sys::console::log_1(&JsValue::from(format_args!($($t)*).to_string())))
}

impl Piispis {
    /// Creates and spawns a new Piispis at the given position
    pub fn spawn(x: i32, y: i32) -> Result<(), JsValue> {
        if !Self::is_valid_position(x, y) {
            return Ok(()); // Silently ignore invalid positions
        }

        let html_element = Self::create_html_element()?;
        let mut rng = thread_rng();

        // Generate random direction and velocity variations
        let direction = if rng.gen_bool(0.5) { -1 } else { 1 };
        let velocity_x_variation = rng.gen_range(0, 5);
        let velocity_y_variation = rng.gen_range(0, 7);

        let piispis = Rc::new(RefCell::new(Piispis {
            position: Position { x, y },
            velocity: Velocity {
                x: direction * INITIAL_VELOCITY_X + velocity_x_variation,
                y: INITIAL_VELOCITY_Y + velocity_y_variation,
            },
            html_element: Some(html_element),
        }));

        // Initial position update
        piispis.borrow_mut().update_position()?;

        // Set up animation loop
        Self::start_animation_loop(piispis)?;

        Ok(())
    }

    fn create_html_element() -> Result<web_sys::HtmlElement, JsValue> {
        let window = web_sys::window().ok_or("No window found")?;
        let document = window.document().ok_or("No document found")?;

        let html_element = document
            .create_element("div")?
            .dyn_into::<web_sys::HtmlElement>()?;

        html_element.set_class_name("piispis");

        let canvas = document
            .get_element_by_id("canvas")
            .ok_or("Canvas element not found")?;

        canvas.append_child(&html_element)?;

        Ok(html_element)
    }

    fn start_animation_loop(piispis: Rc<RefCell<Piispis>>) -> Result<(), JsValue> {
        let window = web_sys::window().ok_or("No window found")?;

        let piispis_clone = piispis.clone();
        let callback = Closure::wrap(Box::new(move || {
            let should_continue = {
                let mut piispis_ref = piispis_clone.borrow_mut();
                match piispis_ref.update_position() {
                    Ok(still_alive) => still_alive,
                    Err(_) => false,
                }
            };

            if !should_continue {
                // Piispis has been removed, stop the animation
                return;
            }
        }) as Box<dyn FnMut()>);

        window.set_interval_with_callback_and_timeout_and_arguments_0(
            callback.as_ref().unchecked_ref(),
            ANIM_DELAY_MS,
        )?;

        callback.forget();
        Ok(())
    }

    /// Checks if the given position is within valid canvas bounds
    fn is_valid_position(_x: i32, y: i32) -> bool {
        let half_height = PIISPIS_HEIGHT / 2;

        // If over the bottom of canvas -> valid position
        y - half_height > 0
    }

    /// Updates the piispis position and returns whether it should continue animating
    fn update_position(&mut self) -> Result<bool, JsValue> {
        let html_element = match &self.html_element {
            Some(element) => element,
            None => return Ok(false),
        };

        // Update physics
        self.velocity.y += ACCELERATION_Y;
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;

        // Check bounds
        if !Self::is_valid_position(self.position.x, self.position.y) {
            html_element.remove();
            self.html_element = None;
            return Ok(false);
        }

        // Update DOM position
        self.update_dom_position(html_element)?;
        Ok(true)
    }

    fn update_dom_position(&self, html_element: &web_sys::HtmlElement) -> Result<(), JsValue> {
        let style = html_element.style();

        let top_px = (CANVAS_HEIGHT - self.position.y) - PIISPIS_HEIGHT / 2;
        let left_px = self.position.x - PIISPIS_WIDTH / 2;

        style.set_property("top", &format!("{}px", top_px))?;
        style.set_property("left", &format!("{}px", left_px))?;

        Ok(())
    }
}

#[wasm_bindgen]
pub fn main() -> Result<(), JsValue> {
    setup_canvas()?;
    setup_mouse_handler()?;
    Ok(())
}

fn setup_canvas() -> Result<web_sys::Element, JsValue> {
    let window = web_sys::window().ok_or("No window found")?;
    let document = window.document().ok_or("No document found")?;
    let body = document.body().ok_or("No body found")?;

    let canvas = document.create_element("div")?;
    canvas.set_id("canvas");
    body.append_child(&canvas)?;

    Ok(canvas)
}

fn setup_mouse_handler() -> Result<(), JsValue> {
    let document = web_sys::window()
        .ok_or("No window found")?
        .document()
        .ok_or("No document found")?;

    let canvas = document
        .get_element_by_id("canvas")
        .ok_or("Canvas element not found")?;

    let callback = Closure::wrap(Box::new(|event: web_sys::MouseEvent| {
        let spawn_count = 5;
        let click_x = event.offset_x();
        let click_y = CANVAS_HEIGHT - event.offset_y();

        for _ in 0..spawn_count {
            if let Err(e) = Piispis::spawn(click_x, click_y) {
                console_log!("Error spawning piispis: {:?}", e);
            }
        }
    }) as Box<dyn Fn(_)>);

    canvas.add_event_listener_with_callback("mouseup", callback.as_ref().unchecked_ref())?;
    callback.forget();

    Ok(())
}
