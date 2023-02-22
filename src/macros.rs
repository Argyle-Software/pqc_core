/// Helper function for zeroing the target if the `zeroize` feature is enabled. 
/// 
/// Used for code brevity purposes. 
/// 
/// Replaces:
/// 
/// ```
/// #[cfg(feature = "zeroize")]
/// sensitive_data.zeroize();
/// ``` 
/// 
/// ### Arguments:
/// 
/// * target - T: Zeroize
/// 
/// ### Usage:
/// ```
/// zeroize!(sensitive_data);
#[cfg(feature = "zero")]
#[macro_export]
macro_rules! zero {
  ($target: ident) => {
    #[cfg(feature = "zeroize")]
    $target.zeroize(); 
  };
}

#[cfg(feature = "unique_feature")]
#[macro_export]
macro_rules! assert_unique_feature {
  () => {};
  ($first:tt $(,$rest:tt)*) => {
    $(
      #[cfg(all(feature = $first, feature = $rest))]
      compile_error!(concat!("features \"", $first, "\" and \"", $rest, "\" cannot be used together"));
    )*
    assert_unique_feature!($($rest),*);
  }
}

#[cfg(feature = "zero")]
pub use zero;
#[cfg(feature = "unique_feature")]
pub use assert_unique_feature;