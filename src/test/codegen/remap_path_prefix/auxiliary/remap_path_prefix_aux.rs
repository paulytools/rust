// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// ignore-tidy-linelength

// compile-flags: -g  -Zremap-path-prefix-from={{cwd}} -Zremap-path-prefix-to=/the/aux-cwd -Zremap-path-prefix-from={{src-base}}/remap_path_prefix/auxiliary -Zremap-path-prefix-to=/the/aux-src

#[inline]
pub fn some_aux_function() -> i32 {
    1234
}
