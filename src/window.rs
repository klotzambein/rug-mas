use speedy2d::color::Color;
use speedy2d::dimen::Vector2;
use speedy2d::shape::Rectangle;
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::Graphics2D;

use crate::simulation::Simulation;

#[derive(Debug, Default)]
pub struct MyWindowHandler {
    size: (f32, f32),
    sim: Option<Simulation>,
}

impl WindowHandler<Simulation> for MyWindowHandler {
    fn on_draw(&mut self, _helper: &mut WindowHelper<Simulation>, graphics: &mut Graphics2D) {
        let sim = if let Some(sim) = self.sim.as_ref() {
            sim
        } else {
            return;
        };
        graphics.clear_screen(Color::WHITE);
        let agents = sim.agents().agents();

        let agent_count = agents.len();
        let agent_x_count = (agent_count as f64).sqrt().round() as usize;
        let agent_y_count =
            agent_count / agent_x_count + (agent_count % agent_x_count != 0) as usize;
        let agent_size = Vector2::new(
            self.size.0 / agent_x_count as f32,
            self.size.1 / agent_y_count as f32,
        );
        let agent_rect = Rectangle::new(agent_size / 20., agent_size * 19. / 20.);

        for x in 0..agent_x_count {
            for y in 0..agent_y_count {
                let idx = x * agent_y_count + y;
                if idx >= agent_count {
                    break;
                }

                let agent = &agents[idx];
                let offset = Vector2::new(agent_size.x * x as f32, agent_size.y * y as f32);
                let rect = agent_rect.with_offset(offset);
                graphics.draw_rectangle(rect.clone(), Color::BLACK);

                let size = rect.size();
                let width = size.x / 5.;
                let val = agent.cash / 60000. * size.y;
                let mut bl = rect.bottom_left();

                graphics.draw_rectangle(
                    Rectangle::from_tuples((bl.x, bl.y - val), (bl.x + width * 0.9, bl.y)),
                    Color::BLUE,
                );

                let colors = [
                    Color::RED,
                    Color::GREEN,
                    Color::CYAN,
                    Color::MAGENTA,
                    Color::YELLOW,
                ];
                for (j, a) in agent.assets.iter().copied().enumerate() {
                    bl.x += width;
                    let val = a as f32 / 1000. * size.y;
                    graphics.draw_rectangle(
                        Rectangle::from_tuples((bl.x, bl.y - val), (bl.x + width * 0.9, bl.y)),
                        colors[j],
                    );
                }
            }
        }
    }

    fn on_user_event(&mut self, helper: &mut WindowHelper<Simulation>, user_event: Simulation) {
        self.sim = Some(user_event);
        helper.request_redraw();
    }

    fn on_resize(
        &mut self,
        helper: &mut WindowHelper<Simulation>,
        size_pixels: speedy2d::dimen::Vector2<u32>,
    ) {
        self.size = (size_pixels.x as f32, size_pixels.y as f32);
        helper.request_redraw();
    }
}
