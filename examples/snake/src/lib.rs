use hecs::*;
use web::*;

struct Game {
    time: i32,
    ctx: CanvasContext,
    canvas_width: i32,
    canvas_height: i32,
    width: i32,
    height: i32,
    direction: Direction,
    world: World,
    head: Entity,
    made_move: bool,
}

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

// ECS components
struct SnakeHead(i32);
struct SnakeBody(i32);
struct Food;
struct Position(i32, i32);
struct Color(String);

const MAP_WIDTH: i32 = 30;
const MAP_HEIGHT: i32 = 30;

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
                (SnakeHead(1),Color(GREEN.to_string()),Position(MAP_WIDTH/2,MAP_HEIGHT/2))
            );
            let mut g = Game {
                time: 0,
                ctx,
                canvas_width: width as i32,
                canvas_height:height as i32,
                width: MAP_WIDTH,
                height: MAP_HEIGHT,
                direction: Direction::Down,
                head,
                world,
                made_move: false,
            };
            g.reset();
            Mutex::new(g)
        };
    }
    SINGLETON.lock()
}

impl Game {
    fn reset(&mut self) {
        self.ctx
            .clear_rect(0, 0, self.canvas_width, self.canvas_height);
        self.world.clear();
        self.head = self.world.spawn((
            SnakeHead(1),
            Color(GREEN.to_string()),
            Position(MAP_WIDTH / 2, MAP_HEIGHT / 2),
        ));
        self.spawn_food();
    }

    fn spawn_food(&mut self) {
        // create initial food
        self.world.spawn((
            Food,
            Color(RED.to_string()),
            Position(
                (random() * MAP_WIDTH as f64) as i32,
                (random() * MAP_HEIGHT as f64) as i32,
            ),
        ));
    }
}

fn move_snake_system(game: &mut Game) {
    let (last_head_pos, next_head_pos) = {
        let mut pos = game.world.get_mut::<Position>(game.head).unwrap();
        let p = Position(pos.0, pos.1);
        match game.direction {
            Direction::Up => pos.1 -= 1,
            Direction::Right => pos.0 += 1,
            Direction::Down => pos.1 += 1,
            Direction::Left => pos.0 -= 1,
        }
        (p, Position(pos.0, pos.1))
    };
    let mut body_to_remove = vec![];
    let mut bit_tail = false;
    if next_head_pos.0 < 0
        || next_head_pos.1 < 0
        || next_head_pos.0 > game.width
        || next_head_pos.1 > game.height
    {
        game.reset();
        return;
    }
    for (id, (body, pos)) in &mut game.world.query::<(&mut SnakeBody, &Position)>() {
        body.0 -= 1;
        if body.0 == 0 {
            body_to_remove.push(id);
        } else {
            if pos.0 == next_head_pos.0 && pos.1 == next_head_pos.1 {
                bit_tail = true;
                break;
            }
        }
    }
    if bit_tail {
        game.reset();
        return;
    }
    for b in body_to_remove.into_iter() {
        game.world.despawn(b).unwrap();
    }
    let snake_level = game.world.get::<SnakeHead>(game.head).unwrap().0;
    game.world.spawn((
        SnakeBody(snake_level),
        Color(FORESTGREEN.to_string()),
        last_head_pos,
    ));
}

fn render_system(game: &Game) {
    game.ctx
        .clear_rect(0, 0, game.canvas_width, game.canvas_height);
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
        {
            game.world.despawn(id).unwrap();
        }
        {
            let mut head = game.world.get_mut::<SnakeHead>(game.head).unwrap();
            head.0 += 1;
        }
        game.spawn_food();
    }
}

const ITERATION_TIME: i32 = 100;

#[no_mangle]
pub fn main() {
    add_event_listener(DOM_BODY, "keydown", |event| {
        let mut game = game();
        if game.made_move {
            return;
        }
        game.made_move = true;
        let key_down_event = KeyDownEvent::from_event(event);
        let key_code = key_down_event.key_code();
        match key_code {
            87 | 38 => {
                if let Direction::Down = game.direction {
                } else {
                    game.direction = Direction::Up
                }
            }
            68 | 39 => {
                if let Direction::Left = game.direction {
                } else {
                    game.direction = Direction::Right
                }
            }
            83 | 40 => {
                if let Direction::Up = game.direction {
                } else {
                    game.direction = Direction::Down
                }
            }
            65 | 37 => {
                if let Direction::Right = game.direction {
                } else {
                    game.direction = Direction::Left
                }
            }
            _ => (),
        };
    });

    game().ctx.set_fill_color("red");

    request_animation_loop(|delta| {
        let mut game = game();
        game.time += delta as i32;
        if game.time > ITERATION_TIME {
            game.time %= ITERATION_TIME;
            move_snake_system(&mut game);
            eat_system(&mut game);
        }
        render_system(&game);
        game.made_move = false;
    });
}
