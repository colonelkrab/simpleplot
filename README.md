# simpleplot
usage:
```toml
# cargo.toml
[dependencies]
simpleplot = {git = "https://github.com/colonelkrab/simpleplot.git"}
```
```rust
// main.rs
use crate::rand::gen_range;
use macroquad::prelude::*;
use simpleplot::Plot;

#[macroquad::main("Demo Plot")]
async fn main() {
    const MAX_Y: f32 = 10.0;
    const WIDTH_MARGIN_PERCENT: f32 = 1.0;
    const STEP_BY: usize = 5;

    let mut data: Vec<(f32, f32)> = Vec::new();
    for i in 0..100 {
        data.push(((i + 1) as f32, gen_range(2.0, MAX_Y)));
    }

    let mut plot = Plot::new(&data, MAX_Y, WIDTH_MARGIN_PERCENT, STEP_BY);
    plot.draw().await;
}
```
Press Up/DOWN arrow keys to zoom in/out \
Press LEFT/RIGHT arrow keys to shift left and right by 1 unit \
Press WASD keys for faster actions 
