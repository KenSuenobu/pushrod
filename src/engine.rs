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

use sdl2::video::Window;
use sdl2::Sdl;

use pushrod_widgets::caches::WidgetCache;
use pushrod_widgets::event::PushrodEvent::DrawFrame;
use pushrod_widgets::event::{Event, PushrodEvent};
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
    /// directly by the `run` method.  The cache is also provide as a way to get access to any
    /// `Widget`s in the list that need to be modified as the result of acting upon an `Event`.
    ///
    /// If this method is not implemented, it does not have any effect on the main application.
    fn handle_event(&mut self, _event: Event, _cache: &mut WidgetCache) {}

    /// This callback is used when the screen needs to be built for the first time.  It is called
    /// by the `Engine`'s `run` method before the event loop starts.  The `cache` is sent such that
    /// `Widget`s can be added to the display list by using the `WidgetCache`'s functions.
    ///
    /// This function **must** be implemented, as it creates a layout for the application `Window`
    /// upon creation.
    fn build_layout(&mut self, cache: &mut WidgetCache);
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

    /// Broadcasts a generated `PushrodEvent` to the current `Widget`, capturing the response, and
    /// forwarding it on to the application if an event was returned.
    fn send_event_to_widget(&mut self, widget_id: u32, event: PushrodEvent) {
        let handled_event = self.cache.get(widget_id).handle_event(event);

        if let Some(x) = handled_event {
            self.handler
                .handle_event(Event::Pushrod(x), &mut self.cache)
        }
    }

    /// Sends an event to all `Widget`s.
    fn send_event_to_all(&mut self, event: PushrodEvent) {
        let cache_size = self.cache.size();

        for i in 0..cache_size {
            let handled_event = self.cache.get(i).handle_event(event.clone());

            if let Some(x) = handled_event {
                self.handler
                    .handle_event(Event::Pushrod(x), &mut self.cache)
            }
        }
    }

    /// This function handles the `MouseMotion` event, converting it into an `Event` that can be
    /// used by `Pushrod`.  The X and Y coordinates are translated into relative offsets based on the
    /// position of the `Widget`.  This way, the X and Y coordinates can be based on drawing
    /// functions inside the `Widget` if necessary.
    fn handle_mouse_move(&mut self, x: u32, y: u32) {
        let cur_widget_id = self.current_widget_id;

        self.current_widget_id = self.cache.id_at_point(x as u32, y as u32);

        if cur_widget_id != self.current_widget_id {
            let exited_event = PushrodEvent::WidgetMouseExited {
                widget_id: cur_widget_id,
            };

            self.handler
                .handle_event(Event::Pushrod(exited_event.clone()), &mut self.cache);

            self.send_event_to_widget(cur_widget_id, exited_event);

            let entered_event = PushrodEvent::WidgetMouseEntered {
                widget_id: self.current_widget_id,
            };

            self.handler
                .handle_event(Event::Pushrod(entered_event.clone()), &mut self.cache);

            self.send_event_to_widget(self.current_widget_id, entered_event);
        }

        let points = self
            .cache
            .get(self.current_widget_id)
            .properties()
            .get_origin();
        let event = PushrodEvent::MouseMoved {
            widget_id: self.current_widget_id,
            x: (x - points.0),
            y: (y - points.1),
        };

        self.handler
            .handle_event(Event::Pushrod(event.clone()), &mut self.cache);

        self.send_event_to_widget(self.current_widget_id, event);
    }

    /// Handles a `MouseButton` event, which indicates that a mouse button has been pressed or released.
    fn handle_mouse_button(&mut self, mouse_button: u32, state: bool) {
        let event = PushrodEvent::MouseButton {
            widget_id: self.current_widget_id,
            button: mouse_button,
            state,
        };

        self.handler
            .handle_event(Event::Pushrod(event.clone()), &mut self.cache);

        if !state {
            self.send_event_to_all(event);
        } else {
            self.send_event_to_widget(self.current_widget_id, event);
        }
    }

    /// Handles a draw frame event.
    fn handle_draw_frame(&mut self, timestamp: u128) {
        let event = DrawFrame { timestamp };

        self.handler
            .handle_event(Event::Pushrod(event.clone()), &mut self.cache);

        let widget_count = self.cache.size();

        for i in 0..widget_count {
            let handled_event = self.cache.get(i).handle_event(event.clone());

            if let Some(x) = handled_event {
                self.handler
                    .handle_event(Event::Pushrod(x), &mut self.cache)
            }
        }
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

        // Call handler.build_layout() - this allows the application to build its `Window` contents,
        // preparing the application for use.  (This is where the deserialization will occur.)
        self.handler.build_layout(&mut self.cache);

        'running: loop {
            let start = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis();

            // Process events first
            for event in event_pump.poll_iter() {
                match event {
                    // TODO Change this to a callback that allows the application to handle a
                    // TODO closure of a window.  Returning true breaks the loop, false does not.
                    sdl2::event::Event::Quit { .. } => break 'running,

                    sdl2::event::Event::MouseMotion { x, y, .. } => {
                        self.handle_mouse_move(x as u32, y as u32);
                    }

                    sdl2::event::Event::MouseButtonDown { mouse_btn, .. } => {
                        self.handle_mouse_button(mouse_btn as u32, true);
                    }

                    sdl2::event::Event::MouseButtonUp { mouse_btn, .. } => {
                        self.handle_mouse_button(mouse_btn as u32, false);
                    }

                    unhandled_event => eprintln!("Event: {:?}", unhandled_event),
                }
            }

            // Tick event
            self.handle_draw_frame(
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis(),
            );

            if self.cache.invalidated() {
                // Draw after events are processed.
                self.cache.refresh(&mut canvas);
            }

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
