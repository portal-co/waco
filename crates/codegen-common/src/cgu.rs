use core::{
    convert::Infallible,
    ops::{Deref, DerefMut},
    pin::Pin,
};

use awaiter_trait::{Awaiter, Coroutine};

pub trait Codegen<T, Cx: ?Sized> {
    type Manifest;
    fn codegen(&mut self, input: T, context: &Cx) -> Self::Manifest;
}
impl<'a, T, Cx: ?Sized, U: Codegen<T, Cx>> Codegen<T, dyn Deref<Target = Cx> + 'a> for U {
    type Manifest = <U as Codegen<T, Cx>>::Manifest;
    fn codegen(&mut self, input: T, context: &(dyn Deref<Target = Cx> + 'a)) -> Self::Manifest {
        self.codegen(input, Deref::deref(context))
    }
}
pub trait CodegenExt<T, Cx: ?Sized>: Codegen<T, Cx> {
    fn codegen_many<U>(
        &mut self,
        x: impl Iterator<Item = (T, U)>,
        context: &Cx,
    ) -> impl Iterator<Item = (Self::Manifest, U)> {
        return x.map(move |(v, u)| (self.codegen(v, context), u));
    }
    fn codegen_thread<E, U>(
        &mut self,
        mut x: impl FnMut() -> Result<(T, U), E>,
        context: &Cx,
        mut take: impl FnMut(U, Self::Manifest) -> Result<(), E>,
    ) -> Result<Infallible, E> {
        loop {
            let (a, u) = x()?;
            let a = self.codegen(a, context);
            take(u, a)?;
        }
    }
    async fn codegen_task<E, U>(
        &mut self,
        mut x: impl AsyncFnMut() -> Result<(T, U), E>,
        context: &Cx,
        mut take: impl AsyncFnMut(U, Self::Manifest) -> Result<(), E>,
        coro: &impl Coroutine,
    ) -> Result<Infallible, E> {
        return coro
            .exec(|a| {
                self.codegen_thread(
                    || {
                        let mut x = x();
                        let x = unsafe { Pin::new_unchecked(&mut x) };
                        a.r#await(x)
                    },
                    context,
                    |a2, b| {
                        let mut x = take(a2, b);
                        let x = unsafe { Pin::new_unchecked(&mut x) };
                        a.r#await(x)
                    },
                )
            })
            .await;
    }
}
impl<T, Cx: ?Sized, C: Codegen<T, Cx> + ?Sized> CodegenExt<T, Cx> for C {}
