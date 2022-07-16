use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;
use js_sys::Math;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


// functions exported to JS
#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
    
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

// Get DOM Window/Document/Body
fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn document() -> web_sys::Document {
    window()
        .document()
        .expect("should have a document on window")
}

fn body() -> web_sys::HtmlElement {
    document().body().expect("document should have a body")
}

// Droplet Animation struct
struct Droplet {
    x: f64,
    y: f64,
    l: f64,
    xs: f64,
    ys: f64,
}

// Create Rain animation scene on canvas
#[wasm_bindgen]
pub fn setup_rain() {

    // global objects
    let window = window();
    let document = document();

    // get the canvas element
    let canvas = document.get_element_by_id("rain").unwrap();
    let canvas: HtmlCanvasElement = canvas
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();   
    
    // set canvas dimensions
    let canvas_w: u32 = window.inner_width().unwrap().as_f64().unwrap() as u32;
    let canvas_h: u32 = window.inner_height().unwrap().as_f64().unwrap() as u32;
    
    canvas.set_width(canvas_w);
    canvas.set_height(canvas_h);

    // need rendering context
    let ctx = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
    
    // setup
    let w: f64 = canvas.width() as f64;
    let h: f64 = canvas.height() as f64;
    ctx.set_stroke_style(&"rgba(255, 255, 255, 0.5)".into());
    ctx.set_line_width(1.0);
    ctx.set_line_cap("round");

    // create droplet structs
    let mut droplets = Vec::new();
    let num_drops: i32 = 500;

    for _ in 0..num_drops {
        let d: Droplet = Droplet {
            x: Math::random() * w,
            y: Math::random() * h,
            l: Math::random() * (1 as f64),
            xs: Math::random() * (4 as f64) - (2 as f64),
            ys: Math::random() * (10 as f64) + (10 as f64),
        };

        droplets.push(d);
    }

    // draw droplets aka screen refresh
    fn draw(ctx: &web_sys::CanvasRenderingContext2d, droplets: &mut Vec<Droplet>, w: f64, h: f64) {
        
        ctx.clear_rect(0.0, 0.0, w, h);

        // draw droplet line for each droplet
        for i in 0..droplets.len() {
            let d = &mut droplets[i];
            ctx.begin_path();
            ctx.move_to(d.x, d.y);
            ctx.line_to(d.x + d.l * d.xs, d.y + d.l * d.ys);
            ctx.stroke();
        }

        // move droplets along screen
        move_drops(droplets, w, h);

    }

    fn move_drops(droplets: &mut Vec<Droplet>, w: f64, h: f64) {

        for i in 0..droplets.len() {
            let d = &mut droplets[i];
            d.x += d.xs;
            d.y += d.ys;

            // recalculate droplets off screen
            if d.x > w || d.y > h {
                d.x = Math::random() * w;
                d.y = -20.0;
            }
        }
    }

    // configure set timeout
    let drop_cb = Closure::wrap(Box::new(move || {
        draw(&ctx, &mut droplets, w, h);
    }) as Box<dyn FnMut()>);

    let _ = window.set_interval_with_callback_and_timeout_and_arguments_0(drop_cb.as_ref().unchecked_ref(), 30);
    drop_cb.forget();
}


// Setup web page
#[wasm_bindgen]
pub async fn setup() -> Result<(), JsValue> {
    
    // global objects
    let window = window();
    let document = document();
    let body = body();

    #[cfg(web_sys_unstable_apis)]
    let navigator = window.navigator();
    
    #[cfg(web_sys_unstable_apis)]
    let clipboard = navigator.clipboard().ok_or({ println!("value is not Some!"); 0 }).unwrap();

    // set title
    document.set_title("Tom Jones");

    // create main element
    let main_div = document.create_element("div")?;
    main_div.set_class_name("main");

    // create name header
    let name_head = document.create_element("h1")?;
    name_head.set_class_name("header-tom");
    name_head.append_child(&document.create_text_node("Tom "))?;

    let name_head_jones = document.create_element("span")?;
    name_head_jones.set_class_name("header-jones");
    name_head_jones.append_child(&document.create_text_node("Jones"))?;
    name_head.append_child(&name_head_jones)?;

    // append name header to main element
    main_div.append_child(&name_head)?;

    // create link box
    let link_box = document.create_element("div")?;
    link_box.set_class_name("link-box");

    // create the links
    let github_span = document.create_element("span")?;
    github_span.set_class_name("blue");
    
    let github_a = document.create_element("a")?;
    github_a.append_child(&document.create_text_node("Github"))?;
    github_a.set_attribute("href", "https://github.com/lostVkng")?;
    github_a.set_attribute("target", "_blank")?;
    github_span.append_child(&github_a)?;

    let spacer_span = document.create_element("span")?;
    spacer_span.set_inner_html("&#47;&#47;");
    spacer_span.set_class_name("spacer");

    let twitter_span = document.create_element("span")?;
    twitter_span.set_class_name("blue");

    let twitter_a = document.create_element("a")?;
    twitter_a.append_child(&document.create_text_node("Twitter"))?;
    twitter_a.set_attribute("href", "https://twitter.com/tomjonesiv")?;
    twitter_a.set_attribute("target", "_blank")?;
    twitter_span.append_child(&twitter_a)?;

    link_box.append_child(&github_span)?;
    link_box.append_child(&spacer_span)?;
    link_box.append_child(&twitter_span)?;

    // append link box to main element
    main_div.append_child(&link_box)?;

    // create Canvas
    let canvas = document.create_element("canvas")?;
    canvas.set_id("rain");

    // add elements to DOM
    body.append_child(&main_div)?;
    body.append_child(&canvas)?;

    let rain_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
        
        let _ = setup_rain();
    }));

    let _rain_start = window.set_timeout_with_callback_and_timeout_and_arguments_0(rain_closure.as_ref().unchecked_ref(), 2000);

    rain_closure.forget();
    Ok(())
}