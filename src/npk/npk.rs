 
use crate::npk::img::Imgindex;

#[derive(Default, Debug)]
pub struct Npk{
    //文件头
    pub head:[u8;20],
    //img索引表
    pub index:Vec<Imgindex>,
    //校验位
    pub check:[u8;32],
    //data
    pub img_count:i32,

}
use crate::Access;
use std::io::Read;
use std::io::Write;
use std::io::Result;

impl Access for Npk{
    fn read(&mut self,mut r:&mut dyn Read)->Result<()>{
        r.read_exact(&mut self.head)?;
        let mut img_count_byte:[u8;4] = [self.head[16],self.head[17],self.head[18],self.head[19]];
        self.img_count =  i32::from_le_bytes(img_count_byte);
        println!("IMG文件个数:{}",self.img_count);
         
        for i in 0..self.img_count{
            let mut img: [u8; 264] = [0;264];
            r.read_exact(&mut img)?;
            let index_temp = Imgindex{
                offset:img[0..4].try_into().unwrap(),
                size:img[4..8].try_into().unwrap(),
                check:img[8..264].try_into().unwrap(), 
            };
            self.index.push(index_temp);
        }
        for i in 0..self.img_count{
            let img_inex = self.index.pop().unwrap();
            let offset = i32::from_le_bytes(img_inex.offset);
            let size = i32::from_le_bytes(img_inex.size);
            let check: String = img_inex.decodeName().unwrap();
            println!("文件名称:{}",check);
        }


         
        Ok(())
    }

    fn wirte(&mut self, w:&mut dyn Write)->Result<()>{
        Ok(())
    }


}