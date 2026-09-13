use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Board {
    #[wasm_bindgen]
    pub width: usize,

    #[wasm_bindgen]
    pub height: usize,

    buffer: Vec<char>,
}

#[wasm_bindgen]
impl Board {
    #[wasm_bindgen(constructor)]
    pub fn new(width: usize, height: usize) -> Board {
        Board {
            width: width,
            height: height,
            buffer: vec![' '; width * height],
        }
    }

    #[wasm_bindgen]
    pub fn put(self: &mut Board, x: usize, y: usize, ch: char) -> Result<(), JsError> {
        if x >= self.width {
            Err(JsError::new("put() out of x bounds"))
        } else if y >= self.height {
            Err(JsError::new("put() out of y bounds"))
        } else {
            self.buffer[x + y * self.width] = ch;
            Ok(())
        }
    }

    #[wasm_bindgen]
    pub fn get(self: &Board, x: usize, y: usize) -> Result<String, JsError> {
        if x >= self.width {
            Err(JsError::new("put() out of x bounds"))
        } else if y >= self.height {
            Err(JsError::new("put() out of y bounds"))
        } else {
            Ok(self.buffer[x + y * self.width].to_string())
        }
    }

    #[wasm_bindgen]
    pub fn print(self: &Board) -> String {
        self.buffer
            .iter()
            .map(|ch| {
                if *ch == ' ' {
                    "&nbsp;".to_string()
                } else {
                    ch.to_string()
                }
            })
            .collect()
    }
}

#[test]
fn test_board() {
    let mut board = Board::new(2, 2);
    assert!(board.put(1, 1, 'x').is_ok());
    assert_eq!(board.print(), "&nbsp;&nbsp;&nbsp;x".to_string());
}
