use std::io::Read;
use std::io::Write;
 
pub trait Access{
    fn read(&mut self, r:&mut dyn Read);

    fn wirte(&mut self, w:&mut dyn Write);

}

