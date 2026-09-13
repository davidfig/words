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

    // On OK, returns whether we have hit the end of the pages
    #[wasm_bindgen]
    pub fn player_text(self: &mut Board, text: String, page: usize) -> Result<bool, JsError> {
        let place = self.width * page;
        if place >= text.len() {
            Err(JsError::new("out of text for page"))
        } else {
            for i in 0..self.width {
                match text.chars().nth(i + place) {
                    Some(ch) => self.put(i, self.height - 1, ch)?,
                    None => self.put(i, self.height - 1, ' ')?,
                };
            }
            Ok(place + self.width < text.len())
        }
    }
}

#[test]
fn test_board() {
    let mut board = Board::new(2, 2);
    assert!(board.put(1, 1, 'x').is_ok());
    assert_eq!(board.print(), "&nbsp;&nbsp;&nbsp;x".to_string());
}

#[test]
fn test_player_text() {
    let mut board = Board::new(2, 2);
    assert_eq!(board.player_text("Try".into(), 0).unwrap(), true);
    assert_eq!(board.print(), "&nbsp;&nbsp;Tr");
    assert_eq!(board.player_text("Try".into(), 1).unwrap(), false);
    assert_eq!(board.print(), "&nbsp;&nbsp;y&nbsp;");
}
