use crate::abi::Endian;
use super::{Cc, cvs, LinkerFlavor, Lld, PanicStrategy, TargetOptions};

pub fn opts() -> TargetOptions {
    let linker_script = r"
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
    let pre_link_args = TargetOptions::link_args(
        LinkerFlavor::Gnu(Cc::No, Lld::No),
        &["--threads=1", "-z", "notext"],
    );

    TargetOptions {
        allow_asm: true,
        c_int_width: "64".into(),
        dll_prefix: "".into(),
        dynamic_linking: true,
        eh_frame_header: false,
        emit_debug_gdb_scripts: false,
        endian: Endian::Little,
        env: "".into(),
        executables: true,
        features: "+solana".into(),
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
        .. Default::default()
    }
}
