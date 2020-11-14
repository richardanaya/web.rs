use hecs::*;
use web::*;

struct Game {
    time: u32,
    ctx: CanvasContext,
    canvas_width: u32,
    canvas_height: u32,
    width: u32,
    height: u32,
    direction: Direction,
    world: World,
    head: Entity,
}

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

// ECS components
struct SnakeHead;
struct SnakeBody;
struct Food;
struct Position(u32, u32);
struct Color(String);

const MAP_WIDTH: u32 = 10;
const MAP_HEIGHT: u32 = 10;

fn game() -> MutexGuard<'static, Game> {
    lazy_static::lazy_static! {
        static ref SINGLETON: Mutex<Game> = {
            // create graphics context
            let screen = get_element_by_id("screen");
            let width: f64 = get_property(&screen, "width");
            let height: f64 = get_property(&screen, "height");
            let ctx = CanvasContext::from_canvas_element(&screen);

            // create snake
            let mut world = World::new();
            let head = world.spawn(
                (SnakeHead,Color(GREEN.to_string()),Position(MAP_WIDTH/2,MAP_HEIGHT/2))
            );

            // create initial food
            world.spawn(
                (Food,Color(RED.to_string()),Position((random()*MAP_WIDTH as f64) as u32,(random()*MAP_HEIGHT as f64) as u32))
            );

            Mutex::new(Game {
                time: 0,
                ctx,
                canvas_width: width as u32,
                canvas_height:height as u32,
                width: MAP_WIDTH,
                height: MAP_HEIGHT,
                direction: Direction::Down,
                head,
                world,
            })
        };
    }
    SINGLETON.lock()
}

fn move_snake_system(game: &mut Game) {
    let last_pos = {
        let mut pos = game.world.get_mut::<Position>(game.head).unwrap();
        let p = Position(pos.0, pos.1);
        match game.direction {
            Direction::Up => pos.1 -= 1,
            Direction::Right => pos.0 += 1,
            Direction::Down => pos.1 += 1,
            Direction::Left => pos.0 -= 1,
        }
        p
    };
    game.world
        .spawn((SnakeBody, Color("light green".to_string()), last_pos));
}

fn render_system(game: &Game) {
    for (_id, (pos, color)) in &mut game.world.query::<(&Position, &Color)>() {
        game.ctx.set_fill_color(&color.0);
        game.ctx.fill_rect(
            pos.0 * (game.canvas_width / MAP_WIDTH),
            pos.1 * (game.canvas_height / MAP_HEIGHT),
            game.canvas_width / MAP_WIDTH,
            game.canvas_height / MAP_HEIGHT,
        );
    }
}

fn eat_system(game: &mut Game) {
    let (head_x, head_y) = {
        let p = game.world.get::<Position>(game.head).unwrap();
        (p.0, p.1)
    };
    let mut food_to_remove = None;
    for (id, (_, pos)) in &mut game.world.query::<(&Food, &Position)>() {
        if pos.0 == head_x && pos.1 == head_y {
            food_to_remove = Some(id);
            break;
        }
    }
    if let Some(id) = food_to_remove {
        game.world.despawn(id).unwrap();
    }
}

#[no_mangle]
pub fn main() {
    add_event_listener(DOM_BODY, "keydown", |event| {
        let key_down_event = KeyDownEvent::from_event(event);
        let key_code = key_down_event.key_code();
        let mut game = game();
        match key_code {
            87 => game.direction = Direction::Up,
            68 => game.direction = Direction::Right,
            83 => game.direction = Direction::Down,
            65 => game.direction = Direction::Left,
            _ => (),
        };
        game.time += 1000;
    });

    game().ctx.set_fill_color("red");

    request_animation_loop(|delta| {
        let mut game = game();
        game.time += delta as u32;
        if game.time > 1000 {
            game.time %= 1000;
            move_snake_system(&mut game);
            eat_system(&mut game);
        }
        render_system(&game)
    });
}
