use std::os::fd::{AsFd, OwnedFd};

use crate::{GluonConvertable, GluonReadError, GluonWriteError};

impl GluonConvertable for OwnedFd {
    fn write<'a, 'b: 'a>(
        &'b self,
        data: &mut crate::GluonDataBuilder<'a>,
    ) -> Result<(), GluonWriteError> {
        data.write_fd(self.as_fd())
    }

    fn read(data: &mut crate::GluonDataReader) -> Result<Self, GluonReadError> {
        data.read_fd()
    }
}
impl GluonConvertable for String {
    fn write<'a, 'b: 'a>(
        &'b self,
        data: &mut crate::GluonDataBuilder<'a>,
    ) -> Result<(), GluonWriteError> {
        data.write_str(self)
    }

    fn read(data: &mut crate::GluonDataReader) -> Result<Self, GluonReadError> {
        data.read_string()
    }
}

impl GluonConvertable for bool {
    fn write<'a, 'b: 'a>(
        &'b self,
        data: &mut crate::GluonDataBuilder<'a>,
    ) -> Result<(), GluonWriteError> {
        data.write_bool(*self)
    }
    fn read(data: &mut crate::GluonDataReader) -> Result<Self, GluonReadError> {
        data.read_bool()
    }
}
impl GluonConvertable for u64 {
    fn write<'a, 'b: 'a>(
        &'b self,
        data: &mut crate::GluonDataBuilder<'a>,
    ) -> Result<(), GluonWriteError> {
        data.write_u64(*self)
    }
    fn read(data: &mut crate::GluonDataReader) -> Result<Self, GluonReadError> {
        data.read_u64()
    }
}
impl GluonConvertable for i64 {
    fn write<'a, 'b: 'a>(
        &'b self,
        data: &mut crate::GluonDataBuilder<'a>,
    ) -> Result<(), GluonWriteError> {
        data.write_i64(*self)
    }
    fn read(data: &mut crate::GluonDataReader) -> Result<Self, GluonReadError> {
        data.read_i64()
    }
}
impl GluonConvertable for f64 {
    fn write<'a, 'b: 'a>(
        &'b self,
        data: &mut crate::GluonDataBuilder<'a>,
    ) -> Result<(), GluonWriteError> {
        data.write_f64(*self)
    }
    fn read(data: &mut crate::GluonDataReader) -> Result<Self, GluonReadError> {
        data.read_f64()
    }
}
impl GluonConvertable for u32 {
    fn write<'a, 'b: 'a>(
        &'b self,
        data: &mut crate::GluonDataBuilder<'a>,
    ) -> Result<(), GluonWriteError> {
        data.write_u32(*self)
    }
    fn read(data: &mut crate::GluonDataReader) -> Result<Self, GluonReadError> {
        data.read_u32()
    }
}
impl GluonConvertable for i32 {
    fn write<'a, 'b: 'a>(
        &'b self,
        data: &mut crate::GluonDataBuilder<'a>,
    ) -> Result<(), GluonWriteError> {
        data.write_i32(*self)
    }
    fn read(data: &mut crate::GluonDataReader) -> Result<Self, GluonReadError> {
        data.read_i32()
    }
}
impl GluonConvertable for f32 {
    fn write<'a, 'b: 'a>(
        &'b self,
        data: &mut crate::GluonDataBuilder<'a>,
    ) -> Result<(), GluonWriteError> {
        data.write_f32(*self)
    }
    fn read(data: &mut crate::GluonDataReader) -> Result<Self, GluonReadError> {
        data.read_f32()
    }
}
impl GluonConvertable for u16 {
    fn write<'a, 'b: 'a>(
        &'b self,
        data: &mut crate::GluonDataBuilder<'a>,
    ) -> Result<(), GluonWriteError> {
        data.write_u16(*self)
    }
    fn read(data: &mut crate::GluonDataReader) -> Result<Self, GluonReadError> {
        data.read_u16()
    }
}
impl GluonConvertable for i16 {
    fn write<'a, 'b: 'a>(
        &'b self,
        data: &mut crate::GluonDataBuilder<'a>,
    ) -> Result<(), GluonWriteError> {
        data.write_i16(*self)
    }
    fn read(data: &mut crate::GluonDataReader) -> Result<Self, GluonReadError> {
        data.read_i16()
    }
}
impl GluonConvertable for u8 {
    fn write<'a, 'b: 'a>(
        &'b self,
        data: &mut crate::GluonDataBuilder<'a>,
    ) -> Result<(), GluonWriteError> {
        data.write_u8(*self)
    }
    fn read(data: &mut crate::GluonDataReader) -> Result<Self, GluonReadError> {
        data.read_u8()
    }
}
impl GluonConvertable for i8 {
    fn write<'a, 'b: 'a>(
        &'b self,
        data: &mut crate::GluonDataBuilder<'a>,
    ) -> Result<(), GluonWriteError> {
        data.write_i8(*self)
    }
    fn read(data: &mut crate::GluonDataReader) -> Result<Self, GluonReadError> {
        data.read_i8()
    }
}
