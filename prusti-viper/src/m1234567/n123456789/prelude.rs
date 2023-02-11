pub struct EncodingError;
pub type EncodingResult<T> = Result<T, EncodingError>;

pub struct Encoder<'v, 'tcx> {
    marker: std::marker::PhantomData<(&'v (), &'tcx ())>,
}

pub struct Aaaaaaaaaaaaaaaa<'tcx> {
    marker: std::marker::PhantomData<(&'tcx (),)>,
}

pub struct Bbbbbbbbbbbbb<'tcx> {
    marker: std::marker::PhantomData<(&'tcx (),)>,
}

impl<'tcx> Bbbbbbbbbbbbb<'tcx> {
    pub fn encode(_: &Encoder<'_, 'tcx>, _: ty::Ty<'tcx>) -> EncodingResult<Bbbbbbbbbbbbb<'tcx>> {
        todo!()
    }
}

pub use prusti_common::vir::polymorphic_vir as vir;

pub mod ty {
    pub struct Ty<'tcx> {
        marker: std::marker::PhantomData<(&'tcx (),)>,
    }
}

pub use prusti_common::vir_expr;
