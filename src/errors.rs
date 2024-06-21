use crate::Maybe;


pub trait OnError<A, B> {
    fn on_error(&self, f: FnOnce(error: B)) -> Maybe<A, B>;
}

impl<A, B> OnError<A, B> for Result<A, B> {
    fn on_error(&self, f: FnOnce(error: B) -> Maybe<A, B> {
        if let Err(error) {
            f(error)
        }
        self.into() // todo: impl From for Maybe
    }
}
