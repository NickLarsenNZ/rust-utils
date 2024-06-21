use crate::Maybe;

pub trait OnError<T, E> {
    fn on_error<F, O: FnOnce(E) -> F>(self, op: O) -> Maybe<T, F>;
}

impl<T, E> OnError<T, E> for Result<T, E> {
    fn on_error<F, O: FnOnce(E) -> F>(self, op: O) -> Maybe<T, F> {
        match self {
            Ok(t) => Maybe::A(t),
            Err(e) => Maybe::B(op(e)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn result_on_error() {
        let error_value = 1u8;
        let result: Result<String, u8> = Result::Err(error_value);

        let mut indicator = u8::default();
        result.on_error(|e| indicator = e);

        assert_eq!(indicator, error_value);
    }
}
