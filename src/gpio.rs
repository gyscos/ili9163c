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

pub struct DummyPin;

impl Pin for DummyPin {
    fn high(&mut self) {}
    fn low(&mut self) {}
}


#[cfg(test)]
pub struct DebugPin {
    name: ::std::string::String,
}

#[cfg(test)]
use ::std::string::ToString;

#[cfg(test)]
impl DebugPin {
    pub fn new<S: ToString>(name: S) -> Self {
        DebugPin { name: name.to_string() }
    }
}

#[cfg(test)]
impl Pin for DebugPin {
    fn high(&mut self) {
        println!("{} HIGH", self.name);
    }

    fn low(&mut self) {
        println!("{} LOW", self.name);
    }
}
