// Pushrod
// Image Widget Example
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
    IMAGE_JUSTIFY_LEFT, IMAGE_JUSTIFY_RIGHT, PROPERTY_BORDER_COLOR, PROPERTY_BORDER_WIDTH,
    PROPERTY_FONT_NAME, PROPERTY_FONT_SIZE, PROPERTY_FONT_STYLE, PROPERTY_IMAGE_FILENAME,
    PROPERTY_IMAGE_POSITION, PROPERTY_IMAGE_SCALED, PROPERTY_MAIN_COLOR, PROPERTY_TEXT,
    PROPERTY_TEXT_JUSTIFICATION, TEXT_JUSTIFY_CENTER,
};
use pushrod_widgets::system_widgets::image_button_widget::ImageButtonWidget;
use pushrod_widgets::system_widgets::image_widget::{
    ImageWidget, COMPASS_CENTER, COMPASS_E, COMPASS_N, COMPASS_NE, COMPASS_NW, COMPASS_S,
    COMPASS_SE, COMPASS_SW, COMPASS_W,
};
use pushrod_widgets::widget::Widget;
use sdl2::pixels::Color;

#[derive(Default)]
pub struct PushrodExample {}

impl EventHandler for PushrodExample {
    fn handle_event(&mut self, event: Event, _c: &mut WidgetCache) {
        match event {
            Pushrod(pushrod_event) => match pushrod_event {
                PushrodEvent::DrawFrame { .. } => {}
                _ => {}
            },
            Event::SDL2(x) => {
                eprintln!("SDL2 unhandled event: {:?}", x);
            }
        }
    }

    fn build_layout(&mut self, cache: &mut WidgetCache) {
        let mut widget1 = ImageWidget::default();

        widget1
            .properties()
            .set_origin(20, 16)
            .set_bounds(60, 60)
            .set_color(PROPERTY_MAIN_COLOR, Color::BLACK)
            .set_value(PROPERTY_IMAGE_POSITION, COMPASS_NW)
            .set(
                PROPERTY_IMAGE_FILENAME,
                String::from("assets/rust-48x48.jpg"),
            );

        cache.add(Box::new(widget1), String::from("widget1"), 0);

        let mut widget2 = ImageWidget::default();

        widget2
            .properties()
            .set_origin(90, 16)
            .set_bounds(60, 60)
            .set_color(PROPERTY_MAIN_COLOR, Color::BLACK)
            .set_value(PROPERTY_IMAGE_POSITION, COMPASS_N)
            .set(
                PROPERTY_IMAGE_FILENAME,
                String::from("assets/rust-48x48.jpg"),
            );

        cache.add(Box::new(widget2), String::from("widget2"), 0);

        let mut widget3 = ImageWidget::default();

        widget3
            .properties()
            .set_origin(160, 16)
            .set_bounds(60, 60)
            .set_color(PROPERTY_MAIN_COLOR, Color::BLACK)
            .set_value(PROPERTY_IMAGE_POSITION, COMPASS_NE)
            .set(
                PROPERTY_IMAGE_FILENAME,
                String::from("assets/rust-48x48.jpg"),
            );

        cache.add(Box::new(widget3), String::from("widget3"), 0);

        let mut widget4 = ImageWidget::default();

        widget4
            .properties()
            .set_origin(20, 86)
            .set_bounds(60, 60)
            .set_color(PROPERTY_MAIN_COLOR, Color::BLACK)
            .set_value(PROPERTY_IMAGE_POSITION, COMPASS_W)
            .set(
                PROPERTY_IMAGE_FILENAME,
                String::from("assets/rust-48x48.jpg"),
            );

        cache.add(Box::new(widget4), String::from("widget4"), 0);

        let mut widget5 = ImageWidget::default();

        widget5
            .properties()
            .set_origin(90, 86)
            .set_bounds(60, 60)
            .set_color(PROPERTY_MAIN_COLOR, Color::BLACK)
            .set_value(PROPERTY_IMAGE_POSITION, COMPASS_CENTER)
            .set(
                PROPERTY_IMAGE_FILENAME,
                String::from("assets/rust-48x48.jpg"),
            );

        cache.add(Box::new(widget5), String::from("widget5"), 0);

        let mut widget6 = ImageWidget::default();

        widget6
            .properties()
            .set_origin(160, 86)
            .set_bounds(60, 60)
            .set_color(PROPERTY_MAIN_COLOR, Color::BLACK)
            .set_value(PROPERTY_IMAGE_POSITION, COMPASS_E)
            .set(
                PROPERTY_IMAGE_FILENAME,
                String::from("assets/rust-48x48.jpg"),
            );

        cache.add(Box::new(widget6), String::from("widget6"), 0);

        let mut widget7 = ImageWidget::default();

        widget7
            .properties()
            .set_origin(20, 156)
            .set_bounds(60, 60)
            .set_color(PROPERTY_MAIN_COLOR, Color::BLACK)
            .set_value(PROPERTY_IMAGE_POSITION, COMPASS_SW)
            .set(
                PROPERTY_IMAGE_FILENAME,
                String::from("assets/rust-48x48.jpg"),
            );

        cache.add(Box::new(widget7), String::from("widget7"), 0);

        let mut widget8 = ImageWidget::default();

        widget8
            .properties()
            .set_origin(90, 156)
            .set_bounds(60, 60)
            .set_color(PROPERTY_MAIN_COLOR, Color::BLACK)
            .set_value(PROPERTY_IMAGE_POSITION, COMPASS_S)
            .set(
                PROPERTY_IMAGE_FILENAME,
                String::from("assets/rust-48x48.jpg"),
            );

        cache.add(Box::new(widget8), String::from("widget8"), 0);

        let mut widget9 = ImageWidget::default();

        widget9
            .properties()
            .set_origin(160, 156)
            .set_bounds(60, 60)
            .set_color(PROPERTY_MAIN_COLOR, Color::BLACK)
            .set_value(PROPERTY_IMAGE_POSITION, COMPASS_SE)
            .set(
                PROPERTY_IMAGE_FILENAME,
                String::from("assets/rust-48x48.jpg"),
            );

        cache.add(Box::new(widget9), String::from("widget9"), 0);

        let mut widget10 = ImageWidget::default();

        widget10
            .properties()
            .set_origin(230, 16)
            .set_bounds(80, 80)
            .set_color(PROPERTY_MAIN_COLOR, Color::BLACK)
            .set_value(PROPERTY_IMAGE_POSITION, COMPASS_NW)
            .set_bool(PROPERTY_IMAGE_SCALED)
            .set(
                PROPERTY_IMAGE_FILENAME,
                String::from("assets/rust-48x48.jpg"),
            );

        cache.add(Box::new(widget10), String::from("widget10"), 0);

        let mut widget11 = ImageWidget::default();

        widget11
            .properties()
            .set_origin(260, 46)
            .set_bounds(120, 120)
            .set_color(PROPERTY_MAIN_COLOR, Color::BLACK)
            .set_value(PROPERTY_IMAGE_POSITION, COMPASS_NW)
            .set_bool(PROPERTY_IMAGE_SCALED)
            .set(
                PROPERTY_IMAGE_FILENAME,
                String::from("assets/rust-48x48.jpg"),
            );

        cache.add(Box::new(widget11), String::from("widget11"), 0);

        let mut widget12 = ImageWidget::default();

        widget12
            .properties()
            .set_origin(320, 86)
            .set_bounds(160, 160)
            .set_color(PROPERTY_MAIN_COLOR, Color::BLACK)
            .set_value(PROPERTY_IMAGE_POSITION, COMPASS_NW)
            .set_bool(PROPERTY_IMAGE_SCALED)
            .set(
                PROPERTY_IMAGE_FILENAME,
                String::from("assets/rust-48x48.jpg"),
            );

        cache.add(Box::new(widget12), String::from("widget12"), 0);

        let mut button1 = ImageButtonWidget::default();

        button1
            .properties()
            .set_origin(20, 250)
            .set_bounds(460, 50)
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
            .set_value(PROPERTY_IMAGE_POSITION, IMAGE_JUSTIFY_LEFT)
            .set_bool(PROPERTY_IMAGE_SCALED)
            .set(
                PROPERTY_IMAGE_FILENAME,
                String::from("assets/rust-48x48.jpg"),
            )
            .set(PROPERTY_TEXT, String::from("Left Justified Image"));

        cache.add(Box::new(button1), String::from("button1"), 0);

        let mut button2 = ImageButtonWidget::default();

        button2
            .properties()
            .set_origin(20, 310)
            .set_bounds(460, 50)
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
            .set_value(PROPERTY_IMAGE_POSITION, IMAGE_JUSTIFY_RIGHT)
            .set_bool(PROPERTY_IMAGE_SCALED)
            .set(
                PROPERTY_IMAGE_FILENAME,
                String::from("assets/rust-48x48.jpg"),
            )
            .set(PROPERTY_TEXT, String::from("Right Justified Image"));

        cache.add(Box::new(button2), String::from("button2"), 0);
    }
}

pub fn main() {
    let (sdl_context, _, window) = init_application("pushrod example", 500, 370);
    let mut engine = Engine::new(Box::new(PushrodExample::default()), &window);

    engine.run(sdl_context, window);
}
