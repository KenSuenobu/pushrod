// Pushrod
// Grid Widget Example
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
use pushrod_widgets::cache::widget_cache::WidgetCache;
use pushrod_widgets::primitives::init_application;
use pushrod_widgets::properties::{
    PROPERTY_BORDER_COLOR, PROPERTY_BORDER_WIDTH, PROPERTY_GRID_COLOR, PROPERTY_GRID_CONNECTED,
    PROPERTY_GRID_SPACING, PROPERTY_MAIN_COLOR,
};
use pushrod_widgets::system_widgets::grid_widget::GridWidget;
use pushrod_widgets::widget::Widget;
use sdl2::pixels::Color;

#[derive(Default)]
pub struct PushrodExample {}

impl EventHandler for PushrodExample {
    fn build_layout(&mut self, cache: &mut WidgetCache) {
        let mut grid_widget1 = GridWidget::default();

        grid_widget1
            .properties()
            .set_origin(20, 20)
            .set_bounds(280, 440)
            .set_value(PROPERTY_BORDER_WIDTH, 1)
            .set_color(PROPERTY_BORDER_COLOR, Color::BLACK)
            .set_color(PROPERTY_MAIN_COLOR, Color::WHITE)
            .set_color(PROPERTY_GRID_COLOR, Color::BLACK)
            .set_value(PROPERTY_GRID_SPACING, 10)
            .set_bool(PROPERTY_GRID_CONNECTED);

        cache.add(Box::new(grid_widget1), String::from("grid1"), 0);

        let mut grid_widget2 = GridWidget::default();

        grid_widget2
            .properties()
            .set_origin(320, 20)
            .set_bounds(280, 440)
            .set_value(PROPERTY_BORDER_WIDTH, 1)
            .set_color(PROPERTY_BORDER_COLOR, Color::BLACK)
            .set_color(PROPERTY_MAIN_COLOR, Color::WHITE)
            .set_color(PROPERTY_GRID_COLOR, Color::BLACK)
            .set_value(PROPERTY_GRID_SPACING, 10);

        cache.add(Box::new(grid_widget2), String::from("grid2"), 0);
    }
}

pub fn main() {
    let (sdl_context, _, window) = init_application("pushrod example", 620, 480);
    let mut engine = Engine::new(Box::new(PushrodExample::default()), &window);

    engine.run(sdl_context, window);
}
