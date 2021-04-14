// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// A FFI to get DateFormat information

//#![no_main] // https://github.com/unicode-org/icu4x/issues/395

use icu::locid::Locale;
use icu::locid::macros::langid;

#[no_mangle]
pub extern fn GetDateFormat2(value_length: usize) -> usize {
    let locale: Locale = langid!("en").into();

    return locale.language.as_str().len() + value_length;
}
