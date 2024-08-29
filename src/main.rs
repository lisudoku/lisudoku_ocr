use std::{env, process};
use lisudoku_solver::types::SudokuConstraints;
use sudoku_image_parser::CellCandidates;
use serde::{Deserialize, Serialize};

mod sudoku_image_parser;
mod tesseract;
mod line_detection;

use crate::sudoku_image_parser::parse_image_at_path;

#[derive(Serialize, Deserialize)]
struct OcrResultJson {
  grid: String,
  candidates: Vec<CellCandidates>,
}

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() != 2 {
    eprintln!("Usage: lisudoku_ocr path");
    process::exit(1);
  }

  let image_path = &args[1];

  eprintln!("Parsing image at {}", image_path);

  let ocr_result = parse_image_at_path(image_path).expect("Unable to parse sudoku grid from image");

  let ocr_result_json = OcrResultJson {
    grid: SudokuConstraints::new(9, ocr_result.given_digits).to_import_string(),
    candidates: ocr_result.candidates,
  };

  println!("{}", serde_json::to_string(&ocr_result_json).unwrap());
}
