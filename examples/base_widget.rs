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

use pushrod::engine::{Engine, EventHandler};
use pushrod_widgets::caches::WidgetCache;
use pushrod_widgets::event::Event::Pushrod;
use pushrod_widgets::event::{Event, PushrodEvent};
use pushrod_widgets::primitives::init_application;
use pushrod_widgets::properties::{
    PROPERTY_BORDER_COLOR, PROPERTY_BORDER_WIDTH, PROPERTY_FONT_NAME, PROPERTY_FONT_SIZE,
    PROPERTY_FONT_STYLE, PROPERTY_MAIN_COLOR, PROPERTY_TEXT,
};
use pushrod_widgets::system_widgets::base_widget::BaseWidget;
use pushrod_widgets::system_widgets::text_widget::TextWidget;
use pushrod_widgets::widget::Widget;
use sdl2::pixels::Color;

/// This const is used to store the original color of the `Widget` so that when the mouse leaves
/// the scope of the `Widget`, its main color is restored.
pub const PROPERTY_ORIGINAL_COLOR: u32 = 10000;

#[derive(Default)]
pub struct PushrodExample {
    text_id: u32,
}

impl EventHandler for PushrodExample {
    fn handle_event(&mut self, event: Event, cache: &mut WidgetCache) {
        match event {
            Pushrod(pushrod_event) => match pushrod_event {
                PushrodEvent::MouseMoved { .. } => {}
                PushrodEvent::MouseButton {
                    widget_id,
                    button,
                    state,
                } => {
                    eprintln!(
                        "Mouse button: widget={} button={} state={}",
                        widget_id, button, state
                    );
                }
                PushrodEvent::WidgetMouseEntered { widget_id } => {
                    if widget_id != self.text_id {
                        cache
                            .get(widget_id)
                            .properties()
                            .set_value(PROPERTY_BORDER_WIDTH, 5);
                        cache
                            .get(widget_id)
                            .properties()
                            .set_color(PROPERTY_MAIN_COLOR, Color::GREY);
                    }

                    cache
                        .get(self.text_id)
                        .properties()
                        .set(PROPERTY_TEXT, format!("Current Widget ID: {}", widget_id));
                    cache.get(self.text_id).invalidate();
                }
                PushrodEvent::WidgetMouseExited { widget_id } => {
                    if widget_id != self.text_id {
                        let original_color = cache
                            .get(widget_id)
                            .properties()
                            .get_color(PROPERTY_ORIGINAL_COLOR, Color::WHITE);

                        cache
                            .get(widget_id)
                            .properties()
                            .set_value(PROPERTY_BORDER_WIDTH, 1);
                        cache
                            .get(widget_id)
                            .properties()
                            .set_color(PROPERTY_MAIN_COLOR, original_color);
                    }
                }
                PushrodEvent::DrawFrame { .. } => {}
                x => eprintln!("Pushrod unhandled event: {:?}", x),
            },
            Event::SDL2(x) => {
                eprintln!("SDL2 unhandled event: {:?}", x);
            }
        }
    }

    fn build_layout(&mut self, cache: &mut WidgetCache) {
        let mut base_widget = BaseWidget::default();

        base_widget
            .properties()
            .set_origin(50, 50)
            .set_bounds(540, 380)
            .set_value(PROPERTY_BORDER_WIDTH, 1)
            .set_color(PROPERTY_BORDER_COLOR, Color::BLACK)
            .set_color(PROPERTY_MAIN_COLOR, Color::GREEN)
            .set_color(PROPERTY_ORIGINAL_COLOR, Color::GREEN);

        let base_widget_id = cache.add(Box::new(base_widget), String::from("widget1"), 0);

        let mut box1 = BaseWidget::default();

        box1.properties()
            .set_origin(75, 75)
            .set_bounds(490, 330)
            .set_value(PROPERTY_BORDER_WIDTH, 1)
            .set_color(PROPERTY_BORDER_COLOR, Color::RED)
            .set_color(PROPERTY_MAIN_COLOR, Color::BLUE)
            .set_color(PROPERTY_ORIGINAL_COLOR, Color::BLUE);

        let box1_id = cache.add(Box::new(box1), String::from("box1"), base_widget_id);

        let mut box2 = BaseWidget::default();

        box2.properties()
            .set_origin(100, 100)
            .set_bounds(75, 75)
            .set_value(PROPERTY_BORDER_WIDTH, 1)
            .set_color(PROPERTY_BORDER_COLOR, Color::WHITE)
            .set_color(PROPERTY_MAIN_COLOR, Color::MAGENTA)
            .set_color(PROPERTY_ORIGINAL_COLOR, Color::MAGENTA);

        cache.add(Box::new(box2), String::from("box2"), box1_id);

        let mut box3 = BaseWidget::default();

        box3.properties()
            .set_origin(200, 100)
            .set_bounds(75, 75)
            .set_value(PROPERTY_BORDER_WIDTH, 1)
            .set_color(PROPERTY_BORDER_COLOR, Color::WHITE)
            .set_color(PROPERTY_MAIN_COLOR, Color::CYAN)
            .set_color(PROPERTY_ORIGINAL_COLOR, Color::CYAN);

        cache.add(Box::new(box3), String::from("box3"), box1_id);

        let mut box4 = BaseWidget::default();

        box4.properties()
            .set_origin(300, 100)
            .set_bounds(75, 75)
            .set_value(PROPERTY_BORDER_WIDTH, 1)
            .set_color(PROPERTY_BORDER_COLOR, Color::WHITE)
            .set_color(PROPERTY_MAIN_COLOR, Color::YELLOW)
            .set_color(PROPERTY_ORIGINAL_COLOR, Color::YELLOW);

        cache.add(Box::new(box4), String::from("box4"), box1_id);

        let mut text1 = TextWidget::default();

        text1
            .properties()
            .set_origin(10, 10)
            .set_bounds(500, 26)
            .set(
                PROPERTY_FONT_NAME,
                String::from("assets/OpenSans-Regular.ttf"),
            )
            .set_value(PROPERTY_FONT_SIZE, 18)
            .set_value(PROPERTY_FONT_STYLE, sdl2::ttf::FontStyle::NORMAL.bits())
            .set_color(PROPERTY_MAIN_COLOR, Color::RGBA(255, 255, 255, 1))
            .set(PROPERTY_TEXT, String::from("Current Widget ID: 0"));

        self.text_id = cache.add(Box::new(text1), String::from("text1"), 0);
    }
}

pub fn main() {
    let (sdl_context, _, window) = init_application("pushrod example", 640, 480);
    let mut engine = Engine::new(Box::new(PushrodExample::default()), &window);

    engine.run(sdl_context, window);
}
