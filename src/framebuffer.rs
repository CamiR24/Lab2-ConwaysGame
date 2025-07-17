use raylib::prelude::*;

pub struct FrameBuffer {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Color>,
    background_color: Color,
    current_color: Color,
}

impl FrameBuffer {
    pub fn new(width: usize, height: usize, background_color: Color) -> Self {
        let pixels = vec![background_color; width * height];
        FrameBuffer {
            width,
            height,
            pixels,
            background_color,
            current_color: Color::WHITE,
        }
    }

    pub fn clear(&mut self) {
        self.pixels.fill(self.background_color);
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    pub fn point(&mut self, x: usize, y: usize, color: Color) {
        if x < self.width && y < self.height {
            self.pixels[y * self.width + x] = color;
        }
    }
    
    pub fn get_color(&self, x: usize, y: usize) -> Color {
        if x < self.width && y < self.height {
            self.pixels[y * self.width + x]
        } else {
            Color::BLACK
        }
    }

    pub fn step(&mut self) {
        let mut new_pixels = vec![Color::BLACK; self.width * self.height];
    
        for x in 0..self.width {
            for y in 0..self.height {
                let alive_neighbors = self.count_neighbors(x, y);
                let current = self.get_color(x, y);
    
                new_pixels[y * self.width + x] = match (current == Color::WHITE, alive_neighbors) {
                    (true, 2) | (true, 3) => Color::WHITE,  // survives
                    (false, 3) => Color::WHITE,             // reproduction
                    _ => Color::BLACK,                      // dies
                };
            }
        }
    
        self.pixels = new_pixels;
    }

    fn count_neighbors(&self, x: usize, y: usize) -> usize {
        let mut count = 0;
        
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                
                // Wraparound para los bordes
                let nx = ((x as i32 + dx).rem_euclid(self.width as i32)) as usize;
                let ny = ((y as i32 + dy).rem_euclid(self.height as i32)) as usize;
                
                if self.get_color(nx, ny) == Color::WHITE {
                    count += 1;
                }
            }
        }
        
        count
    }

    pub fn render(&self, d: &mut RaylibDrawHandle, cell_size_x: usize, cell_size_y: usize) {
        for x in 0..self.width {
            for y in 0..self.height {
                let color = self.get_color(x, y);
                if color == Color::WHITE {
                    d.draw_rectangle(
                        (x * cell_size_x) as i32,
                        (y * cell_size_y) as i32,
                        cell_size_x as i32,
                        cell_size_y as i32,
                        color,
                    );
                }
            }
        }
    }
    
    //definición de patrones iniciales
    pub fn set_glider(&mut self, start_x: usize, start_y: usize) {
        let pattern = vec![
            (1, 0), (2, 1), (0, 2), (1, 2), (2, 2)
        ];
        
        for (x, y) in pattern {
            self.point(start_x + x, start_y + y, Color::WHITE);
        }
    }
    
    pub fn set_blinker(&mut self, start_x: usize, start_y: usize) {
        let pattern = vec![
            (0, 1), (1, 1), (2, 1)
        ];
        
        for (x, y) in pattern {
            self.point(start_x + x, start_y + y, Color::WHITE);
        }
    }
    
    pub fn set_toad(&mut self, start_x: usize, start_y: usize) {
        let pattern = vec![
            (1, 0), (2, 0), (3, 0),
            (0, 1), (1, 1), (2, 1)
        ];
        
        for (x, y) in pattern {
            self.point(start_x + x, start_y + y, Color::WHITE);
        }
    }
    
    pub fn set_beacon(&mut self, start_x: usize, start_y: usize) {
        let pattern = vec![
            (0, 0), (1, 0),
            (0, 1),
            (3, 2),
            (2, 3), (3, 3)
        ];
        
        for (x, y) in pattern {
            self.point(start_x + x, start_y + y, Color::WHITE);
        }
    }
    
    pub fn set_lwss(&mut self, start_x: usize, start_y: usize) {
        // Lightweight Spaceship
        let pattern = vec![
            (0, 1), (3, 1),
            (4, 2),
            (0, 3), (4, 3),
            (1, 4), (2, 4), (3, 4), (4, 4)
        ];
        
        for (x, y) in pattern {
            self.point(start_x + x, start_y + y, Color::WHITE);
        }
    }
    
    pub fn set_pulsar(&mut self, start_x: usize, start_y: usize) {
        let pattern = vec![
            (2, 0), (3, 0), (4, 0), (8, 0), (9, 0), (10, 0),
            
            (0, 2), (5, 2), (7, 2), (12, 2),
            (0, 3), (5, 3), (7, 3), (12, 3),
            (0, 4), (5, 4), (7, 4), (12, 4),
            
            (2, 5), (3, 5), (4, 5), (8, 5), (9, 5), (10, 5),
            
            (2, 7), (3, 7), (4, 7), (8, 7), (9, 7), (10, 7),
            
            (0, 8), (5, 8), (7, 8), (12, 8),
            (0, 9), (5, 9), (7, 9), (12, 9),
            (0, 10), (5, 10), (7, 10), (12, 10),
            
            (2, 12), (3, 12), (4, 12), (8, 12), (9, 12), (10, 12),
        ];
        
        for (x, y) in pattern {
            self.point(start_x + x, start_y + y, Color::WHITE);
        }
    }
    
    pub fn set_pentadecathlon(&mut self, start_x: usize, start_y: usize) {
        let pattern = vec![
            (1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0), (7, 0), (8, 0),
            (0, 1), (9, 1),
            (0, 2), (9, 2),
            (1, 3), (8, 3),
            (1, 4), (2, 4), (3, 4), (4, 4), (5, 4), (6, 4), (7, 4), (8, 4),
        ];
        
        for (x, y) in pattern {
            self.point(start_x + x, start_y + y, Color::WHITE);
        }
    }
    
    pub fn set_gosper_glider_gun(&mut self, start_x: usize, start_y: usize) {
        let pattern = vec![
            (24, 0),
            (22, 1), (24, 1),
            (12, 2), (13, 2), (20, 2), (21, 2), (34, 2), (35, 2),
            (11, 3), (15, 3), (20, 3), (21, 3), (34, 3), (35, 3),
            (0, 4), (1, 4), (10, 4), (16, 4), (20, 4), (21, 4),
            (0, 5), (1, 5), (10, 5), (14, 5), (16, 5), (17, 5), (22, 5), (24, 5),
            (10, 6), (16, 6), (24, 6),
            (11, 7), (15, 7),
            (12, 8), (13, 8)
        ];
        
        for (x, y) in pattern {
            self.point(start_x + x, start_y + y, Color::WHITE);
        }
    }
    
    pub fn set_diehard(&mut self, start_x: usize, start_y: usize) {
        let pattern = vec![
            (6, 0),
            (0, 1), (1, 1),
            (1, 2), (5, 2), (6, 2), (7, 2)
        ];
        
        for (x, y) in pattern {
            self.point(start_x + x, start_y + y, Color::WHITE);
        }
    }
    
    pub fn set_acorn(&mut self, start_x: usize, start_y: usize) {
        let pattern = vec![
            (1, 0),
            (3, 1),
            (0, 2), (1, 2), (4, 2), (5, 2), (6, 2)
        ];
        
        for (x, y) in pattern {
            self.point(start_x + x, start_y + y, Color::WHITE);
        }
    }

    //llamar patrones iniciales
    pub fn set_initial_pattern(&mut self) {
        self.clear();
        
        self.set_glider(10, 10);
        self.set_glider(20, 20);
        self.set_glider(30, 30);
        
        self.set_blinker(50, 10);
        self.set_blinker(50, 15);
        self.set_blinker(50, 20);
        
        self.set_toad(70, 10);
        self.set_beacon(70, 20);
        
        self.set_lwss(10, 50);
        self.set_lwss(20, 60);
        
        self.set_pulsar(40, 40);
        
        self.set_pentadecathlon(10, 80);
        
        //agregar el Gosper Glider Gun si hay espacio
        if self.width > 80 && self.height > 80 {
            self.set_gosper_glider_gun(5, 5);
        }
        
        self.set_diehard(80, 10);
        self.set_acorn(80, 80);
        
        //patrones aleatorios
        for i in 0..10 {
            self.set_glider(i * 8, 90);
        }
    }
}