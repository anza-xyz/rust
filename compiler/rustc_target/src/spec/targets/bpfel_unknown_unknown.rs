use crate::spec::{Endian, Cc, Lld, LinkerFlavor, PanicStrategy, Target, TargetMetadata, TargetOptions};

pub(crate) fn target() -> Target {
    let linker_script = r"
PHDRS
{
  text PT_LOAD ;
  rodata PT_LOAD ;
  dynamic PT_DYNAMIC ;
}

SECTIONS
{
  . = SIZEOF_HEADERS;
  .text : { *(.text*) } :text
  .rodata : { *(.rodata*) } :rodata
  .data.rel.ro : { *(.data.rel.ro*) } :rodata
  .dynamic : { *(.dynamic) } :dynamic
}
";
    let pre_link_args = TargetOptions::link_args(
        LinkerFlavor::Gnu(Cc::No, Lld::No),
        &["--threads=1", "-z", "notext"],
    );
    Target {
        llvm_target: "bpf".into(),
        pointer_width: 64,
        arch: "bpf".into(),
        data_layout: "e-m:e-p:64:64-i64:64-n32:64-S128".into(),
        metadata: TargetMetadata { description: None, tier: None, host_tools: None, std: None },
        options: TargetOptions {
            allow_asm: true,
            endian: Endian::Little,
            c_int_width: "64".into(),
            env: "".into(),
            features: "+solana".into(),
            vendor: "solana".into(),
            linker_flavor: LinkerFlavor::Gnu(Cc::No, Lld::Yes),
            linker: Some("rust-lld".into()),
            link_script: Some(linker_script.into()),
            pre_link_args,
            executables: true,
            dll_prefix: "".into(),
            dynamic_linking: true,
            only_cdylib: true,
            no_default_libraries: true,
            panic_strategy: PanicStrategy::Abort,
            position_independent_executables: true,
            requires_lto: false,
            singlethread: true,
            max_atomic_width: Some(64),
            eh_frame_header: false,
            main_needs_argc_argv: false,
            emit_debug_gdb_scripts: false,
            .. Default::default()
        },
    }
}
