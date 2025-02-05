use crate::abi::Endian;
use crate::spec::{Cc, cvs, LinkerFlavor, Lld, PanicStrategy, TargetOptions};

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
    KEEP(*(.text.abort_v3))
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
  .dynsym 0xFFFFFFFF00000000 : {
    *(.dynsym)
    . = ALIGN(8);
  } :dynsym
   .strtab : { *(.strtab) } :other
   .dynstr : { *(.dynstr) } :other
  /DISCARD/ : {
      *(.comment*)
      *(.eh_frame*)
      *(*hash*)
      *(.bss*)
      *(.data*)
      *(.rel.dyn*)
      *(.dynamic)
    }
}
PHDRS
{
  text PT_LOAD FLAGS(1);
  rodata PT_LOAD FLAGS(4);
  stack PT_GNU_STACK FLAGS(6);
  heap PT_LOAD FLAGS(6);
  dynsym PT_NULL FLAGS(0);
  other PT_NULL FLAGS(0);
}
";

pub fn opts() -> TargetOptions {
    let pre_link_args = TargetOptions::link_args(
        LinkerFlavor::Gnu(Cc::No, Lld::No),
        &["--threads=1", "-z", "notext", "--Bdynamic"],
    );

    TargetOptions {
        allow_asm: true,
        c_int_width: "64".into(),
        default_hidden_visibility: true,
        dll_prefix: "".into(),
        dynamic_linking: true,
        eh_frame_header: false,
        emit_debug_gdb_scripts: false,
        endian: Endian::Little,
        env: "".into(),
        executables: true,
        families: cvs!["solana"],
        link_script: Some(V0_LINKER_SCRIPT.into()),
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
        sbf_v3_link_script: Some(V3_LINKER_SCRIPT.into()),
        .. Default::default()
    }
}
