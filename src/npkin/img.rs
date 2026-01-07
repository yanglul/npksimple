use std::default;
use std::io::{BufReader,BufWriter,Read,Write,Seek};
use std::fs::File;
use std::ops::Deref;


 
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


#[derive(Debug,Clone,PartialEq)]
#[repr(i32)]
pub enum COMPRESSEDT{
    UNCOMPRESSED = 0x05,
    COMPRESSED = 0x06,
}
 



 
use bevy::ecs::relationship::RelationshipSourceCollection;
use bevy::reflect::Reflect;
use bevy::utils::default;
use image::{ImageBuffer,RgbaImage };
#[derive(Debug,Clone)]
pub struct ImageFrame{
    tp:i32,
    compressed:COMPRESSEDT,
    width:u32,
    height:u32,
    length:u32,
    x:u32,
    y:u32,
    frame_width:u32,
    frame_height:u32,
    raw_data:Vec<u8>,
    image:RgbaImage,
}

impl ImageFrame{
    pub fn default()->ImageFrame{
        ImageFrame{
            tp:TYPE_INDEXED,
            compressed:COMPRESSEDT::COMPRESSED,
            width:0,
            height:0,
            length:0,
            x:0,
            y:0,
            frame_width:0,
            frame_height:0,
            raw_data:Vec::new(),
            image: RgbaImage::new(0,0),
        }
    }
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
    fn reference_push_back(&mut self,index:usize);
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
impl Frame  for ImageFrame{
    fn get_type(&self)->i32{ return self.tp;}
    fn is_reference(&self)->bool{return false;}
    fn is_argb(&self)->bool{
        return  self.tp==TYPE_ARGB1555||self.tp==TYPE_ARGB4444||self.tp==TYPE_ARGB8888;
    }
    fn is_indexed(&self)->bool{ return is_reference_type(self.tp); }
    
    fn is_compressed(&self)->bool { return self.compressed==COMPRESSEDT::COMPRESSED; }

    fn is_fxt(&self)->bool { return false;  }
    
    fn as_any(&self) -> &dyn Any { self }
    
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    
    fn reference_push_back(&mut self,_index:usize){}
}   

pub struct ReferenceFrame{
    reference:usize,
    tp:i32,
    frame:Box<dyn Frame>,
}
impl ReferenceFrame{
    fn default() -> Self {
        ReferenceFrame{reference:0,tp:TYPE_REFERENCE,frame:Box::new(ImageFrame::default())}
    }
    fn set_ref(&mut self,bf:Box<dyn Frame>){
        self.frame = bf;
    }
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
    fn reference_push_back(&mut self, index: usize) {self.reference = self.reference+index}
    
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
    image:RgbaImage,
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
    fn reference_push_back(&mut self, _: usize) { todo!() }
}

pub struct  V2img<'a>{
    versin:i32,
    name:String,
    frames_size:i32,
    frames:Vec<HashMap<&'a str,i32>>,
    row_datas:HashMap<i32, Vec<u8>>,
}
use crate::npkin::NPK_MAGIC;
fn verify_magic(mc:[u8;16]){
    for i in 0..16{
        if NPK_MAGIC[i]!=mc[i]{
            panic!("Not a Img file.");
        }
    }
}
use std::collections::VecDeque;
use std::collections::HashMap;
impl V2img <'_>{
    pub fn default() -> Self {
        V2img{versin:2,name:"".to_string(),frames_size:0,frames: Vec::new(),row_datas:HashMap::new()}
    }
    pub fn set_name(&mut self, name: String){
        self.name = name;
    }

    pub fn read_header(&mut self,data:Vec<u8>){
        let magic = &data[0..16];
        verify_magic(magic.try_into().unwrap());
        let frame_size_bytes = &data[28..32];
        self.frames_size = i32::from_le_bytes(frame_size_bytes.try_into().unwrap());
    }
    pub fn read_frames(&mut self,data:Vec<u8>){
        let mut queue: VecDeque<u8> = VecDeque::from(data.clone());
        let mut frmeinfo: Vec<HashMap<&str,i32>> = Vec::new();
        let mut frme_data_info: HashMap<i32,Vec<u8>> = HashMap::new();
        for i in 0..self.frames_size {
            let tp_byte:Vec<_>= queue.drain(..4).collect();
            let tp = i32::from_le_bytes(tp_byte.try_into().unwrap());
            let mut  map = HashMap::new();
            if TYPE_REFERENCE==tp{
                let mut ref_frame = ReferenceFrame::default();
                ref_frame.tp = tp;
                let ref_c:Vec<u8> = queue.drain(..4).collect();
                let ref_s = i32::from_le_bytes(ref_c.try_into().unwrap());
                map.insert("tp", tp);
                map.insert("ref", ref_s);
                
            }else{
                map.insert("tp", tp);
                map.insert("ref", i);
                let compressed  = i32::from_le_bytes(queue.drain(..4).collect::<Vec<u8>>().try_into().unwrap());
                map.insert("compressed", compressed);
                let width = i32::from_le_bytes(queue.drain(..4).collect::<Vec<u8>>().try_into().unwrap());
                map.insert("width", width);
                let height = i32::from_le_bytes(queue.drain(..4).collect::<Vec<u8>>().try_into().unwrap());
                map.insert("height", height);
                let length = i32::from_le_bytes(queue.drain(..4).collect::<Vec<u8>>().try_into().unwrap());
                map.insert("length", length);
                let x = i32::from_le_bytes(queue.drain(..4).collect::<Vec<u8>>().try_into().unwrap());
                map.insert("x", x);
                let y = i32::from_le_bytes(queue.drain(..4).collect::<Vec<u8>>().try_into().unwrap());
                map.insert("y", y);
                let frame_width = i32::from_le_bytes(queue.drain(..4).collect::<Vec<u8>>().try_into().unwrap());
                map.insert("frame_width", frame_width);
                let frame_height = i32::from_le_bytes(queue.drain(..4).collect::<Vec<u8>>().try_into().unwrap());
                map.insert("frame_height", frame_height);
                
            }
            frmeinfo.push(map);
        }
        for i in 0..self.frames_size {
            let f = frmeinfo.get(i as usize).unwrap();
            let tp = f.get("tp").unwrap();
            if !is_reference_type(*tp){
                let length = *f.get("length").unwrap() as usize;
                let row_data:Vec<u8> = queue.drain(..length).collect::<Vec<u8>>().try_into().unwrap();
                frme_data_info.insert(i, row_data);
            }


        }
        self.frames = frmeinfo;
        self.row_datas = frme_data_info;
    }


}

 

fn read_frame(q:&mut VecDeque<u8>,tp:i32 )->HashMap<&str,i32>{
    let mut  map = HashMap::new();
    if TYPE_REFERENCE==tp{
        let mut ref_frame = ReferenceFrame::default();
        ref_frame.tp = tp;
        let ref_c:Vec<u8> = q.drain(..4).collect();
        let ref_s = i32::from_le_bytes(ref_c.try_into().unwrap());
        map.insert("tp", tp);
        map.insert("ref", ref_s);
        map
    }else{ 
        let compressed  = i32::from_le_bytes(q.drain(..4).collect::<Vec<u8>>().try_into().unwrap());
        map.insert("compressed", compressed);
        let width = i32::from_le_bytes(q.drain(..4).collect::<Vec<u8>>().try_into().unwrap());
        map.insert("width", width);
        let height = i32::from_le_bytes(q.drain(..4).collect::<Vec<u8>>().try_into().unwrap());
        map.insert("height", height);
        let length = i32::from_le_bytes(q.drain(..4).collect::<Vec<u8>>().try_into().unwrap());
        map.insert("length", length);
        let x = i32::from_le_bytes(q.drain(..4).collect::<Vec<u8>>().try_into().unwrap());
        map.insert("x", x);

        let y = i32::from_le_bytes(q.drain(..4).collect::<Vec<u8>>().try_into().unwrap());
        map.insert("y", y);
        let frame_width = i32::from_le_bytes(q.drain(..4).collect::<Vec<u8>>().try_into().unwrap());
        map.insert("frame_width", frame_width);
        let frame_height = i32::from_le_bytes(q.drain(..4).collect::<Vec<u8>>().try_into().unwrap());
        map.insert("frame_height", frame_height);
        map
    }
}



static MAX_COLOR_SIZE:i32 = 255;
use image::Rgba; 
#[derive(Default, Debug)]
pub struct Palette{
    colors:Vec<Rgba<u8>>

}
 
pub struct V4img{
    versin:i32,
    name:String,
    palette:Palette,
    frames:Vec<Box<dyn Frame>>
}

impl V4img{
    pub fn default() -> Self {
        V4img{versin:2,name:"".to_string(),palette:Palette::default(), frames: Vec::new()}
    }
}


static DDS_IMAGE:&str = "DDS";
#[derive(Default, Debug)]
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
#[derive(Default, Debug)]
pub struct Ddsable{
    idxm:IndexMap<i32,DDS>
}
 
pub struct V5img{
    versin:i32,
    name:String,
    dds_table:Ddsable,
    frames:Vec<Box<dyn Frame>>
}
impl V5img{
    pub fn default() -> Self {
        V5img{versin:2,name:"".to_string(),dds_table:Ddsable::default(), frames: Vec::new()}
    }
}

 
pub struct V6img{
    versin:i32,
    name:String,
    palettes:Vec<Palette>,
    frames:Vec<Box<dyn Frame>>
}
impl V6img{
    pub fn default() -> Self {
        V6img{versin:2,name:"".to_string(),palettes:Vec::new(), frames: Vec::new()}
    }
}
pub trait Img{
    fn add_ref_frame(&mut self,index:usize,ref_index:usize);
    fn add_frame(&mut self,index:usize,f:Box<dyn Frame>);
    fn add_frame_tp(&mut self,index:usize,tp:i32,pic:RgbaImage); 
    fn get_frame(&self,index:usize)->Box<Vec<u8>>;
    fn get_name(&self)->String;
    fn get_version(&self)->i32;
}


 

impl Img for V2img<'_>{

     
    fn add_ref_frame(&mut self,index:usize,ref_index:usize){
        
    }

    fn add_frame(&mut self,index:usize,f:Box<dyn Frame>){
        
    }
    fn add_frame_tp(&mut self,index:usize,tp:i32,pic:RgbaImage){
        let mut temp_image = ImageFrame::default();
        temp_image.tp = tp;
        temp_image.width = pic.width();
        temp_image.height = pic.height();
        let mut cood = pic.enumerate_pixels();
        let temp = cood.next().unwrap();
        let mut minx = temp.0;
        let mut miny = temp.1;
        for (x,y,_pixel) in cood{
            if x<minx{
                minx = x;
            }
            if y<miny{
                miny = y;
            }
        }
        temp_image.x = minx;
        temp_image.y = miny;
        temp_image.frame_width = pic.width();
        temp_image.frame_height = pic.height();
        temp_image.raw_data = pic.into_vec();
     
        
        self.add_frame(index,Box::new(temp_image));



    }

    fn get_frame(&self,index:usize)->Box<Vec<u8>> {
        let temp = self.frames.get(index).unwrap();
        let ref_c = temp.get("ref").unwrap();
        Box::new(self.row_datas.get(ref_c).unwrap().clone())
    }

    fn get_name(&self)->String{
        self.name.clone()
    }

    fn get_version(&self)->i32{
        self.versin
    }

   


}