use hecs::*;
use web::*;

struct Game {
    time: u32,
    ctx: CanvasContext,
    width: u32,
    height: u32,
    direction: Direction,
    world: World,
}

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

struct SnakeHead;
struct SnakeBody;
struct Food;
struct Position(f64, f64);
struct Color(String);
struct Size(f64);

fn game() -> MutexGuard<'static, Game> {
    lazy_static::lazy_static! {
        static ref SINGLETON: Mutex<Game> = {
            let screen = get_element_by_id("screen");
            let width: f64 = get_property(&screen, "width");
            let height: f64 = get_property(&screen, "height");
            let ctx = CanvasContext::from_canvas_element(&screen);
            let mut world = World::new();
            world.spawn(
                (SnakeHead,Color("green".to_string()),Position(0.0,0.0),Size(10.0))
            );
            world.spawn(
                (Food,Color("red".to_string()),Position(50.0,50.0),Size(10.0))
            );
            Mutex::new(Game {
                time: 0,
                ctx,
                width: width as u32,
                height: height as u32,
                direction: Direction::Down,
                world,
            })
        };
    }
    SINGLETON.lock()
}

fn move_snake(game: &Game) {
    for (_id, (_, pos)) in &mut game.world.query::<(&SnakeHead, &mut Position)>() {
        pos.0 += 10.0;
    }
}

fn render_system(game: &Game) {
    for (_id, (pos, color, size)) in &mut game.world.query::<(&Position, &Color, &Size)>() {
        game.ctx.set_fill_color(&color.0);
        game.ctx.fill_rect(pos.0, pos.1, size.0, size.0);
    }
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

    request_animation_loop(|delta| {
        let mut game = game();
        game.time += delta as u32;
        if game.time > 1000 {
            game.time %= 1000;
            move_snake(&game)
        }
        render_system(&game)
    });
}
