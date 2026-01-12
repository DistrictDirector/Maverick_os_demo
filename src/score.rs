use prism::Context;
use prism::canvas::{Image, ShapeType};
use stork::{Canvas, GameObject, Target, Location};
use std::collections::HashSet;

#[derive(Debug)]
pub struct ScoreManager {
    pub score: usize,
    canvas_size: (f32, f32),
    scored_pipes: HashSet<u32>,
    bird_was_left_of_pipe: HashSet<u32>,
    number_images: Vec<image::ImageBuffer<image::Rgba<u8>, Vec<u8>>>,
}

impl ScoreManager {
    pub fn new(canvas_size: (f32, f32)) -> Self {
        let mut number_images = Vec::new();
        
        let num0_bytes = include_bytes!("../assets/0.png");
        let num0_img = image::load_from_memory(num0_bytes).expect("Failed to load number 0 image");
        number_images.push(num0_img.to_rgba8());
        
        let num1_bytes = include_bytes!("../assets/1.png");
        let num1_img = image::load_from_memory(num1_bytes).expect("Failed to load number 1 image");
        number_images.push(num1_img.to_rgba8());
        
        let num2_bytes = include_bytes!("../assets/2.png");
        let num2_img = image::load_from_memory(num2_bytes).expect("Failed to load number 2 image");
        number_images.push(num2_img.to_rgba8());
        
        let num3_bytes = include_bytes!("../assets/3.png");
        let num3_img = image::load_from_memory(num3_bytes).expect("Failed to load number 3 image");
        number_images.push(num3_img.to_rgba8());
        
        let num4_bytes = include_bytes!("../assets/4.png");
        let num4_img = image::load_from_memory(num4_bytes).expect("Failed to load number 4 image");
        number_images.push(num4_img.to_rgba8());
        
        let num5_bytes = include_bytes!("../assets/5.png");
        let num5_img = image::load_from_memory(num5_bytes).expect("Failed to load number 5 image");
        number_images.push(num5_img.to_rgba8());
        
        let num6_bytes = include_bytes!("../assets/6.png");
        let num6_img = image::load_from_memory(num6_bytes).expect("Failed to load number 6 image");
        number_images.push(num6_img.to_rgba8());
        
        let num7_bytes = include_bytes!("../assets/7.png");
        let num7_img = image::load_from_memory(num7_bytes).expect("Failed to load number 7 image");
        number_images.push(num7_img.to_rgba8());
        
        let num8_bytes = include_bytes!("../assets/8.png");
        let num8_img = image::load_from_memory(num8_bytes).expect("Failed to load number 8 image");
        number_images.push(num8_img.to_rgba8());
        
        let num9_bytes = include_bytes!("../assets/9.png");
        let num9_img = image::load_from_memory(num9_bytes).expect("Failed to load number 9 image");
        number_images.push(num9_img.to_rgba8());

        Self {
            score: 0,
            scored_pipes: HashSet::new(),
            bird_was_left_of_pipe: HashSet::new(),
            number_images,
            canvas_size,
        }
    }

    pub fn check_score(&mut self, canvas: &Canvas, pipe_counter: u32, pipe_width: f32) {
        if let Some(bird) = canvas.get_game_object("flappybird") {
            let bird_center_x = bird.position.0 + 25.0; 
            
            for i in 0..pipe_counter {
                if self.scored_pipes.contains(&i) {
                    continue;
                }
                
                let pipe_name = format!("toppipe_{}", i);
                if let Some(pipe) = canvas.get_game_object(&pipe_name) {
                    let pipe_center_x = pipe.position.0 + pipe_width / 2.0;
                    
                    if bird_center_x < pipe_center_x {
                        self.bird_was_left_of_pipe.insert(i);
                    }

                    else if self.bird_was_left_of_pipe.contains(&i) && bird_center_x > pipe_center_x {
                        self.score += 1;
                        self.scored_pipes.insert(i);
                        self.bird_was_left_of_pipe.remove(&i);
                        println!("Score: {}", self.score);
                    }
                }
            }
        }
    }

    pub fn update_display(&mut self, ctx: &mut Context, canvas: &mut Canvas) {
        let score_str = self.score.to_string();
        let digit_width = 24.0;
        let digit_height = 38.0;
        let spacing = 5.0;
        let total_width = (score_str.len() as f32) * (digit_width + spacing) - spacing;
        let start_x = self.canvas_size.0 - total_width - 20.0;
        let start_y = 20.0;
        
        for idx in 0..10 {
            let name = format!("score_digit_{}", idx);
            canvas.remove_game_object(&name);
        }
        
        for (idx, digit_char) in score_str.chars().enumerate() {
            let digit = digit_char.to_digit(10).unwrap() as usize;
            let digit_image = self.number_images[digit].clone();
            
            let img_obj = Image {
                shape: ShapeType::Rectangle(0.0, (digit_width, digit_height), 0.0),
                image: digit_image.into(),
                color: None
            };
            
            let x_pos = start_x + (idx as f32) * (digit_width + spacing);
            
            let digit_obj = GameObject::new_rect(
                ctx,
                format!("score_digit_{}", idx),
                img_obj,
                (digit_width, digit_height),
                (x_pos, start_y),
                vec!["score".to_string()],
                (0.0, 0.0),
                (1.0, 1.0),
                0.0,
            );
            
            canvas.add_game_object(format!("score_digit_{}", idx), digit_obj);
        }
    }

    pub fn reset(&mut self, ctx: &mut Context, canvas: &mut Canvas) {
        self.score = 0;
        self.scored_pipes.clear();
        self.bird_was_left_of_pipe.clear();
        
        for idx in 0..10 {
            canvas.remove_game_object(&format!("score_digit_{}", idx));
        }
        
        self.update_display(ctx, canvas);
    }
}