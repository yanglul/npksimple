use std::io::{BufReader,BufWriter,Read,Write,Seek};
use std::fs::File;

pub trait Access{
    fn read (&mut self, r:BufReader<File>)->std::io::Result<()>;

    fn write<W: Write>(&mut self, w:BufWriter<W>)->std::io::Result<()>;

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
use crate::npkin::IMG_NAME_KEY;
impl Imgindex{
    pub fn decodeName(&self)->Result<String,std::string::FromUtf8Error >{
        let mut name = [0 as u8;256];
        for i in 0..256{
            name[i] = (self.check[i]^IMG_NAME_KEY[i]) as u8
        }

        return String::from_utf8(name.to_vec());
    }
}


 

static TYPE_INDEXED:i32 = 0x0E;
static TYPE_ARGB1555:i32 = 0x0E;
static TYPE_ARGB4444:i32 = 0x0F;
static  TYPE_ARGB8888:i32 = 0x10;
static  TYPE_REFERENCE:i32 = 0x11;
static  TYPE_FXT1:i32 = 0x12;
static  TYPE_FXT2:i32 = 0x13;
static  TYPE_FXT3:i32 = 0x14;


#[derive(PartialEq)]
#[repr(i32)]
pub enum COMPRESSEDT{
    UNCOMPRESSED = 0x05,
    COMPRESSED = 0x06,
}
 



 
use bevy::prelude::Image;
pub struct ImageFrame{
    tp:i32,
    compressed:COMPRESSEDT,
    width:i32,
    height:i32,
    length:i32,
    x:i32,
    y:i32,
    frame_width:i32,
    frame_height:i32,
    raw_data:Vec<u8>,
    image:Image,
}


 
fn is_reference_type(tp:i32)->bool {
        return tp == TYPE_REFERENCE;
}



use std::any::Any;
trait Frame: Any{
    fn get_type(&self)->i32;
    fn is_reference(&self)->bool;
    fn is_argb(&self)->bool;
    fn is_indexed(&self)->bool;
    fn is_compressed(&self)->bool;
    fn is_fxt(&self)->bool;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
impl Frame  for ImageFrame{
    fn get_type(&self)->i32{
        return self.tp;
    }
    fn is_reference(&self)->bool{
        return false;
    }
    fn is_argb(&self)->bool{
        return  self.tp==TYPE_ARGB1555||self.tp==TYPE_ARGB4444||self.tp==TYPE_ARGB8888;
    }
    fn is_indexed(&self)->bool{
        return is_reference_type(self.tp);
    }
    
    fn is_compressed(&self)->bool {
        return self.compressed==COMPRESSEDT::COMPRESSED;
    }

    fn is_fxt(&self)->bool {
        return false;
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    
}   

pub struct ReferenceFrame{
    reference:i32,
    tp:i32,

}

impl Frame for ReferenceFrame{
    fn get_type(&self)->i32{return self.tp;}
    fn is_reference(&self)->bool{return true; }
    fn is_argb(&self)->bool{return false; }
    fn is_indexed(&self)->bool{return false;}
    fn is_compressed(&self)->bool {return false;}
    fn is_fxt(&self)->bool { return false;}
    fn as_any(&self) -> &dyn Any {self}
    fn as_any_mut(&mut self) -> &mut dyn Any {self}
}

pub struct DdsImageFrame{
    tp:i32,
    compressed:COMPRESSEDT,
    width:i32,
    height:i32,
    length:i32,
    x:i32,
    y:i32,
    frame_width:i32,
    frame_height:i32,
    raw_data:Vec<u8>,
    image:Image,
    dds_index:i32,
    left_cut: i32,
    up_cut: i32,
    right_cut: i32,
    down_cut: i32,
}

impl Frame for DdsImageFrame{
    fn get_type(&self)->i32{return self.tp;}
    fn is_reference(&self)->bool{return false; }
    fn is_argb(&self)->bool{return false; }
    fn is_indexed(&self)->bool{return false;}
    fn is_compressed(&self)->bool {return self.compressed==COMPRESSEDT::COMPRESSED;}
    fn is_fxt(&self)->bool { return self.tp == TYPE_FXT1 || self.tp == TYPE_FXT2 || self.tp == TYPE_FXT3}
    fn as_any(&self) -> &dyn Any {self}
    fn as_any_mut(&mut self) -> &mut dyn Any {self}
}


pub struct V2img{
    versin:i32,
    name:String,
    frames:Vec<Box<dyn Frame>>
}

static MAX_COLOR_SIZE:i32 = 255;
use image::Rgba; 
pub struct Palette{
    colors:Vec<Rgba<u8>>

}

pub struct V4img{
    versin:i32,
    name:String,
    palette:Palette,
    frames:Vec<Box<dyn Frame>>
}




static DDS_IMAGE:&str = "DDS";

pub struct DDS{
    title:i32,
    pixel_format: i32,
    index:i32,
    full_length:i32,
    length:i32,
    width:i32,
    height:i32,
    raw_data:Vec<u8>
}

use indexmap::IndexMap;
pub struct Ddsable{
    idxm:IndexMap<i32,DDS>
}

pub struct V5img{
    versin:i32,
    name:String,
    dds_table:Ddsable,
    frames:Vec<Box<dyn Frame>>
}

pub struct V6img{
    versin:i32,
    name:String,
    palettes:Vec<Palette>,
    frames:Vec<Box<dyn Frame>>
}