use web::*;

struct Game {
    ctx: CanvasContext,
    width: u32,
    height: u32,
    direction: Direction,
}

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

fn game() -> MutexGuard<'static, Game> {
    lazy_static::lazy_static! {
        static ref SINGLETON: Mutex<Game> = {
            let screen = get_element_by_id("screen");
            let width: f64 = get_property(&screen, "width");
            let height: f64 = get_property(&screen, "height");
            let ctx = CanvasContext::from_canvas_element(&screen);
            Mutex::new(Game {
                ctx,
                width: width as u32,
                height: height as u32,
                direction: Direction::Down,
            })
        };
    }
    SINGLETON.lock()
}

#[no_mangle]
pub fn main() {
    add_event_listener(DOM_BODY, "keydown", |event| {
        let key_down_event = KeyDownEvent::from_event(event);
        let key_code = key_down_event.key_code();
        let mut game = game();
        log(&format!("{}", key_code));
        match key_code {
            87 => game.direction = Direction::Up,
            68 => game.direction = Direction::Right,
            83 => game.direction = Direction::Down,
            65 => game.direction = Direction::Left,
            _ => (),
        };
    });

    game().ctx.set_fill_color("red");

    request_animation_loop(|_delta| {
        let game = game();
        for x in 0..game.width / 10 {
            for y in 0..game.height / 10 {
                game.ctx.fill_rect(x * 10, y * 10, 10, 10);
            }
        }
    });
}
