use std::process;
use lisudoku_solver::types::SudokuConstraints;
use lisudoku_ocr::parse_image_at_url;
use serde_json::json;

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

  let ocr_result = parse_image_at_url(&image_url, true).await.expect("Unable to parse sudoku grid from image");

  let ocr_result_json = json!({
    "grid": SudokuConstraints::new(9, ocr_result.given_digits).to_import_string(),
    "candidates": ocr_result.candidates,
  });

  println!("{}", &ocr_result_json);
}
