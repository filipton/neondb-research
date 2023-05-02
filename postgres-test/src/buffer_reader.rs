use anyhow::Result;

pub struct BufferReader<'a> {
    buffer: &'a [u8],
    pos: usize,
}

impl<'a> BufferReader<'a> {
    pub fn new(buffer: &'a [u8]) -> Self {
        Self { buffer, pos: 0 }
    }

    pub fn read_u8(&mut self) -> Result<u8> {
        let res = self.buffer[self.pos];
        self.pos += 1;

        Ok(res)
    }

    pub fn read_u16(&mut self) -> Result<u16> {
        let bytes: [u8; 2] = self.buffer[self.pos..self.pos + 2].try_into()?;
        let res = u16::from_be_bytes(bytes);
        self.pos += 2;

        Ok(res)
    }

    pub fn read_u32(&mut self) -> Result<u32> {
        let bytes: [u8; 4] = self.buffer[self.pos..self.pos + 4].try_into()?;
        let res = u32::from_be_bytes(bytes);
        self.pos += 4;

        Ok(res)
    }

    pub fn read_u64(&mut self) -> Result<u64> {
        let bytes: [u8; 8] = self.buffer[self.pos..self.pos + 8].try_into()?;
        let res = u64::from_be_bytes(bytes);
        self.pos += 8;

        Ok(res)
    }

    pub fn read_string(&mut self, length: usize) -> Result<String> {
        let res = String::from_utf8_lossy(&self.buffer[self.pos..self.pos + length]).to_string();
        self.pos += length;

        Ok(res)
    }

    pub fn read_cstring(&mut self) -> Result<String> {
        let mut res = String::new();
        loop {
            let c = self.buffer[self.pos];
            self.pos += 1;

            if c == 0 {
                break;
            }
            res.push(c as char);
        }

        Ok(res)
    }

    pub fn read_bytes(&mut self, length: usize) -> Result<Vec<u8>> {
        let res = self.buffer[self.pos..self.pos + length].to_vec();
        self.pos += length;

        Ok(res)
    }
}
