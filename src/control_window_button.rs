/*
 *  Me TV — It's TV for me computer.
 *
 *  A GTK+/GStreamer client for watching and recording DVB.
 *
 *  Copyright © 2017, 2018  Russel Winder
 *
 *  This program is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

use std::cell::RefCell;
use std::rc::Rc;

use gtk;
use gtk::prelude::*;

use channel_names::encode_to_mrl;
use control_window::ControlWindow;
use frontend_manager::FrontendId;
use frontend_window::FrontendWindow;
use metvcomboboxtext::{MeTVComboBoxText, MeTVComboBoxTextExt};

/// A `ControlWindowButton` is a `gtk::Box` but there is no inheritance so use
/// a bit of composition.
pub struct ControlWindowButton {
    pub control_window: Rc<ControlWindow>, // FrontendWindow instance needs access to this.
    pub frontend_id: FrontendId, // ControlWindow instance needs access to this for searching.
    pub widget: gtk::Box, // ControlWindow instance needs access to this for packing.
    pub frontend_button: gtk::ToggleButton, // FrontendWindow needs access to this.
    pub channel_selector: MeTVComboBoxText, // FrontendWindow needs read access to this.
    frontend_window: RefCell<Option<Rc<FrontendWindow>>>,
}

impl ControlWindowButton {
    /// Construct a new button representing an available front end.
    ///
    /// The adapter and frontend numbers for the label for a toggle button that is used
    /// to start and stop a frontend window displaying the stream for that frontend. Below
    /// is a drop down list button to select the channel to tune the front end to.
    ///
    /// This function is executed in the GTK event loop thread.
    pub fn new(control_window: &Rc<ControlWindow>, fei: FrontendId) -> Rc<ControlWindowButton> {
        let frontend_id = fei;
        let frontend_button = gtk::ToggleButton::new_with_label(
            format!("adaptor{}\nfrontend{}", frontend_id.adapter, frontend_id.frontend).as_ref()
        );
        let channel_selector = MeTVComboBoxText::new_with_core_model(&control_window.channel_names_store);
        let widget = gtk::Box::new(gtk::Orientation::Vertical, 0);
        widget.pack_start(&frontend_button, true, true, 0);
        widget.pack_start(&channel_selector, true, true, 0);
        let cwb = Rc::new(ControlWindowButton {
            control_window: control_window.clone(),
            frontend_id,
            widget,
            frontend_button,
            channel_selector,
            frontend_window: RefCell::new(None),
        });
        cwb.reset_active_channel();
        cwb.channel_selector.connect_changed({
            let c_w_b = cwb.clone();
            move |_| Self::on_channel_changed(&c_w_b, c_w_b.channel_selector.get_active())
        });
        cwb.frontend_button.connect_toggled({
            let c_w_b = cwb.clone();
            move |_| {
                if c_w_b.control_window.is_channels_store_loaded() {
                    Self::toggle_button(&c_w_b);
                } else {
                    let dialog = gtk::MessageDialog::new(
                        Some(&c_w_b.control_window.window),
                        gtk::DialogFlags::MODAL,
                        gtk::MessageType::Info,
                        gtk::ButtonsType::Ok,
                        "No channel file, so no channel list, so cannot play a channel.");
                    dialog.run();
                    dialog.destroy();
                    //button.set_active(false); // TODO causes the reissuing of the signal. :-(
                }
            }
        });
        cwb
    }

    /// Set the active channel to 0.
    pub fn reset_active_channel(&self) {
        self.channel_selector.set_active(0);
        if let Some(ref frontend_window) = *self.frontend_window.borrow() {
            frontend_window.channel_selector.set_active(0);
            frontend_window.fullscreen_channel_selector.set_active(0);
        }
    }

    /// Set the state of the channel control widgets.
    fn set_channel_index(&self, channel_index: i32) {
        let current = self.channel_selector.get_active();
        if current != channel_index {
            self.channel_selector.set_active(channel_index);
            if let Some(ref frontend_window) = *self.frontend_window.borrow() {
                frontend_window.channel_selector.set_active(channel_index);
                frontend_window.fullscreen_channel_selector.set_active(channel_index);
            }
        }
    }

    /// Toggle the button.
    ///
    /// This function is called after the change of state of the frontend_button.
    fn toggle_button(control_window_button: &Rc<ControlWindowButton>) {
        if control_window_button.frontend_button.get_active() {
            if control_window_button.control_window.is_channels_store_loaded() && control_window_button.channel_selector.get_active() >= 0 {
                let frontend_window = FrontendWindow::new(&control_window_button);
                match control_window_button.frontend_window.replace(Some(frontend_window)) {
                    Some(_) => panic!("Inconsistent state of frontend,"),
                    None => {},
                };
            }
        } else {
            match control_window_button.frontend_window.replace(None) {
                Some(ref frontend_window) => frontend_window.stop(),
                None => panic!("Inconsistent state of frontend,"),
            }
        }
    }

    /// Callback for an observed channel change.
    pub fn on_channel_changed(control_window_button: &Rc<ControlWindowButton>, channel_index: i32) {
        let status = control_window_button.frontend_button.get_active();
        if let Some(ref frontend_window) = *control_window_button.frontend_window.borrow() {
            if status {
                frontend_window.engine.stop();
            }
            control_window_button.set_channel_index(channel_index);
            frontend_window.engine.set_mrl(&encode_to_mrl(&control_window_button.channel_selector.get_active_text().unwrap()));
            if status {
                // TODO Must handle not being able to tune to a channel better than panicking.
                frontend_window.engine.play();
            }
        }
    }

}
