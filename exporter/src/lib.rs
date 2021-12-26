use std::{fs::File, io::Write};



pub struct ComplexNum {
    real: f64,
    imag: f64,
}

impl ComplexNum {
    pub fn new(x: f64, y: f64) -> Self {
        ComplexNum {
            real: x,
            imag: y,
        }
    }

    pub fn abs(&self) -> f64 { // module
        (self.real * self.real + self.imag * self.imag).sqrt()
    }

    pub fn multiply(&self, complex: &Self) -> Self {
        let a2 = self.real * complex.real - self.imag * complex.imag;
        let b2 = self.real * complex.imag + self.imag * complex.real;
        ComplexNum{real: a2, imag: b2}
    }

    pub fn add(&self, complex: &Self) -> Self {
        let a2 = self.real + complex.real;
        let b2 = self.imag + complex.imag;
        ComplexNum{real: a2, imag: b2}
    }
}


// c2 = c0 + c1
// (a0 + b0i) + (a1 + b1i) = a0 + a1 + b0i + b1i
// a2 = a0 + a1 
// b2 = b0 + b1
// c2 = a2 + b2i

// c2 = c0 * c1
// (a0 + b0i) * (a1 + b1i) = a0a1 + a0b1i + b0a1i - b0b1
// a2 = a0a1 - b0b1
// b2 = a0b1 + b0a1
// c2 = a2 + b2i


#[derive(Copy, Clone)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Pixel {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Pixel {
            r,
            g,
            b,
        }
    }
}


pub struct Image {
    w: u32,
    h: u32,
    pixels: Vec::<Pixel>,
}


impl Image {
    pub fn new(w: u32, h: u32) -> Self {
        Image {
            w,
            h,
            pixels: vec![Pixel::new(0, 0, 0); w as usize * h as usize],
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, pxl: Pixel) {
        let position = x as usize + self.w as usize * y as usize;
        self.pixels[position] = pxl;
    }

    fn padding(&self) -> u32{
        let mut num= 0;
        let reminder = 3 * self.w % 4;
        if reminder != 0 {
            num = 4 - reminder;
        }
        return num;
    }

    fn bmp_file_header(&self, file: &mut File) {
        let bm = [0x42, 0x4D];
        file.write_all(&bm).unwrap();

        let size = 3 * self.w * self.h + 54 + self.padding() * self.h;
        file.write_all(&size.to_le_bytes()).unwrap();
        
        file.write_all(&[0, 0, 0, 0]).unwrap();
        file.write_all(&54u32.to_le_bytes()).unwrap();
    }


    fn bmp_info_header(&self, file: &mut File) {
        file.write_all(&40u32.to_le_bytes()).unwrap();
        file.write_all(&self.w.to_le_bytes()).unwrap();
        file.write_all(&self.h.to_le_bytes()).unwrap();
        file.write_all(&1u16.to_le_bytes()).unwrap();
        file.write_all(&24u16.to_le_bytes()).unwrap();

        file.write_all(&0u32.to_le_bytes()).unwrap();
        file.write_all(&0u32.to_le_bytes()).unwrap();
        file.write_all(&0u32.to_le_bytes()).unwrap();
        file.write_all(&0u32.to_le_bytes()).unwrap();

        file.write_all(&0u32.to_le_bytes()).unwrap();
        file.write_all(&0u32.to_le_bytes()).unwrap();
    }

    fn bmp_main_data(&self, file: &mut File) {
        for y in (0..self.h).rev() {
            for x in 0..self.w {
                let index = self.w as usize * y as usize + x as usize;
                let pxl = self.pixels[index];
                file.write_all(&[pxl.b, pxl.g, pxl.r]).unwrap();
                
            }

            for _ in 0..self.padding() {
                file.write_all(&[0]).unwrap();
            }
        }
    }
    

    pub fn save(&self, path: &str) {
        
        let mut file = File::create(path).unwrap();
        self.bmp_file_header(&mut file);
        self.bmp_info_header(&mut file);
        self.bmp_main_data(&mut file);
        
    }
}

