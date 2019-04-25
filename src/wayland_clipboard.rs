/*
Copyright 2017 Avraham Weinstock

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

   http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

use std::error::Error;
use std::ffi::c_void;
use std::sync::{Arc, Mutex};

use smithay_clipboard::WaylandClipboard;
use wayland_client::protocol::wl_seat;
use wayland_client::sys::client::wl_display;
use wayland_client::{Display, GlobalManager, NewProxy};

use common::ClipboardProvider;

/// Clipboard context for Wayland clipboards.
pub struct WaylandClipboardContext {
    seat_name: Arc<Mutex<String>>,
    clip: WaylandClipboard,
}

impl WaylandClipboardContext {
    /// Create a new clipboard context.
    pub fn new(display: &Display, manager: &GlobalManager) -> Self {
        let seat_name = Arc::new(Mutex::new(String::new()));
        let seat_name_clone = seat_name.clone();
        manager
            .instantiate_range(2, 6, move |proxy: NewProxy<wl_seat::WlSeat>| {
                proxy.implement_closure(
                    move |event, _| {
                        if let wl_seat::Event::Name { name } = event {
                            *seat_name_clone.lock().unwrap() = name
                        }
                    },
                    (),
                )
            })
            .unwrap();

        WaylandClipboardContext {
            seat_name,
            clip: WaylandClipboard::new_threaded(display),
        }
    }

    /// Create a new clipboard context from an external pointer.
    pub unsafe fn new_from_external(display: *mut c_void, manager: &GlobalManager) -> Self {
        let seat_name = Arc::new(Mutex::new(String::new()));
        let seat_name_clone = seat_name.clone();
        manager
            .instantiate_range(2, 6, move |proxy: NewProxy<wl_seat::WlSeat>| {
                proxy.implement_closure(
                    move |event, _| {
                        if let wl_seat::Event::Name { name } = event {
                            *seat_name_clone.lock().unwrap() = name
                        }
                    },
                    (),
                )
            })
            .unwrap();

        WaylandClipboardContext {
            seat_name,
            clip: WaylandClipboard::new_threaded_from_external(display as *mut wl_display),
        }
    }
}

impl ClipboardProvider for WaylandClipboardContext {
    fn get_contents(&mut self) -> Result<String, Box<Error>> {
        Ok(self.clip.load(self.seat_name.lock().unwrap().clone()))
    }

    fn set_contents(&mut self, data: String) -> Result<(), Box<Error>> {
        self.clip
            .store(self.seat_name.lock().unwrap().clone(), data);
        Ok(())
    }
}
