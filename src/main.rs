use std::process;
use lisudoku_solver::types::SudokuConstraints;
use lisudoku_ocr::{parse_image_at_path, parse_image_from_bytes, CellCandidates, OcrResult};
use serde::{Deserialize, Serialize};
use warp::hyper::body::Bytes;
use url::Url;

#[derive(Serialize, Deserialize)]
struct OcrResultJson {
  grid: String,
  candidates: Vec<CellCandidates>,
}

async fn get_external_image_data(image_url: &str) -> Result<Bytes, Box<dyn std::error::Error>> {
  eprintln!("Fetching image data at {}", image_url);

  let image_data = reqwest::get(image_url)
    .await?
    .bytes()
    .await?;

  Ok(image_data)
}

async fn parse_image(image_url: &str) -> Result<OcrResult, Box<dyn std::error::Error>> {
  if Url::parse(&image_url).is_ok() {
    eprintln!("Detected parameter as external URL");
    let image_data = get_external_image_data(&image_url).await?;
    let ocr_result = parse_image_from_bytes(&image_data)?;
    return Ok(ocr_result);
  }

  eprintln!("Detected parameter as local file path");
  let ocr_result = parse_image_at_path(image_url)?;
  Ok(ocr_result)
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

  let ocr_result = parse_image(&image_url).await.expect("Unable to parse sudoku grid from image");
  
  let ocr_result_json = OcrResultJson {
    grid: SudokuConstraints::new(9, ocr_result.given_digits).to_import_string(),
    candidates: ocr_result.candidates,
  };

  println!("{}", serde_json::to_string(&ocr_result_json).unwrap());
}
