use crate::array2::{ArrayBuilder, ArrayBuilderImpl, DecimalArrayBuilder};
use crate::error::{Result, RwError};
use crate::types::{DataType, DataTypeKind, DataTypeRef};
use risingwave_proto::data::DataType as DataTypeProto;
use risingwave_proto::data::DataType_TypeName;
use std::any::Any;
use std::convert::TryFrom;
use std::sync::Arc;

pub const MAX_PRECISION: u32 = 28;

#[derive(Debug)]
pub struct DecimalType {
    nullable: bool,
    precision: u32,
    scale: u32,
}

impl DecimalType {
    pub fn get_precision(&self) -> u32 {
        self.precision
    }

    pub fn get_scale(&self) -> u32 {
        self.scale
    }

    pub fn new(nullable: bool, precision: u32, scale: u32) -> Result<Self> {
        ensure!(precision <= MAX_PRECISION);
        ensure!(scale <= precision);
        Ok(Self {
            nullable,
            precision,
            scale,
        })
    }

    pub fn create(nullable: bool, precision: u32, scale: u32) -> Result<DataTypeRef> {
        ensure!(precision <= MAX_PRECISION);
        ensure!(scale <= precision);
        Ok(Arc::new(Self {
            nullable,
            precision,
            scale,
        }) as DataTypeRef)
    }
}

impl DataType for DecimalType {
    fn data_type_kind(&self) -> DataTypeKind {
        DataTypeKind::Decimal
    }

    fn is_nullable(&self) -> bool {
        self.nullable
    }

    fn create_array_builder(self: Arc<Self>, capacity: usize) -> Result<ArrayBuilderImpl> {
        DecimalArrayBuilder::new(capacity).map(|x| x.into())
    }

    fn to_protobuf(&self) -> Result<DataTypeProto> {
        let mut proto = DataTypeProto::new();
        proto.set_type_name(DataType_TypeName::DECIMAL);
        proto.set_is_nullable(self.nullable);
        proto.set_scale(self.scale);
        proto.set_precision(self.precision);
        Ok(proto)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl<'a> TryFrom<&'a DataTypeProto> for DecimalType {
    type Error = RwError;

    fn try_from(proto: &'a DataTypeProto) -> Result<Self> {
        ensure!(proto.get_type_name() == DataType_TypeName::DECIMAL);
        DecimalType::new(
            proto.get_is_nullable(),
            proto.get_precision(),
            proto.get_scale(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_precision_and_scale() {
        let decimal_type = DecimalType {
            nullable: true,
            precision: 10,
            scale: 5,
        };
        assert_eq!(decimal_type.get_precision(), 10);
        assert_eq!(decimal_type.get_scale(), 5);
    }

    #[test]
    fn test_create_decimal_with_wrong_precison_and_scale() {
        let larger_scale = DecimalType::create(true, 5, 6);
        assert!(larger_scale.is_err());
        let larger_precision = DecimalType::create(true, 40, 20);
        assert!(larger_precision.is_err());
    }
}
