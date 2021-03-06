use crate::deriving::generic::ty::*;
use crate::deriving::generic::*;
use crate::deriving::path_std;

use rustc_expand::base::{Annotatable, ExtCtxt};
use rustc_span::Span;
use syntax::ast::MetaItem;

pub fn expand_deriving_copy(
    cx: &mut ExtCtxt<'_>,
    span: Span,
    mitem: &MetaItem,
    item: &Annotatable,
    push: &mut dyn FnMut(Annotatable),
) {
    let trait_def = TraitDef {
        span,
        attributes: Vec::new(),
        path: path_std!(cx, marker::Copy),
        additional_bounds: Vec::new(),
        generics: LifetimeBounds::empty(),
        is_unsafe: false,
        supports_unions: true,
        methods: Vec::new(),
        associated_types: Vec::new(),
    };

    trait_def.expand(cx, mitem, item, push);
}
