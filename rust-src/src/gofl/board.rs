#[derive(Clone, Copy)]
pub struct Cell {
    pub past: bool,
    pub curr: bool,
}

impl Cell {
    pub fn new(past: bool, curr: bool) -> Self {
        Cell { past, curr }
    }
}

pub struct Board {
    wid: usize,
    hei: usize,
    cell_scale: usize,
    pub cells: Vec<Cell>,
    next: Vec<Cell>,
    frn_state: u64,
}

impl Board {
    #[inline]
    pub fn idx_into(&self, x: usize, y: usize) -> usize {
        y * self.wid + x
    }
    pub fn new(wid: usize, hei: usize, cell_scale: usize, frn_seed: u64) -> Self {
        let cells = vec![Cell::new(false, false); wid * hei];
        let next = cells.clone();

        Self {
            wid,
            hei,
            cell_scale,
            cells,
            next,
            frn_state: frn_seed,
        }
    }

    // fake deterministic random number generator
    pub fn next_frn(&mut self) -> u32 {
        let mut x = self.frn_state;

        x ^= x << 13;
        x ^= x >> 11;
        x ^= x << 17;

        self.frn_state = x;

        x as u32
    }

    pub fn set_cross(&mut self) {
        for y in 0..self.hei {
            self.cells[y * self.wid + self.wid / 2].curr = true;
        }
        for x in 0..self.wid {
            self.cells[(self.hei / 2) * self.wid + x].curr = true;
        }
    }

    pub fn set_horizontal(&mut self) {
        for x in 0..self.wid {
            self.cells[(self.hei / 2) * self.wid + x].curr = true;
        }
    }

    pub fn set_vertical(&mut self) {
        for y in 0..self.hei {
            self.cells[y * self.wid + self.wid / 2].curr = true;
        }
    }

    pub fn set_diagonals(&mut self) {
        for y in 0..self.hei {
            for x in 0..self.wid {
                if x == y || y + 1 == self.wid - x {
                    self.cells[y * self.wid + x].curr = true;
                }
                if x == y {
                    self.cells[y * self.wid + x].curr = true;
                }
            }
        }
    }

    pub fn set_diagon(&mut self) {
        for y in 0..self.hei {
            for x in 0..self.wid {
                if y + 1 == self.wid - x {
                    self.cells[y * self.wid + x].curr = true;
                }
            }
        }
    }
    pub fn set_diagonal(&mut self) {
        for y in 0..self.hei {
            for x in 0..self.wid {
                if x == y {
                    self.cells[y * self.wid + x].curr = true;
                }
            }
        }
    }

    pub fn top_glider(&mut self) {
        let a = 3_usize;
        let rep = 6_usize;
        let b = 0_usize;
        for i in 0..self.hei {
            for j in 0..self.wid {
                // Glider from top to left
                if i == b && j % rep == a + 1 {
                    self.cells[i * self.wid + j].curr = true;
                } else if i == b + 1 && j % rep == a {
                    self.cells[i * self.wid + j].curr = true;
                } else if i == b + 2 && (j % rep == a || j % rep == a + 1 || j % rep == a + 2) {
                    self.cells[i * self.wid + j].curr = true;
                }
            }
        }
    }
    pub fn bot_glider(&mut self) {
        let a = 3_usize;
        let rep = 6_usize;
        let b = self.hei - 1;
        for i in 0..self.hei {
            for j in 0..self.wid {
                // Glider from bottom to right
                if i == b && j % rep == a + 1 {
                    self.cells[i * self.wid + j].curr = true;
                } else if i == b - 1 && j % rep == a + 2 {
                    self.cells[i * self.wid + j].curr = true;
                } else if i == b - 2 && (j % rep == a || j % rep == a + 1 || j % rep == a + 2) {
                    self.cells[i * self.wid + j].curr = true;
                }
            }
        }
    }
    pub fn rig_glider(&mut self) {
        let a = 3_usize;
        let rep = 6_usize;
        let b = self.wid - 1;
        for i in 0..self.hei {
            for j in 0..self.wid {
                // Glider from right to top
                if j == b && i % rep == a + 1 {
                    self.cells[i * self.wid + j].curr = true;
                } else if j == b - 1 && i % rep == a {
                    self.cells[i * self.wid + j].curr = true;
                } else if j == b - 2 && (i % rep == a || i % rep == a + 1 || i % rep == a + 2) {
                    self.cells[i * self.wid + j].curr = true;
                }
            }
        }
    }
    pub fn lef_glider(&mut self) {
        let a = 3_usize;
        let rep = 6_usize;
        let b = 0_usize;
        for i in 0..self.hei {
            for j in 0..self.wid {
                // Glider from left to bottom
                if j == b && i % rep == a + 1 {
                    self.cells[i * self.wid + j].curr = true;
                } else if j == b + 1 && i % rep == a + 2 {
                    self.cells[i * self.wid + j].curr = true;
                } else if j == b + 2 && (i % rep == a || i % rep == a + 1 || i % rep == a + 2) {
                    self.cells[i * self.wid + j].curr = true;
                }
            }
        }
    }
}

pub trait BoardTrait {
    fn get_alive_neigh(&self, x: usize, y: usize, walls: bool) -> usize;
    fn draw_onto(&self, frame: &mut [u8], frame_width: usize);
    fn update_next(&mut self, walls: bool, random: bool);
}

impl BoardTrait for Board {
    fn get_alive_neigh(&self, x: usize, y: usize, walls: bool) -> usize {
        let mut count = 0;

        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let mut nx = x as isize + dx;
                let mut ny = y as isize + dy;

                if nx < 0 {
                    if walls {
                        nx = self.wid as isize - 1;
                    } else {
                        continue;
                    }
                } else if nx >= self.wid as isize {
                    if walls {
                        nx = 0;
                    } else {
                        continue;
                    }
                }

                if ny < 0 {
                    if walls {
                        ny = self.hei as isize - 1;
                    } else {
                        continue;
                    }
                } else if ny >= self.hei as isize {
                    if walls {
                        ny = 0;
                    } else {
                        continue;
                    }
                }

                match self.cells[self.idx_into(nx as usize, ny as usize)].curr {
                    true => count += 1,
                    false => {}
                }
            }
        }

        count
    }

    fn update_next(&mut self, walls: bool, random: bool) {
        for y in 0..self.hei {
            for x in 0..self.wid {
                let idx = self.idx_into(x, y);
                let cell = self.cells[idx];
                let n = self.get_alive_neigh(x, y, walls);

                let next_state = match cell.curr {
                    true => {
                        if n < 2 || n > 3 {
                            false
                        } else if random {
                            let rand = self.next_frn();
                            if rand <= 800 { false } else { true }
                        } else {
                            true
                        }
                    }
                    false => {
                        if n == 3 {
                            true
                        } else if random && n == 1 {
                            let rand = self.next_frn();
                            if rand <= 10_000 { true } else { false }
                        } else {
                            false
                        }
                    }
                };
                self.next[idx].past = self.next[idx].curr;
                self.next[idx].curr = next_state;
            }
        }

        self.cells.copy_from_slice(&self.next);
    }

    fn draw_onto(&self, frame: &mut [u8], frame_width: usize) {
        for y in 0..self.hei {
            for x in 0..self.wid {
                let cell = self.cells[self.idx_into(x, y)];

                let color = match cell.curr {
                    true => match cell.past {
                        true => [140, 10, 150, 255],
                        _ => [10, 10, 220, 255],
                    },
                    false => match cell.past {
                        false => [10, 10, 10, 255],
                        _ => [20, 120, 20, 255],
                    },
                };

                // Draw the cell as a square of pixels:
                let px = x * self.cell_scale;
                let py = y * self.cell_scale;

                for dy in 0..self.cell_scale {
                    for dx in 0..self.cell_scale {
                        let ix = px + dx;
                        let iy = py + dy;

                        let idx = 4 * (iy * frame_width + ix);
                        frame[idx..idx + 4].copy_from_slice(&color);
                    }
                }
            }
        }
    }
}
