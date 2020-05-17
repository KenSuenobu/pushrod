// Pushrod
// Base Widget Example
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

use pushrod::engine::{EventHandler, Engine};
use pushrod_events::event::Event;
use pushrod_events::event::Event::Pushrod;
use pushrod_widgets::caches::WidgetCache;
use pushrod_widgets::system_widgets::base_widget::BaseWidget;
use pushrod_widgets::widget::Widget;
use pushrod_widgets::properties::{PROPERTY_BORDER_WIDTH, PROPERTY_BORDER_COLOR, PROPERTY_MAIN_COLOR};
use sdl2::pixels::Color;

#[derive(Default)]
pub struct PushrodExample { }

impl EventHandler for PushrodExample {
    fn handle_event(&mut self, current_widget_id: u32, event: Event) {
        eprintln!("Event received: {:?}", event);
    }

    fn build_layout(&mut self, cache: &mut WidgetCache) {
        eprintln!("Layout called.");

        let mut base_widget = BaseWidget::default();

        &base_widget.properties().set_origin(50, 50);
        &base_widget.properties().set_bounds(540, 380);
        &base_widget.properties().set_value(PROPERTY_BORDER_WIDTH, 1);
        &base_widget.properties().set_color(PROPERTY_BORDER_COLOR, Color::RGBA(0, 0, 0, 255));
        &base_widget.properties().set_color(PROPERTY_MAIN_COLOR, Color::RGBA(0, 255, 0, 255));

        cache.add(Box::new(base_widget), String::from("widget1"), 0);
    }
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("pushrod example", 640, 480)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    let mut engine = Engine::new(Box::new(PushrodExample::default()), &window);

    engine.run(sdl_context, window);
}