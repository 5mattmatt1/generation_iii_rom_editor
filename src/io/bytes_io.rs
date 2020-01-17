pub struct BytesReader<'a>
{
    buffer: &'a [u8], 
    idx: usize
}

pub struct BytesWriter<'a>
{
    buffer: &'a mut [u8],
    idx: usize
}

pub struct BytesIO<'a>
{
    buffer: &'a mut [u8],
    idx: usize
}

impl<'a> BytesReader<'a>
{
    pub fn new(buffer: &'a [u8]) -> Self
    {
        Self {
            buffer: buffer,
            idx: 0
        }
    }

    pub fn seek(&mut self, idx: usize)
    {
        self.idx = idx;
    }
}

impl<'a> BytesWriter<'a>
{
    pub fn new(buffer: &'a mut [u8]) -> Self
    {
        Self {
            buffer: buffer,
            idx: 0
        }
    }

    pub fn seek(&mut self, idx: usize)
    {
        self.idx = idx;
    }
}

impl<'a> BytesIO<'a>
{
    pub fn new(buffer: &'a mut [u8]) -> Self
    {
        Self {
            buffer: buffer,
            idx: 0
        }
    }

    pub fn seek(&mut self, idx: usize)
    {
        self.idx = idx;
    }
}