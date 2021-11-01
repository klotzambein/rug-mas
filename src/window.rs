use std::error::Error;
use std::f32::consts::PI;
use std::sync::Arc;

use plotters::prelude::IntoDrawingArea;
use plotters_backend::{BackendCoord, DrawingBackend, DrawingErrorKind};
use speedy2d::color::Color;
use speedy2d::dimen::Vector2;
use speedy2d::font::{Font, TextAlignment, TextLayout, TextOptions};
use speedy2d::shape::Rectangle;
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::Graphics2D;

use crate::report::Reporter;
use crate::simulation::Simulation;

#[derive(Debug)]
pub struct Data {
    pub sim: Simulation,
    pub report: Reporter,
}

#[derive(Debug)]
pub struct MyWindowHandler {
    size: Vector2<f32>,
    data: Option<Data>,
}

impl WindowHandler<Data> for MyWindowHandler {
    fn on_draw(&mut self, _helper: &mut WindowHelper<Data>, graphics: &mut Graphics2D) {
        let data = if let Some(data) = self.data.as_ref() {
            data
        } else {
            return;
        };

        if self.size.magnitude_squared() < 2.0 {
            return;
        }

        let sim = &data.sim;

        graphics.clear_screen(Color::WHITE);
        let window = Rect::from_size(self.size);
        let (content_agent, content_plots) = window.inset(3.0).split_horizontal_at(1.0);

        let agents = sim.agents().agents();
        let agent_count = agents.len();

        let agent_rects = content_agent.grid_even(agent_count);

        for (agent, rect) in agents.iter().zip(agent_rects) {
            let rect = rect.inset_percentage(5.0);
            graphics.draw_rectangle(rect.speedy2d(), Color::BLACK);
            let rect = rect.inset_percentage(5.0);

            let count = 2 + agent.assets.len();
            let mut rects = rect.split_vertical(count);

            let val = agent.cash.log10() / 5.0; // up to 5 digits
            graphics.draw_rectangle(
                rects.next().unwrap().bar_scale_up(val).speedy2d(),
                Color::GREEN,
            );

            let colors = [
                Color::RED,
                Color::CYAN,
                Color::MAGENTA,
                Color::BLUE,
                Color::YELLOW,
            ];
            for (j, a) in agent.assets.iter().copied().enumerate() {
                let val = (a as f32).log10() / 4.0; // up to 4 digits
                graphics.draw_rectangle(
                    rects.next().unwrap().bar_scale_up(val).speedy2d(),
                    colors[j],
                );
            }
            for (j, (a, r)) in agent
                .state
                .iter()
                .copied()
                .zip(rects.next().unwrap().split_horizontal(agent.state.len()))
                .enumerate()
            {
                let color = colors[j];
                let color = Color::from_rgb(color.r() * a, color.g() * a, color.b() * a);
                graphics.draw_rectangle(r.inset_percentage(15.0).speedy2d(), color);
            }
        }

        // let backend = PlottersBackendSpeedy {
        //     rect: content_plots,
        //     g: graphics,
        // };
        // data.report.render_chart(backend.into_drawing_area());
    }

    fn on_user_event(&mut self, helper: &mut WindowHelper<Data>, user_event: Data) {
        self.data = Some(user_event);
        helper.request_redraw();
    }

    fn on_resize(&mut self, helper: &mut WindowHelper<Data>, size_pixels: Vector2<u32>) {
        self.size = size_pixels.into_f32();
        helper.request_redraw();
    }
}

impl Default for MyWindowHandler {
    fn default() -> Self {
        MyWindowHandler {
            size: Vector2::ZERO,
            data: None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    top_left: Vector2<f32>,
    bottom_right: Vector2<f32>,
}

impl Rect {
    pub fn from_size(size: Vector2<f32>) -> Rect {
        Rect {
            top_left: Vector2::ZERO,
            bottom_right: size,
        }
    }

    pub fn width(self) -> f32 {
        self.bottom_right.x - self.top_left.x
    }

    pub fn split_vertical(self, count: usize) -> impl Iterator<Item = Rect> {
        let width = self.width() / count as f32;
        (0..count).map(move |i| {
            let i = i as f32;
            let mut rect = self;
            rect.top_left.x += width * i;
            rect.bottom_right.x = rect.top_left.x + width;
            rect
        })
    }

    pub fn split_vertical_at(self, val: f32) -> (Rect, Rect) {
        let split_point = (self.width() * val) + self.top_left.x;

        let mut a = self;
        a.bottom_right.x = split_point;

        let mut b = self;
        b.top_left.x = split_point;

        (a, b)
    }

    pub fn height(self) -> f32 {
        self.bottom_right.y - self.top_left.y
    }

    pub fn split_horizontal(self, count: usize) -> impl Iterator<Item = Rect> {
        let height = self.height() / count as f32;
        (0..count).map(move |i| {
            let i = i as f32;
            let mut rect = self;
            rect.top_left.y += height * i;
            rect.bottom_right.y = rect.top_left.y + height;
            rect
        })
    }

    pub fn split_horizontal_at(self, val: f32) -> (Rect, Rect) {
        let split_point = (self.height() * val) + self.top_left.y;

        let mut a = self;
        a.bottom_right.y = split_point;

        let mut b = self;
        b.top_left.y = split_point;

        (a, b)
    }

    pub fn grid_even(self, count: usize) -> impl Iterator<Item = Rect> {
        let area = self.width() * self.height();
        let area_cell = area / count as f32;
        let y_count = (self.width() / area_cell.sqrt()).round() as usize;
        let x_count = count / y_count + (count % y_count != 0) as usize;

        self.split_horizontal(x_count)
            .flat_map(move |r| r.split_vertical(y_count))
            .take(count)
    }

    #[must_use]
    pub fn inset(self, amount: f32) -> Rect {
        let width = self.width();
        let amount_x = if width < amount * 2.0 {
            width / 2.0
        } else {
            amount
        };

        let height = self.height();
        let amount_y = if height < amount * 2.0 {
            height / 2.0
        } else {
            amount
        };

        let mut rect = self;
        rect.top_left.x += amount_x;
        rect.top_left.y += amount_y;
        rect.bottom_right.x -= amount_x;
        rect.bottom_right.y -= amount_y;

        rect
    }

    #[must_use]
    pub fn inset_percentage(self, percentage: f32) -> Rect {
        let abs = (self.width() + self.height()) * percentage / 200.0;
        self.inset(abs)
    }

    #[must_use]
    pub fn speedy2d(self) -> Rectangle {
        Rectangle::new(self.top_left, self.bottom_right)
    }

    #[must_use]
    pub fn bar_scale_up(self, value: f32) -> Rect {
        let mut rect = self;
        rect.top_left.y += self.height() * (1. - value);
        rect
    }
}

pub struct PlottersBackendSpeedy<'a> {
    rect: Rect,
    g: &'a mut Graphics2D,
}

impl DrawingBackend for PlottersBackendSpeedy<'_> {
    type ErrorType = Arc<dyn Error + 'static + Send + Sync>;

    fn get_size(&self) -> (u32, u32) {
        (self.rect.width() as u32, self.rect.height() as u32)
    }

    fn ensure_prepared(&mut self) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        // nothing to do
        Ok(())
    }

    fn present(&mut self) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        // nothing to do
        Ok(())
    }

    fn draw_pixel(
        &mut self,
        _point: BackendCoord,
        _color: plotters_backend::BackendColor,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        unimplemented!()
    }

    fn draw_line<S: plotters_backend::BackendStyle>(
        &mut self,
        from: BackendCoord,
        to: BackendCoord,
        style: &S,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let color = style.color();
        let color = Color::from_rgba(
            color.rgb.0 as f32 / 255.,
            color.rgb.1 as f32 / 255.,
            color.rgb.2 as f32 / 255.,
            color.alpha as f32,
        );
        self.g.draw_line(
            Vector2::from(from).into_f32() + self.rect.top_left,
            Vector2::from(to).into_f32() + self.rect.top_left,
            style.stroke_width() as f32,
            color,
        );
        Ok(())
    }

    fn draw_rect<S: plotters_backend::BackendStyle>(
        &mut self,
        upper_left: BackendCoord,
        bottom_right: BackendCoord,
        style: &S,
        fill: bool,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let color = style.color();
        let color = Color::from_rgba(
            color.rgb.0 as f32 / 255.,
            color.rgb.1 as f32 / 255.,
            color.rgb.2 as f32 / 255.,
            color.alpha as f32,
        );
        if fill {
            self.g.draw_rectangle(
                Rectangle::from_tuples(upper_left, bottom_right)
                    .into_f32()
                    .with_offset(self.rect.top_left),
                color,
            );
        } else {
            self.draw_line(upper_left, (bottom_right.0, upper_left.1), style)?;
            self.draw_line((bottom_right.0, upper_left.1), bottom_right, style)?;
            self.draw_line(bottom_right, (upper_left.0, bottom_right.1), style)?;
            self.draw_line((upper_left.0, bottom_right.1), upper_left, style)?;
        }
        Ok(())
    }

    fn draw_path<S: plotters_backend::BackendStyle, I: IntoIterator<Item = BackendCoord>>(
        &mut self,
        path: I,
        style: &S,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        if style.color().alpha == 0.0 {
            return Ok(());
        }

        if style.stroke_width() == 1 {
            let mut begin: Option<BackendCoord> = None;
            for end in path.into_iter() {
                if let Some(begin) = begin {
                    let result = self.draw_line(begin, end, style);
                    if result.is_err() {
                        return result;
                    }
                }
                begin = Some(end);
            }
        } else {
            let p: Vec<_> = path.into_iter().collect();
            let v = plotters_backend::rasterizer::polygonize(&p[..], style.stroke_width());
            return self.fill_polygon(v, &style.color());
        }
        Ok(())
    }

    fn draw_circle<S: plotters_backend::BackendStyle>(
        &mut self,
        center: BackendCoord,
        radius: u32,
        style: &S,
        fill: bool,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let radius = radius as f32;
        let points = (0..201).map(|x| x as f32 / 100.0 * PI).map(|x| {
            (
                (x.sin() * radius) as i32 + center.0,
                (x.cos() * radius) as i32 + center.1,
            )
        });
        if fill {
            self.fill_polygon(points, style)?;
        } else {
            self.draw_path(points, style)?;
        }
        Ok(())
    }

    fn fill_polygon<S: plotters_backend::BackendStyle, I: IntoIterator<Item = BackendCoord>>(
        &mut self,
        vert: I,
        style: &S,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let color = style.color();
        let color = Color::from_rgba(
            color.rgb.0 as f32 / 255.,
            color.rgb.1 as f32 / 255.,
            color.rgb.2 as f32 / 255.,
            color.alpha as f32,
        );

        let vert_data = vert
            .into_iter()
            .flat_map(|(x, y)| [x as f32, y as f32])
            .collect();
        let tri_idx = earcutr::earcut(&vert_data, &Vec::new(), 2);

        for i in 0..tri_idx.len() / 3 {
            let a = tri_idx[i] * 2;
            let b = tri_idx[i + 1] * 2;
            let c = tri_idx[i + 2] * 2;

            let a = Vector2::new(vert_data[a], vert_data[a + 1]) + self.rect.top_left;
            let b = Vector2::new(vert_data[b], vert_data[b + 1]) + self.rect.top_left;
            let c = Vector2::new(vert_data[c], vert_data[c + 1]) + self.rect.top_left;
            self.g.draw_triangle([a, b, c], color)
        }

        Ok(())
    }

    fn draw_text<TStyle: plotters_backend::BackendTextStyle>(
        &mut self,
        text: &str,
        style: &TStyle,
        pos: BackendCoord,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        return Ok(());
        let font = Font::new(include_bytes!("../Mechanical.otf")).unwrap();
        let color = style.color();
        let color = Color::from_rgba(
            color.rgb.0 as f32 / 255.,
            color.rgb.1 as f32 / 255.,
            color.rgb.2 as f32 / 255.,
            color.alpha as f32,
        );
        let text = font.layout_text(
            text,
            style.size() as f32,
            TextOptions::new().with_wrap_to_width(
                f32::MAX,
                match style.anchor().h_pos {
                    plotters_backend::text_anchor::HPos::Left => TextAlignment::Left,
                    plotters_backend::text_anchor::HPos::Right => TextAlignment::Right,
                    plotters_backend::text_anchor::HPos::Center => TextAlignment::Center,
                },
            ),
        );
        self.g.draw_text(
            Vector2::from(pos).into_f32() + self.rect.top_left,
            color,
            &text,
        );
        Ok(())
    }

    fn estimate_text_size<TStyle: plotters_backend::BackendTextStyle>(
        &self,
        text: &str,
        style: &TStyle,
    ) -> Result<(u32, u32), DrawingErrorKind<Self::ErrorType>> {
        let layout = style
            .layout_box(text)
            .map_err(|e| DrawingErrorKind::FontError(Box::new(e)))?;
        Ok((
            ((layout.1).0 - (layout.0).0) as u32,
            ((layout.1).1 - (layout.0).1) as u32,
        ))
    }

    fn blit_bitmap<'a>(
        &mut self,
        _pos: BackendCoord,
        (_iw, _ih): (u32, u32),
        _src: &'a [u8],
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        unimplemented!()
    }
}
