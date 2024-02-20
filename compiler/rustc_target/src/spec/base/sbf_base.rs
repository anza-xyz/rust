use crate::abi::Endian;
use crate::spec::{LinkerFlavor, Cc, Lld, PanicStrategy, TargetOptions};

pub(crate) fn opts() -> TargetOptions {
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
  TargetOptions {
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
  }
}
