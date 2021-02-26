use crate::abi::call::{ArgAbi, FnAbi};

fn classify_ret_ty<Ty>(ret: &mut ArgAbi<'_, Ty>) {
    if ret.layout.is_aggregate() || ret.layout.size.bits() > 64 {
        if ret.layout.size.bits() != 128 {
            ret.make_indirect();
        }
    } else {
        ret.extend_integer_width_to(64);
    }
}

fn classify_arg_ty<Ty>(arg: &mut ArgAbi<'_, Ty>) {
    if arg.layout.is_aggregate() || arg.layout.size.bits() > 64 {
        if arg.layout.size.bits() != 128 {
            arg.make_indirect();
        }
    } else {
        arg.extend_integer_width_to(64);
    }
}

pub fn compute_abi_info<Ty>(fty: &mut FnAbi<'_,Ty>) {
    if !fty.ret.is_ignore() {
        classify_ret_ty(&mut fty.ret);
    }

    for arg in &mut fty.args {
        if arg.is_ignore() {
            continue;
        }
        classify_arg_ty(arg);
    }
}
