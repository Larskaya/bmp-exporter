
// use exporter::Image;
use exporter::Pixel;
use exporter::ComplexNum;

use sfml::graphics::Color;
use sfml::system::Vector2f;
use sfml::window::mouse::Button;
use sfml::{window::{Event, Style}, graphics::{VertexArray, PrimitiveType, RenderWindow, RenderTarget, RenderStates}};


fn pixel_color(x: f64, y: f64) -> Pixel {
    let  c = ComplexNum::new(x, y);

    let mut z = ComplexNum::new(0.0, 0.0);

    let n = 1000.0;
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


fn render( va: &mut VertexArray, w: u32, h: u32, lx: f64, rx: f64, dy: f64, uy: f64) {
    for x in 0..w {
        for y in 0..h {
            let cx: f64 = x as f64 / (w-1) as f64 * (rx - lx) + lx;
            let cy: f64 = y as f64 / (h-1) as f64 * (uy - dy) + dy;
            let index = w as usize * y as usize + x as usize;
            let pxl = pixel_color(cx, cy);
            va[index].position = Vector2f::new(x as f32, y as f32);
            va[index].color = Color::rgb(pxl.r , pxl.g , pxl.b );
        }

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

                    render(&mut pixels, w, h, lx, rx, dy, uy);

                }
            }
        }
        // Activate the window for OpenGL rendering
        window.set_active(true);


        window.draw_vertex_array(&pixels, &RenderStates::default());
    
        // End the current frame and display its contents on screen
        window.display();
    }

    // let w = 5000;
    // let h = 5000;

    
    // let mut image = Image::new(w, h);


    // for x in 0..w {
    //     for y in 0..h {
    //         let cx = 4.0 / (w as f64 - 1.0) * x as f64 - 2.0;
    //         let cy = 4.0 / -(h  as f64 + 1.0) * y as f64 + 2.0;
    //         let pxl = pixel_color(cx, cy);
    //         // image.set_pixel(x, y, pxl);
    //     }
    // }
    

    // image.save("image.bmp");

}
