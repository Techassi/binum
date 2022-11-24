pub use binbuf_macros::*;

macro_rules! from_buffer_and_readable_impl {
    ($SelfT:ty) => {
        impl FromBuffer for $SelfT {
            const SIZE: usize = (Self::BITS / 8) as usize;

            fn as_be(buf: &mut ReadBuffer) -> ReadBufferResult<Self> {
                let b = buf.read_slice(Self::SIZE)?;
                Ok(Self::from_be_bytes(b.try_into().unwrap()))
            }

            fn as_le(buf: &mut ReadBuffer) -> ReadBufferResult<Self> {
                let b = buf.read_slice(Self::SIZE)?;
                Ok(Self::from_le_bytes(b.try_into().unwrap()))
            }
        }

        impl Readable for $SelfT {
            fn read<E: Endianness>(buf: &mut ReadBuffer) -> ReadBufferResult<Self> {
                E::read(buf)
            }
        }
    };
}

macro_rules! into_buffer_and_writeable_impl {
    ($SelfT:ty) => {
        impl IntoBuffer for $SelfT {
            const SIZE: usize = (Self::BITS / 8) as usize;

            fn as_be(&self, buf: &mut WriteBuffer) -> WriteBufferResult {
                let b = self.to_be_bytes();
                buf.write_slice(&b[..])
            }

            fn as_le(&self, buf: &mut WriteBuffer) -> WriteBufferResult {
                let b = self.to_le_bytes();
                buf.write_slice(&b[..])
            }
        }

        impl Writeable for $SelfT {
            fn write<E: Endianness>(&self, buf: &mut WriteBuffer) -> WriteBufferResult {
                E::write(*self, buf)
            }
        }
    };
}

pub(crate) use from_buffer_and_readable_impl;
pub(crate) use into_buffer_and_writeable_impl;
