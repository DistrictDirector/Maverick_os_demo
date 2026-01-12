use prism::Context;
use prism::canvas::{Image, ShapeType};
use stork::{Canvas, GameObject};
use rand::Rng;

#[derive(Debug)]
pub struct PipeManager {
    pub pipe_spawn_timer: f32,
    pub pipe_counter: u32,
    pub toppipe_image: image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
    pub bottompipe_image: image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
    pub pipe_width: f32,
    pub pipe_height: f32,
    pub gap_size: f32,
    canvas_size: (f32, f32),
}

impl PipeManager {
    pub fn new(
        pipe_width: f32,
        pipe_height: f32,
        gap_size: f32,
        canvas_size: (f32, f32),
    ) -> Self {
        let toppipe_bytes = include_bytes!("../assets/toppipe.png");
        let toppipe_img = image::load_from_memory(toppipe_bytes)
            .expect("Failed to load toppipe image");
        let toppipe_image = toppipe_img.to_rgba8();
        
        let bottompipe_bytes = include_bytes!("../assets/bottompipe.png");
        let bottompipe_img = image::load_from_memory(bottompipe_bytes)
            .expect("Failed to load bottompipe image");
        let bottompipe_image = bottompipe_img.to_rgba8();

        Self {
            pipe_spawn_timer: 0.0,
            pipe_counter: 0,
            toppipe_image,
            bottompipe_image,
            pipe_width,
            pipe_height,
            gap_size,
            canvas_size,
        }
    }

    pub fn update(&mut self, ctx: &mut Context, canvas: &mut Canvas) {
        const PIPE_SPAWN_INTERVAL: f32 = 2.0;
        
        self.pipe_spawn_timer += 0.016;
        
        if self.pipe_spawn_timer >= PIPE_SPAWN_INTERVAL {
            self.spawn_pipe_pair(ctx, canvas);
            self.pipe_spawn_timer = 0.0;
        }
        
        self.remove_offscreen_pipes(canvas);
    }

    pub fn spawn_pipe_pair(&mut self, ctx: &mut Context, canvas: &mut Canvas) {
        let mut rng = rand::thread_rng();
        
        let base_height = 112.0;
        let min_gap_y = 150.0;
        let max_gap_y = self.canvas_size.1 - base_height - self.gap_size / 2.0 - 10.0;
        let gap_y = rng.gen_range(min_gap_y..max_gap_y);
        
        let toppipe_img_obj = Image {
            shape: ShapeType::Rectangle(0.0, (self.pipe_width, self.pipe_height), 0.0),
            image: self.toppipe_image.clone().into(),
            color: None
        };
        
        let bottompipe_img_obj = Image {
            shape: ShapeType::Rectangle(0.0, (self.pipe_width, self.pipe_height), 0.0),
            image: self.bottompipe_image.clone().into(),
            color: None
        };
        
        let toppipe = GameObject::new_rect(
            ctx,
            format!("toppipe_{}", self.pipe_counter),
            toppipe_img_obj,
            (self.pipe_width, self.pipe_height),
            (self.canvas_size.0 + 100.0, gap_y - self.gap_size / 2.0 - self.pipe_height),
            vec!["pipe".to_string(), "obstacle".to_string()],
            (-3.0, 0.0),
            (1.0, 1.0),
            0.0,
        );
        
        let bottompipe = GameObject::new_rect(
            ctx,
            format!("bottompipe_{}", self.pipe_counter),
            bottompipe_img_obj,
            (self.pipe_width, self.pipe_height),
            (self.canvas_size.0 + 100.0, gap_y + self.gap_size / 2.0),
            vec!["pipe".to_string(), "obstacle".to_string()],
            (-3.0, 0.0),
            (1.0, 1.0),
            0.0,
        );
        
        canvas.add_game_object(format!("toppipe_{}", self.pipe_counter), toppipe);
        canvas.add_game_object(format!("bottompipe_{}", self.pipe_counter), bottompipe);
        
        self.pipe_counter += 1;
    }

    pub fn remove_offscreen_pipes(&mut self, canvas: &mut Canvas) {
        let mut pipes_to_remove: Vec<String> = Vec::new();
        
        for i in 0..self.pipe_counter {
            let toppipe_name = format!("toppipe_{}", i);
            let bottompipe_name = format!("bottompipe_{}", i);
            
            if let Some(obj) = canvas.get_game_object(&toppipe_name) {
                if obj.position.0 < -self.pipe_width - 50.0 {
                    pipes_to_remove.push(toppipe_name);
                }
            }
            
            if let Some(obj) = canvas.get_game_object(&bottompipe_name) {
                if obj.position.0 < -self.pipe_width - 50.0 {
                    pipes_to_remove.push(bottompipe_name);
                }
            }
        }
        
        for name in pipes_to_remove {
            canvas.remove_game_object(&name);
        }
    }

    pub fn reset(&mut self, canvas: &mut Canvas) {
        for i in 0..self.pipe_counter {
            canvas.remove_game_object(&format!("toppipe_{}", i));
            canvas.remove_game_object(&format!("bottompipe_{}", i));
        }
        
        self.pipe_counter = 0;
        self.pipe_spawn_timer = 0.0;
    }
}