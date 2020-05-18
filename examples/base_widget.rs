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
use pushrod_widgets::properties::{
    PROPERTY_BORDER_COLOR, PROPERTY_BORDER_WIDTH, PROPERTY_MAIN_COLOR,
};
use pushrod_widgets::system_widgets::base_widget::BaseWidget;
use pushrod_widgets::widget::Widget;
use sdl2::pixels::Color;
use std::borrow::BorrowMut;
use pushrod_widgets::event::{PushrodEvent, Event};
use pushrod_widgets::event::Event::Pushrod;

/// This const is used to store the original color of the `Widget` so that when the mouse leaves
/// the scope of the `Widget`, its main color is restored.
pub const PROPERTY_ORIGINAL_COLOR: u32 = 10000;

#[derive(Default)]
pub struct PushrodExample {}

impl EventHandler for PushrodExample {
    fn handle_event(&mut self, event: Event, cache: &mut WidgetCache) {
        match event {
            Pushrod(pushrod_event) => match pushrod_event {
                PushrodEvent::MouseMoved { widget_id, x, y } => {
                    eprintln!("Mouse moved: widget={} x={} y={}", widget_id, x, y);
                }
                PushrodEvent::MouseScrolled { .. } => {}
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
                PushrodEvent::WidgetClicked { .. } => {}
                PushrodEvent::WidgetSelected { .. } => {}
                PushrodEvent::WidgetToggled { .. } => {}
                PushrodEvent::WidgetRadioSelected { .. } => {}
                PushrodEvent::WidgetRadioUnselected { .. } => {}
                PushrodEvent::WidgetMouseEntered { widget_id } => {
                    cache
                        .get(widget_id)
                        .borrow_mut()
                        .properties()
                        .set_value(PROPERTY_BORDER_WIDTH, 5);
                    cache
                        .get(widget_id)
                        .borrow_mut()
                        .properties()
                        .set_color(PROPERTY_MAIN_COLOR, Color::RGBA(128, 128, 128, 255));
                }
                PushrodEvent::WidgetMouseExited { widget_id } => {
                    let original_color = cache
                        .get(widget_id)
                        .borrow_mut()
                        .properties()
                        .get_color(PROPERTY_ORIGINAL_COLOR, Color::RGBA(255, 255, 255, 255));

                    cache
                        .get(widget_id)
                        .borrow_mut()
                        .properties()
                        .set_value(PROPERTY_BORDER_WIDTH, 1);
                    cache
                        .get(widget_id)
                        .borrow_mut()
                        .properties()
                        .set_color(PROPERTY_MAIN_COLOR, original_color);
                }
                PushrodEvent::WidgetFocusGained { .. } => {}
                PushrodEvent::WidgetFocusLost { .. } => {}
                PushrodEvent::WidgetTabSelected { .. } => {}
                PushrodEvent::WidgetValueChanged { .. } => {}
                PushrodEvent::WidgetMoved { .. } => {}
                PushrodEvent::WidgetVisibilityChanged { .. } => {}
            },
            Event::SDL2(x) => {
                eprintln!("SDL2 unhandled event: {:?}", x);
            }
        }
    }

    fn build_layout(&mut self, cache: &mut WidgetCache) {
        eprintln!("Layout called.");

        let mut base_widget = BaseWidget::default();

        &base_widget
            .properties()
            .set_origin(50, 50)
            .set_bounds(540, 380)
            .set_value(PROPERTY_BORDER_WIDTH, 1)
            .set_color(PROPERTY_BORDER_COLOR, Color::RGBA(0, 0, 0, 255))
            .set_color(PROPERTY_MAIN_COLOR, Color::RGBA(0, 255, 0, 255))
            .set_color(PROPERTY_ORIGINAL_COLOR, Color::RGBA(0, 255, 0, 255));

        let base_widget_id = cache.add(Box::new(base_widget), String::from("widget1"), 0);

        let mut box1 = BaseWidget::default();

        &box1
            .properties()
            .set_origin(75, 75)
            .set_bounds(490, 330)
            .set_value(PROPERTY_BORDER_WIDTH, 1)
            .set_color(PROPERTY_BORDER_COLOR, Color::RGBA(255, 0, 0, 255))
            .set_color(PROPERTY_MAIN_COLOR, Color::RGBA(0, 0, 255, 255))
            .set_color(PROPERTY_ORIGINAL_COLOR, Color::RGBA(0, 0, 255, 255));

        let box1_id = cache.add(Box::new(box1), String::from("box1"), base_widget_id);

        let mut box2 = BaseWidget::default();

        &box2
            .properties()
            .set_origin(100, 100)
            .set_bounds(75, 75)
            .set_value(PROPERTY_BORDER_WIDTH, 1)
            .set_color(PROPERTY_BORDER_COLOR, Color::RGBA(255, 255, 255, 255))
            .set_color(PROPERTY_MAIN_COLOR, Color::RGBA(255, 0, 255, 255))
            .set_color(PROPERTY_ORIGINAL_COLOR, Color::RGBA(255, 0, 255, 255));

        cache.add(Box::new(box2), String::from("box2"), box1_id);

        let mut box3 = BaseWidget::default();

        &box3
            .properties()
            .set_origin(200, 100)
            .set_bounds(75, 75)
            .set_value(PROPERTY_BORDER_WIDTH, 1)
            .set_color(PROPERTY_BORDER_COLOR, Color::RGBA(255, 255, 255, 255))
            .set_color(PROPERTY_MAIN_COLOR, Color::RGBA(255, 255, 0, 255))
            .set_color(PROPERTY_ORIGINAL_COLOR, Color::RGBA(255, 255, 0, 255));

        cache.add(Box::new(box3), String::from("box3"), box1_id);

        let mut box4 = BaseWidget::default();

        &box4
            .properties()
            .set_origin(300, 100)
            .set_bounds(75, 75)
            .set_value(PROPERTY_BORDER_WIDTH, 1)
            .set_color(PROPERTY_BORDER_COLOR, Color::RGBA(255, 255, 255, 255))
            .set_color(PROPERTY_MAIN_COLOR, Color::RGBA(0, 255, 255, 255))
            .set_color(PROPERTY_ORIGINAL_COLOR, Color::RGBA(0, 255, 255, 255));

        cache.add(Box::new(box4), String::from("box4"), box1_id);
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
