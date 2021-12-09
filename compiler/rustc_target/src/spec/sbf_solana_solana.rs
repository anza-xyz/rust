use crate::spec::Target;
use crate::spec::sbf_base;

pub fn target() -> Target {
    Target {
        llvm_target: "sbf".to_string(),
        pointer_width: 64,
        arch: "sbf".to_string(),
        data_layout: "e-m:e-p:64:64-i64:64-n32:64-S128".to_string(),
        options: sbf_base::opts(),
    }
}
