// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

struct S;

trait Tr {
    type A;
}

impl Tr for S {
    type A = S;
}

fn f<T: Tr>() {
    match S {
        T::A {} => {} //~ ERROR `T::A` does not name a struct or a struct variant
    }
}

fn g<T: Tr<A = S>>() {
    match S {
        T::A {} => {} //~ ERROR `T::A` does not name a struct or a struct variant
    }
}

fn main() {
    match S {
        S::A {} => {} //~ ERROR ambiguous associated type
    }
}
