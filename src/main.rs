use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color, DrawParam, Text};
use ggez::mint::Point2;
use ggez::{Context, ContextBuilder, GameResult};

fn main() {
    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("my_game", "HoutarouOreki")
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let my_game = MyGame::new(&mut ctx);
    ctx.gfx.set_window_title("HHHHHhhhhhhhhhhhhhhhhhhhh");

    // Run!
    event::run(ctx, event_loop, my_game);
}

struct MyGame {
    text_position: f32,
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        MyGame { text_position: 0.0 }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.text_position += ctx.time.delta().as_secs_f32() * 100.0;
        if self.text_position > (ctx.gfx.window().inner_size().width as f32) {
            self.text_position = 0.0;
        }
        println!("{}", self.text_position);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        let text = Text::new("H");
        let text_draw_param = DrawParam::new().dest(Point2 {
            x: self.text_position,
            y: 0f32,
        });
        canvas.draw(&text, text_draw_param);
        canvas.finish(ctx)
    }
}
