pub mod errors;

/// A generalisation of [std::result::Result] which removes the positive and
/// negative connotations of the variants.
///
/// It is purposfully not annotated with `#[must_use]` so that the caller can
/// throw away the return value without `let _ =`.
/// It is also not annotated with `#[non_exhaustive]` to remain similar to [`Result`]
pub enum Maybe<A, B> {
    A(A),
    B(B),
}

impl<A, B> Maybe<A, B> {
    pub fn is_a(&self) -> bool {
        match self {
            Maybe::A(_) => true,
            Maybe::B(_) => false,
        }
    }

    pub fn is_b(&self) -> bool {
        match self {
            Maybe::A(_) => false,
            Maybe::B(_) => true,
        }
    }
}

impl<T, E> From<Maybe<T, E>> for Result<T, E> {
    fn from(value: Maybe<T, E>) -> Self {
        match value {
            Maybe::A(t) => Self::Ok(t),
            Maybe::B(e) => Self::Err(e),
        }
    }
}

impl<T, E> From<Result<T, E>> for Maybe<T, E> {
    fn from(value: Result<T, E>) -> Self {
        match value {
            Result::Ok(t) => Self::A(t),
            Result::Err(e) => Self::B(e),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn maybe_into_result() {
        let maybe: Maybe<String, u8> = Maybe::A("string".into());
        let result: Result<_, _> = maybe.into();

        assert!(result.is_ok())
    }

    #[test]
    fn result_into_maybe() {
        let result: Result<String, u8> = Result::Ok("string".into());
        let maybe: Maybe<_, _> = result.into();

        assert!(maybe.is_a())
    }
}
