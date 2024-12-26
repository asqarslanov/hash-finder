#[cfg(feature = "ecosystem")]
pub use ecosystem::find;
#[cfg(not(feature = "ecosystem"))]
pub use no_ecosystem::find;

#[cfg(feature = "ecosystem")]
mod ecosystem;
#[cfg(not(feature = "ecosystem"))]
mod no_ecosystem;
