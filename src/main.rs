use ggez::{
    conf::WindowMode,
    event::{self, EventHandler},
    graphics::{Canvas, Color, DrawMode, DrawParam, Mesh, MeshBuilder, Text},
    Context, ContextBuilder, GameResult,
};
use rand::Rng;
use std::sync::mpsc::{self, Receiver};
use std::thread;
use std::time::Duration;

struct State {
    total_points: u64,
    inside_points: u64,
    points: Vec<(f32, f32, bool)>,
    max_display_points: usize,
    rx: Receiver<Vec<(f32, f32, bool)>>,
}

impl State {
    fn new(rx: Receiver<Vec<(f32, f32, bool)>>) -> Self {
        Self {
            total_points: 0,
            inside_points: 0,
            points: Vec::new(),
            max_display_points: 30_000,
            rx,
        }
    }
}

impl EventHandler for State {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Recibir todos los lotes disponibles desde los hilos
        while let Ok(batch) = self.rx.try_recv() {
            for (x, y, inside) in batch {
                self.total_points += 1;
                if inside {
                    self.inside_points += 1;
                }
                self.points.push((x, y, inside));
            }
        }

        // Mantener solo los últimos puntos visibles
        if self.points.len() > self.max_display_points {
            let excess = self.points.len() - self.max_display_points;
            self.points.drain(0..excess);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let (w, h) = ctx.gfx.drawable_size();
        let mut canvas = Canvas::from_frame(ctx, Color::from_rgb(20, 20, 30));

        let mut mb = MeshBuilder::new();
        for &(x, y, inside) in &self.points {
            let sx = w / 2.0 + x * (w / 2.0 - 20.0);
            let sy = h / 2.0 - y * (h / 2.0 - 20.0);
            let color = if inside {
                Color::from_rgb(100, 220, 100)
            } else {
                Color::from_rgb(220, 100, 100)
            };
            let _ = mb.circle(DrawMode::fill(), [sx, sy], 1.3, 0.1, color);
        }

        let mesh = Mesh::from_data(ctx, mb.build());
        canvas.draw(&mesh, DrawParam::default());

        let pi_estimate = 4.0 * self.inside_points as f64 / self.total_points as f64;
        let text = Text::new(format!(
            "π ≈ {:.10}\nPuntos totales: {}\nPuntos visibles: {}",
            pi_estimate, self.total_points, self.points.len()
        ));
        canvas.draw(&text, DrawParam::default().dest([10.0, 10.0]));

        canvas.finish(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    let (tx, rx) = mpsc::channel::<Vec<(f32, f32, bool)>>();

    // Cantidad de hilos de cálculo
    let num_threads = 2;

    for _ in 0..num_threads {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            let mut rng = rand::thread_rng();
            loop {
                let mut batch = Vec::with_capacity(1_000);
                for _ in 0..1_000 {
                    let x = rng.gen_range(-1.0..1.0);
                    let y = rng.gen_range(-1.0..1.0);
                    let inside = x * x + y * y <= 1.0;
                    batch.push((x, y, inside));
                }

                // Enviar el lote de puntos
                if tx_clone.send(batch).is_err() {
                    break;
                }

                // Pequeña pausa para evitar colapsar el canal
                thread::sleep(Duration::from_millis(10));
                
            }
        });
    }

    drop(tx); // Cierra el canal original, solo quedan los clones activos

    let (ctx, event_loop) = ContextBuilder::new("Monte Carlo π", "gts")
        .window_mode(WindowMode::default().dimensions(800.0, 800.0))
        .build()?;

    let state = State::new(rx);
    event::run(ctx, event_loop, state)
}
fn inside(x: f32, y: f32) ->bool{
    if x*x + y*y <=1{
        true;
    }else{
        false;
    }
