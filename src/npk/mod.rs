pub mod npk;
pub mod img;
static NPK_MAGIC: [u8; 16] = [b'N', b'e', b'o', b'p', b'l', b'e', b'P', b'a', b'c', b'k', b'_', b'B', b'i', b'l', b'l', b'\0'];  

static IMG_NAME_KEY: [u8; 256] = [ b'p', b'u', b'c', b'h', b'i', b'k', b'o', b'n', b'@',
 b'n', b'e', b'o', b'p', b'l', b'e', b' ', b'd', b'u', b'n', b'g', b'e', b'o', b'n', 
 b' ', b'a', b'n', b'd', b' ', b'f', b'i', b'g', b'h', b't', b'e', b'r', b' ', b'D', 
 b'N', b'F', b'D', b'N', b'F', b'D', b'N', b'F', b'D', b'N', b'F', b'D', b'N', b'F', 
 b'D', b'N', b'F', b'D', b'N', b'F', b'D', b'N', b'F', b'D', b'N', b'F', b'D', b'N', 
 b'F', b'D', b'N', b'F', b'D', b'N', b'F', b'D', b'N', b'F', b'D', b'N', b'F', b'D', 
 b'N', b'F', b'D', b'N', b'F', b'D', b'N', b'F', b'D', b'N', b'F', b'D', b'N', b'F', 
 b'D', b'N', b'F', b'D', b'N', b'F', b'D', b'N', b'F', b'D', b'N', b'F', b'D', b'N', 
 b'F', b'D', b'N', b'F', b'D', b'N', b'F', b'D', b'N', b'F', b'D', b'N', b'F', b'D', 
 b'N', b'F', b'D', b'N', b'F', b'D', b'N', b'F', b'D', b'N', b'F', b'D', b'N', b'F', 
 b'D', b'N', b'F', b'D', b'N', b'F', b'D', b'N', b'F', b'D', b'N', b'F', b'D', b'N', 
 b'F', b'D', b'N', b'F', b'D', b'N', b'F', b'D', b'N', b'F', b'D', b'N', b'F', b'D', 
 b'N', b'F', b'D', b'N', b'F', b'D', b'N', b'F', b'D', b'N', b'F', b'D', b'N', b'F', 
 b'D', b'N', b'F', b'D', b'N', b'F', b'D', b'N', b'F', b'D', b'N', b'F', b'D', b'N', 
 b'F', b'D', b'N', b'F', b'D', b'N', b'F', b'D', b'N', b'F', b'D', b'N', b'F', b'D', 
 b'N', b'F', b'D', b'N', b'F', b'D', b'N', b'F', b'D', b'N', b'F', b'D', b'N', b'F', 
 b'D', b'N', b'F', b'D', b'N', b'F', b'D', b'N', b'F', b'D', b'N', b'F', b'D', b'N', 
 b'F', b'D', b'N', b'F', b'D', b'N', b'F', b'D', b'N', b'F', b'D', b'N', b'F', b'D', 
 b'N', b'F', b'D', b'N', b'F', b'D', b'N', b'F', b'\0'];