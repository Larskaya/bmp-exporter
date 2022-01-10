
use std::sync::mpsc::Sender;
use std::sync::mpsc::channel;
use std::time::Instant;

// use exporter::Image;
use exporter::Pixel;
use exporter::ComplexNum;

use sfml::graphics::Color;
use sfml::graphics::Vertex;
use sfml::system::Vector2f;

use sfml::window::mouse::Button;
use sfml::{window::{Event, Style}, graphics::{VertexArray, PrimitiveType, RenderWindow, RenderTarget, RenderStates}};

use std::thread::{self, JoinHandle};

fn get_thread(tx: Sender<Vertex>, start_x: u32, w: u32, h: u32, rx: f64, lx: f64, uy: f64, dy: f64) -> JoinHandle<()> {
    
    let handle = thread::spawn(move || {
        for x in 0..w {
            for y in 0..h {
                let cx: f64 = x as f64 / (w-1) as f64 * (rx - lx) + lx;
                let cy: f64 = y as f64 / (h-1) as f64 * (uy - dy) + dy;
                
                let pxl = pixel_color(cx, cy);
                let vertex = Vertex::new(
                    Vector2f::new((start_x + x) as f32, y as f32), 
                    Color::rgb(pxl.r , pxl.g , pxl.b ),
                    Vector2f::new(x as f32, y as f32),
                );
                let value = vertex;
                tx.send(value).unwrap();
            }
        }
    });
    return handle
    
}



fn pixel_color(x: f64, y: f64) -> Pixel {
    let  c = ComplexNum::new(x, y);

    let mut z = ComplexNum::new(0.0, 0.0);

    let n = 500.0;
    let mut i = 0.0;
    while i < n && z.abs() < 2.0 {
        z = z.multiply(&z).add(&c);
        i+=1.0;
    }
    let r = (i / n * 255.0) as u8;
    Pixel::new(r, r, r)
}


fn translate(v: f64, d: f64, lv: f64, hv: f64) -> f64 {
    v / (d-1.0) * (hv - lv) + lv
}


fn render(va: &mut VertexArray, w: u32, h: u32, lx: f64, rx: f64, dy: f64, uy: f64) {
    let parts = 8;
    
    let mut threads = Vec::new();
    let (vertex_tx, vertex_rx) = channel();

    let need_w = w / parts;
    let part_w = (rx - lx) / parts as f64;
     
    for i in 0..parts { 
        let lx2 = lx + part_w * i as f64;
        let rx2 = lx2 + part_w;

        let start_x = need_w * i;
        let handle = get_thread(vertex_tx.clone(), start_x, need_w, h, rx2, lx2, uy, dy);
        threads.push(handle);
    }

    for index in 0..w as usize * h as usize{
        let rec = vertex_rx.recv().unwrap();
        va[index] = rec;
    }   

    for thread in threads {
        thread.join().unwrap();
    }
   
}


fn main() {
    let w = 2000;
    let h = 1500;
    let mut window = RenderWindow::new((w, h),
                                "so beautiful image",
                                Style::CLOSE,
                                &Default::default());

    window.set_framerate_limit(60);
    
    let mut pixels = VertexArray::new(
        PrimitiveType::POINTS, 
        w as usize * h as usize,
    );
    let mut lx = -2.0;
    let mut rx = 2.0;
    let mut dy = -1.5;
    let mut uy = 1.5;
    render(&mut pixels, w, h, lx, rx, dy, uy);

    while window.is_open() {
        // let inst = Instant::now();

        while let Some(event) = window.poll_event() {
            if event == Event::Closed {
                window.close();
            }

            if let Event::MouseButtonPressed { button, x, y } = event {
                if button == Button::LEFT {
                    println!("{}:{}", x, y);
                    
                    let cx = translate(x as f64, w as f64, lx, rx); // here click
                    let cy = translate(y as f64, h as f64, -dy, -uy);

                    let lenght_x = rx - lx;
                    let lenght_y = dy - uy;
                    lx = cx - lenght_x / 8.0;
                    rx = cx + lenght_x / 8.0;
                    dy = cy - lenght_y / 8.0;
                    uy = cy + lenght_y / 8.0;


                    let inst = Instant::now();
                    render(&mut pixels, w, h, lx, rx, dy, uy);
                    println!("Render time: {}", (Instant::now() - inst).as_secs_f64());
                }
            }
        }
        // Activate the window for OpenGL rendering
        window.set_active(true);


        window.draw_vertex_array(&pixels, &RenderStates::default());
    
        // End the current frame and display its contents on screen
        window.display();
        
    }
}
