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

//! File open/save dialogs, macOS implementation.

#![allow(non_upper_case_globals)]

use std::ffi::OsString;

use cocoa::base::{id, nil, YES};
use cocoa::foundation::{NSArray, NSInteger};

use crate::dialog::FileDialogOptions;
use crate::util::{from_nsstring, make_nsstring};

const NSModalResponseOK: NSInteger = 1;
const NSModalResponseCancel: NSInteger = 0;

pub(crate) unsafe fn show_open_file_dialog_sync(options: FileDialogOptions) -> Option<OsString> {
    let nsopenpanel = class!(NSOpenPanel);
    let panel: id = msg_send![nsopenpanel, openPanel];

    // set options

    if options.show_hidden {
        msg_send![panel, setShowsHiddenFiles: YES];
    }

    // A vector of NSStrings. this must outlive `nsarray_allowed_types`.
    let allowed_types = options.allowed_types.as_ref().map(|specs| {
        specs
            .iter()
            .flat_map(|spec| spec.extensions.iter().map(|s| make_nsstring(s)))
            .collect::<Vec<_>>()
    });

    let nsarray_allowed_types = allowed_types
        .as_ref()
        .map(|types| NSArray::arrayWithObjects(nil, types.as_slice()));
    if let Some(nsarray) = nsarray_allowed_types {
        msg_send![panel, setAllowedFileTypes: nsarray];
    }

    let result: NSInteger = msg_send![panel, runModal];
    match result {
        NSModalResponseOK => {
            let url: id = msg_send![panel, URL];
            let path: id = msg_send![url, path];
            let path: OsString = from_nsstring(path).into();
            Some(path)
        }
        NSModalResponseCancel => None,
        _ => unreachable!(),
    }
}
