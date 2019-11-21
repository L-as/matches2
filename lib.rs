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
/// Syntax: `unwrap_match!(` *expression* `,` *pattern* `=>` *result* [, *error message* ]`)`
///
/// The macro evaluates to *result* if *pattern* matches, otherwise it panics with the *error message* or a default one
/// that contains the pattern in it.
/// NB: The error message is passed through to panic! verbatim, so you can do `unwrap_match!(..., "{}", 2)`.
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
///     let i = unwrap_match!(foo, Foo::B(i) | Foo::A(i) if i < 100 => i, "An error message, omittable");
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
	};
	($expression:expr, $(|)* $pattern:pat $(|$pattern_extra:pat)* $(if $ifguard:expr)* => $result:expr, $($msg:tt)+) => {
		match $expression {
			$pattern $(|$pattern_extra)* $(if $ifguard)* => $result,
			_ => panic!($($msg)+)
		}
	}
}

/// Returns Option::Some if pattern matches with the inner value, or Option::None otherwise
///
/// This macro is especially useful with `Iterator::filter_map`.
///
/// # Examples
/// ```
/// #[macro_use]
/// extern crate matches2;
///
/// enum Foo {
///     A(i32),
///     B(f64),
/// }
///
/// fn main() {
///     let data = [Foo::A(1), Foo::B(2.0), Foo::A(3), Foo::B(4.0), Foo::A(5)];
///     let vec = data.iter()
///         .filter_map(|foo| option_match!(foo, &Foo::A(i) if i <= 3 => i))
///         .collect::<Vec<i32>>();
///     assert_eq!(&vec[..], &[1, 3]);
/// }
/// ```
#[macro_export]
macro_rules! option_match {
    ($expression:expr, $($pattern:pat)|* $(if $ifguard:expr)? => $result:expr) => {
        match $expression {
            $($pattern)|* $(if $ifguard)? => Some($result),
            _ => None
        }
    };
}

/// Assert that an expression matches a refutable pattern.
///
/// Syntax: `assert_matches!(` *expression* `,` *pattern* [, *error message* ]`)`
///
/// If the pattern does not match, this macro panics with the given error message or a default one
/// that contains the pattern in it.
/// NB: The error message is passed through to panic! verbatim, so you can do `assert_matches!(..., "{}", 2)`.
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
	};
	($expression:expr, $(|)* $pattern:pat $(|$pattern_extra:pat)* $(if $ifguard:expr)*, $($msg:tt)+) => {
		match $expression {
			$pattern $(|$pattern_extra)* $(if $ifguard)* => (),
			_ => panic!($($msg)+)
		}
	}
}

/// Assert that an expression matches a refutable pattern using debug assertions.
///
/// Syntax: `debug_assert_matches!(` *expression* `,` *pattern* [, *error message* ]`)`
///
/// If the pattern does not match while debug assertions are enabled, this macro panics with
/// the given error message or a default one that contains the pattern in it.
/// NB: The error message is passed through to panic! verbatim, so you can do `debug_assert_matches!(..., "{}", 2)`.
///
/// # Examples
///
/// ```
/// #[macro_use]
/// extern crate matches2;
///
/// fn main() {
///     let data = [1, 2, 3];
///     debug_assert_matches!(data.get(1), Some(_), "This is not supposed to happen");
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
			matches!(bar.as_bytes()[1], b'0'..=b'9')
		));
	}

	#[test]
	fn assert_matches_works() {
		let foo = Some("-12");
		assert_matches!(foo, Some(bar) if
			matches!(bar.as_bytes()[0], b'+' | b'-') &&
			matches!(bar.as_bytes()[1], b'0'..=b'9')
		);
	}

	#[test]
	#[should_panic(expected = "assertion failed: `Some(\"-AB\")` does not match ")]
	fn assert_matches_panics() {
		let foo = Some("-AB");
		assert_matches!(foo, Some(bar) if
			matches!(bar.as_bytes()[0], b'+' | b'-') &&
			matches!(bar.as_bytes()[1], b'0'..=b'9')
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
