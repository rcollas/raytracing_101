use pixels::{Pixels, SurfaceTexture};
use std::error::Error;
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};
mod algebra;
use crate::algebra::vec3::Vec3;
use crate::algebra::quadratic::compute_quadratic;

const WIDTH: u32 = 640;
const HEIGHT: u32 = 640;
const DEFAULT_RESOLUTION: LogicalSize<f64> = LogicalSize::new(WIDTH as f64, HEIGHT as f64);

#[derive(Copy, Clone)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}


#[derive(Copy, Clone)]
struct Sphere {
    coordinates: Vec3<f64>,
    radius: f64,
    color: Color,
}

struct Ray {
    direction: Vec3<f64>, 
}

impl Sphere {
    fn intersect(self: Sphere, ray: &Ray, origin: Vec3<f64>) -> (f64, f64) {
        let co = origin - self.coordinates;
        let a = ray.direction * ray.direction;
        let b = 2.0 * (co * ray.direction);
        let c = co * co - self.radius * self.radius;
        compute_quadratic(a, b, c)
    }
}

struct World {
    origin: Vec3<f64>,
    objects: Vec<Sphere>,
    background: Color,
}

struct ApplicationState {
    name: String,
    window: Window,
    resolution: LogicalSize<f64>,
    world: World
}

impl ApplicationState {
    fn redraw(self: &ApplicationState) {
        self.window.request_redraw();
    }

    fn draw(self: &ApplicationState) {
        let surface_texture = SurfaceTexture::new(
            self.window.inner_size().width,
            self.window.inner_size().height,
            &self.window,
        );
        let mut pixels = Pixels::new(self.resolution.width as u32, self.resolution.height as u32, surface_texture).unwrap();

        let frame = pixels.frame_mut();
        let mut results: Vec<Color> = Vec::with_capacity((HEIGHT * WIDTH) as usize);

        for y in -((HEIGHT/2) as i32)..(HEIGHT/2) as i32 {
            for x in -((WIDTH/2) as i32)..(WIDTH/2) as i32 {
                let vx = x as f64 / WIDTH as f64;
                let vy = y as f64 / HEIGHT as f64;
                let ray = Ray { direction: Vec3 { x: vx.into(), y: vy.into(), z: 1.0} };

                // compute the closest sphere that intersects the ray if any
                let mut closest_sphere: Option<&Sphere> = None;
                for sphere in &self.world.objects {
                    let mut closest_t = f64::INFINITY;
                    let (t1, t2) = sphere.intersect(&ray, self.world.origin);
                    if (1.0..closest_t).contains(&t1)  {
                        closest_t = t1;
                        closest_sphere = Some(sphere);
                    }
                    if (1.0..closest_t).contains(&t2) {
                        closest_t = t2;
                        closest_sphere = Some(sphere);
                    }
                }
                if closest_sphere.is_some() {
                    results.push(closest_sphere.unwrap().color);
                } else {
                    results.push(self.world.background);
                }
            }
        }

        // set pixels color for every pixel of the frame
        for (x, pixel) in frame.chunks_exact_mut(4).enumerate() {
            pixel[0] = results[x].r;
            pixel[1] = results[x].g;
            pixel[2] = results[x].b;
            pixel[3] = results[x].a;
        }
        pixels.render().unwrap();

        self.window.request_redraw();
    }
}

struct Application {
    state: ApplicationState,
    event_loop: EventLoop<()>,
}

impl Application {
    pub fn new(name: String, resolution: Option<LogicalSize<f64>>) -> Result<Self, Box<dyn Error>> {
        let event_loop = EventLoop::new()?;
        event_loop.set_control_flow(ControlFlow::Wait);

        let resolution = match resolution {
            Some(resolution) => resolution,
            None => DEFAULT_RESOLUTION,
        };

        let window = Application::init_window(&name, resolution, &event_loop)?;

        let mut world = World {
            origin: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
            objects: Vec::new(),
            background: Color { r: 255, g: 255, b: 255, a: 255 },
        };
        let sphere_1 = Sphere {
            coordinates: Vec3 { x: 0.0, y: 0.0, z: 30.0 },
            radius: 5.0,
            color: Color { r: 136, g: 47, b: 164, a: 255 },
        };
        let sphere_2 = Sphere {
            coordinates: Vec3 { x: 2.5, y: 2.5, z: 23.0 },
            radius: 5.0,
            color: Color { r: 255, g: 0, b: 0, a: 255 },
        };
        let sphere_3 = Sphere {
            coordinates: Vec3 { x: 2.5, y: 2.5, z: 25.0 },
            radius: 5.0,
            color: Color { r: 0, g: 0, b: 255, a: 255 },
        };
        world.objects.push(sphere_1);
        world.objects.push(sphere_2);
        world.objects.push(sphere_3);
        let state = ApplicationState { name, window, resolution, world };

        Ok(Application {
            state,
            event_loop,
        })
    }

    fn init_window(
        name: &String,
        resolution: LogicalSize<f64>,
        event_loop: &EventLoop<()>,
    ) -> Result<Window, Box<dyn Error>> {
        let window = WindowBuilder::new()
            .with_title(name)
            .with_inner_size(resolution)
            .with_min_inner_size(resolution)
            .build(event_loop)?;

        Ok(window)
    }

    pub fn run(self: Application) {
        let mut has_draw = false;
        let _ = self.event_loop.run(move |event, elwt| match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("The close button was pressed, stopping...");
                elwt.exit();
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(_),
                ..
            } => {
                self.state.redraw();
                println!("Window resized.")
            }
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => {
                if !has_draw {
                    self.state.draw();
                    has_draw = true;
                }
            }
            _ => (),
        });
    }
}


fn main() {
    let application = Application::new(String::from("my wonderful application"), None);
    match application {
        Ok(application) => {
            println!("{} created. Running...", String::from(&application.state.name));
            application.run();
        }
        Err(err) => {
            println!("Can't create the application: {}", err);
        }
    }
}
