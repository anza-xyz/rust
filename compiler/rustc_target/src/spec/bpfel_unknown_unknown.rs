use crate::spec::abi::Abi;
use crate::spec::{LinkerFlavor, LldFlavor, PanicStrategy,
           Target, TargetOptions, TargetResult};

// All the calling conventions trigger an assertion(Unsupported calling
// convention) in llvm on BPF
pub fn abi_blacklist() -> Vec<Abi> {
    vec![
        Abi::Cdecl,
        Abi::Stdcall,
        Abi::Fastcall,
        Abi::Vectorcall,
        Abi::Thiscall,
        Abi::Aapcs,
        Abi::Win64,
        Abi::SysV64,
        Abi::PtxKernel,
        Abi::Msp430Interrupt,
        Abi::X86Interrupt,
        Abi::AmdGpuKernel,
    ]
}

pub fn target() -> TargetResult {
    Ok(Target {
        llvm_target: "bpf".to_string(),
        data_layout: "e-m:e-p:64:64-i64:64-n32:64-S128".to_string(),
        target_endian: "little".to_string(),
        target_pointer_width: "64".to_string(),
        target_c_int_width: "64".to_string(),
        target_os: "unknown".to_string(),
        target_env: String::new(),
        target_vendor: "unknown".to_string(),
        arch: "bpf".to_string(),
        linker_flavor: LinkerFlavor::Lld(LldFlavor::Ld),

        options: TargetOptions {
            executables: true,
            dll_prefix: "".to_string(),
            dynamic_linking: true,
            no_builtins: true,
            no_default_libraries: true,
            panic_strategy: PanicStrategy::Abort,
            position_independent_executables: true,
            singlethread: true,
            max_atomic_width: Some(64),
            unsupported_abis: abi_blacklist(),
            eh_frame_header: false,
            .. Default::default()
        },
    })
}
