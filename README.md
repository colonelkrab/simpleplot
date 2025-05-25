# simpleplot
usage:
```
use macroquad::prelude::*;
use simpleplot::Plot;

#[macroquad::main("Plot")]
async fn main() {
    let data: Vec<(f32, f32)> = vec![(0.0,2.0), (1.0,5.0), (2.0, 9.0), (5.0,10.0)] // example data
    let mut plot: Plot = Plot::new(&data, max_y, width_magrin_percent, step_by);
/*
data should be sorted in ascending order of x-axis
max_y is the maximum possible value on y axis (in the example data it is 10.0)
if step by is 1, x-axis linearly increases by 1. if it is 2 axis increases by 2 and so on.
*/
    plot.draw().await;
}
```
Press Up/DOWN arrow Keys to zoom in/out \
Press LEFT/RIGHT arrow keys to shift left and right by 1 unit \
Press A/D keys for speed shifting left and right\
