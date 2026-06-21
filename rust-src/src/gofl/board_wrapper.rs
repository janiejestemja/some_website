use crate::gofl::board::{Board, BoardTrait};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct BoardWrapper {
    board: Board,
}

#[wasm_bindgen]
impl BoardWrapper {
    #[wasm_bindgen(constructor)]
    pub fn new(
        wid: i64,
        hei: i64,
        frn_seed: u64,
    ) -> Self {
        let board = Board::new(
            wid as usize, 
            hei as usize, 
            2u8 as usize,
            frn_seed,
        );

        Self {
            board,
        }
    }

    #[wasm_bindgen]
    pub fn add(&self, left: i64, right: i64) -> i64 {
        left + right
    }

    #[wasm_bindgen]
    pub fn count_alive(&self) -> usize {
        self
            .board
            .cells
            .iter()
            .filter(|c| c.curr)
            .count()
    }

    #[wasm_bindgen]
    pub fn reset_board(&mut self) {
        for cell in self.board.cells.iter_mut() {
            cell.past = false;
            cell.curr = false;
        }
    }

    #[wasm_bindgen]
    pub fn next_generation(&mut self) {
        self.board.update_next(true, true);
    }

    #[wasm_bindgen]
    pub fn index_into_board(&self, x: usize, y: usize) -> u8 {
        let curr = self.board.cells[self.board.idx_into(x, y)].curr;
        let past = self.board.cells[self.board.idx_into(x, y)].past;
        if past { return 2u8 } else if curr { return 1u8 };
        0u8
    }

    #[wasm_bindgen]
    pub fn set_cross(&mut self) {
        self.board.set_cross();
    }

    #[wasm_bindgen]
    pub fn set_diagonals(&mut self) {
        self.board.set_diagonals();
    }
}
