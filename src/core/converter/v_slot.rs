use super::{
    super::error::{CompilationError, CompilationErrorKind as ErrorKind},
    BaseConverter as BC, BaseIR, CoreConverter, Element, JsExpr as Js,
};
use crate::core::{flags::RuntimeHelper, parser::ElementType, util::find_dir};

pub fn convert_v_slot<'a>(bc: &BC, e: &mut Element<'a>) -> (BaseIR<'a>, bool) {
    todo!()
}

pub fn check_build_as_slot(bc: &BC, e: &Element, tag: &Js) -> bool {
    if let Some(found) = find_dir(e, "slot") {
        debug_assert!(e.tag_type != ElementType::Template);
        let dir = found.get_ref();
        if !e.is_component() {
            let error = CompilationError::new(ErrorKind::VSlotMisplaced)
                .with_location(dir.location.clone());
            bc.emit_error(error);
        }
    }
    use RuntimeHelper::{KeepAlive, Teleport};
    match tag {
        Js::Symbol(KeepAlive) => true,
        Js::Symbol(Teleport) => true,
        _ => e.is_component(),
    }
}
