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

### Local image file

`cargo run src/test_images/image9.png`

[See image](src/test_images/image9.png)

Returns a json with the grid string and marked cell candidates
```
{"grid":"473652090050943726926178345500704913300019007090300004005230079030000002200400030","candidates":[{"cell":{"row":0,"col":6},"values":[1,8]},{"cell":{"row":0,"col":8},"values":[1,8]},{"cell":{"row":1,"col":0},"values":[1,8]},{"cell":{"row":1,"col":2},"values":[1,8]},{"cell":{"row":3,"col":1},"values":[6,8]},{"cell":{"row":3,"col":2},"values":[2,8]},{"cell":{"row":3,"col":4},"values":[2,6,8]},{"cell":{"row":4,"col":1},"values":[4,6,8]},{"cell":{"row":4,"col":2},"values":[2,4,8]},{"cell":{"row":4,"col":3},"values":[5,8]},{"cell":{"row":4,"col":6},"values":[2,6]},{"cell":{"row":4,"col":7},"values":[5,6,8]},{"cell":{"row":5,"col":0},"values":[1,7]},{"cell":{"row":5,"col":2},"values":[1,7]},{"cell":{"row":5,"col":4},"values":[2,6,8]},{"cell":{"row":5,"col":5},"values":[5,6]},{"cell":{"row":5,"col":6},"values":[2,5,6]},{"cell":{"row":5,"col":7},"values":[5,8]},{"cell":{"row":6,"col":0},"values":[6,8]},{"cell":{"row":6,"col":1},"values":[1,4,8]},{"cell":{"row":6,"col":5},"values":[1,6]},{"cell":{"row":6,"col":6},"values":[1,4,8]},{"cell":{"row":7,"col":0},"values":[6,7]},{"cell":{"row":7,"col":2},"values":[4,7,9]},{"cell":{"row":7,"col":3},"values":[5,8]},{"cell":{"row":7,"col":4},"values":[6,8,9]},{"cell":{"row":7,"col":5},"values":[1,5,6,7]},{"cell":{"row":7,"col":6},"values":[1,4]},{"cell":{"row":7,"col":7},"values":[5,6]},{"cell":{"row":8,"col":1},"values":[1,8]},{"cell":{"row":8,"col":2},"values":[7,9]},{"cell":{"row":8,"col":4},"values":[6,9]},{"cell":{"row":8,"col":5},"values":[5,6,7]},{"cell":{"row":8,"col":6},"values":[5,6]},{"cell":{"row":8,"col":8},"values":[1,8]}]}
```

### External image URL

`cargo run https://upload.wikimedia.org/wikipedia/commons/thumb/e/e0/Sudoku_Puzzle_by_L2G-20050714_standardized_layout.svg/1200px-Sudoku_Puzzle_by_L2G-20050714_standardized_layout.svg.png`

### Instal crate globally

Running `cargo install lisudoku-ocr` will globally install the `lisudoku-ocr` binary so you can run

`lisudoku-ocr any/file/path.png`

## Contribute

Join the [discord server](https://discord.gg/SGV8TQVSeT).

## Running tests

`RUST_TEST_THREADS=1 cargo test`
