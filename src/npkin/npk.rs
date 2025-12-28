use crate::npkin::img::{Imgindex, V2img, V4img, V5img, V6img};
use crate::npkin::oggv::Ogg;

 
pub enum Texture{
    Ogg(Ogg),
    V2(V2img),
    V4(V4img),
    V5(V5img),
    V6(V6img),
}

#[derive(Default)]
pub struct Npk{
    //文件头
    pub head:[u8;20],
    //img索引表
    pub index:Vec<Imgindex>,
    //校验位
    pub check:[u8;32],
    //图片数量
    pub img_count:i32,
 
    pub textures: Vec<Texture>,



}
 
use std::io::{Read,BufReader};
 
use std::io::Result;
use std::io::SeekFrom;
use std::io::Seek;
use std::fs::File;

impl Npk{
    pub fn read(&mut self,mut r:BufReader<File>)->Result<()>{
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
        r.read_exact(&mut self.check)?;
        for i in 0..self.img_count{
            let img_inex = self.index.pop().unwrap();
            let offset = i32::from_le_bytes(img_inex.offset);
            let size = i32::from_le_bytes(img_inex.size);
            let check_name: String = img_inex.decodeName().unwrap();
            let _ = r.seek(SeekFrom::Start(offset as u64));
            // let mut limit_read = r.take(size as u64);
            // let mut data = vec![0u8;size as usize];
            let mut data = vec![0u8; size as usize];
            let _ = r.read_exact(&mut data);
            // limit_read.read_to_end(&mut data);
            println!("文件名称:{}",check_name);
             
            if check_name.ends_with(".ogg"){
                let ogg = Ogg{
                    name: check_name,
                    data: data,
                }; 
                self.textures.push(Texture::Ogg(ogg));
            }else{
                let version_byte = &data[24..28];
                let version = i32::from_le_bytes(version_byte.try_into().unwrap());
                 
                match version {
                    2 =>{
                        let mut v2img = V2img::default();
                        v2img.set_name(check_name);
                        let frame_size_bytes = &data[28..32];
                        let frame_size = i32::from_le_bytes(frame_size_bytes.try_into().unwrap());
                        


                    },
                    4=>{

                    },
                    5=>{

                    },
                    6=>{

                    },
                    _=>{
                        panic!("未知版本图片")
                    }

                    
                }
            }

        }
        

         
        Ok(())
    }

 


}