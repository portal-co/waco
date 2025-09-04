use core::{error::Error, fmt::Display};

pub trait Core {
    type Type: Clone;
    type Var: Clone;
    fn decl_var(&mut self, t: Self::Type) -> Self::Var;
    fn set(&mut self, target: Target<Self::Var>, val: Val<Self::Var>) -> Result<(), CoreError>;

    type SSA: Clone;
    fn get_var(&mut self, v: Val<Self::Var>) -> Option<Self::SSA> {
        return None;
    }
    fn set_var(&mut self, val: Self::Var, s: Val<Self::SSA>) -> Result<(), CoreError> {
        return Err(CoreError::Unsupported);
    }

    type Block: Clone;
    fn new_block(&mut self) -> Self::Block;
    fn enter_block(&mut self, k: Self::Block);
    fn exit_block(&mut self, term: Term<Self::Block>);
}

#[non_exhaustive]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Target<T> {
    Just(T),
}
#[non_exhaustive]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Val<T> {
    Just(T),
}
#[non_exhaustive]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Term<T> {
    Just(T),
}

#[non_exhaustive]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum CoreError {
    Unsupported,
}
impl Display for CoreError{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self{
            CoreError::Unsupported => write!(f,"unsupported"),
        }
    }
}
impl Error for CoreError{

}