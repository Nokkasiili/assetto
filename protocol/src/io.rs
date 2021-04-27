use anyhow::{bail, Context};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use md5::Digest;
use std::borrow::Cow;
use std::convert::{TryFrom, TryInto};
use std::io::{Cursor, Read};
use std::iter;
use std::marker::PhantomData;
use thiserror::Error;

#[derive(Debug)]
pub enum Optional<T> {
    None,
    Some(T),
}

/// Trait implemented for types which can be read
/// from a buffer.
pub trait Readable {
    /// Reads this type from the given buffer.
    fn read(buffer: &mut Cursor<&[u8]>) -> anyhow::Result<Self>
    where
        Self: Sized;
}

/// Trait implemented for types which can be written
/// to a buffer.
pub trait Writeable: Sized {
    /// Writes this value to the given buffer.
    fn write(&self, buffer: &mut Vec<u8>) -> anyhow::Result<()>;
}

impl<'a, T> Writeable for &'a T
where
    T: Writeable,
{
    fn write(&self, buffer: &mut Vec<u8>) -> anyhow::Result<()> {
        T::write(*self, buffer)?;
        Ok(())
    }
}

/// Error when reading a value.
#[derive(Debug, Error)]
pub enum Error {
    #[error("unexpected end of input: failed to read value of type `{0}`")]
    UnexpectedEof(&'static str),
}

macro_rules! integer_impl {
    ($($int:ty, $read_fn:tt, $write_fn:tt),* $(,)?) => {
        $(
            impl Readable for $int {
                fn read(buffer: &mut Cursor<&[u8]>) -> anyhow::Result<Self> {
                    buffer.$read_fn::<LittleEndian>().map_err(anyhow::Error::from)
                }
            }

            impl Writeable for $int {
                fn write(&self, buffer: &mut Vec<u8>) -> anyhow::Result<()> {
                    buffer.$write_fn::<LittleEndian>(*self)?;
                    Ok(())
                }
            }
        )*
    }
}

integer_impl! {
    u16, read_u16, write_u16,
    u32, read_u32, write_u32,
    u64, read_u64, write_u64,

    i16, read_i16, write_i16,
    i32, read_i32, write_i32,
    i64, read_i64, write_i64,

    f32, read_f32, write_f32,
    f64, read_f64, write_f64,
}

impl Readable for u8 {
    fn read(buffer: &mut Cursor<&[u8]>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        buffer.read_u8().map_err(anyhow::Error::from)
    }
}

impl Writeable for u8 {
    fn write(&self, buffer: &mut Vec<u8>) -> anyhow::Result<()> {
        buffer.write_u8(*self)?;
        Ok(())
    }
}

impl Readable for i8 {
    fn read(buffer: &mut Cursor<&[u8]>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        buffer.read_i8().map_err(anyhow::Error::from)
    }
}

impl Writeable for i8 {
    fn write(&self, buffer: &mut Vec<u8>) -> anyhow::Result<()> {
        buffer.write_i8(*self)?;
        Ok(())
    }
}

impl<T> Readable for Option<T>
where
    T: Readable,
{
    fn read(buffer: &mut Cursor<&[u8]>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        // Assume boolean prefix.
        let present = bool::read(buffer)?;

        if present {
            Ok(Some(T::read(buffer)?))
        } else {
            Ok(None)
        }
    }
}

impl<T> Writeable for Option<T>
where
    T: Writeable,
{
    fn write(&self, buffer: &mut Vec<u8>) -> anyhow::Result<()> {
        let present = self.is_some();
        present.write(buffer)?;

        if let Some(value) = self {
            value.write(buffer)?;
        }

        Ok(())
    }
}

impl Readable for String {
    fn read(buffer: &mut Cursor<&[u8]>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let length: usize = u8::read(buffer)
            .context("failed to read string length")?
            .into();

        // Read string into buffer.
        let mut temp = vec![0u8; length];
        buffer
            .read_exact(&mut temp)
            .map_err(|_| Error::UnexpectedEof("String"))?;
        let s = std::str::from_utf8(&temp).context("string contained invalid UTF8")?;

        Ok(s.to_owned())
    }
}

impl Writeable for String {
    fn write(&self, buffer: &mut Vec<u8>) -> anyhow::Result<()> {
        (self.len() as u8).write(buffer)?;
        /*        for i in self.chars() {
            buffer.extend_from_slice(&u32::from(i).to_le_bytes())
        }*/
        buffer.extend_from_slice(self.as_bytes());

        Ok(())
    }
}

impl Readable for bool {
    fn read(buffer: &mut Cursor<&[u8]>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let x = u8::read(buffer)?;

        if x == 0 {
            Ok(false)
        } else if x == 1 {
            Ok(true)
        } else {
            Err(anyhow::anyhow!("invalid boolean tag {}", x))
        }
    }
}

impl Writeable for bool {
    fn write(&self, buffer: &mut Vec<u8>) -> anyhow::Result<()> {
        let x = if *self { 1u8 } else { 0 };
        x.write(buffer)?;

        Ok(())
    }
}
#[derive(Debug, Clone)]
pub struct MD5Array(pub [u8; 16]);
//pub struct Digest(pub [u8; 16]);
impl Readable for MD5Array {
    fn read(buffer: &mut std::io::Cursor<&[u8]>) -> Result<Self, anyhow::Error> {
        let mut md5 = [0u8; 16];
        buffer.read_exact(&mut md5)?;
        Ok(MD5Array(md5))
    }
}
impl Writeable for MD5Array {
    fn write(&self, buffer: &mut Vec<u8>) -> Result<(), anyhow::Error> {
        buffer.extend_from_slice(&self.0);
        Ok(())
    }
}
impl From<Digest> for MD5Array {
    fn from(digest: Digest) -> Self {
        MD5Array(digest.0)
    }
}

pub struct WideString(pub String);
impl Readable for WideString {
    fn read(buffer: &mut Cursor<&[u8]>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let length: usize = u8::read(buffer)
            .context("failed to read string length")?
            .into();
        //let real_length = 4 * length;
        let mut string = String::new();
        for _ in 0..length {
            let mut temp = [0u8; 4];
            buffer.read_exact(&mut temp)?;
            if let Some(c) = char::from_u32(u32::from_le_bytes(temp)) {
                string.push(c);
            } else {
                bail!("could convert");
            }
        }
        //buffer.read_u8()?;
        Ok(WideString(string))
    }
}

impl Writeable for WideString {
    fn write(&self, buffer: &mut Vec<u8>) -> anyhow::Result<()> {
        (self.0.chars().count() as u8).write(buffer)?;
        for i in self.0.chars() {
            buffer.extend_from_slice(&u32::from(i).to_le_bytes())
        }
        // buffer.extend_from_slice(&[0x00]);

        Ok(())
    }
}
impl From<WideString> for String {
    fn from(x: WideString) -> Self {
        x.0
    }
}

impl From<&String> for WideString {
    fn from(x: &String) -> Self {
        Self(x.clone())
    }
}

pub const MAX_LENGTH: usize = 1024 * 1024; // 2^20 elements
pub struct LengthPrefixedVec<'a, P, T>(pub Cow<'a, [T]>, PhantomData<P>)
where
    [T]: ToOwned<Owned = Vec<T>>;

impl<'a, P, T> Readable for LengthPrefixedVec<'a, P, T>
where
    T: Readable,
    [T]: ToOwned<Owned = Vec<T>>,
    P: TryInto<usize> + Readable,
    P::Error: std::error::Error + Send + Sync + 'static,
{
    fn read(buffer: &mut Cursor<&[u8]>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let length: usize = P::read(buffer)?.try_into()?;

        if length > MAX_LENGTH {
            bail!("array length too large ({} > {})", length, MAX_LENGTH);
        }

        let vec = iter::repeat_with(|| T::read(buffer))
            .take(length)
            .collect::<anyhow::Result<Vec<T>>>()?;
        Ok(Self(Cow::Owned(vec), PhantomData))
    }
}

impl<'a, P, T> Writeable for LengthPrefixedVec<'a, P, T>
where
    T: Writeable,
    [T]: ToOwned<Owned = Vec<T>>,
    P: TryFrom<usize> + Writeable,
    P::Error: std::error::Error + Send + Sync + 'static,
{
    fn write(&self, buffer: &mut Vec<u8>) -> anyhow::Result<()> {
        P::try_from(self.0.len())?.write(buffer)?;
        self.0
            .iter()
            .for_each(|item| item.write(buffer).expect("failed to write to vec"));

        Ok(())
    }
}
impl<'a, P, T> From<LengthPrefixedVec<'a, P, T>> for Vec<T>
where
    [T]: ToOwned<Owned = Vec<T>>,
{
    fn from(x: LengthPrefixedVec<'a, P, T>) -> Self {
        x.0.into_owned()
    }
}

impl<'a, P, T> From<&'a [T]> for LengthPrefixedVec<'a, P, T>
where
    [T]: ToOwned<Owned = Vec<T>>,
{
    fn from(slice: &'a [T]) -> Self {
        Self(Cow::Borrowed(slice), PhantomData)
    }
}

impl<'a, P, T> From<Vec<T>> for LengthPrefixedVec<'a, P, T>
where
    [T]: ToOwned<Owned = Vec<T>>,
{
    fn from(vec: Vec<T>) -> Self {
        Self(Cow::Owned(vec), PhantomData)
    }
}

pub type BytePrefixedVec<'a, T> = LengthPrefixedVec<'a, u8, T>;
