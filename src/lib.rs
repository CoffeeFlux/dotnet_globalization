// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// A FFI to get DateFormat information

//#![no_main] // https://github.com/unicode-org/icu4x/issues/395

use icu::datetime::{options::length, DateTimeFormat};
use icu_provider::inv::InvariantDataProvider;
use icu::locid::Locale;
use icu::locid::macros::langid;

pub type DotnetGlobalizationContext = DotnetGlobalizationContextImpl;

#[no_mangle]
pub extern "C" fn Initialize(p_data: *const u8) -> *mut DotnetGlobalizationContext {

    let context = DotnetGlobalizationContextImpl {
        p_data: p_data,
        //provider: get_provider()
    };

    Box::into_raw(Box::new(context))
}

#[no_mangle]
pub unsafe extern "C" fn GetDateFormat(context: *const DotnetGlobalizationContext, value_length: usize) -> usize {
    let locale: Locale = langid!("en-us").into();
    //let provider : &InvariantDataProvider = &(*context).provider;
    let provider = get_provider();

    let options = length::Bag {
        date: Some(length::Date::Medium),
        time: Some(length::Time::Short),
        ..Default::default()
    };

    let dtf = DateTimeFormat::try_new(locale, &provider, &options.into())
        .expect("Failed to create DateTimeFormat instance.");

    return dtf.symbols.months.format.wide.0[7].len() + value_length + 608;
}

pub fn get_provider() -> InvariantDataProvider {
    InvariantDataProvider
}

#[repr(C)]
pub struct DotnetGlobalizationContextImpl {
    p_data: *const u8,
    //provider: InvariantDataProvider,
    // Other members...
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let context = Initialize(std::ptr::null());
        assert_ne!(context, std::ptr::null());
        
        unsafe {
            let res = GetDateFormat(context, 5);
            assert_eq!(res, 613);
        }
    }
}