use tetra::graphics::{self, Color, Rectangle, Texture};
use tetra::input::{self, Key};
use tetra::math::Vec2;
use tetra::window;
use tetra::{Context, ContextBuilder, State};

const WINDOW_WIDTH: f32 = 640.0;
const WINDOW_HEIGHT: f32 = 480.0;
const PADDLE_SPEED: f32 = 8.0;
const BALL_SPEED: f32 = 5.0;
const PADDLE_SPIN: f32 = 4.0;
const BALL_ACC: f32 = 0.05;

fn main() -> tetra::Result {
    ContextBuilder::new("Pong", WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
        .quit_on_escape(true)
        .build()?
        .run(GameState::new)
}

struct GameState {
    player1: Entity,
    player2: Entity,
    ball: Entity,
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let texture1 = Texture::new(ctx, "./resources/player1.png")?;
        let texture2 = Texture::new(ctx, "./resources/player2.png")?;
        let textureball = Texture::new(ctx, "./resources/ball.png")?;
        let pos1 = Vec2::new(16.0, (WINDOW_HEIGHT - texture1.height() as f32) / 2.0); 
        let pos2 = Vec2::new(WINDOW_WIDTH - texture2.width() as f32 - 16.0, (WINDOW_HEIGHT - texture2.height() as f32) / 2.0);
        let posball = Vec2::new(WINDOW_WIDTH / 2.0 - textureball.width() as f32 / 2.0, WINDOW_HEIGHT / 2.0 - textureball.height() as f32 / 2.0);
        let vball = Vec2::new(-BALL_SPEED, 0.0);
       
        Ok(GameState {
            player1: Entity::new(texture1, pos1), 
            player2: Entity::new(texture2, pos2),
            ball: Entity::with_velocity(textureball, posball, vball),
        }) 
    }
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));
        self.player1.texture.draw(ctx, self.player1.position);
        self.player2.texture.draw(ctx, self.player2.position);
        self.ball.texture.draw(ctx, self.ball.position);
        Ok(())
    }
    
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        if input::is_key_down(ctx, Key::W) {
            self.player1.position.y -= PADDLE_SPEED;
        }
        if input::is_key_down(ctx, Key::S) {
            self.player1.position.y += PADDLE_SPEED;
        }
        if input::is_key_down(ctx, Key::Up) {
            self.player2.position.y -= PADDLE_SPEED;
        }
        if input::is_key_down(ctx, Key::Down) {
            self.player2.position.y += PADDLE_SPEED;
        }
        self.ball.position += self.ball.velocity;
        let p1bounds = self.player1.bounds();
        let p2bounds = self.player2.bounds();
        let ballbounds = self.ball.bounds();

        let hit = if ballbounds.intersects(&p1bounds) {
            Some(&self.player1)
        } else if ballbounds.intersects(&p2bounds) {
            Some(&self.player2)
        } else {
            None
        };
        if let Some(paddle) = hit {
            self.ball.velocity.x *= -1.0 + BALL_ACC;
            let offset = (paddle.centre().y - self.ball.centre().y) / paddle.texture.height() as f32;
            self.ball.velocity.y += PADDLE_SPIN * -offset;
        }
        Ok(())
    }
}

struct Entity {
    texture: Texture,
    position: Vec2<f32>,
    velocity: Vec2<f32>,
}

impl Entity {
    fn new(texture: Texture, position: Vec2<f32>) -> Entity {
        Entity { texture, position, velocity: Vec2::zero()}
    }

    fn with_velocity(texture: Texture, position: Vec2<f32>, velocity: Vec2<f32>) -> Entity {
        Entity { texture, position, velocity }
    }
    
    fn bounds(&self) -> Rectangle {
        Rectangle::new(
            self.position.x,
            self.position.y,
            self.texture.width() as f32,
            self.texture.height() as f32,
        )
    }

    fn centre(&self) -> Vec2<f32> {
        Vec2::new(
            self.position.x + (self.texture.width() as f32 / 2.0),
            self.position.y + (self.texture.height() as f32 / 2.0),
        )
    }
}
