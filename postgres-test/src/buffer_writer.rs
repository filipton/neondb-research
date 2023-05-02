pub struct BufferWriter {
    pub buffer: Vec<u8>,
    offset: usize,
    header_pos: usize,
}

impl BufferWriter {
    pub fn new() -> Self {
        Self {
            buffer: vec![0; 256],
            offset: 5,
            header_pos: 0,
        }
    }

    fn ensure_size(&mut self, size: usize) {
        let remaining = self.buffer.len() - self.offset;
        if remaining < size {
            let mut new_buffer = vec![0; self.buffer.len() * 2];
            new_buffer[..self.offset].copy_from_slice(&self.buffer[..self.offset]);
            self.buffer = new_buffer;
        }
    }

    pub fn write_i32(&mut self, value: i32) -> &mut Self {
        self.ensure_size(4);
        self.buffer[self.offset..self.offset + 4].copy_from_slice(&value.to_be_bytes());
        self.offset += 4;

        self
    }

    pub fn write_i16(&mut self, value: i16) -> &mut Self {
        self.ensure_size(2);
        self.buffer[self.offset..self.offset + 2].copy_from_slice(&value.to_be_bytes());
        self.offset += 2;

        self
    }

    pub fn write_cstring(&mut self, value: &str) -> &mut Self {
        self.ensure_size(value.len() + 1);
        self.buffer[self.offset..self.offset + value.len()].copy_from_slice(value.as_bytes());
        self.buffer.push(0);
        self.offset += value.len() + 1;

        self
    }

    // I think its correct but in original implementation its not like this
    /*
    pub fn write_string(&mut self, value: &str) -> &mut Self {
        self.write_i32(value.len() as i32);
        self.buffer.extend_from_slice(value.as_bytes());
        self.offset += value.len();

        self
    }
    */

    pub fn add(&mut self, other: &[u8]) -> &mut Self {
        self.ensure_size(other.len());
        self.buffer[self.offset..self.offset + other.len()].copy_from_slice(other);
        self.offset += other.len();

        self
    }

    fn join(&mut self, code: Option<u8>) -> Vec<u8> {
        if let Some(code) = code {
            self.buffer[self.header_pos] = code;
            let length = self.offset as u32 - (self.header_pos + 1) as u32;

            self.buffer[self.header_pos + 1..self.header_pos + 5]
                .copy_from_slice(&length.to_be_bytes());

            return self.buffer[..self.offset].to_vec();
        }

        return self.buffer[5..self.offset].to_vec();
    }

    pub fn flush(&mut self, code: Option<u8>) -> Vec<u8> {
        let res = self.join(code);
        self.offset = 5;
        self.header_pos = 0;
        self.buffer = vec![0; 256];

        res
    }

    pub fn get_length(&self) -> usize {
        self.offset
    }
}
