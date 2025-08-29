use pelican_ui::{Component, Context, Plugins, Plugin, maverick_start, start, Application, PelicanEngine, MaverickOS};
use pelican_ui::drawable::{Drawable, Component, Align};
use pelican_ui::runtime::{Services, ServiceList};
use pelican_ui::layout::{Layout, SizeRequest, Area};
use pelican_ui::events::{OnEvent, Event, TickEvent};
use std::collections::BTreeMap;
use pelican_ui_std::Button;
use std::sync::mpsc::channel;
use pelican_ui::hardware::{ImageOrientation, Camera};
use image::RgbaImage;

use pelican_ui_std::{
    Interface, Stack, Page, Text, TextStyle,
    Offset, Content, Icon, ExpandableText, 
    Header, AppPage, ExpandableImage,
    Size, Padding,
};

#[derive(Debug, Clone)]
pub enum CameraControlEvent {
    StartProcessed,
    StartUnprocessed,
    Stop,
}

impl Event for CameraControlEvent {
    fn pass(self: Box<Self>, _ctx: &mut Context, children: Vec<((f32, f32), (f32, f32))>) -> Vec<Option<Box<dyn Event>>> {
        children.into_iter().map(|_| Some(self.clone() as Box<dyn Event>)).collect()
    }
}

pub struct MyApp;

impl Services for MyApp {
    fn services() -> ServiceList {
        ServiceList(BTreeMap::new())
    }
}

impl Plugins for MyApp {
    fn plugins(_ctx: &mut Context) -> Vec<Box<dyn Plugin>> { 
        vec![] 
    }
}

impl Application for MyApp {
    async fn new(ctx: &mut Context) -> Box<dyn Drawable> {
        let home = FirstScreen::new(ctx);
        let interface = Interface::new(ctx, Box::new(home), None, None);
        Box::new(interface)
    }
}

start!(MyApp);

#[derive(Debug, Component)]
pub struct CameraFeed(Stack, ExpandableImage, #[skip] Option<Camera>, #[skip] bool, #[skip] bool);

impl CameraFeed {
    pub fn new(ctx: &mut Context) -> Self {
        let blank = ctx.theme.brand.illustrations.get("blank").unwrap_or_else(|| {
            ctx.assets.add_image(RgbaImage::new(320, 240))
        });
        
        let camera = Camera::new_unprocessed().ok();
        let has_camera = camera.is_some();
        
        CameraFeed(
            Stack(Offset::Center, Offset::Center, Size::Static(320.0), Size::Static(240.0), Padding::default()),
            ExpandableImage::new(blank, Some((320.0, 240.0))),
            camera,
            false, 
            has_camera
        )
    }

    // Start the camera with option for unprocessed feed
    pub fn start_camera(&mut self, ctx: &mut Context, unprocessed: bool) {
        self.3 = unprocessed;
        self.2 = if unprocessed {
            Camera::new_unprocessed().ok()
        } else {
            Camera::new().ok()
        };
        self.4 = self.2.is_some();
    }

    pub fn stop_camera(&mut self) {
        self.2 = None;
        self.4 = false;
    }

    pub fn is_active(&self) -> bool {
        self.4
    }
}

// Handle events for CameraFeed
impl OnEvent for CameraFeed {
    fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
        if let Some(TickEvent) = event.downcast_ref::<TickEvent>() {
            if let Some(ref mut camera) = self.2 {
                if let Some(raw_frame) = camera.get_frame() {
                    let image = ctx.assets.add_image(raw_frame);
                    self.1.image().image = image;
                }
            }
        } else if let Some(control_event) = event.downcast_ref::<CameraControlEvent>() {
            match control_event {
                CameraControlEvent::StartProcessed => {
                    self.start_camera(ctx, false);
                },
                CameraControlEvent::StartUnprocessed => {
                    self.start_camera(ctx, true);
                },
                CameraControlEvent::Stop => {
                    self.stop_camera();
                },
            }
        }
        true
    }
}

#[derive(Debug, Component)]
pub struct FirstScreen(Stack, Page);

impl OnEvent for FirstScreen {
    fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
        true
    }
}

impl AppPage for FirstScreen {
    fn has_nav(&self) -> bool { false }

    fn navigate(self: Box<Self>, _ctx: &mut Context, _index: usize) -> Result<Box<dyn AppPage>, Box<dyn AppPage>> {
        Err(self)
    }
}

impl FirstScreen {
    pub fn new(ctx: &mut Context) -> Self {
        let font_size = ctx.theme.fonts.size;

        let header = Header::home(ctx, "Ramp Stack", None);

        let title = Text::new(
            ctx,
            "Maverick OS Demo",
            TextStyle::Heading,
            font_size.h2,
            Align::Center,
        );

        let subtitle = ExpandableText::new(
            ctx,
            "Demonstrates various hardware features of Maverick OS",
            TextStyle::Primary,
            font_size.md,
            Align::Center,
            None,
        );


        // Notification Button

        let send_notification = Button::primary(ctx, "Send Notification", |ctx: &mut Context| {
            ctx.hardware.push_notification("Reminder", "Don't forget your meeting at 3 PM today.");
        });


        // Haptic Feedback Button

        let haptic_feedback = Button::primary(ctx, "Haptic Feedback", |ctx: &mut Context| {
            ctx.hardware.haptic();
        });


        // Safe Area Insets Button

        let get_safe_area_insets = Button::primary(ctx, "Get Safe Area Insets", |ctx: &mut Context| {
            let (top, right, bottom, left) = ctx.hardware.safe_area_insets();
            println!("Safe Area Insets: top: {}, bottom: {}, left: {}, right: {}", top, bottom, left, right);
        });


        // Camera Buttons

        let open_camera = Button::primary(ctx, "Open Camera", |ctx: &mut Context| {
            match ctx.hardware.open_camera() {
                Ok(_) => println!("Camera opened successfully"),
                Err(e) => println!("Failed to open camera: {:?}", e),
            }
        });

        let unprocessed_camera = Button::primary(ctx, "Unprocessed Camera", |ctx: &mut Context| {
            match ctx.hardware.open_unprocessed_camera() {
                Ok(_) => println!("Unprocessed camera opened"),
                Err(e) => println!("Failed to open unprocessed camera: {:?}", e),
            }
        });

        let camera_feed = CameraFeed::new(ctx);
        
        let status_text = if camera_feed.is_active() {
            Text::new(ctx, "Camera Status: Active", TextStyle::Primary, font_size.sm, Align::Center)
        } else {
            Text::new(ctx, "Camera Status: Inactive", TextStyle::Primary, font_size.sm, Align::Center)
        };
        
        let start_processed_camera = Button::primary(ctx, "Start Processed Camera", |ctx: &mut Context| {
            ctx.trigger_event(CameraControlEvent::StartProcessed);
        });

        let start_unprocessed_camera = Button::primary(ctx, "Start Unprocessed Camera", |ctx: &mut Context| {
            ctx.trigger_event(CameraControlEvent::StartUnprocessed);
        });

        let stop_camera = Button::primary(ctx, "Stop Camera", |ctx: &mut Context| {
            ctx.trigger_event(CameraControlEvent::Stop);
        });


        // Photo Picker Button

        let open_photo_picker = Button::primary(ctx, "Open Photo Picker", |ctx: &mut Context| {
            let (tx, rx) = channel::<(Vec<u8>, ImageOrientation)>();
            ctx.hardware.open_photo_picker(tx);
            
            std::thread::spawn(move || {
                if let Ok((image_data, orientation)) = rx.recv() {
                    println!("Received image data of length: {}", image_data.len());
                    println!("Image orientation: {:?}", orientation);
                }
            });
        });


        // Cloud Storage Buttons

        let cloud_save = Button::primary(ctx, "Cloud Save", |ctx: &mut Context| {
            match ctx.hardware.cloud_save("username", "frankie") {
                Ok(_) => println!("Successfully saved to cloud"),
                Err(e) => println!("Failed to save to cloud: {}", e),
            }
        });

        let get_cloud_save = Button::primary(ctx, "Get Cloud Save", |ctx: &mut Context| {
            if let Some(value) = ctx.hardware.cloud_get("username") {
                println!("Cloud Save Value: {}", value);
            } else {
                println!("No value found for key 'username'");
            }
        });

        let cloud_remove = Button::primary(ctx, "Cloud Remove", |ctx: &mut Context| {
            match ctx.hardware.cloud_remove("username") {
                Ok(_) => println!("Successfully removed from cloud"),
                Err(e) => println!("Failed to remove from cloud: {}", e),
            }
        });

        let cloud_remove_all = Button::primary(ctx, "Cloud Remove All", |ctx: &mut Context| {
            match ctx.hardware.cloud_clear() {
                Ok(_) => println!("Successfully cleared all cloud data"),
                Err(e) => println!("Failed to clear cloud data: {:?}", e),
            }
        });

        let content = Content::new(
            ctx,
            Offset::Center,
            vec![
                Box::new(title),
                Box::new(subtitle),
                Box::new(send_notification),
                Box::new(haptic_feedback),
                Box::new(get_safe_area_insets),
                Box::new(open_camera),
                Box::new(unprocessed_camera),
                Box::new(status_text),
                Box::new(camera_feed),
                Box::new(start_processed_camera),
                Box::new(start_unprocessed_camera),
                Box::new(stop_camera),
                Box::new(open_photo_picker),
                Box::new(cloud_save),
                Box::new(get_cloud_save),
                Box::new(cloud_remove),
                Box::new(cloud_remove_all),
            ],
        );

        FirstScreen(Stack::default(), Page::new(Some(header), content, None))
    }
}