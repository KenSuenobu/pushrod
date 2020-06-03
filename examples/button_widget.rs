// Pushrod
// Button Widget Example
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
use pushrod_widgets::properties::{PROPERTY_BORDER_COLOR, PROPERTY_BORDER_WIDTH, PROPERTY_FONT_NAME, PROPERTY_FONT_SIZE, PROPERTY_FONT_STYLE, PROPERTY_MAIN_COLOR, PROPERTY_PROGRESS, PROPERTY_PROGRESS_COLOR, PROPERTY_TEXT, PROPERTY_TEXT_JUSTIFICATION, TEXT_JUSTIFY_CENTER, PROPERTY_TOGGLED};
use pushrod_widgets::system_widgets::button_widget::ButtonWidget;
use pushrod_widgets::system_widgets::progress_widget::ProgressWidget;
use pushrod_widgets::widget::Widget;
use sdl2::pixels::Color;
use pushrod_widgets::system_widgets::toggle_button_widget::ToggleButtonWidget;

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
        let mut button1 = ButtonWidget::default();

        button1
            .properties()
            .set_origin(20, 20)
            .set_bounds(360, 60)
            .set_color(PROPERTY_MAIN_COLOR, Color::WHITE)
            .set(
                PROPERTY_FONT_NAME,
                String::from("assets/OpenSans-Regular.ttf"),
            )
            .set_value(PROPERTY_FONT_SIZE, 18)
            .set_value(PROPERTY_FONT_STYLE, sdl2::ttf::FontStyle::NORMAL.bits())
            .set_value(PROPERTY_TEXT_JUSTIFICATION, TEXT_JUSTIFY_CENTER)
            .set_value(PROPERTY_BORDER_WIDTH, 2)
            .set_color(PROPERTY_BORDER_COLOR, Color::BLACK)
            .set(PROPERTY_TEXT, String::from("Click Me!"));

        cache.add(Box::new(button1), String::from("button1"), 0);

        let mut button2 = ToggleButtonWidget::default();

        button2
            .properties()
            .set_origin(20, 100)
            .set_bounds(170, 60)
            .set_color(PROPERTY_MAIN_COLOR, Color::WHITE)
            .set(
                PROPERTY_FONT_NAME,
                String::from("assets/OpenSans-Regular.ttf"),
            )
            .set_value(PROPERTY_FONT_SIZE, 18)
            .set_value(PROPERTY_FONT_STYLE, sdl2::ttf::FontStyle::NORMAL.bits())
            .set_value(PROPERTY_TEXT_JUSTIFICATION, TEXT_JUSTIFY_CENTER)
            .set_value(PROPERTY_BORDER_WIDTH, 2)
            .set_color(PROPERTY_BORDER_COLOR, Color::BLACK)
            .set(PROPERTY_TEXT, String::from("Toggle Me!"));

        cache.add(Box::new(button2), String::from("button2"), 0);

        let mut button3 = ToggleButtonWidget::default();

        button3
            .properties()
            .set_origin(210, 100)
            .set_bounds(170, 60)
            .set_color(PROPERTY_MAIN_COLOR, Color::WHITE)
            .set(
                PROPERTY_FONT_NAME,
                String::from("assets/OpenSans-Regular.ttf"),
            )
            .set_value(PROPERTY_FONT_SIZE, 18)
            .set_value(PROPERTY_FONT_STYLE, sdl2::ttf::FontStyle::NORMAL.bits())
            .set_value(PROPERTY_TEXT_JUSTIFICATION, TEXT_JUSTIFY_CENTER)
            .set_value(PROPERTY_BORDER_WIDTH, 2)
            .set_color(PROPERTY_BORDER_COLOR, Color::BLACK)
            .set_bool(PROPERTY_TOGGLED)
            .set(PROPERTY_TEXT, String::from("Toggle Me!"));

        cache.add(Box::new(button3), String::from("button3"), 0);
    }
}

pub fn main() {
    let (sdl_context, _, window) = init_application("pushrod example", 400, 180);
    let mut engine = Engine::new(Box::new(PushrodExample::default()), &window);

    engine.run(sdl_context, window);
}
