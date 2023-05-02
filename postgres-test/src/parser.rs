use anyhow::Result;

use crate::buffer_reader::BufferReader;

#[derive(Debug)]
pub enum MessageCodes {
    DataRow,
    ParseComplete,
    BindComplete,
    CloseComplete,
    CommandComplete,
    ReadyForQuery,
    NoData,
    NotificationResponse,
    AuthenticationResponse,
    ParameterStatus,
    BackendKeyData,
    ErrorMessage,
    NoticeMessage,
    RowDescriptionMessage,
    ParameterDescriptionMessage,
    PortalSuspended,
    ReplicationStart,
    EmptyQuery,
    CopyIn,
    CopyOut,
    CopyDone,
    CopyData,
}

impl MessageCodes {
    pub fn from_byte(input: u8) -> Result<Self> {
        return match input {
            b'D' => Ok(Self::DataRow),
            b'1' => Ok(Self::ParseComplete),
            b'2' => Ok(Self::BindComplete),
            b'3' => Ok(Self::CloseComplete),
            b'C' => Ok(Self::CommandComplete),
            b'Z' => Ok(Self::ReadyForQuery),
            b'n' => Ok(Self::NoData),
            b'A' => Ok(Self::NotificationResponse),
            b'R' => Ok(Self::AuthenticationResponse),
            b'S' => Ok(Self::ParameterStatus),
            b'K' => Ok(Self::BackendKeyData),
            b'E' => Ok(Self::ErrorMessage),
            b'N' => Ok(Self::NoticeMessage),
            b'T' => Ok(Self::RowDescriptionMessage),
            b't' => Ok(Self::ParameterDescriptionMessage),
            b's' => Ok(Self::PortalSuspended),
            b'W' => Ok(Self::ReplicationStart),
            b'I' => Ok(Self::EmptyQuery),
            b'G' => Ok(Self::CopyIn),
            b'H' => Ok(Self::CopyOut),
            b'c' => Ok(Self::CopyDone),
            b'd' => Ok(Self::CopyData),
            _ => anyhow::bail!("Invalid message code: {}", input),
        };
    }
}

pub struct Parser<'a> {
    reader: BufferReader<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(buffer: &'a [u8]) -> Self {
        Self {
            reader: BufferReader::new(buffer),
        }
    }

    pub fn parse(&mut self) -> Result<()> {
        while self.reader.has_next() {
            let code = self.reader.read_byte()?;
            let code = MessageCodes::from_byte(code)?;
            println!("Code: {:?}", code);

            let length = self.reader.read_i32()?;

            match code {
                MessageCodes::BindComplete
                | MessageCodes::ParseComplete
                | MessageCodes::CloseComplete
                | MessageCodes::EmptyQuery
                | MessageCodes::NoData
                | MessageCodes::PortalSuspended
                | MessageCodes::CopyDone
                | MessageCodes::ReplicationStart => {
                    println!("NOOP");
                }
                MessageCodes::RowDescriptionMessage => {
                    let num_fields = self.reader.read_i16()?;
                    for _ in 0..num_fields {
                        _ = self.reader.read_cstring()?; // name
                        _ = self.reader.read_i32()?; // table_id
                        _ = self.reader.read_i16()?; // column_id
                        _ = self.reader.read_i32()?; // type_id
                        _ = self.reader.read_i16()?; // type_size
                        _ = self.reader.read_i32()?; // type_modifier
                        _ = self.reader.read_i16()?; // mode
                    }
                }
                MessageCodes::DataRow => {
                    let num_fields = self.reader.read_i16()?;
                    for _ in 0..num_fields {
                        let length = self.reader.read_i32()? as usize;
                        println!("{}", self.reader.read_string(length)?);
                    }
                }
                MessageCodes::CommandComplete => {
                    self.reader.read_cstring()?; // command
                }
                MessageCodes::ReadyForQuery => {
                    _ = self.reader.read_string(1)?; // status
                }
                MessageCodes::NotificationResponse => {
                    _ = self.reader.read_i32()?; //process id
                    _ = self.reader.read_cstring()?; // channel
                    _ = self.reader.read_cstring()?; // payload
                }
                MessageCodes::AuthenticationResponse => {
                    let code = self.reader.read_i32()?;

                    match code {
                        0 => {}
                        3 => {}
                        5 => {
                            if length == 12 {
                                _ = self.reader.read_bytes(4)?; // salt
                            }
                        }
                        10 => {
                            anyhow::bail!("AuthenticationSASL is not supported");
                        }
                        11 | 12 => {
                            _ = self.reader.read_string(length as usize - 8)?; // auth data
                        }
                        _ => {
                            anyhow::bail!("Unknown authentication code: {}", code);
                        }
                    }
                }
                MessageCodes::ParameterStatus => {
                    _ = self.reader.read_cstring()?; // name
                    _ = self.reader.read_cstring()?; // value
                }
                MessageCodes::BackendKeyData => {
                    _ = self.reader.read_i32()?; // process id
                    _ = self.reader.read_i32()?; // secret key
                }
                MessageCodes::ErrorMessage | MessageCodes::NoticeMessage => {
                    let mut field_type = self.reader.read_string(1)?;
                    while field_type != "\0" {
                        let val = self.reader.read_cstring()?;
                        println!("{}: {}", field_type, val);
                        field_type = self.reader.read_string(1)?;
                    }
                }
                MessageCodes::ParameterDescriptionMessage => {
                    let num_params = self.reader.read_i16()?;
                    for _ in 0..num_params {
                        println!("type_id {}", self.reader.read_i32()?);
                    }
                }
                MessageCodes::CopyIn | MessageCodes::CopyOut => {
                    _ = self.reader.read_byte()?; // is binary
                    let num_columns = self.reader.read_i16()?;

                    for _ in 0..num_columns {
                        _ = self.reader.read_i16()?; // column type
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }
}
