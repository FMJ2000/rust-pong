use std::path::Path;
use tetra::graphics::{self, Color, Rectangle, text::{Text, Font}, Texture};
use tetra::input::{self, Key, KeyModifier};
use tetra::math::Vec2;
use tetra::window;
use tetra::{Context, ContextBuilder, State};

const WINDOW_WIDTH: f32 = 640.0;
const WINDOW_HEIGHT: f32 = 480.0;
const PADDLE_SPEED: f32 = 0.8;
const PADDLE_MAX: f32 = 20.0;
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
    pos1: Vec2<f32>,
    pos2: Vec2<f32>,
    posball: Vec2<f32>,
    vball: Vec2<f32>,
    player1: Entity,
    player2: Entity,
    ball: Entity,
    score: Score,
    state: u8,
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let texture1 = Texture::new(ctx, "./resources/player1.png")?;
        let texture2 = Texture::new(ctx, "./resources/player2.png")?;
        let textureball = Texture::new(ctx, "./resources/ball.png")?;
        
        let pos1 = Vec2::new(16.0, (WINDOW_HEIGHT - texture1.height() as f32) / 2.0); 
        let pos2 = Vec2::new(WINDOW_WIDTH - texture2.width() as f32 - 16.0, (WINDOW_HEIGHT - texture2.height() as f32) / 2.0);
        let posball: Vec2<f32> = Vec2::new(WINDOW_WIDTH / 2.0 - textureball.width() as f32 / 2.0, WINDOW_HEIGHT / 2.0 - textureball.height() as f32 / 2.0);
        let vball: Vec2<f32>= Vec2::new(-BALL_SPEED, 0.0);
        Ok(GameState {
            pos1, pos2, posball, vball,
            player1: Entity::new(texture1, pos1), 
            player2: Entity::new(texture2, pos2),
            ball: Entity::with_velocity(textureball, posball, vball),
            score: Score::new(ctx)?,
            state: 1,
        })
    }

    fn restart(&mut self) {
        self.player1.position = self.pos1;
        self.player2.position = self.pos2;
        self.ball.position = self.posball;
        self.ball.velocity = self.vball;
        self.score.value = 0;
        self.state = 1;
    }
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));
        self.player1.texture.draw(ctx, self.player1.position);
        self.player2.texture.draw(ctx, self.player2.position);
        self.ball.texture.draw(ctx, self.ball.position);
        self.score.text.draw(ctx, self.score.position);
        Ok(())
    }
    
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        if input::is_key_modifier_down(ctx, KeyModifier::Ctrl) {
            if input::is_key_down(ctx, Key::W) {
                window::quit(ctx);
            }
            if input::is_key_down(ctx, Key::R) {
                self.restart();
            }
        }
        if self.state == 1 {
            if input::is_key_down(ctx, Key::W) {
                self.player1.velocity.y -= if self.player1.velocity.y - PADDLE_SPEED > PADDLE_MAX {
                    0.0
                } else {
                    PADDLE_SPEED
                }
            }
            if input::is_key_down(ctx, Key::S) {
                self.player1.velocity.y += if self.player1.velocity.y + PADDLE_SPEED > PADDLE_MAX {
                    0.0
                } else {
                    PADDLE_SPEED
                }
            }
            if input::is_key_down(ctx, Key::Up) {
                self.player2.velocity.y -= if self.player2.velocity.y - PADDLE_SPEED > PADDLE_MAX {
                    0.0
                } else {
                    PADDLE_SPEED
                }
            }
            if input::is_key_down(ctx, Key::Down) {
                self.player2.velocity.y += if self.player2.velocity.y + PADDLE_SPEED > PADDLE_MAX {
                    0.0
                } else {
                    PADDLE_SPEED
                }
            }
            self.player1.position += self.player1.velocity;
            self.player2.position += self.player2.velocity;
            self.player1.velocity *= 0.9;
            self.player2.velocity *= 0.9;
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
                self.ball.velocity.x *= -(1.0 + BALL_ACC);
                let offset = (paddle.centre().y - self.ball.centre().y) / paddle.texture.height() as f32;
                self.ball.velocity.y += PADDLE_SPIN * -offset;
                self.score.value += 1;
                self.score.text.set_content(format!("Score: {}", self.score.value));
            }
            if self.ball.position.y <= 0.0 || self.ball.position.y + self.ball.texture.height() as f32 >= WINDOW_HEIGHT {
                self.ball.velocity.y = -self.ball.velocity.y;
            }
            if self.ball.position.x < 0.0 {
                //window::quit(ctx);
                self.state = 0;
                self.score.text.set_content(format!("Player 2 wins with {} points!", self.score.value));
                println!("Player 2 wins!");
            }
            if self.ball.position.x > WINDOW_WIDTH {
                //window::quit(ctx);
                self.state = 0;
                self.score.text.set_content(format!("Player 1 wins with {} points!", self.score.value));
                println!("Player 1 wins!");
            }
            if self.player1.position.y <= 0.0 || self.player1.position.y + self.player1.texture.height() as f32 >= WINDOW_HEIGHT {
                self.player1.velocity *= -1.5;
            }
            if self.player2.position.y <= 0.0 || self.player2.position.y + self.player2.texture.height() as f32 >= WINDOW_HEIGHT {
                self.player2.velocity *= -1.5;
            }
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

struct Score {
    value: u32,
    text: Text,
    position: Vec2<f32>,
}

impl Score {
    fn new(ctx: &mut Context) -> tetra::Result<Score> {
        let text = Text::new(String::from("Score: 0"), Font::vector(ctx, Path::new("./resources/InputMono-Regular.ttf"), 14.0)?);
        let position = Vec2::new(64.0, 16.0);
        Ok(Score { value: 0, text, position })
    }
}
