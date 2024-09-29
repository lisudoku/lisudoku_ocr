use std::process;
use lisudoku_solver::types::SudokuConstraints;
use lisudoku_ocr::{parse_image_at_url, CellCandidates};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct OcrResultJson {
  grid: String,
  candidates: Vec<CellCandidates>,
}

#[tokio::main]
async fn main() {
  let image_url = match std::env::args().nth(1) {
    Some(url) => url,
    None => {
      eprintln!("Usage: lisudoku_ocr path");
      process::exit(1);
    }
};

  eprintln!("Parsing image at {}", image_url);

  let ocr_result = parse_image_at_url(&image_url).await.expect("Unable to parse sudoku grid from image");
  
  let ocr_result_json = OcrResultJson {
    grid: SudokuConstraints::new(9, ocr_result.given_digits).to_import_string(),
    candidates: ocr_result.candidates,
  };

  println!("{}", serde_json::to_string(&ocr_result_json).unwrap());
}
