use anyhow::Result;

pub struct BufferReader<'a> {
    pub buffer: &'a [u8],
    pub pos: usize,
}

impl<'a> BufferReader<'a> {
    pub fn new(buffer: &'a [u8]) -> Self {
        Self { buffer, pos: 0 }
    }

    pub fn has_next(&self) -> bool {
        self.pos < self.buffer.len()
    }

    pub fn set_buffer(&mut self, pos: usize, buffer: &'a [u8]) {
        self.buffer = buffer;
        self.pos = pos;
    }

    pub fn read_byte(&mut self) -> Result<u8> {
        let res = self.buffer[self.pos];
        self.pos += 1;

        Ok(res)
    }

    pub fn read_i16(&mut self) -> Result<i16> {
        let bytes: [u8; 2] = self.buffer[self.pos..self.pos + 2].try_into()?;
        let res = i16::from_be_bytes(bytes);
        self.pos += 2;

        Ok(res)
    }

    pub fn read_i32(&mut self) -> Result<i32> {
        let bytes: [u8; 4] = self.buffer[self.pos..self.pos + 4].try_into()?;
        let res = i32::from_be_bytes(bytes);
        self.pos += 4;

        Ok(res)
    }

    pub fn read_i64(&mut self) -> Result<i64> {
        let bytes: [u8; 8] = self.buffer[self.pos..self.pos + 8].try_into()?;
        let res = i64::from_be_bytes(bytes);
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
