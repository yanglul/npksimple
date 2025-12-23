 
#[derive(Default, Debug)]
pub struct Npk{
    //文件头
    pub head:[u8;20],
    //img索引表
    pub index:Vec<[u8;264]>,
    //校验位
    pub check:[u8;32],
    //data
    pub img_count:i32,

}
use crate::Access;
use std::io::Read;
use std::io::Write;

impl Access for Npk{
    fn read(&mut self,mut r:&mut dyn Read){
        let mut head_byte =  [0 as u8,20];
        let len = r.read_exact(&mut self.head).unwrap();
        let mut img_count_byte:[u8;4] = [self.head[16],self.head[17],self.head[18],self.head[19]];
        let img_count =  i32::from_le_bytes(img_count_byte);
        let 


        // r.read(&self.head);
        // let img_count =  i16::from_le_bytes(self.head[16..]);
    }

    fn wirte(&mut self, w:&mut dyn Write){
    }


}