// Pushrod
// Hide/Show Widget Example
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
    PROPERTY_FONT_STYLE, PROPERTY_HIDDEN, PROPERTY_MAIN_COLOR, PROPERTY_TEXT,
    PROPERTY_TEXT_JUSTIFICATION, TEXT_JUSTIFY_CENTER,
};
use pushrod_widgets::system_widgets::base_widget::BaseWidget;
use pushrod_widgets::system_widgets::button_widget::ButtonWidget;
use pushrod_widgets::system_widgets::text_widget::TextWidget;
use pushrod_widgets::widget::Widget;
use sdl2::pixels::Color;

#[derive(Default)]
pub struct PushrodExample {
    base1_id: u32,
    base2_id: u32,
    base3_id: u32,
    base4_id: u32,
    button1_id: u32,
    button2_id: u32,
    button3_id: u32,
    button4_id: u32,
}

impl PushrodExample {
    fn toggle_hide_show(&mut self, widget_id: u32, cache: &mut WidgetCache) {
        let hidden = cache.get(widget_id).properties().get_bool(PROPERTY_HIDDEN);

        if hidden {
            cache.set_hidden(widget_id, false);
        } else {
            cache.set_hidden(widget_id, true);
        }

        cache.get(0).invalidate();
    }
}

impl EventHandler for PushrodExample {
    fn handle_event(&mut self, event: Event, cache: &mut WidgetCache) {
        match event {
            Pushrod(pushrod_event) => match pushrod_event {
                PushrodEvent::DrawFrame { .. } => {}
                PushrodEvent::WidgetClicked {
                    widget_id,
                    button,
                    clicks,
                } => {
                    if button == 1 && clicks == 1 {
                        if widget_id == self.button1_id {
                            self.toggle_hide_show(self.base1_id, cache);
                        } else if widget_id == self.button2_id {
                            self.toggle_hide_show(self.base2_id, cache);
                        } else if widget_id == self.button3_id {
                            self.toggle_hide_show(self.base3_id, cache);
                        } else if widget_id == self.button4_id {
                            self.toggle_hide_show(self.base4_id, cache);
                        }
                    }
                }
                x => eprintln!("Pushrod unhandled event: {:?}", x),
            },
            Event::SDL2(x) => {
                eprintln!("SDL2 unhandled event: {:?}", x);
            }
        }
    }

    fn build_layout(&mut self, cache: &mut WidgetCache) {
        let mut base1 = BaseWidget::default();

        &base1
            .properties()
            .set_origin(20, 20)
            .set_bounds(150, 150)
            .set_color(PROPERTY_MAIN_COLOR, Color::RED)
            .set_value(PROPERTY_BORDER_WIDTH, 2)
            .set_color(PROPERTY_BORDER_COLOR, Color::BLACK);

        self.base1_id = cache.add(Box::new(base1), String::from("base1"), 0);

        let mut base2 = BaseWidget::default();

        &base2
            .properties()
            .set_origin(180, 20)
            .set_bounds(150, 150)
            .set_color(PROPERTY_MAIN_COLOR, Color::GREEN)
            .set_value(PROPERTY_BORDER_WIDTH, 2)
            .set_color(PROPERTY_BORDER_COLOR, Color::BLACK);

        self.base2_id = cache.add(Box::new(base2), String::from("base2"), 0);

        let mut base2_1 = BaseWidget::default();

        &base2_1
            .properties()
            .set_origin(210, 40)
            .set_bounds(110, 110)
            .set_color(PROPERTY_MAIN_COLOR, Color::GREY)
            .set_value(PROPERTY_BORDER_WIDTH, 2)
            .set_color(PROPERTY_BORDER_COLOR, Color::RED);

        cache.add(Box::new(base2_1), String::from("base2_1"), self.base2_id);

        let mut text1 = TextWidget::default();

        &text1
            .properties()
            .set_origin(360, 70)
            .set_bounds(130, 26)
            .set(
                PROPERTY_FONT_NAME,
                String::from("assets/OpenSans-Regular.ttf"),
            )
            .set_value(PROPERTY_FONT_SIZE, 18)
            .set_value(PROPERTY_FONT_STYLE, sdl2::ttf::FontStyle::NORMAL.bits())
            .set_color(PROPERTY_MAIN_COLOR, Color::RGBA(255, 255, 255, 1))
            .set(PROPERTY_TEXT, String::from("BOO!!!"));

        cache.add(Box::new(text1), String::from("text1"), 0);

        let mut base3 = BaseWidget::default();

        &base3
            .properties()
            .set_origin(340, 20)
            .set_bounds(150, 150)
            .set_color(PROPERTY_MAIN_COLOR, Color::MAGENTA)
            .set_value(PROPERTY_BORDER_WIDTH, 2)
            .set_color(PROPERTY_BORDER_COLOR, Color::BLACK);

        self.base3_id = cache.add(Box::new(base3), String::from("base3"), 0);

        let mut base4 = BaseWidget::default();

        &base4
            .properties()
            .set_origin(500, 20)
            .set_bounds(150, 150)
            .set_color(PROPERTY_MAIN_COLOR, Color::CYAN)
            .set_value(PROPERTY_BORDER_WIDTH, 2)
            .set_color(PROPERTY_BORDER_COLOR, Color::BLACK);

        let base4_id = cache.add(Box::new(base4), String::from("base4"), 0);

        let mut base4_1 = BaseWidget::default();

        &base4_1
            .properties()
            .set_origin(520, 40)
            .set_bounds(110, 110)
            .set_color(PROPERTY_MAIN_COLOR, Color::GREEN)
            .set_value(PROPERTY_BORDER_WIDTH, 2)
            .set_color(PROPERTY_BORDER_COLOR, Color::BLACK);

        self.base4_id = cache.add(Box::new(base4_1), String::from("base4_1"), base4_id);

        let mut base4_2 = BaseWidget::default();

        &base4_2
            .properties()
            .set_origin(560, 70)
            .set_bounds(100, 70)
            .set_color(PROPERTY_MAIN_COLOR, Color::WHITE)
            .set_value(PROPERTY_BORDER_WIDTH, 2)
            .set_color(PROPERTY_BORDER_COLOR, Color::BLACK);

        cache.add(Box::new(base4_2), String::from("base4_2"), self.base4_id);

        let mut button1 = ButtonWidget::default();

        &button1
            .properties()
            .set_origin(20, 180)
            .set_bounds(150, 32)
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
            .set(PROPERTY_TEXT, String::from("Hide/Show"));

        self.button1_id = cache.add(Box::new(button1), String::from("button1"), 0);

        let mut button2 = ButtonWidget::default();

        &button2
            .properties()
            .set_origin(180, 180)
            .set_bounds(150, 32)
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
            .set(PROPERTY_TEXT, String::from("Hide/Show"));

        self.button2_id = cache.add(Box::new(button2), String::from("button2"), 0);

        let mut button3 = ButtonWidget::default();

        &button3
            .properties()
            .set_origin(340, 180)
            .set_bounds(150, 32)
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
            .set(PROPERTY_TEXT, String::from("Hide/Show"));

        self.button3_id = cache.add(Box::new(button3), String::from("button3"), 0);

        let mut button4 = ButtonWidget::default();

        &button4
            .properties()
            .set_origin(500, 180)
            .set_bounds(150, 32)
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
            .set(PROPERTY_TEXT, String::from("Hide/Show"));

        self.button4_id = cache.add(Box::new(button4), String::from("button4"), 0);
    }
}

pub fn main() {
    let (sdl_context, _, window) = init_application("pushrod example", 670, 230);
    let mut engine = Engine::new(Box::new(PushrodExample::default()), &window);

    engine.run(sdl_context, window);
}
