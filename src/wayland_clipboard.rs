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

use smithay_clipboard::WaylandClipboard;
use wayland_client::Display;

use common::ClipboardProvider;

/// Clipboard context for Wayland clipboards.
pub struct WaylandClipboardContext {
    clip: WaylandClipboard,
}

impl WaylandClipboardContext {
    /// Create a new clipboard context.
    ///
    /// This is the only way to create a new Wayland clipboard. Calling [`ClipboardProvider::new`]
    /// will always fail since a Display is required for the Wayland clipboard.
    ///
    /// [`ClipboardProvider::new`]: ../trait.ClipboardProvider.html
    pub fn new(display: &Display) -> Self {
        WaylandClipboardContext {
            clip: WaylandClipboard::new_threaded(display),
        }
    }
}

impl ClipboardProvider for WaylandClipboardContext {
    fn get_contents(&mut self) -> Result<String, Box<Error>> {
        Ok(self.clip.load("seat0"))
    }

    fn set_contents(&mut self, data: String) -> Result<(), Box<Error>> {
        self.clip.store("seat0".into(), data);
        Ok(())
    }
}
