use crate::spec::Target;
use crate::spec::sbf_base;

pub fn target() -> Target {
    Target {
        llvm_target: "bpfel".into(),
        pointer_width: 64,
        arch: "bpf".into(),
        data_layout: "e-m:e-p:64:64-i64:64-n32:64-S128".into(),
        options: sbf_base::opts(),
    }
}
