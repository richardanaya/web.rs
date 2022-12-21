use core::hash::{Hash, Hasher};
pub use html_color::*;
pub use js::*;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

pub fn random() -> f64 {
    let random = js!(r#"
        function(){
            return Math.random();
        }"#);
    random.invoke(&[])
}

pub fn console_log(message: &str) {
    let console_log = js!(r#"
        function(message){
            console.log(message);
        }"#);
    console_log.invoke(&[message.into()]);
}

pub fn console_error(message: &str) {
    let console_error = js!(r#"
        function(message){
            console.error(message);
        }"#);
    console_error.invoke(&[message.into()]);
}

pub fn console_warn(message: &str) {
    let console_warn = js!(r#"
        function(message){
            console.warn(message);
        }"#);
    console_warn.invoke(&[message.into()]);
}

pub fn console_time(label: &str) {
    let console_time = js!(r#"
        function(label){
            console.time(label);
        }"#);
    console_time.invoke(&[label.into()]);
}

pub fn console_time_end(label: &str) {
    let console_time_end = js!(r#"
        function(label){
            console.timeEnd(label);
        }"#);
    console_time_end.invoke(&[label.into()]);
}

pub fn query_selector(selector: &str) -> ExternRef {
    let query_selector = js!(r#"
        function(selector){
            return document.querySelector(selector);
        }"#);
    query_selector.invoke_and_return_object(&[selector.into()])
}

pub fn element_set_inner_html(element: &ExternRef, html: &str) {
    let set_inner_html = js!(r#"
        function(element, html){
            element.innerHTML = html;
        }"#);
    set_inner_html.invoke(&[element.into(), html.into()]);
}

pub fn element_add_class(element: &ExternRef, class: &str) {
    let add_class = js!(r#"
        function(element, class){
            element.classList.add(class);
        }"#);
    add_class.invoke(&[element.into(), class.into()]);
}

pub fn element_remove_class(element: &ExternRef, class: &str) {
    let remove_class = js!(r#"
        function(element, class){
            element.classList.remove(class);
        }"#);
    remove_class.invoke(&[element.into(), class.into()]);
}

pub fn element_set_attribute(element: &ExternRef, attribute: &str, value: &str) {
    let set_attribute = js!(r#"
        function(element, attribute, value){
            element.setAttribute(attribute, value);
        }"#);
    set_attribute.invoke(&[element.into(), attribute.into(), value.into()]);
}

pub fn element_remove(element: &ExternRef) {
    let remove = js!(r#"
        function(element){
            element.remove();
        }"#);
    remove.invoke(&[element.into()]);
}

pub struct FunctionHandle(ExternRef);

impl PartialEq for FunctionHandle {
    fn eq(&self, other: &Self) -> bool {
        self.0.value == other.0.value
    }
}

impl Eq for FunctionHandle {}

impl Hash for FunctionHandle {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.value.hash(state);
    }
}

// Change Events
pub struct ChangeEvent {
    pub value: String,
}

static CHANGE_EVENT_HANDLERS: Mutex<
    Option<HashMap<Arc<FunctionHandle>, Box<dyn FnMut(ChangeEvent) + Send + 'static>>>,
> = Mutex::new(None);

fn add_change_event_handler(
    id: Arc<FunctionHandle>,
    handler: Box<dyn FnMut(ChangeEvent) + Send + 'static>,
) {
    let mut handlers = CHANGE_EVENT_HANDLERS.lock().unwrap();
    if let Some(h) = handlers.as_mut() {
        h.insert(id, handler);
    } else {
        let mut h = HashMap::new();
        h.insert(id, handler);
        *handlers = Some(h);
    }
}

fn remove_change_event_handler(id: &Arc<FunctionHandle>) {
    let mut handlers = CHANGE_EVENT_HANDLERS.lock().unwrap();
    if let Some(h) = handlers.as_mut() {
        h.remove(id);
    }
}

#[no_mangle]
pub extern "C" fn web_handle_change_event(id: i64, allocation_id: usize) {
    let mut handlers = CHANGE_EVENT_HANDLERS.lock().unwrap();
    if let Some(h) = handlers.as_mut() {
        for (key, handler) in h.iter_mut() {
            if key.0.value == id {
                let value = extract_string_from_memory(allocation_id);
                handler(ChangeEvent { value });
            }
        }
    }
}

pub fn add_change_event_listener(
    element: &ExternRef,
    handler: impl FnMut(ChangeEvent) + Send + 'static,
) -> Arc<FunctionHandle> {
    let function_ref = js!(r#"
        function(element ){
            const handler = (e) => {
                const value = e.target.value;
                const allocationId = this.writeUtf8ToMemory(value);
                this.module.instance.exports.web_handle_change_event_handler(id, allocationId);
            };
            const id = this.storeObject(handler);
            element.addEventListener("change",handler);
            return id;
        }"#)
    .invoke_and_return_bigint(&[element.into()]);
    let function_handle = Arc::new(FunctionHandle(ExternRef {
        value: function_ref,
    }));
    add_change_event_handler(function_handle.clone(), Box::new(handler));
    function_handle
}

pub fn element_remove_change_listener(element: &ExternRef, function_handle: &Arc<FunctionHandle>) {
    let remove_change_listener = js!(r#"
        function(element, f){
            element.removeEventListener("change", f);
        }"#);
    remove_change_listener.invoke(&[element.into(), (&(function_handle.0)).into()]);
    remove_change_event_handler(function_handle);
}

// Mouse Events
pub struct MouseEvent {
    pub offset_x: f64,
    pub offset_y: f64,
}

static MOUSE_EVENT_HANDLERS: Mutex<
    Option<HashMap<Arc<FunctionHandle>, Box<dyn FnMut(MouseEvent) + Send + 'static>>>,
> = Mutex::new(None);

fn add_mouse_event_handler(
    id: Arc<FunctionHandle>,
    handler: Box<dyn FnMut(MouseEvent) + Send + 'static>,
) {
    let mut handlers = MOUSE_EVENT_HANDLERS.lock().unwrap();
    if let Some(h) = handlers.as_mut() {
        h.insert(id, handler);
    } else {
        let mut h = HashMap::new();
        h.insert(id, handler);
        *handlers = Some(h);
    }
}

fn remove_mouse_event_handler(id: &Arc<FunctionHandle>) {
    let mut handlers = MOUSE_EVENT_HANDLERS.lock().unwrap();
    if let Some(h) = handlers.as_mut() {
        h.remove(id);
    }
}

#[no_mangle]
pub extern "C" fn web_handle_mouse_event_handler(id: i64, x: f64, y: f64) {
    let mut handlers = MOUSE_EVENT_HANDLERS.lock().unwrap();
    if let Some(h) = handlers.as_mut() {
        for (key, handler) in h.iter_mut() {
            if key.0.value == id {
                handler(MouseEvent {
                    offset_x: x,
                    offset_y: y,
                });
            }
        }
    }
}

pub fn element_add_click_listener(
    element: &ExternRef,
    handler: impl FnMut(MouseEvent) + Send + 'static,
) -> Arc<FunctionHandle> {
    let function_ref = js!(r#"
        function(element ){
            const handler = (e) => {
                this.module.instance.exports.web_handle_mouse_event_handler(id,e.offsetX, e.offsetY);
            };
            const id = this.storeObject(handler);
            element.addEventListener("click",handler);
            return id;
        }"#).invoke_and_return_bigint(&[element.into()]);
    let function_handle = Arc::new(FunctionHandle(ExternRef {
        value: function_ref,
    }));
    add_mouse_event_handler(function_handle.clone(), Box::new(handler));
    function_handle
}

pub fn element_remove_click_listener(element: &ExternRef, function_handle: &Arc<FunctionHandle>) {
    let remove_click_listener = js!(r#"
        function(element, f){
            element.removeEventListener("click", f);
        }"#);
    remove_click_listener.invoke(&[element.into(), (&(function_handle.0)).into()]);
    remove_mouse_event_handler(function_handle);
}

pub fn element_add_mouse_move_listener(
    element: &ExternRef,
    handler: impl FnMut(MouseEvent) + Send + 'static,
) -> Arc<FunctionHandle> {
    let function_ref = js!(r#"
        function(element ){
            const handler = (e) => {
                this.module.instance.exports.web_handle_mouse_event_handler(id,e.offsetX, e.offsetY);
            };
            const id = this.storeObject(handler);
            element.addEventListener("mousemove",handler);
            return id;
        }"#).invoke_and_return_bigint(&[element.into()]);
    let function_handle = Arc::new(FunctionHandle(ExternRef {
        value: function_ref,
    }));
    add_mouse_event_handler(function_handle.clone(), Box::new(handler));
    function_handle
}

pub fn element_remove_mouse_move_listener(
    element: &ExternRef,
    function_handle: &Arc<FunctionHandle>,
) {
    let remove_mouse_move_listener = js!(r#"
        function(element, f){
            element.removeEventListener("mousemove", f);
        }"#);
    remove_mouse_move_listener.invoke(&[element.into(), (&(function_handle.0)).into()]);
    remove_mouse_event_handler(function_handle);
}

pub fn element_add_mouse_down_listener(
    element: &ExternRef,
    handler: impl FnMut(MouseEvent) + Send + 'static,
) -> Arc<FunctionHandle> {
    let function_ref = js!(r#"
        function(element ){
            const handler = (e) => {
                this.module.instance.exports.web_handle_mouse_event_handler(id,e.offsetX, e.offsetY);
            };
            const id = this.storeObject(handler);
            element.addEventListener("mousedown",handler);
            return id;
        }"#).invoke_and_return_bigint(&[element.into()]);
    let function_handle = Arc::new(FunctionHandle(ExternRef {
        value: function_ref,
    }));
    add_mouse_event_handler(function_handle.clone(), Box::new(handler));
    function_handle
}

pub fn element_remove_mouse_down_listener(
    element: &ExternRef,
    function_handle: &Arc<FunctionHandle>,
) {
    let remove_mouse_down_listener = js!(r#"
        function(element, f){
            element.removeEventListener("mousedown", f);
        }"#);
    remove_mouse_down_listener.invoke(&[element.into(), (&(function_handle.0)).into()]);
    remove_mouse_event_handler(function_handle);
}

pub fn element_add_mouse_up_listener(
    element: &ExternRef,
    handler: impl FnMut(MouseEvent) + Send + 'static,
) -> Arc<FunctionHandle> {
    let function_ref = js!(r#"
        function(element ){
            const handler = (e) => {
                this.module.instance.exports.web_handle_mouse_event_handler(id,e.offsetX, e.offsetY);
            };
            const id = this.storeObject(handler);
            element.addEventListener("mouseup",handler);
            return id;
        }"#).invoke_and_return_bigint(&[element.into()]);
    let function_handle = Arc::new(FunctionHandle(ExternRef {
        value: function_ref,
    }));
    add_mouse_event_handler(function_handle.clone(), Box::new(handler));
    function_handle
}

pub fn element_remove_mouse_up_listener(
    element: &ExternRef,
    function_handle: &Arc<FunctionHandle>,
) {
    let remove_mouse_up_listener = js!(r#"
        function(element, f){
            element.removeEventListener("mouseup", f);
        }"#);
    remove_mouse_up_listener.invoke(&[element.into(), (&(function_handle.0)).into()]);
    remove_mouse_event_handler(function_handle);
}

// Request Animation Frame

static ANIMATION_FRAME_EVENT_HANDLERS: Mutex<
    Option<HashMap<i64, Box<dyn FnMut() + Send + 'static>>>,
> = Mutex::new(None);

fn add_animation_frame_event_handler(
    function_handle: i64,
    handler: Box<dyn FnMut() + Send + 'static>,
) {
    let mut h = ANIMATION_FRAME_EVENT_HANDLERS.lock().unwrap();
    if h.is_none() {
        *h = Some(HashMap::new());
    }
    h.as_mut().unwrap().insert(function_handle, handler);
}

#[no_mangle]
pub extern "C" fn web_handle_animation_frame_event_handler(id: i64) {
    let mut c = None;
    {
        let mut handlers = ANIMATION_FRAME_EVENT_HANDLERS.lock().unwrap();
        if let Some(h) = handlers.as_mut() {
            // remove
            if let Some(handler) = h.remove(&id) {
                c = Some(handler);
            }
        }
    }
    if let Some(mut c) = c {
        c();
    }
}

pub fn request_animation_frame(handler: impl FnMut() + Send + Sync + 'static) {
    let function_ref = js!(r#"
        function(){
            const handler = () => {
                this.module.instance.exports.web_handle_animation_frame_event_handler(id);
                this.deleteObject(id);
            };
            const id = this.storeObject(handler);
            requestAnimationFrame(handler);
            return id;
        }"#)
    .invoke_and_return_bigint(&[]);
    add_animation_frame_event_handler(function_ref, Box::new(handler));
}

// Keyboard Events

pub struct KeyboardEvent {
    pub key_code: f64,
}

static KEYBOARD_EVENT_HANDLERS: Mutex<
    Option<HashMap<Arc<FunctionHandle>, Box<dyn FnMut(KeyboardEvent) + Send + 'static>>>,
> = Mutex::new(None);

fn add_keyboard_event_handler(
    function_handle: Arc<FunctionHandle>,
    handler: Box<dyn FnMut(KeyboardEvent) + Send + 'static>,
) {
    let mut h = KEYBOARD_EVENT_HANDLERS.lock().unwrap();
    if h.is_none() {
        *h = Some(HashMap::new());
    }
    h.as_mut().unwrap().insert(function_handle, handler);
}

fn remove_keyboard_event_handler(function_handle: &Arc<FunctionHandle>) {
    let mut h = KEYBOARD_EVENT_HANDLERS.lock().unwrap();
    if h.is_none() {
        return;
    }
    h.as_mut().unwrap().remove(function_handle);
}

#[no_mangle]
pub extern "C" fn web_handle_keyboard_event_handler(id: i64, key_code: f64) {
    let mut handlers = KEYBOARD_EVENT_HANDLERS.lock().unwrap();
    if let Some(h) = handlers.as_mut() {
        for (key, handler) in h.iter_mut() {
            if key.0.value == id {
                handler(KeyboardEvent { key_code });
            }
        }
    }
}

pub fn element_add_key_down_listener(
    element: &ExternRef,
    handler: impl FnMut(KeyboardEvent) + Send + 'static,
) -> Arc<FunctionHandle> {
    let function_ref = js!(r#"
        function(element ){
            const handler = (e) => {
                this.module.instance.exports.web_handle_keyboard_event_handler(id,e.keyCode);
            };
            const id = this.storeObject(handler);
            element.addEventListener("keydown",handler);
            return id;
        }"#)
    .invoke_and_return_bigint(&[element.into()]);
    let function_handle = Arc::new(FunctionHandle(ExternRef {
        value: function_ref,
    }));
    add_keyboard_event_handler(function_handle.clone(), Box::new(handler));
    function_handle
}

pub fn element_remove_key_down_listener(
    element: &ExternRef,
    function_handle: &Arc<FunctionHandle>,
) {
    let remove_key_down_listener = js!(r#"
        function(element, f){
            element.removeEventListener("keydown", f);
        }"#);
    remove_key_down_listener.invoke(&[element.into(), (&(function_handle.0)).into()]);
    remove_keyboard_event_handler(function_handle);
}

pub fn element_add_key_up_listener(
    element: &ExternRef,
    handler: impl FnMut(KeyboardEvent) + Send + 'static,
) -> Arc<FunctionHandle> {
    let function_ref = js!(r#"
        function(element ){
            const handler = (e) => {
                this.module.instance.exports.web_handle_keyboard_event_handler(id,e.keyCode);
            };
            const id = this.storeObject(handler);
            element.addEventListener("keyup",handler);
            return id;
        }"#)
    .invoke_and_return_bigint(&[element.into()]);
    let function_handle = Arc::new(FunctionHandle(ExternRef {
        value: function_ref,
    }));
    add_keyboard_event_handler(function_handle.clone(), Box::new(handler));
    function_handle
}

pub fn element_remove_key_up_listener(element: &ExternRef, function_handle: &Arc<FunctionHandle>) {
    let remove_key_up_listener = js!(r#"
        function(element, f){
            element.removeEventListener("keyup", f);
        }"#);
    remove_key_up_listener.invoke(&[element.into(), (&(function_handle.0)).into()]);
    remove_keyboard_event_handler(function_handle);
}

pub fn get_property_f64(element: &ExternRef, property: &str) -> f64 {
    let get_property = js!(r#"
        function(element, property){
            return element[property];
        }"#);
    get_property.invoke(&[element.into(), property.into()])
}

pub fn set_property_f64(element: &ExternRef, property: &str, value: f64) {
    let set_property = js!(r#"
        function(element, property, value){
            element[property] = value;
        }"#);
    set_property.invoke(&[element.into(), property.into(), value.into()]);
}

pub fn get_property_bool(element: &ExternRef, property: &str) -> bool {
    let get_property = js!(r#"
        function(element, property){
            return element[property]?1:0;
        }"#);
    let v = get_property.invoke(&[element.into(), property.into()]);
    v == 1.0
}

pub fn set_property_bool(element: &ExternRef, property: &str, value: bool) {
    let set_property = js!(r#"
        function(element, property, value){
            element[property] = value !==0;
        }"#);
    set_property.invoke(&[element.into(), property.into(), value.into()]);
}

pub fn get_property_string(element: &ExternRef, property: &str) -> String {
    let get_property = js!(r#"
        function(element, property){
            const text = element[property];
            const allocationId = this.writeUtf8ToMemory(text);
            return allocationId;
        }"#);
    let text_allocation_id = get_property.invoke(&[element.into(), property.into()]);
    let text = extract_string_from_memory(text_allocation_id as usize);
    text
}

pub fn set_property_string(element: &ExternRef, property: &str, value: &str) {
    let set_property = js!(r#"
        function(element, property, value){
            element[property] = value;
        }"#);
    set_property.invoke(&[element.into(), property.into(), value.into()]);
}

pub struct CanvasContext(ExternRef);

impl CanvasContext {
    pub fn from_element(element: &ExternRef) -> Self {
        let get_context = js!(r#"
            function(element){
                return element.getContext("2d");
            }"#);
        let ctx_ref = get_context.invoke_and_return_object(&[element.into()]);
        CanvasContext(ctx_ref)
    }

    pub fn set_fill_style(&self, style: &str) {
        let set_fill_style = js!(r#"
            function(ctx, style){
                ctx.fillStyle = style;
            }"#);
        set_fill_style.invoke(&[(&self.0).into(), style.into()]);
    }

    pub fn fill_rect(&self, x: f64, y: f64, width: f64, height: f64) {
        let fill_rect = js!(r#"
            function(ctx, x, y, width, height){
                ctx.fillRect(x,y,width,height);
            }"#);
        fill_rect.invoke(&[
            (&self.0).into(),
            x.into(),
            y.into(),
            width.into(),
            height.into(),
        ]);
    }

    pub fn clear_rect(&self, x: f64, y: f64, width: f64, height: f64) {
        let clear_rect = js!(r#"
            function(ctx, x, y, width, height){
                ctx.clearRect(x,y,width,height);
            }"#);
        clear_rect.invoke(&[
            (&self.0).into(),
            x.into(),
            y.into(),
            width.into(),
            height.into(),
        ]);
    }

    pub fn set_font(&self, font: &str) {
        let set_font = js!(r#"
            function(ctx, font){
                ctx.font = font;
            }"#);
        set_font.invoke(&[(&self.0).into(), font.into()]);
    }

    pub fn set_text_align(&self, align: &str) {
        let set_text_align = js!(r#"
            function(ctx, align){
                ctx.textAlign = align;
            }"#);
        set_text_align.invoke(&[(&self.0).into(), align.into()]);
    }

    pub fn set_text_baseline(&self, baseline: &str) {
        let set_text_baseline = js!(r#"
            function(ctx, baseline){
                ctx.textBaseline = baseline;
            }"#);
        set_text_baseline.invoke(&[(&self.0).into(), baseline.into()]);
    }

    pub fn fill_text(&self, text: &str, x: f64, y: f64) {
        let fill_text = js!(r#"
            function(ctx, text, x, y){
                ctx.fillText(text,x,y);
            }"#);
        fill_text.invoke(&[(&self.0).into(), text.into(), x.into(), y.into()]);
    }

    pub fn measure_text(&self, text: &str) -> f64 {
        let measure_text = js!(r#"
            function(ctx, text){
                return ctx.measureText(text).width;
            }"#);
        measure_text.invoke(&[(&self.0).into(), text.into()])
    }

    pub fn set_line_width(&self, width: f64) {
        let set_line_width = js!(r#"
            function(ctx, width){
                ctx.lineWidth = width;
            }"#);
        set_line_width.invoke(&[(&self.0).into(), width.into()]);
    }

    pub fn set_stroke_style(&self, style: &str) {
        let set_stroke_style = js!(r#"
            function(ctx, style){
                ctx.strokeStyle = style;
            }"#);
        set_stroke_style.invoke(&[(&self.0).into(), style.into()]);
    }

    pub fn stroke_rect(&self, x: f64, y: f64, width: f64, height: f64) {
        let stroke_rect = js!(r#"
            function(ctx, x, y, width, height){
                ctx.strokeRect(x,y,width,height);
            }"#);
        stroke_rect.invoke(&[
            (&self.0).into(),
            x.into(),
            y.into(),
            width.into(),
            height.into(),
        ]);
    }

    pub fn begin_path(&self) {
        let begin_path = js!(r#"
            function(ctx){
                ctx.beginPath();
            }"#);
        begin_path.invoke(&[(&self.0).into()]);
    }

    pub fn move_to(&self, x: f64, y: f64) {
        let move_to = js!(r#"
            function(ctx, x, y){
                ctx.moveTo(x,y);
            }"#);
        move_to.invoke(&[(&self.0).into(), x.into(), y.into()]);
    }

    pub fn line_to(&self, x: f64, y: f64) {
        let line_to = js!(r#"
            function(ctx, x, y){
                ctx.lineTo(x,y);
            }"#);
        line_to.invoke(&[(&self.0).into(), x.into(), y.into()]);
    }

    pub fn stroke(&self) {
        let stroke = js!(r#"
            function(ctx){
                ctx.stroke();
            }"#);
        stroke.invoke(&[(&self.0).into()]);
    }

    pub fn close_path(&self) {
        let close_path = js!(r#"
            function(ctx){
                ctx.closePath();
            }"#);
        close_path.invoke(&[(&self.0).into()]);
    }

    pub fn fill(&self) {
        let fill = js!(r#"
            function(ctx){
                ctx.fill();
            }"#);
        fill.invoke(&[(&self.0).into()]);
    }

    pub fn arc(&self, x: f64, y: f64, radius: f64, start_angle: f64, end_angle: f64) {
        let arc = js!(r#"
            function(ctx, x, y, radius, start_angle, end_angle){
                ctx.arc(x,y,radius,start_angle,end_angle);
            }"#);
        arc.invoke(&[
            (&self.0).into(),
            x.into(),
            y.into(),
            radius.into(),
            start_angle.into(),
            end_angle.into(),
        ]);
    }

    pub fn arc_to(&self, x1: f64, y1: f64, x2: f64, y2: f64, radius: f64) {
        let arc_to = js!(r#"
            function(ctx, x1, y1, x2, y2, radius){
                ctx.arcTo(x1,y1,x2,y2,radius);
            }"#);
        arc_to.invoke(&[
            (&self.0).into(),
            x1.into(),
            y1.into(),
            x2.into(),
            y2.into(),
            radius.into(),
        ]);
    }

    pub fn bezier_curve_to(&self, cp1x: f64, cp1y: f64, cp2x: f64, cp2y: f64, x: f64, y: f64) {
        let bezier_curve_to = js!(r#"
            function(ctx, cp1x, cp1y, cp2x, cp2y, x, y){
                ctx.bezierCurveTo(cp1x,cp1y,cp2x,cp2y,x,y);
            }"#);
        bezier_curve_to.invoke(&[
            (&self.0).into(),
            cp1x.into(),
            cp1y.into(),
            cp2x.into(),
            cp2y.into(),
            x.into(),
            y.into(),
        ]);
    }

    pub fn quadratic_curve_to(&self, cpx: f64, cpy: f64, x: f64, y: f64) {
        let quadratic_curve_to = js!(r#"
            function(ctx, cpx, cpy, x, y){
                ctx.quadraticCurveTo(cpx,cpy,x,y);
            }"#);
        quadratic_curve_to.invoke(&[(&self.0).into(), cpx.into(), cpy.into(), x.into(), y.into()]);
    }

    pub fn rect(&self, x: f64, y: f64, width: f64, height: f64) {
        let rect = js!(r#"
            function(ctx, x, y, width, height){
                ctx.rect(x,y,width,height);
            }"#);
        rect.invoke(&[
            (&self.0).into(),
            x.into(),
            y.into(),
            width.into(),
            height.into(),
        ]);
    }

    pub fn clip(&self) {
        let clip = js!(r#"
            function(ctx){
                ctx.clip();
            }"#);
        clip.invoke(&[(&self.0).into()]);
    }

    pub fn draw_image(&self, image: &ExternRef, dx: f64, dy: f64) {
        let draw_image = js!(r#"
            function(ctx, image, dx, dy){
                ctx.drawImage(image,dx,dy);
            }"#);
        draw_image.invoke(&[(&self.0).into(), image.into(), dx.into(), dy.into()]);
    }

    pub fn draw_image_with_size(
        &self,
        image: &ExternRef,
        dx: f64,
        dy: f64,
        dwidth: f64,
        dheight: f64,
    ) {
        let draw_image_with_size = js!(r#"
            function(ctx, image, dx, dy, dwidth, dheight){
                ctx.drawImage(image,dx,dy,dwidth,dheight);
            }"#);
        draw_image_with_size.invoke(&[
            (&self.0).into(),
            image.into(),
            dx.into(),
            dy.into(),
            dwidth.into(),
            dheight.into(),
        ]);
    }

    pub fn draw_image_with_source(
        &self,
        image: &ExternRef,
        sx: f64,
        sy: f64,
        swidth: f64,
        sheight: f64,
        dx: f64,
        dy: f64,
        dwidth: f64,
        dheight: f64,
    ) {
        let draw_image_with_source = js!(r#"
            function(ctx, image, sx, sy, swidth, sheight, dx, dy, dwidth, dheight){
                ctx.drawImage(image,sx,sy,swidth,sheight,dx,dy,dwidth,dheight);
            }"#);
        draw_image_with_source.invoke(&[
            (&self.0).into(),
            image.into(),
            sx.into(),
            sy.into(),
            swidth.into(),
            sheight.into(),
            dx.into(),
            dy.into(),
            dwidth.into(),
            dheight.into(),
        ]);
    }
}

pub fn local_storage_set(key: &str, value: &str) {
    let local_storage_set = js!(r#"
        function(key, value){
            localStorage.setItem(key, value);
        }"#);
    local_storage_set.invoke(&[key.into(), value.into()]);
}

pub fn local_storage_remove(key: &str) {
    let local_storage_remove = js!(r#"
        function(key){
            localStorage.removeItem(key);
        }"#);
    local_storage_remove.invoke(&[key.into()]);
}

pub fn local_storage_get(key: &str) -> Option<String> {
    let local_storage_get = js!(r#"
        function(key){
            const text = localStorage.getItem(key);
            if(text === null){
                return 0;
            }
            const allocationId = this.writeUtf8ToMemory(text);
            return allocationId;
        }"#);
    let text_allocation_id = local_storage_get.invoke(&[key.into()]);
    if text_allocation_id == 0.0 {
        return None;
    }
    let text = extract_string_from_memory(text_allocation_id as usize);
    Some(text)
}

pub fn local_storage_clear() {
    let local_storage_clear = js!(r#"
        function(){
            localStorage.clear();
        }"#);
    local_storage_clear.invoke(&[]);
}
