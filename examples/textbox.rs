// Copyright 2019 The xi-editor Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use druid::widget::{Column, DynLabel, Padding, TextBox};
use druid::{AppLauncher, Data, LocalizedString, MenuDesc, Widget, WindowDesc};

fn main() {
    let window = WindowDesc::new(build_widget).menu(make_main_menu());
    AppLauncher::with_window(window)
        .use_simple_logger()
        .launch("typing is fun!".to_string())
        .expect("launch failed");
}

fn build_widget() -> impl Widget<String> {
    let mut col = Column::new();

    let textbox = TextBox::new();
    let textbox_2 = TextBox::new();
    let label = DynLabel::new(|data: &String, _env| format!("value: {}", data));

    col.add_child(Padding::uniform(5.0, textbox), 1.0);
    col.add_child(Padding::uniform(5.0, textbox_2), 1.0);
    col.add_child(Padding::uniform(5.0, label), 1.0);
    col
}

fn make_main_menu<T: Data>() -> MenuDesc<T> {
    let edit_menu = MenuDesc::new(LocalizedString::new("common-menu-edit-menu"))
        .append(druid::menu::sys::common::cut())
        .append(druid::menu::sys::common::copy())
        .append(druid::menu::sys::common::paste());

    MenuDesc::platform_default()
        .unwrap_or(MenuDesc::empty())
        .append(edit_menu)
}
