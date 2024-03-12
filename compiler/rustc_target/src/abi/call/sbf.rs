// see https://github.com/llvm/llvm-project/blob/main/llvm/lib/Target/BPF/BPFCallingConv.td
use crate::abi::call::{ArgAbi, FnAbi, Reg, Uniform};

fn classify_ret<Ty>(ret: &mut ArgAbi<'_, Ty>) {
    let size = ret.layout.size;
    let bits = size.bits();
    if !ret.layout.is_aggregate() && bits <= 64 {
        ret.extend_integer_width_to(64);
        return;
    }

    if bits <= 128 {
        ret.cast_to(Uniform {unit: Reg::i64(), total: size});
    } else {
        ret.make_indirect();
    }
}

fn classify_arg<Ty>(arg: &mut ArgAbi<'_, Ty>) {
    let size = arg.layout.size;
    let bits = size.bits();
    if !arg.layout.is_aggregate() && bits <= 64 {
        arg.extend_integer_width_to(64);
        return;
    }

    if bits <= 128 {
        arg.cast_to(Uniform {unit: Reg::i64(), total: size});
    } else {
        arg.make_indirect();
    }
}

pub fn compute_abi_info<Ty>(fn_abi: &mut FnAbi<'_, Ty>) {
    if !fn_abi.ret.is_ignore() {
        classify_ret(&mut fn_abi.ret);
    }

    for arg in fn_abi.args.iter_mut() {
        if arg.is_ignore() {
            continue;
        }
        classify_arg(arg);
    }
}
