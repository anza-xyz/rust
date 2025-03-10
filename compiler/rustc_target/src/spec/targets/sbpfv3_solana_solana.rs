use crate::spec::Target;
use crate::spec::base::sbf_base;

pub(crate) fn target() -> Target {
    sbf_base::sbf_target("v3")
}
