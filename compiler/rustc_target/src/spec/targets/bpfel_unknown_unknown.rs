use crate::spec::Target;
use crate::spec::sbf_base;
use crate::spec::TargetMetadata;

pub(crate) fn target() -> Target {
    Target {
        metadata: TargetMetadata { description: None, tier: None, host_tools: None, std: None },
        llvm_target: "bpfel".into(),
        pointer_width: 64,
        arch: "bpf".into(),
        data_layout: "e-m:e-p:64:64-i64:64-n32:64-S128".into(),
        options: sbf_base::opts(),
    }
}