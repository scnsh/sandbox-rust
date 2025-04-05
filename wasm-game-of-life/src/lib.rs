mod utils;

use std::fmt;

use wasm_bindgen::prelude::*;

extern crate js_sys;

extern crate fixedbitset;
use fixedbitset::FixedBitSet;

extern crate web_sys;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
#[repr(u8)] // 各セルを1バイトで表す
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: FixedBitSet, // width * height のセルを格納する
}

impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;

        // self.height-1, self.width-1とすることで、負の数にならずにテーブルの端のセルをチェックできる
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }
}

/// 公開メソッド、JavaScriptにエクスポートする
#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                log!(
                    "cell: [{}, {}] is initially {:?} and has {} live neighbors.",
                    row,
                    col,
                    cell,
                    live_neighbors
                );

                next.set(
                    idx,
                    match (cell, live_neighbors) {
                        // Rule 1: 過疎(近傍の生きたセルが2つ以下のときに死亡)
                        (true, n) if n < 2 => false,
                        // Rule 2: 生存(近傍の生きたセルが2つか3つのときに生存)
                        (true, 2) | (true, 3) => true,
                        // Rule 3: 過密(近傍の生きたセルが4つ以上のときに死亡)
                        (true, n) if n > 3 => false,
                        // Rule 4: 誕生(死んでいるセルの近傍にちょうど3つの生きたセルがあるときに誕生)
                        (false, 3) => true,
                        // Rule 5: その他(その他の場合は変化なし)
                        (otherwise, _) => otherwise,
                    },
                );
            }
        }

        log!("     it becomes: {:?}", next);

        self.cells = next;
    }

    pub fn new() -> Self {
        utils::set_panic_hook();

        let width = 64;
        let height = 64;
        let size = (width * height) as usize;
        let mut cells = FixedBitSet::with_capacity(size);

        for i in 0..size {
            if js_sys::Math::random() < 0.5 {
                cells.set(i, true);
            } else {
                cells.set(i, false);
            }
        }

        Universe {
            width: width,
            height: height,
            cells,
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const u32 {
        self.cells.as_slice().as_ptr() as *const u32
    }

    /// universeの横幅をセットする
    /// dead状態の全てのセルをリセットする
    pub fn set_width(&mut self, width: u32) {
        let size = (width * self.height) as usize;
        let mut cells = FixedBitSet::with_capacity(size);
        for i in 0..size {
            cells.set(i, false);
        }
        self.width = width;
        self.cells = cells;
    }

    /// universeの高さをセットする
    /// dead状態の全てのセルをリセットする
    pub fn set_height(&mut self, height: u32) {
        let size = (self.width * height) as usize;
        let mut cells = FixedBitSet::with_capacity(size);
        for i in 0..size {
            cells.set(i, false);
        }
        self.height = height;
        self.cells = cells;
    }
}

impl Universe {
    /// universe全体から死んでいる/生きているセルを取得する
    pub fn get_cells(&self) -> &FixedBitSet {
        &self.cells
    }

    /// 縦と横をセルの配列として走査して生きているセルに変換する
    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells {
            let idx = self.get_index(*row, *col);
            self.cells.set(idx, true);
        }
    }
}

/// Display traitを実装することで、to_string()でUniverseを文字列として表示できるようになる
impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let symbol = if self.cells[idx] { '■' } else { '□' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}
