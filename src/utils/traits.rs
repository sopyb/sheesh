pub trait IteratorExt: Iterator + Clone {
    fn peek(&mut self) -> Option<Self::Item>;
    fn advance(&mut self) -> Option<Self::Item>;
}

impl<I: Iterator + Clone> IteratorExt for I {
    fn peek(&mut self) -> Option<Self::Item> {
        self.clone().next()
    }

    fn advance(&mut self) -> Option<Self::Item> {
        self.next()
    }
}
