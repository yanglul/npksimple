use std::io::Read;
use std::io::Write;
 
 
pub trait Access{
    fn read(&mut self, r:&mut dyn Read)->std::io::Result<()>;

    fn wirte(&mut self, w:&mut dyn Write)->std::io::Result<()>;

}
#[derive( Debug)]
pub struct Imgindex{
    //偏移量
    pub offset:[u8;4],
    //img大小
    pub size:[u8;4],
    //解密后名称
    pub check:[u8;256],
    //

}


impl Default for Imgindex{
    fn default() -> Self {
        Self {
            offset: [0;4],
            size: [0;4],
            check:[0;256],
            
        }
    }
}
use crate::npk::IMG_NAME_KEY;
impl Imgindex{
    pub fn decodeName(&self)->Result<String,std::string::FromUtf8Error >{
        let mut name = [0 as u8;256];
        for i in 0..256{
            name[i] = (self.check[i]^IMG_NAME_KEY[i]) as u8
        }

        return String::from_utf8(name.to_vec());
    }
}

pub struct Img{

}