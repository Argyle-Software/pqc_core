/// Helper function for zeroing the target if the `zeroize` feature is enabled. 
/// 
/// Used for code brevity purposes. Targets must implement `Zeroize`
/// 
/// Replaces:
/// 
/// ```ignore
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
/// # use pqc_core::zero;
/// # use zeroize::Zeroize;
/// # let mut sensitive_data = 42u32;
/// # let mut more_sensitive_data = [1, 2, 3];
/// zero!(sensitive_data, more_sensitive_data);
/// # assert_eq!(sensitive_data, 0);
/// # assert_eq!(more_sensitive_data, [0, 0, 0]);
/// ```
#[cfg(feature = "zero")]
#[macro_export]
macro_rules! zero {
  ($($target:ident),*) => {
    #[cfg(feature = "zeroize")]
    {
      $( $target.zeroize(); )*
    }
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

/// Instantiates a variable number of mutable default variables
/// 
/// # Example
/// ```
/// # use pqc_core::batch_default;
/// batch_default!(String, a, b, c);
/// # assert_eq!(a, "".to_string());
/// # assert_eq!(b, "".to_string());
/// # assert_eq!(c, "".to_string());
/// ```
#[cfg(feature = "batch_default")]
#[macro_export]
macro_rules! batch_default {
  ($typename: ty, $($varname: ident),+) => {
      $(let $varname: $typename = Default::default();)+
  };
}

#[cfg(feature = "zero")]
pub use zero;

#[cfg(feature = "unique_feature")]
pub use assert_unique_feature;

#[cfg(feature = "batch_default")]
pub use batch_default;

#[cfg(test)]
mod tests {
  use super::*;
  use zeroize::Zeroize;
  #[test]
  fn variadic_zero() {
    let mut a = 42u32;
    let mut b = [1, 2, 3];
    let mut c = [4, 5, 6];

    zero!(a);
    assert_eq!(a, 0);

    zero!(b, c);
    assert_eq!(b, [0, 0, 0]);
    assert_eq!(c, [0, 0, 0]);
  }
}
