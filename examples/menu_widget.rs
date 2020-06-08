// Pushrod
// Menu Item and Popup Menu Widget Example
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
use pushrod_widgets::properties::{PROPERTY_FONT_NAME, PROPERTY_FONT_SIZE, PROPERTY_FONT_STYLE, PROPERTY_MAIN_COLOR, PROPERTY_TEXT, PROPERTY_MENU_ITEM_ID, PROPERTY_DISABLED};
use pushrod_widgets::widget::Widget;
use sdl2::pixels::Color;
use pushrod_widgets::system_widgets::menu_item_widget::MenuItemWidget;

#[derive(Default)]
pub struct PushrodExample {}

impl EventHandler for PushrodExample {
    fn handle_event(&mut self, event: Event, _cache: &mut WidgetCache) {
        match event {
            Pushrod(pushrod_event) => match pushrod_event {
                PushrodEvent::DrawFrame { .. } => {}
                PushrodEvent::WidgetMenuItemSelected {
                    widget_id: _, menu_item
                } => eprintln!("Menu item ID selected: {}", menu_item),
                x => eprintln!("Pushrod unhandled event: {:?}", x),
            },
            Event::SDL2(x) => {
                eprintln!("SDL2 unhandled event: {:?}", x);
            }
        }
    }

    fn build_layout(&mut self, cache: &mut WidgetCache) {
        for i in 0..7 {
            let mut menu_item_widget = MenuItemWidget::default();

            menu_item_widget
                .properties()
                .set_origin(20, 20 + (i * 30))
                .set_bounds(360, 30)
                .set_color(PROPERTY_MAIN_COLOR, Color::WHITE)
                .set(
                    PROPERTY_FONT_NAME,
                    String::from("assets/OpenSans-Regular.ttf"),
                )
                .set_value(PROPERTY_FONT_SIZE, 18)
                .set_value(PROPERTY_FONT_STYLE, sdl2::ttf::FontStyle::NORMAL.bits())
                .set_value(PROPERTY_MENU_ITEM_ID, (i + 1) as i32)
                .set(PROPERTY_TEXT, format!("Menu Item {}", i + 1));

            if i % 2 == 1 {
                menu_item_widget.properties()
                    .set_bool(PROPERTY_DISABLED);
            }

            cache.add(Box::new(menu_item_widget), format!("widget{}", i), 0);
        }
    }
}

pub fn main() {
    let (sdl_context, _, window) = init_application("pushrod example", 400, 260);
    let mut engine = Engine::new(Box::new(PushrodExample::default()), &window);

    engine.run(sdl_context, window);
}
