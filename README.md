# lisudoku_ocr

This is a library for detecting sudoku grids from images.

## Features

* Detects classic sudoku digits and pencilmarks.
* It works on screenshots and probably won't on random photos.

## Installation

It uses OpenCV to detect lines and to split up the puzzle into squares and then Tesseract to detect digits.

You must have `opencv` installed. Follow the steps at https://github.com/twistedfall/opencv-rust/blob/master/INSTALL.md.

Download https://github.com/Shreeshrii/tessdata_shreetest/raw/master/digits.traineddata to `/usr/local/share/tessdata/`.

## Running

Local image file

`cargo run src/test_images/image9.png`

External image URL

`cargo run https://upload.wikimedia.org/wikipedia/commons/thumb/e/e0/Sudoku_Puzzle_by_L2G-20050714_standardized_layout.svg/1200px-Sudoku_Puzzle_by_L2G-20050714_standardized_layout.svg.png`

Running `cargo install lisudoku-ocr` will globally install the `lisudoku-ocr` binary so you can run

`lisudoku-ocr any/file/path.png`

## Contribute

Join the [discord server](https://discord.gg/SGV8TQVSeT).

## Running tests

`RUST_TEST_THREADS=1 cargo test`
