// TODO: move it to separate crate
pub trait Pin {
    fn high(&mut self);
    fn low(&mut self);
}


impl<U, L> Pin for (U, L)
    where U: FnMut(),
          L: FnMut()
{
    fn high(&mut self) {
        self.0()
    }

    fn low(&mut self) {
        self.1()
    }
}
