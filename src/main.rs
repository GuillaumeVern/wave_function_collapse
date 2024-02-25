use std::os::raw;

use minifb::{Key, Window, WindowOptions};
use image::imageops::FilterType::Triangle;

const WIDTH: u32 = 640;
const HEIGHT: u32 = 640;
const COLS: u32 = 8;
const ROWS: u32 = 8;
const CELL_WIDTH: u32 = WIDTH / COLS;
const CELL_HEIGHT: u32 = HEIGHT / ROWS;
const NUM_TILES: u32 = COLS * ROWS;

#[derive(Clone)]
struct Tile{
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    image: image::RgbImage,
}

impl Tile{
    pub fn draw(&mut self, buffer: &mut Vec<u32>, rotation: u16, image: &str) { // rotation in increments of 90°
        self.image = image::open(image).unwrap().resize_exact(CELL_WIDTH as u32, CELL_HEIGHT as u32, Triangle).to_rgb8();
        let mut raw_image = self.image.clone().into_raw();
        let start_pixel = self.x + self.y * WIDTH;
        let mut current_pixel_index = start_pixel;

        self.rotate(&mut raw_image, rotation);

        for i in (0..raw_image.len()).step_by(3){

            // calculate the position of the next pixel to be drawn
            current_pixel_index += 1;
            if (current_pixel_index % self.width == 0) && (current_pixel_index + (WIDTH - self.width) < (buffer.len() - 1) as u32) {
                current_pixel_index += WIDTH - self.width;
            }

            // format separate R, G and B values into a single RGB u32
            let pixel_color = (raw_image[i] as u32) << 16 // R
                        | (raw_image[i + 1] as u32) << 8 // G
                        | (raw_image[i + 2] as u32); // B
            
            // draw pixel
            if current_pixel_index < WIDTH * HEIGHT {
                buffer[current_pixel_index as usize] = pixel_color;
            }
        }
    }

    fn rotate(&mut self, raw_image: &mut Vec<u8>, rotation: u16){
        match rotation {
            0 => (),
            90 => {
                let mut new_raw_image = vec![0; raw_image.len()];
                for i in 0..self.width{
                    for j in 0..self.height{
                        let new_index = (i * self.height + j) * 3;
                        let old_index = (j * self.width + (self.width - i - 1)) * 3;
                        new_raw_image[new_index as usize] = raw_image[old_index as usize];
                        new_raw_image[(new_index + 1) as usize] = raw_image[(old_index + 1) as usize];
                        new_raw_image[(new_index + 2) as usize] = raw_image[(old_index + 2) as usize];
                    }
                }
                *raw_image = new_raw_image;
            },
            180 => {
                let mut new_raw_image = vec![0; raw_image.len()];
                for i in 0..self.width{
                    for j in 0..self.height{
                        let new_index = (i * self.height + j) * 3;
                        let old_index = ((self.width - i - 1) * self.width + (self.height - j - 1)) * 3;
                        new_raw_image[new_index as usize] = raw_image[old_index as usize];
                        new_raw_image[(new_index + 1) as usize] = raw_image[(old_index + 1) as usize];
                        new_raw_image[(new_index + 2) as usize] = raw_image[(old_index + 2) as usize];
                    }
                }
                *raw_image = new_raw_image;
            },
            270 => {
                let mut new_raw_image = vec![0; raw_image.len()];
                for i in 0..self.width{
                    for j in 0..self.height{
                        let new_index = (i * self.height + j) * 3;
                        let old_index = ((self.height - j - 1) * self.width + i) * 3;
                        new_raw_image[new_index as usize] = raw_image[old_index as usize];
                        new_raw_image[(new_index + 1) as usize] = raw_image[(old_index + 1) as usize];
                        new_raw_image[(new_index + 2) as usize] = raw_image[(old_index + 2) as usize];
                    }
                }
                *raw_image = new_raw_image;
            },
            _ => (),
        }
    }

}

struct Board{
    cols: u32,
    rows: u32,
    tiles: Vec<Vec<Tile>>,
}

impl Board{
    pub fn init_tiles(&mut self){
        for i in 0..COLS{
            self.tiles.push(Vec::new());
            for j in 0..ROWS{
                let tile = Tile{
                    x: ((i * self.cols + j) % self.cols) * CELL_WIDTH,
                    y: ((i * self.rows + j) / self.rows) * CELL_HEIGHT,
                    width: CELL_WIDTH,
                    height: CELL_HEIGHT,
                    image: image::open("assets/test.png").unwrap().resize_exact(CELL_WIDTH as u32, CELL_HEIGHT as u32, Triangle).to_rgb8(),
                };
                
                self.tiles[i as usize].push(tile);
            }
        }
    }
}


fn main() {
    let mut buffer: Vec<u32> = vec![0; (WIDTH * HEIGHT) as usize];

    let mut board = Board{
        cols: COLS,
        rows: ROWS,
        tiles: Vec::new(),
    };

    board.init_tiles();


    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH as usize,
        HEIGHT as usize,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {

        // for i in 0..board.tiles.len(){
        //     for j in 0..board.tiles[i as usize].len(){
        //         board.tiles[i as usize][j as usize].draw(&mut buffer, 90);
        //     }
        // }

        // draw first tile
        board.tiles[(COLS / 2) as usize][(ROWS / 2) as usize].draw(&mut buffer, 0, "assets/test.png");

        

        window.update_with_buffer(&buffer, WIDTH as usize, HEIGHT as usize).unwrap();

        


        // draw grid edges
        for i in 0..HEIGHT{
            for j in 0..WIDTH{
                let current_pixel_index = (i * WIDTH + j) as usize;
                if (current_pixel_index as u32 % CELL_WIDTH == 0) || (i % CELL_HEIGHT == 0) {
                    buffer[current_pixel_index] = 0x000000; // black
                }
            }
        }

        
    }
}