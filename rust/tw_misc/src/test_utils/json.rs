// Copyright © 2017-2023 Trust Wallet.
//
// This file is part of Trust. The full Trust copyright notice, including
// terms governing use, modification, and redistribution, is contained in the
// file LICENSE at the root of the source code distribution tree.

use serde_json::Value as Json;
use std::borrow::Cow;

pub trait ToJson {
    fn to_json(&self) -> Json;
}

impl ToJson for Json {
    #[track_caller]
    fn to_json(&self) -> Json {
        self.clone()
    }
}

impl<'a> ToJson for Cow<'a, str> {
    #[track_caller]
    fn to_json(&self) -> Json {
        self.as_ref().to_json()
    }
}

impl<'a> ToJson for &'a str {
    #[track_caller]
    fn to_json(&self) -> Json {
        serde_json::from_str(self).expect("Error on deserializing JSON from string")
    }
}

#[macro_export]
macro_rules! assert_eq_json {
    ($left:expr, $right:expr) => {{
        use $crate::test_utils::json::ToJson;

        let left = $left.to_json();
        let right = $right.to_json();
        assert_eq!(left, right);
    }};
}
