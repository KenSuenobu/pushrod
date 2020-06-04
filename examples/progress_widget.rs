// Pushrod
// Progress Widget Example
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate pushrod;
extern crate sdl2;

use pushrod::engine::{Engine, EventHandler};
use pushrod_widgets::caches::WidgetCache;
use pushrod_widgets::event::Event::Pushrod;
use pushrod_widgets::event::{Event, PushrodEvent};
use pushrod_widgets::primitives::init_application;
use pushrod_widgets::properties::{
    PROPERTY_MAIN_COLOR, PROPERTY_PROGRESS, PROPERTY_PROGRESS_COLOR,
};
use pushrod_widgets::system_widgets::progress_widget::ProgressWidget;
use pushrod_widgets::widget::Widget;
use sdl2::pixels::Color;

#[derive(Default)]
pub struct PushrodExample {}

impl EventHandler for PushrodExample {
    fn handle_event(&mut self, event: Event, _cache: &mut WidgetCache) {
        match event {
            Pushrod(pushrod_event) => match pushrod_event {
                PushrodEvent::DrawFrame { .. } => {}
                x => eprintln!("Pushrod unhandled event: {:?}", x),
            },
            Event::SDL2(x) => {
                eprintln!("SDL2 unhandled event: {:?}", x);
            }
        }
    }

    fn build_layout(&mut self, cache: &mut WidgetCache) {
        let mut progress1 = ProgressWidget::default();

        progress1
            .properties()
            .set_origin(20, 20)
            .set_bounds(360, 40)
            .set_color(PROPERTY_MAIN_COLOR, Color::WHITE)
            .set_color(PROPERTY_PROGRESS_COLOR, Color::RED)
            .set_value(PROPERTY_PROGRESS, 25);

        cache.add(Box::new(progress1), String::from("progress1"), 0);

        let mut progress2 = ProgressWidget::default();

        progress2
            .properties()
            .set_origin(20, 70)
            .set_bounds(360, 40)
            .set_color(PROPERTY_MAIN_COLOR, Color::WHITE)
            .set_color(PROPERTY_PROGRESS_COLOR, Color::GREEN)
            .set_value(PROPERTY_PROGRESS, 50);

        cache.add(Box::new(progress2), String::from("progress2"), 0);

        let mut progress3 = ProgressWidget::default();

        progress3
            .properties()
            .set_origin(20, 120)
            .set_bounds(360, 40)
            .set_color(PROPERTY_MAIN_COLOR, Color::WHITE)
            .set_color(PROPERTY_PROGRESS_COLOR, Color::BLUE)
            .set_value(PROPERTY_PROGRESS, 75);

        cache.add(Box::new(progress3), String::from("progress3"), 0);

        let mut progress4 = ProgressWidget::default();

        progress4
            .properties()
            .set_origin(20, 170)
            .set_bounds(360, 40)
            .set_color(PROPERTY_MAIN_COLOR, Color::WHITE)
            .set_color(PROPERTY_PROGRESS_COLOR, Color::CYAN)
            .set_value(PROPERTY_PROGRESS, 100);

        cache.add(Box::new(progress4), String::from("progress4"), 0);
    }
}

pub fn main() {
    let (sdl_context, _, window) = init_application("pushrod example", 400, 230);
    let mut engine = Engine::new(Box::new(PushrodExample::default()), &window);

    engine.run(sdl_context, window);
}
