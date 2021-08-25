// check-pass

#![feature(const_fn_trait_bound)]
#![feature(const_trait_impl)]
#![feature(const_trait_bound_opt_out)]
#![allow(incomplete_features)]

struct S;

impl PartialEq for S {
    fn eq(&self, _: &S) -> bool {
        true
    }
}

const fn equals_self<T: PartialEq>(t: &T) -> bool {
    true
}

pub const EQ: bool = equals_self(&S);

fn main() {}
