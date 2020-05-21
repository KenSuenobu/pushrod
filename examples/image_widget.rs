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

use sdl2::pixels::Color;
use pushrod::engine::{EventHandler, Engine};
use pushrod_widgets::caches::WidgetCache;
use pushrod_widgets::properties::{PROPERTY_MAIN_COLOR, PROPERTY_IMAGE_POSITION, PROPERTY_IMAGE_FILENAME, PROPERTY_IMAGE_SCALED};
use pushrod_widgets::system_widgets::image_widget::{COMPASS_NW, ImageWidget, COMPASS_N, COMPASS_NE, COMPASS_W, COMPASS_CENTER, COMPASS_E, COMPASS_SW, COMPASS_S, COMPASS_SE};
use pushrod_widgets::widget::Widget;

/// This const is used to store the original color of the `Widget` so that when the mouse leaves
/// the scope of the `Widget`, its main color is restored.
pub const PROPERTY_ORIGINAL_COLOR: u32 = 10000;

#[derive(Default)]
pub struct PushrodExample {
    text_id: u32,
}

impl EventHandler for PushrodExample {
    fn build_layout(&mut self, cache: &mut WidgetCache) {
        let mut widget1 = ImageWidget::default();

        &widget1
            .properties()
            .set_origin(20, 16)
            .set_bounds(60, 60)
            .set_color(PROPERTY_MAIN_COLOR, Color::BLACK)
            .set_value(PROPERTY_IMAGE_POSITION, COMPASS_NW)
            .set(PROPERTY_IMAGE_FILENAME, String::from("assets/rust-48x48.jpg"));

        cache.add(Box::new(widget1), String::from("widget1"), 0);

        let mut widget2 = ImageWidget::default();

        &widget2
            .properties()
            .set_origin(90, 16)
            .set_bounds(60, 60)
            .set_color(PROPERTY_MAIN_COLOR, Color::BLACK)
            .set_value(PROPERTY_IMAGE_POSITION, COMPASS_N)
            .set(PROPERTY_IMAGE_FILENAME, String::from("assets/rust-48x48.jpg"));

        cache.add(Box::new(widget2), String::from("widget2"), 0);

        let mut widget3 = ImageWidget::default();

        &widget3
            .properties()
            .set_origin(160, 16)
            .set_bounds(60, 60)
            .set_color(PROPERTY_MAIN_COLOR, Color::BLACK)
            .set_value(PROPERTY_IMAGE_POSITION, COMPASS_NE)
            .set(PROPERTY_IMAGE_FILENAME, String::from("assets/rust-48x48.jpg"));

        cache.add(Box::new(widget3), String::from("widget3"), 0);

        let mut widget4 = ImageWidget::default();

        &widget4
            .properties()
            .set_origin(20, 86)
            .set_bounds(60, 60)
            .set_color(PROPERTY_MAIN_COLOR, Color::BLACK)
            .set_value(PROPERTY_IMAGE_POSITION, COMPASS_W)
            .set(PROPERTY_IMAGE_FILENAME, String::from("assets/rust-48x48.jpg"));

        cache.add(Box::new(widget4), String::from("widget4"), 0);

        let mut widget5 = ImageWidget::default();

        &widget5
            .properties()
            .set_origin(90, 86)
            .set_bounds(60, 60)
            .set_color(PROPERTY_MAIN_COLOR, Color::BLACK)
            .set_value(PROPERTY_IMAGE_POSITION, COMPASS_CENTER)
            .set(PROPERTY_IMAGE_FILENAME, String::from("assets/rust-48x48.jpg"));

        cache.add(Box::new(widget5), String::from("widget5"), 0);

        let mut widget6 = ImageWidget::default();

        &widget6
            .properties()
            .set_origin(160, 86)
            .set_bounds(60, 60)
            .set_color(PROPERTY_MAIN_COLOR, Color::BLACK)
            .set_value(PROPERTY_IMAGE_POSITION, COMPASS_E)
            .set(PROPERTY_IMAGE_FILENAME, String::from("assets/rust-48x48.jpg"));

        cache.add(Box::new(widget6), String::from("widget6"), 0);

        let mut widget7 = ImageWidget::default();

        &widget7
            .properties()
            .set_origin(20, 156)
            .set_bounds(60, 60)
            .set_color(PROPERTY_MAIN_COLOR, Color::BLACK)
            .set_value(PROPERTY_IMAGE_POSITION, COMPASS_SW)
            .set(PROPERTY_IMAGE_FILENAME, String::from("assets/rust-48x48.jpg"));

        cache.add(Box::new(widget7), String::from("widget7"), 0);

        let mut widget8 = ImageWidget::default();

        &widget8
            .properties()
            .set_origin(90, 156)
            .set_bounds(60, 60)
            .set_color(PROPERTY_MAIN_COLOR, Color::BLACK)
            .set_value(PROPERTY_IMAGE_POSITION, COMPASS_S)
            .set(PROPERTY_IMAGE_FILENAME, String::from("assets/rust-48x48.jpg"));

        cache.add(Box::new(widget8), String::from("widget8"), 0);

        let mut widget9 = ImageWidget::default();

        &widget9
            .properties()
            .set_origin(160, 156)
            .set_bounds(60, 60)
            .set_color(PROPERTY_MAIN_COLOR, Color::BLACK)
            .set_value(PROPERTY_IMAGE_POSITION, COMPASS_SE)
            .set(PROPERTY_IMAGE_FILENAME, String::from("assets/rust-48x48.jpg"));

        cache.add(Box::new(widget9), String::from("widget9"), 0);

        let mut widget10 = ImageWidget::default();

        &widget10
            .properties()
            .set_origin(230,16)
            .set_bounds(80, 80)
            .set_color(PROPERTY_MAIN_COLOR, Color::BLACK)
            .set_value(PROPERTY_IMAGE_POSITION, COMPASS_NW)
            .set_bool(PROPERTY_IMAGE_SCALED)
            .set(PROPERTY_IMAGE_FILENAME, String::from("assets/rust-48x48.jpg"));

        cache.add(Box::new(widget10), String::from("widget10"), 0);

        let mut widget11 = ImageWidget::default();

        &widget11
            .properties()
            .set_origin(260,46)
            .set_bounds(120, 120)
            .set_color(PROPERTY_MAIN_COLOR, Color::BLACK)
            .set_value(PROPERTY_IMAGE_POSITION, COMPASS_NW)
            .set_bool(PROPERTY_IMAGE_SCALED)
            .set(PROPERTY_IMAGE_FILENAME, String::from("assets/rust-48x48.jpg"));

        cache.add(Box::new(widget11), String::from("widget11"), 0);

        let mut widget12 = ImageWidget::default();

        &widget12
            .properties()
            .set_origin(320, 86)
            .set_bounds(160, 160)
            .set_color(PROPERTY_MAIN_COLOR, Color::BLACK)
            .set_value(PROPERTY_IMAGE_POSITION, COMPASS_NW)
            .set_bool(PROPERTY_IMAGE_SCALED)
            .set(PROPERTY_IMAGE_FILENAME, String::from("assets/rust-48x48.jpg"));

        cache.add(Box::new(widget12), String::from("widget12"), 0);
    }
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("pushrod example", 500, 270)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    let mut engine = Engine::new(Box::new(PushrodExample::default()), &window);

    engine.run(sdl_context, window);
}
