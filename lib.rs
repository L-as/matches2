/// Check if an expression matches a refutable pattern.
///
/// Syntax: `matches!(` *expression* `,` *pattern* `)`
///
/// Return a boolean, true if the expression matches the pattern, false otherwise.
///
/// # Examples
///
/// ```
/// #[macro_use]
/// extern crate matches2;
///
/// pub enum Foo<T> {
///     A,
///     B(T),
/// }
///
/// impl<T> Foo<T> {
///     pub fn is_a(&self) -> bool {
///         matches!(*self, Foo::A)
///     }
///
///     pub fn is_b(&self) -> bool {
///         matches!(*self, Foo::B(_))
///     }
/// }
///
/// # fn main() { }
/// ```
#[macro_export]
macro_rules! matches {
	($expression:expr, $($pattern:tt)+) => {
		match $expression {
			$($pattern)+ => true,
			_ => false
		}
	}
}

/// A general version of Option::unwrap for all enum variants.
///
/// Syntax: `unwrap_match!(` *expression* `,` *pattern* `=>` *result* `)`
///
/// The macro evaluates to *result* if *pattern* matches, otherwise it panics.
///
/// # Examples
///
/// ```
/// #[macro_use]
/// extern crate matches2;
///
/// #[derive(Debug)]
/// pub enum Foo<T> {
///     A(T),
///     B(T),
/// }
///
/// fn main() {
///     let foo = Foo::B(4);
///     let i = unwrap_match!(foo, Foo::B(i) | Foo::A(i) if i < 100 => i);
///     assert_eq!(i, 4);
/// }
/// ```
#[macro_export]
macro_rules! unwrap_match {
	($expression:expr, $(|)* $pattern:pat $(|$pattern_extra:pat)* $(if $ifguard:expr)* => $result:expr) => {
		match $expression {
			$pattern $(|$pattern_extra)* $(if $ifguard)* => $result,
			_ => panic!("assertion failed: `{:?}` does not match `{}`", $expression, stringify!($pattern $(|$pattern_extra)* $(if $ifguard)*))
		}
	}
}

/// Assert that an expression matches a refutable pattern.
///
/// Syntax: `assert_matches!(` *expression* `,` *pattern* `)`
///
/// Panic with a message that shows the expression if it does not match the
/// pattern.
///
/// # Examples
///
/// ```
/// #[macro_use]
/// extern crate matches2;
///
/// fn main() {
///     let data = [1, 2, 3];
///     assert_matches!(data.get(1), Some(_));
/// }
/// ```
#[macro_export]
macro_rules! assert_matches {
	($expression:expr, $(|)* $pattern:pat $(|$pattern_extra:pat)* $(if $ifguard:expr)*) => {
		match $expression {
			$pattern $(|$pattern_extra)* $(if $ifguard)* => (),
			_ => panic!("assertion failed: `{:?}` does not match `{}`", $expression, stringify!($pattern $(|$pattern_extra)* $(if $ifguard)*))
		}
	}
}

/// Assert that an expression matches a refutable pattern using debug assertions.
///
/// Syntax: `debug_assert_matches!(` *expression* `,` *pattern* `)`
///
/// If debug assertions are enabled, panic with a message that shows the
/// expression if it does not match the pattern.
///
/// When debug assertions are not enabled, this macro does nothing.
///
/// # Examples
///
/// ```
/// #[macro_use]
/// extern crate matches2;
///
/// fn main() {
///     let data = [1, 2, 3];
///     debug_assert_matches!(data.get(1), Some(_));
/// }
/// ```
#[macro_export]
macro_rules! debug_assert_matches {
	($($arg:tt)*) => (if cfg!(debug_assertions) { assert_matches!($($arg)*); })
}

#[cfg(test)]
mod tests {
	#[test]
	fn matches_works() {
		let foo = Some("-12");
		assert!(matches!(foo, Some(bar) if
			matches!(bar.as_bytes()[0], b'+' | b'-') &&
			matches!(bar.as_bytes()[1], b'0'...b'9')
		));
	}

	#[test]
	fn assert_matches_works() {
		let foo = Some("-12");
		assert_matches!(foo, Some(bar) if
			matches!(bar.as_bytes()[0], b'+' | b'-') &&
			matches!(bar.as_bytes()[1], b'0'...b'9')
		);
	}

	#[test]
	#[should_panic(expected = "assertion failed: `Some(\"-AB\")` does not match ")]
	fn assert_matches_panics() {
		let foo = Some("-AB");
		assert_matches!(foo, Some(bar) if
			matches!(bar.as_bytes()[0], b'+' | b'-') &&
			matches!(bar.as_bytes()[1], b'0'...b'9')
		);
	}

	#[test]
	fn unwrap_match_works() {
		#[allow(dead_code)]
		#[derive(Debug)]
		enum Foo {
			A(u32),
			B(f32),
		}

		let foo = Foo::B(0.5);
		let f = unwrap_match!(foo, Foo::B(f) => f);
		assert_eq!(f, 0.5);
	}

	#[test]
	#[should_panic(expected = "assertion failed: `B(0.5)` does not match `Foo::A(i) if i < 10`")]
	fn unwrap_match_panics() {
		#[allow(dead_code)]
		#[derive(Debug)]
		enum Foo {
			A(u32),
			B(f32),
		}

		let foo = Foo::B(0.5);
		let _i = unwrap_match!(foo, Foo::A(i) if i < 10 => i);
	}
}
