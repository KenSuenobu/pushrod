// Pushrod Events
// Event Trait and Handler
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

use sdl2::pixels::Color;
use sdl2::video::Window;
use sdl2::Sdl;

use pushrod_events::event::Event;
use pushrod_widgets::caches::WidgetCache;
use std::thread::sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// This is an event handler that is passed into a main event loop.  Since there can be multiple
/// windows open at any one time, the event handler that is implemented using this `trait` should
/// be for the window with which it is interacting.
///
/// It is inadvisable to create a single event handler "catch-all" for all application windows.
/// You will most likely get unexpected results.
pub trait EventHandler {
    /// This is the event handler that should be implemented when the `Event` handler is used.
    /// It provides the currently active widget ID and the event that was generated.
    /// Any events that could not be translated by `Pushrod` are either swallowed, or handled
    /// directly by the `run` method.
    fn handle_event(&mut self, current_widget_id: u32, event: Event);
}

/// This is a `Pushrod` main loop struct.  All of the members of this object are
/// private, and used to track interaction with widgets on the screen, and other `SDL2`-related
/// events.
pub struct Engine {
    current_widget_id: u32,
    handler: Box<dyn EventHandler>,
    cache: WidgetCache,
    running: bool,
}

/// This is an implementation of `Pushrod`, the main loop handler.  Multiple `Pushrod`s
/// can be created for multiple windows if your application provides more than one window
/// with which to interact.
impl Engine {
    /// Creates a new `Pushrod` run loop, taking a reference to the `EventHandler` that handles
    /// run loop events for this `Window`.
    pub fn new(handler: Box<dyn EventHandler>, window: &Window) -> Self {
        Self {
            current_widget_id: 0,
            handler,
            cache: WidgetCache::new(window.size().0, window.size().1),
            running: true,
        }
    }

    /// Stops the Pushrod run loop.
    pub fn stop(&mut self) {
        self.running = false;
    }

    /// Retrieves the `WidgetCache`.
    pub fn get_cache(&mut self) -> &mut WidgetCache {
        &mut self.cache
    }

    /// This is the main event handler for the application.  It handles all of the events generated
    /// by the `SDL2` manager, and translates them into events that can be used by the `handle_event`
    /// method.
    pub fn run(&mut self, sdl: Sdl, window: Window) {
        let mut event_pump = sdl.event_pump().unwrap();
        let fps_as_ms = (1000.0 / 60_f64) as u128;
        let mut canvas = window
            .into_canvas()
            .target_texture()
            .accelerated()
            .build()
            .unwrap();

        // Generate the top level BaseWidget as widget ID 0, passing in x, y = 0,0, w, h = window size
        // Call handler.build_layout() - new callback.

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();
        canvas.present();

        'running: loop {
            let start = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis();

            // Process events first
            for event in event_pump.poll_iter() {
                match event {
                    sdl2::event::Event::Quit { .. } => break 'running,
                    unhandled_event => eprintln!("Event: {:?}", unhandled_event),
                }
            }

            // Draw after events are processed.
            self.cache.draw(0, &mut canvas);

            // Then swap the canvas once the draw is complete.
            canvas.present();

            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis();

            if now - start < fps_as_ms {
                let diff = fps_as_ms - (now - start);

                sleep(Duration::from_millis(diff as u64));
            }

            if !self.running {
                break 'running;
            }
        }
    }
}
