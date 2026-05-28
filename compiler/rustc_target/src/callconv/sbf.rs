use rustc_abi::TyAbiInterface;

// see https://github.com/llvm/llvm-project/blob/main/llvm/lib/Target/BPF/BPFCallingConv.td
use crate::callconv::{ArgAbi, FnAbi, Reg, Uniform};

fn classify_ret<Ty>(ret: &mut ArgAbi<'_, Ty>) {
    let size = ret.layout.size;
    let bits = size.bits();
    if !ret.layout.is_aggregate() && bits <= 64 {
        ret.extend_integer_width_to(64);
        return;
    }

    if bits <= 128 {
        ret.cast_to(Uniform::new(Reg::i64(), size));
    } else {
        ret.make_indirect();
    }
}

fn classify_arg<'a, Ty, C>(cx: &C, arg: &mut ArgAbi<'a, Ty>)
where
    Ty: TyAbiInterface<'a, C> + Copy,
{
    if arg.layout.pass_indirectly_in_non_rustic_abis(cx) {
        arg.make_indirect();
        return;
    }

    let size = arg.layout.size;
    let bits = size.bits();

    if !arg.layout.is_aggregate() && bits <= 64 {
        arg.extend_integer_width_to(64);
        return;
    }

    if bits <= 128 {
        arg.cast_to(Uniform::new(Reg::i64(), size));
    } else {
        arg.make_indirect();
    }
}

pub(crate) fn compute_abi_info<'a, Ty, C>(cx: &C, fn_abi: &mut FnAbi<'a, Ty>)
where
    Ty: TyAbiInterface<'a, C> + Copy,
{
    if !fn_abi.ret.is_ignore() {
        classify_ret(&mut fn_abi.ret);
    }

    for arg in fn_abi.args.iter_mut() {
        if arg.is_ignore() {
            continue;
        }
        classify_arg(cx, arg);
    }
}
