//@ add-minicore
//@ compile-flags: --target=sbf-solana-solana
//@ needs-llvm-components: sbf
//
#![crate_type = "lib"]
#![feature(sbf_target_feature, no_core)]
#![no_core]
#![no_std]

extern crate minicore;
use minicore::*;

#[no_mangle]
#[target_feature(enable = "alu32")]
// CHECK: define {{.*}}i8 @foo(i8 {{.*}}%arg) unnamed_addr #0
// CHECK: attributes #0 = { {{.*}}"target-features"="{{[^"]*}}+alu32{{.*}} }
pub unsafe fn foo(arg: u8) -> u8 {
    arg
}
