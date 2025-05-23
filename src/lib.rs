use macroquad::prelude::*;
const RES: f32 = 2.0;
const RENDER_WIDTH: f32 = 1920.0 * RES;
const RENDER_HEIGHT: f32 = 1080.0 * RES;
const ASPECT_RATIO: f32 = 16.0 / 9.0;
pub struct Plot<'a> {
    data: &'a Vec<(f32, f32)>,
    ceil_x: u32,
    ceil_y: u32,
    scale_x: f32,
    scale_y: f32,
    zoom: f32,
    offset_x: i32,
    view_width: f32,
    view_height: f32,
    width_margin_percent: f32,
    camera: Camera2D,
    step_by: usize,
    render_target: RenderTarget,
}

impl Plot<'_> {
    pub fn new<'a>(
        data: &'a Vec<(f32, f32)>,
        max_y: f32,
        width_margin_percent: f32,
        step_by: usize,
    ) -> Plot<'a> {
        let render_target = render_target(RENDER_WIDTH as u32, RENDER_HEIGHT as u32);
        render_target.texture.set_filter(FilterMode::Linear);

        let mut camera = Camera2D::from_display_rect(Rect {
            x: 0.0,
            y: 0.0,
            w: RENDER_WIDTH,
            h: RENDER_HEIGHT,
        });
        camera.render_target = Some(render_target.clone());

        let ceil_y = max_y.ceil() as u32 + 1;
        let ceil_x = data.last().unwrap().0.ceil() as u32 + 1;
        let scale_x = RENDER_WIDTH / ceil_x as f32;
        let scale_y = RENDER_HEIGHT / ceil_y as f32;
        let view_width = (100.0 - width_margin_percent) / 100.0 * screen_width();
        let view_height = view_width * 1.0 / ASPECT_RATIO;
        Plot {
            data,
            ceil_x,
            ceil_y,
            scale_x,
            scale_y,
            zoom: 1.0,
            offset_x: 0,
            view_width,
            view_height,
            width_margin_percent,
            camera,
            step_by,
            render_target,
        }
    }
    fn update_view_size(&mut self) {
        self.view_width = (100.0 - 2.0 * self.width_margin_percent) / 100.0 * screen_width();
        self.view_height = self.view_width * 1.0 / ASPECT_RATIO;
    }
    fn read_input(&mut self) {
        if is_key_pressed(KeyCode::Up) {
            self.zoom += 0.25;
            self.scale_x = RENDER_WIDTH / self.ceil_x as f32 * self.zoom;
        }

        if is_key_pressed(KeyCode::Down) {
            self.zoom -= 0.25;

            self.scale_x = RENDER_WIDTH / self.ceil_x as f32 * self.zoom;
        }

        if is_key_down(KeyCode::Left) {
            self.offset_x -= 1;
        }

        if is_key_down(KeyCode::Right) {
            self.offset_x += 1;
        }
    }
    pub fn draw(&mut self) {
        self.update_view_size();
        self.read_input();

        set_camera(&self.camera);
        clear_background(WHITE);

        for i in (0..self.ceil_x).step_by(self.step_by) {
            let x = self.scale_x * (i as i32 - self.offset_x) as f32;
            draw_line(x, 0.0, x, RENDER_HEIGHT, 1.0 * RES, GRAY);
            draw_text(&i.to_string(), x, RENDER_HEIGHT, 40.0 * RES, BLACK);
        }

        for i in 0..self.ceil_y {
            let y = self.scale_y * i as f32;
            draw_line(0.0, y, RENDER_WIDTH, y, 1.0 * RES, GRAY);

            draw_text(&(self.ceil_y - i).to_string(), 0.0, y, 40.0 * RES, BLACK);
        }

        let mut prev = vec2(0.0, RENDER_HEIGHT);
        for (x, y) in self.data {
            let plot_x = (x - self.offset_x as f32) * self.scale_x;
            let plot_y = RENDER_HEIGHT - y * self.scale_y;
            draw_line(prev.x, prev.y, plot_x, plot_y, 2.0 * RES, LIME);
            prev = vec2(plot_x, plot_y);
        }

        set_default_camera();
        draw_texture_ex(
            &self.render_target.texture,
            5.0 / 100.0 * screen_width(),
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(self.view_width, self.view_height)),
                source: Some(Rect {
                    x: 0.0,
                    y: 0.0,
                    w: RENDER_WIDTH,
                    h: RENDER_HEIGHT,
                }),
                flip_y: true,
                ..Default::default()
            },
        );
    }
}
