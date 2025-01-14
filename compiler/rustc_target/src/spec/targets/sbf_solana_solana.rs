use crate::spec::Target;
use crate::spec::base::sbf_base;

pub fn target() -> Target {
    Target {
        llvm_target: "sbf".into(),
        pointer_width: 64,
        arch: "sbf".into(),
        data_layout: "e-m:e-p:64:64-i64:64-n32:64-S128".into(),
        options: sbf_base::opts(),
        metadata: crate::spec::TargetMetadata {
            description: None,
            tier: None,
            host_tools: None,
            std: None,
        },
    }
}
