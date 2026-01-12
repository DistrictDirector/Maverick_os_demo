use prism::drawable::{Drawable, Component, SizedTree};
use prism::event::{OnEvent, Event, Key};
use prism::{Context, canvas::{Image, ShapeType}};
use prism::layout::{SizeRequest, Area};

use stork::{Canvas, GameObject, Action, Target, GameEvent, AnimatedSprite};

mod pipe;
mod score;

use pipe::PipeManager;
use score::ScoreManager;

//AVAILABLE IMAGES
    //bg.png
    //bullet.png
    //player.pngk
    //flappybird.png
    //toppipe.png
    //bottompipe.png
    //base.png
    //0.png
    //1.png
    //2.png
    //3.png
    //4.png
    //5.png
    //6.png
    //7.png
    //8.png
    //9.png

//AVAILABLE GIFs
    //flappbird.gif

#[derive(Debug)]
pub struct Game {
    canvas: Canvas,
    canvas_size: (f32, f32),
    pipe_manager: PipeManager,
    score_manager: ScoreManager,
    base_width: f32,
    game_over: bool,
}

///! No adding code into this impl, keep it how it is :)
impl OnEvent for Game {
    fn on_event(&mut self, ctx: &mut Context, _tree: &SizedTree, event: Box<dyn Event>) -> Vec<Box<dyn Event>> {
        self.update_game(ctx);
        
        self.canvas.on_event(ctx, _tree, event)
    }
}

impl Component for Game {
    fn children(&self) -> Vec<&dyn Drawable> {
        vec![&self.canvas]
    }
    
    fn children_mut(&mut self) -> Vec<&mut dyn Drawable> {
        vec![&mut self.canvas]
    }
    
    fn request_size(&self, children: Vec<SizeRequest>) -> SizeRequest {
        children.first().cloned().unwrap_or_else(|| SizeRequest::new(0.0, 0.0, f32::MAX, f32::MAX))
    }
    
    fn build(&self, size: (f32, f32), _children: Vec<SizeRequest>) -> Vec<Area> {
        self.canvas.update_size(size);
        
        vec![Area {
            offset: (0.0, 0.0),
            size: size
        }]
    }
}

impl Game {
    fn update_game(&mut self, ctx: &mut Context) {
        self.check_collisions(ctx);
        
        if self.game_over {
            return;
        }
        
        self.pipe_manager.update(ctx, &mut self.canvas);
        self.update_base_positions();
        self.score_manager.check_score(&self.canvas, self.pipe_manager.pipe_counter, self.pipe_manager.pipe_width);
        self.score_manager.update_display(ctx, &mut self.canvas);
        self.check_ceiling_collision();
    }
    
    fn check_collisions(&mut self, ctx: &mut Context) {
        let bird_target = Target::ByName("flappybird".to_string());
        let obstacle_target = Target::ByTag("obstacle".to_string());
        
        if self.canvas.collision_between(&bird_target, &obstacle_target) {
            self.game_over = true;
            self.handle_game_over(ctx);
        }
    }
    
    fn check_ceiling_collision(&mut self) {
        if let Some(bird) = self.canvas.get_game_object_mut("flappybird") {
            if bird.position.1 <= 0.0 {
                bird.position.1 = 0.0;
                bird.momentum.1 = 0.0;
            }
        }
    }
    
    fn handle_game_over(&mut self, ctx: &mut Context) {
        println!("> GAME OVER!! Score: {}", self.score_manager.score);
        
        if let Some(bird) = self.canvas.get_game_object_mut("flappybird") {
            bird.position = (200.0, 300.0);
            bird.momentum = (0.0, 0.0);
        }
        
        self.pipe_manager.reset(&mut self.canvas);
        self.score_manager.reset(ctx, &mut self.canvas);
        
        self.game_over = false;
    }
    
    fn update_base_positions(&mut self) {
        const BASE_SPEED: f32 = -3.0;
        let total_width = self.base_width * 4.0;
        
        if let Some(base1) = self.canvas.get_game_object_mut("base1") {
            base1.position.0 += BASE_SPEED;
            if base1.position.0 < -self.base_width {
                base1.position.0 += total_width;
            }
        }
        
        if let Some(base2) = self.canvas.get_game_object_mut("base2") {
            base2.position.0 += BASE_SPEED;
            if base2.position.0 < -self.base_width {
                base2.position.0 += total_width;
            }
        }
        
        if let Some(base3) = self.canvas.get_game_object_mut("base3") {
            base3.position.0 += BASE_SPEED;
            if base3.position.0 < -self.base_width {
                base3.position.0 += total_width;
            }
        }
        
        if let Some(base4) = self.canvas.get_game_object_mut("base4") {
            base4.position.0 += BASE_SPEED;
            if base4.position.0 < -self.base_width {
                base4.position.0 += total_width;
            }
        }
    }
}

pub struct MyApp;

impl MyApp {
    fn new(ctx: &mut Context) -> impl Drawable {
        let flappybird_width = 50.0;
        let flappybird_height = 35.0;
        let initial_size = (800.0, 600.0);
        let pipe_width = 100.0 * 0.5;
        let pipe_height = 800.0;
        let gap_size = 220.0;
        let base_height = 112.0;
        let base_width = 336.0;

        let bg_bytes = include_bytes!("../assets/bg.png");
        let bg_img = image::load_from_memory(bg_bytes)
            .expect("Failed to load background image");
        let bg_image = bg_img.to_rgba8();
        let background_image = Image {
            shape: ShapeType::Rectangle(0.0, initial_size, 0.0),
            image: bg_image.into(),
            color: None
        };

        let flappybird_gif_bytes = include_bytes!("../assets/flappybird.gif");
        let flappybird_animation = AnimatedSprite::new(
            flappybird_gif_bytes,
            (flappybird_width, flappybird_height),
            12.0  
        ).expect("Failed to load flappy bird animation");

        let flappybird_image = flappybird_animation.get_current_image();

        let base_bytes = include_bytes!("../assets/base.png");
        let base_img = image::load_from_memory(base_bytes)
            .expect("Failed to load base image");
        let base_image = base_img.to_rgba8();

        let mut stork_canvas = Canvas::new(ctx, initial_size);

        let background = GameObject::new(
            ctx,
            "background".to_string(),
            background_image,
            initial_size.0.max(initial_size.1), 
            (0.0, 0.0), 
            vec!["background".to_string()],
            (0.0, 0.0),
            (1.0, 1.0), 
            0.0,
        );

        stork_canvas.add_game_object("background".to_string(), background);
        
        let flappybird = GameObject::new(
            ctx,
            "flappybird".to_string(),
            flappybird_image,
            flappybird_width.max(flappybird_height),
            (200.0, 300.0),
            vec![
                "player".to_string(),
                "flyingbird".to_string(),
            ],
            (0.0, 0.0),
            (0.85, 0.85), 
            0.30,
        )
        .with_animation(flappybird_animation);

        stork_canvas.add_game_object("flappybird".to_string(), flappybird);

        let base_y = initial_size.1 - base_height;
        
        let base1_img_obj = Image {
            shape: ShapeType::Rectangle(0.0, (base_width, base_height), 0.0),
            image: base_image.clone().into(),
            color: None
        };
        
        let base2_img_obj = Image {
            shape: ShapeType::Rectangle(0.0, (base_width, base_height), 0.0),
            image: base_image.clone().into(),
            color: None
        };

        let base1 = GameObject::new(
            ctx,
            "base1".to_string(),
            base1_img_obj,
            base_width.max(base_height),
            (0.0, base_y),
            vec!["ground".to_string(), "obstacle".to_string()],
            (0.0, 0.0),
            (1.0, 1.0),
            0.0,
        );

        let base2 = GameObject::new(
            ctx,
            "base2".to_string(),
            base2_img_obj,
            base_width.max(base_height),
            (base_width, base_y),
            vec!["ground".to_string(), "obstacle".to_string()],
            (0.0, 0.0),
            (1.0, 1.0),
            0.0,
        );
        
        let base3_img_obj = Image {
            shape: ShapeType::Rectangle(0.0, (base_width, base_height), 0.0),
            image: base_image.clone().into(),
            color: None
        };
        
        let base3 = GameObject::new(
            ctx,
            "base3".to_string(),
            base3_img_obj,
            base_width.max(base_height),
            (base_width * 2.0, base_y),
            vec!["ground".to_string(), "obstacle".to_string()],
            (0.0, 0.0),
            (1.0, 1.0),
            0.0,
        );
        
        let base4_img_obj = Image {
            shape: ShapeType::Rectangle(0.0, (base_width, base_height), 0.0),
            image: base_image.clone().into(),
            color: None
        };
        
        let base4 = GameObject::new(
            ctx,
            "base4".to_string(),
            base4_img_obj,
            base_width.max(base_height),
            (base_width * 3.0, base_y),
            vec!["ground".to_string(), "obstacle".to_string()],
            (0.0, 0.0),
            (1.0, 1.0),
            0.0,
        );

        stork_canvas.add_game_object("base1".to_string(), base1);
        stork_canvas.add_game_object("base2".to_string(), base2);
        stork_canvas.add_game_object("base3".to_string(), base3);
        stork_canvas.add_game_object("base4".to_string(), base4);

        stork_canvas.add_event(
            GameEvent::KeyPress {
                key: Key::Character("w".to_string().into()),
                action: Action::ApplyMomentum {  
                    target: Target::ById("flappybird".to_string()),
                    value: (0.0, -10.5)  
                },
                target: Target::ById("flappybird".to_string())
            },
            Target::ById("flappybird".to_string())
        );

        let pipe_manager = PipeManager::new(
            pipe_width,
            pipe_height,
            gap_size,
            initial_size,
        );

        let score_manager = ScoreManager::new(initial_size);

        let mut game = Game { 
            canvas: stork_canvas,
            canvas_size: initial_size,
            pipe_manager,
            score_manager,
            base_width,
            game_over: false,
        };

        game.canvas.add_event(
            GameEvent::Collision {
                action: Action::ApplyMomentum {
                    target: Target::ById("flappybird".to_string()),
                    value: (0.0, 0.0)
                },
                target: Target::ByTag("obstacle".to_string())
            },
            Target::ById("flappybird".to_string())
        );

        game
    }
}

ramp::run!{|ctx: &mut Context| {
    MyApp::new(ctx)
}}