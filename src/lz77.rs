pub fn decompress(buffer: &[u8], offset: usize) -> std::io::Result<Vec<u8>>
{
    use crate::bytes::load_u32;
    let lz_len = (load_u32(buffer, offset + 0x1)? & 0x00_FF_FF_FF) as usize;
    // Why is there a hard coded length of 0x950 in their code?
    // let lz_len = 0x4000; 
    println!("Uncompressed length: 0x{:X}", lz_len);

    let mut de_buf: Vec<u8> = Vec::with_capacity(lz_len);
    de_buf.resize(lz_len, 0);

    // Looks like a way to compress all the flags for a set of 8 bytes
    let mut flags = 0;
    // Is this byte a literal or a length-dist pair?
    let mut literal_flag: bool; 
    let mut idx: usize = 4;
    let mut len: usize = 0;
    let mut dist: usize = 0;
    let mut byte: usize = 0;
    let mut curr_size: usize = 0;
    let mut cdest: usize = 0;
    while curr_size < lz_len
    {
        flags = buffer[offset+idx];
        idx += 1;
        
        for i in 0..8
        {
            // TODO: Maybe convert to != DIST_LEN_FLAG or something like that...
            literal_flag = (flags & (0x80 >> i)) > 0;
        
            if literal_flag
            {
                byte = buffer[offset+idx] as usize;
                idx += 1;
                
                len = (byte >> 4);
                dist = ((byte & 0x0F) << 8);
                
                dist |= buffer[offset+idx] as usize;
                idx += 1;

                len += 3; // Why three?

                if dist > curr_size
                {
                    // Can't exactly read at a dist if haven't filled that part of buffer
                    return Err(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, format!("Attempt to access index {} but current size is {}", dist, curr_size)));
                }

                cdest = curr_size;

                for j in 0..len
                {
                    if curr_size >= lz_len
                    {
                        break;
                    }
                    de_buf[curr_size] = de_buf[cdest - dist - 1 + j];
                    curr_size += 1;
                }
            } else 
            {
                byte = buffer[offset+idx] as usize;
                idx += 1;
                
                if curr_size >= lz_len
                {
                    break;
                }

                de_buf[curr_size] = byte as u8;
                curr_size += 1;
            }
        }

    }

    Ok(de_buf)
}