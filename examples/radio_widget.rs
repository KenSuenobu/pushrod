// Pushrod
// Radio Button Widget Example
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
use pushrod_widgets::properties::{PROPERTY_BORDER_COLOR, PROPERTY_BORDER_WIDTH, PROPERTY_FONT_NAME, PROPERTY_FONT_SIZE, PROPERTY_FONT_STYLE, PROPERTY_GROUP_BACKGROUND_COLOR, PROPERTY_HIDDEN, PROPERTY_MAIN_COLOR, PROPERTY_TEXT, PROPERTY_TEXT_JUSTIFICATION, TEXT_JUSTIFY_CENTER, PROPERTY_TOGGLED, TEXT_JUSTIFY_LEFT, PROPERTY_GROUP_ID};
use pushrod_widgets::system_widgets::group_box_widget::GroupBoxWidget;
use pushrod_widgets::widget::Widget;
use sdl2::pixels::Color;
use pushrod_widgets::system_widgets::radio_button_widget::RadioButtonWidget;

#[derive(Default)]
pub struct PushrodExample { }

impl EventHandler for PushrodExample {
    fn handle_event(&mut self, event: Event, cache: &mut WidgetCache) {
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
        let mut group_box1 = GroupBoxWidget::default();

        group_box1
            .properties()
            .set_origin(10, 6)
            .set_bounds(200, 200)
            .set_value(PROPERTY_BORDER_WIDTH, 1)
            .set_color(PROPERTY_BORDER_COLOR, Color::BLACK)
            .set_color(
                PROPERTY_GROUP_BACKGROUND_COLOR,
                Color::RGBA(224, 224, 244, 255),
            )
            .set(
                PROPERTY_FONT_NAME,
                String::from("assets/OpenSans-Regular.ttf"),
            )
            .set_value(PROPERTY_FONT_SIZE, 14)
            .set_value(PROPERTY_FONT_STYLE, sdl2::ttf::FontStyle::NORMAL.bits())
            .set(PROPERTY_TEXT, String::from(" GROUP 1 "));

        cache.add(Box::new(group_box1), String::from("group_box1"), 0);

        let mut button1 = RadioButtonWidget::default();

        button1
            .properties()
            .set_origin(20, 30)
            .set_bounds(150, 30)
            .set_color(PROPERTY_MAIN_COLOR, Color::WHITE)
            .set(
                PROPERTY_FONT_NAME,
                String::from("assets/OpenSans-Regular.ttf"),
            )
            .set_value(PROPERTY_FONT_SIZE, 18)
            .set_value(PROPERTY_FONT_STYLE, sdl2::ttf::FontStyle::NORMAL.bits())
            .set_value(PROPERTY_TEXT_JUSTIFICATION, TEXT_JUSTIFY_LEFT)
            .set_value(PROPERTY_BORDER_WIDTH, 1)
            .set_color(PROPERTY_BORDER_COLOR, Color::BLACK)
            .set_value(PROPERTY_GROUP_ID, 1)
            .set_bool(PROPERTY_TOGGLED)
            .set(PROPERTY_TEXT, String::from("Item 1"));

        cache.add(Box::new(button1), String::from("button1"), 0);

        let mut button2 = RadioButtonWidget::default();

        button2
            .properties()
            .set_origin(20, 64)
            .set_bounds(150, 30)
            .set_color(PROPERTY_MAIN_COLOR, Color::WHITE)
            .set(
                PROPERTY_FONT_NAME,
                String::from("assets/OpenSans-Regular.ttf"),
            )
            .set_value(PROPERTY_FONT_SIZE, 18)
            .set_value(PROPERTY_FONT_STYLE, sdl2::ttf::FontStyle::NORMAL.bits())
            .set_value(PROPERTY_TEXT_JUSTIFICATION, TEXT_JUSTIFY_LEFT)
            .set_value(PROPERTY_BORDER_WIDTH, 1)
            .set_color(PROPERTY_BORDER_COLOR, Color::BLACK)
            .set_value(PROPERTY_GROUP_ID, 1)
            .set(PROPERTY_TEXT, String::from("Item 2"));

        cache.add(Box::new(button2), String::from("button2"), 0);

        let mut button3 = RadioButtonWidget::default();

        button3
            .properties()
            .set_origin(20, 98)
            .set_bounds(150, 30)
            .set_color(PROPERTY_MAIN_COLOR, Color::WHITE)
            .set(
                PROPERTY_FONT_NAME,
                String::from("assets/OpenSans-Regular.ttf"),
            )
            .set_value(PROPERTY_FONT_SIZE, 18)
            .set_value(PROPERTY_FONT_STYLE, sdl2::ttf::FontStyle::NORMAL.bits())
            .set_value(PROPERTY_TEXT_JUSTIFICATION, TEXT_JUSTIFY_LEFT)
            .set_value(PROPERTY_BORDER_WIDTH, 1)
            .set_color(PROPERTY_BORDER_COLOR, Color::BLACK)
            .set_value(PROPERTY_GROUP_ID, 1)
            .set(PROPERTY_TEXT, String::from("Item 3"));

        cache.add(Box::new(button3), String::from("button3"), 0);

        let mut button4 = RadioButtonWidget::default();

        button4
            .properties()
            .set_origin(20, 132)
            .set_bounds(150, 30)
            .set_color(PROPERTY_MAIN_COLOR, Color::WHITE)
            .set(
                PROPERTY_FONT_NAME,
                String::from("assets/OpenSans-Regular.ttf"),
            )
            .set_value(PROPERTY_FONT_SIZE, 18)
            .set_value(PROPERTY_FONT_STYLE, sdl2::ttf::FontStyle::NORMAL.bits())
            .set_value(PROPERTY_TEXT_JUSTIFICATION, TEXT_JUSTIFY_LEFT)
            .set_value(PROPERTY_BORDER_WIDTH, 1)
            .set_color(PROPERTY_BORDER_COLOR, Color::BLACK)
            .set_value(PROPERTY_GROUP_ID, 1)
            .set(PROPERTY_TEXT, String::from("Item 4"));

        cache.add(Box::new(button4), String::from("button4"), 0);

        let mut button5 = RadioButtonWidget::default();

        button5
            .properties()
            .set_origin(20, 166)
            .set_bounds(150, 30)
            .set_color(PROPERTY_MAIN_COLOR, Color::WHITE)
            .set(
                PROPERTY_FONT_NAME,
                String::from("assets/OpenSans-Regular.ttf"),
            )
            .set_value(PROPERTY_FONT_SIZE, 18)
            .set_value(PROPERTY_FONT_STYLE, sdl2::ttf::FontStyle::NORMAL.bits())
            .set_value(PROPERTY_TEXT_JUSTIFICATION, TEXT_JUSTIFY_LEFT)
            .set_value(PROPERTY_BORDER_WIDTH, 1)
            .set_color(PROPERTY_BORDER_COLOR, Color::BLACK)
            .set_value(PROPERTY_GROUP_ID, 1)
            .set(PROPERTY_TEXT, String::from("Item 5"));

        cache.add(Box::new(button5), String::from("button5"), 0);

        let mut group_box2 = GroupBoxWidget::default();

        group_box2
            .properties()
            .set_origin(220, 6)
            .set_bounds(200, 200)
            .set_value(PROPERTY_BORDER_WIDTH, 1)
            .set_color(PROPERTY_BORDER_COLOR, Color::BLACK)
            .set_color(
                PROPERTY_GROUP_BACKGROUND_COLOR,
                Color::RGBA(224, 224, 244, 255),
            )
            .set(
                PROPERTY_FONT_NAME,
                String::from("assets/OpenSans-Regular.ttf"),
            )
            .set_value(PROPERTY_FONT_SIZE, 14)
            .set_value(PROPERTY_FONT_STYLE, sdl2::ttf::FontStyle::NORMAL.bits())
            .set(PROPERTY_TEXT, String::from(" GROUP 2 "));

        cache.add(Box::new(group_box2), String::from("group_box2"), 0);

        let mut button1_1 = RadioButtonWidget::default();

        button1_1
            .properties()
            .set_origin(240, 30)
            .set_bounds(150, 30)
            .set_color(PROPERTY_MAIN_COLOR, Color::WHITE)
            .set(
                PROPERTY_FONT_NAME,
                String::from("assets/OpenSans-Regular.ttf"),
            )
            .set_value(PROPERTY_FONT_SIZE, 18)
            .set_value(PROPERTY_FONT_STYLE, sdl2::ttf::FontStyle::NORMAL.bits())
            .set_value(PROPERTY_TEXT_JUSTIFICATION, TEXT_JUSTIFY_LEFT)
            .set_value(PROPERTY_BORDER_WIDTH, 1)
            .set_color(PROPERTY_BORDER_COLOR, Color::BLACK)
            .set_value(PROPERTY_GROUP_ID, 2)
            .set(PROPERTY_TEXT, String::from("Item 1"));

        cache.add(Box::new(button1_1), String::from("button1_1"), 0);

        let mut button2_1 = RadioButtonWidget::default();

        button2_1
            .properties()
            .set_origin(240, 64)
            .set_bounds(150, 30)
            .set_color(PROPERTY_MAIN_COLOR, Color::WHITE)
            .set(
                PROPERTY_FONT_NAME,
                String::from("assets/OpenSans-Regular.ttf"),
            )
            .set_value(PROPERTY_FONT_SIZE, 18)
            .set_value(PROPERTY_FONT_STYLE, sdl2::ttf::FontStyle::NORMAL.bits())
            .set_value(PROPERTY_TEXT_JUSTIFICATION, TEXT_JUSTIFY_LEFT)
            .set_value(PROPERTY_BORDER_WIDTH, 1)
            .set_color(PROPERTY_BORDER_COLOR, Color::BLACK)
            .set_value(PROPERTY_GROUP_ID, 2)
            .set_bool(PROPERTY_TOGGLED)
            .set(PROPERTY_TEXT, String::from("Item 2"));

        cache.add(Box::new(button2_1), String::from("button2_1"), 0);

        let mut button3_1 = RadioButtonWidget::default();

        button3_1
            .properties()
            .set_origin(240, 98)
            .set_bounds(150, 30)
            .set_color(PROPERTY_MAIN_COLOR, Color::WHITE)
            .set(
                PROPERTY_FONT_NAME,
                String::from("assets/OpenSans-Regular.ttf"),
            )
            .set_value(PROPERTY_FONT_SIZE, 18)
            .set_value(PROPERTY_FONT_STYLE, sdl2::ttf::FontStyle::NORMAL.bits())
            .set_value(PROPERTY_TEXT_JUSTIFICATION, TEXT_JUSTIFY_LEFT)
            .set_value(PROPERTY_BORDER_WIDTH, 1)
            .set_color(PROPERTY_BORDER_COLOR, Color::BLACK)
            .set_value(PROPERTY_GROUP_ID, 2)
            .set(PROPERTY_TEXT, String::from("Item 3"));

        cache.add(Box::new(button3_1), String::from("button3_1"), 0);

        let mut button4_1 = RadioButtonWidget::default();

        button4_1
            .properties()
            .set_origin(240, 132)
            .set_bounds(150, 30)
            .set_color(PROPERTY_MAIN_COLOR, Color::WHITE)
            .set(
                PROPERTY_FONT_NAME,
                String::from("assets/OpenSans-Regular.ttf"),
            )
            .set_value(PROPERTY_FONT_SIZE, 18)
            .set_value(PROPERTY_FONT_STYLE, sdl2::ttf::FontStyle::NORMAL.bits())
            .set_value(PROPERTY_TEXT_JUSTIFICATION, TEXT_JUSTIFY_LEFT)
            .set_value(PROPERTY_BORDER_WIDTH, 1)
            .set_color(PROPERTY_BORDER_COLOR, Color::BLACK)
            .set_value(PROPERTY_GROUP_ID, 2)
            .set(PROPERTY_TEXT, String::from("Item 4"));

        cache.add(Box::new(button4_1), String::from("button4_1"), 0);

        let mut button5_1 = RadioButtonWidget::default();

        button5_1
            .properties()
            .set_origin(240, 166)
            .set_bounds(150, 30)
            .set_color(PROPERTY_MAIN_COLOR, Color::WHITE)
            .set(
                PROPERTY_FONT_NAME,
                String::from("assets/OpenSans-Regular.ttf"),
            )
            .set_value(PROPERTY_FONT_SIZE, 18)
            .set_value(PROPERTY_FONT_STYLE, sdl2::ttf::FontStyle::NORMAL.bits())
            .set_value(PROPERTY_TEXT_JUSTIFICATION, TEXT_JUSTIFY_LEFT)
            .set_value(PROPERTY_BORDER_WIDTH, 1)
            .set_color(PROPERTY_BORDER_COLOR, Color::BLACK)
            .set_value(PROPERTY_GROUP_ID, 2)
            .set(PROPERTY_TEXT, String::from("Item 5"));

        cache.add(Box::new(button5_1), String::from("button5_1"), 0);
    }
}

pub fn main() {
    let (sdl_context, _, window) = init_application("pushrod example", 430, 210);
    let mut engine = Engine::new(Box::new(PushrodExample::default()), &window);

    engine.run(sdl_context, window);
}
