use rustc_abi::Endian;
use crate::spec::{Cc, cvs, LinkerFlavor, Lld, PanicStrategy, Target, TargetOptions, SymbolVisibility};

const V0_LINKER_SCRIPT: &str = r"
PHDRS
{
  text PT_LOAD ;
  rodata PT_LOAD ;
  data PT_LOAD ;
  dynamic PT_DYNAMIC ;
}

SECTIONS
{
  . = SIZEOF_HEADERS;
  .text : { *(.text*) } :text
  .rodata : { *(.rodata*) } :rodata
  .data.rel.ro : { *(.data.rel.ro*) } :rodata
  .dynamic : { *(.dynamic) } :dynamic
  .dynsym : { *(.dynsym) } :data
  .dynstr : { *(.dynstr) } :data
  .rel.dyn : { *(.rel.dyn) } :data
  /DISCARD/ : {
      *(.eh_frame*)
      *(.gnu.hash*)
      *(.hash*)
    }
}
";

const V3_LINKER_SCRIPT: &str = r"
SECTIONS
{
  .text 0x000000000 : {
    . = 0x00;
    KEEP(*(.text.abort))
     *(.text*)
  } :text
  .rodata 0x100000000 : {
    *(.rodata*)
    *(.data.rel.ro*)
    BYTE(0);
    . = ALIGN(8);
  } :rodata
  .bss.stack 0x200000000 (NOLOAD) : {
      _stack_start = .;
      . = . + 0x1000;
      _stack_end = .;
      . = ALIGN(8);
   } :stack
  .bss.heap 0x300000000 (NOLOAD) : {
        _heap_start = .;
        . = . + 0x1000;
        _heap_end = .;
        . = ALIGN(8);
   } :heap
  /DISCARD/ : {
      *(.comment*)
      *(.eh_frame*)
      *(*hash*)
      *(.bss*)
      *(.data*)
      *(.rel.dyn*)
      *(.dynamic)
      *(.dynsym)
      *(.dynstr)
    }
}
PHDRS
{
  text PT_LOAD FLAGS(1);
  rodata PT_LOAD FLAGS(4);
  stack PT_LOAD FLAGS(6);
  heap PT_LOAD FLAGS(6);
}
";

pub(crate) fn opts(version: &'static str) -> TargetOptions {
    let mut linker_args: Vec<&str> = vec![
        "--threads=1", "-z", "notext", "--Bdynamic"
    ];

    let linker_script = if version == "v3" || version == "v4" {
        V3_LINKER_SCRIPT
    } else {
        linker_args.push("-z");
        linker_args.push("max-page-size=4096");
        V0_LINKER_SCRIPT
    };

    let pre_link_args = TargetOptions::link_args(
        LinkerFlavor::Gnu(Cc::No, Lld::No),
        linker_args.as_slice(),
    );

    let cpu = if version == "v0" {
        "generic"
    } else {
        version
    };

    let features = match version {
        "v4" => "+static-syscalls,+abi-v2",
        "v3" => "+static-syscalls",
        "v0" => "+store-imm,+jmp-ext",
        _ => ""
    };

    TargetOptions {
        allow_asm: true,
        c_int_width: 64,
        default_visibility: Some(SymbolVisibility::Hidden),
        dll_prefix: "".into(),
        dynamic_linking: true,
        eh_frame_header: false,
        emit_debug_gdb_scripts: false,
        endian: Endian::Little,
        env: "".into(),
        executables: true,
        families: cvs!["solana"],
        link_script: Some(linker_script.into()),
        linker: Some("rust-lld".into()),
        linker_flavor: LinkerFlavor::Gnu(Cc::No, Lld::Yes),
        main_needs_argc_argv: false,
        max_atomic_width: Some(64),
        no_default_libraries: true,
        only_cdylib: true,
        os: "solana".into(),
        panic_strategy: PanicStrategy::Abort,
        position_independent_executables: true,
        pre_link_args,
        requires_lto: false,
        singlethread: true,
        vendor: "solana".into(),
        c_enum_min_bits: Some(32),
        cpu: cpu.into(),
        features: features.into(),
        .. Default::default()
    }
}

pub(crate) fn sbf_target(version: &'static str) -> Target {
    Target {
        llvm_target: "sbf".into(),
        pointer_width: 64,
        arch: "sbf".into(),
        data_layout: "e-m:e-p:64:64-i64:64-n32:64-S128".into(),
        options: opts(version),
        metadata: crate::spec::TargetMetadata {
            description: None,
            tier: None,
            host_tools: None,
            std: None,
        },
    }
}
