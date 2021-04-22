use crate::Readable;
use std::io::Cursor;
use std::io::Write;

use crate::Writeable;
use bytes::BytesMut;

#[derive(Default)]
pub struct Codec {
    received_buf: BytesMut,
    staging_buf: Vec<u8>,
}

impl Codec {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn accept(&mut self, bytes: &[u8]) {
        self.received_buf.extend(bytes);
    }

    pub fn encode(&mut self, packet: &impl Writeable, output: &mut Vec<u8>) -> anyhow::Result<()> {
        packet.write(&mut self.staging_buf)?;
        (self.staging_buf.len() as u16).write(output)?;

        output.write(&self.staging_buf)?;

        self.staging_buf.clear();

        Ok(())
    }

    pub fn decode<T>(&mut self, input: &mut Vec<u8>) -> anyhow::Result<Option<T>>
    where
        T: Readable,
    {
        let mut cursor = Cursor::new(&input[..]);

        let packet = T::read(&mut cursor)?;
        Ok(Some(packet))
    }
    pub fn next_packet<T>(&mut self) -> anyhow::Result<Option<T>>
    where
        T: Readable,
    {
        let mut cursor = Cursor::new(&self.received_buf[..]);
        let packet = if let Ok(length) = u16::read(&mut cursor) {
            let length_field_length = cursor.position() as usize;

            if self.received_buf.len() - length_field_length >= length as usize {
                cursor = Cursor::new(
                    &self.received_buf[length_field_length..length_field_length + length as usize],
                );

                let packet = T::read(&mut cursor)?;

                let bytes_read = cursor.position() as usize + length_field_length;
                self.received_buf = self.received_buf.split_off(bytes_read);

                Some(packet)
            } else {
                None
            }
        } else {
            None
        };

        Ok(packet)
    }
}
