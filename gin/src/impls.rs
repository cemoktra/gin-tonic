//! The module add some default implementation for model/protobuf conversions.
//!
//! The 'from-str' feature enables converting a protobuf string into any model that implements
//! the FromStr trait. Note that you cannot use any custom implementation for the String -> Model
//! conversion when this feature is activated.
//!
//! The 'to-str' feature enables converting any model that implements the Display trait into a
//! protobuf string. Note that you cannot use any custom implementation for the Model -> String
//! conversion when this feature is activated.

use crate::traits::{FromModel, FromProto, IntoModel, IntoProto};

impl<Model, Proto> FromProto<Option<Proto>> for Option<Model>
where
    Model: FromProto<Proto>,
{
    type Error = <Model as FromProto<Proto>>::Error;

    fn from_proto(proto: Option<Proto>) -> Result<Self, Self::Error> {
        proto.map(IntoModel::into_model).transpose()
    }
}

impl<Model, Proto> FromModel<Option<Model>> for Option<Proto>
where
    Proto: FromModel<Model>,
{
    fn from_model(model: Option<Model>) -> Self {
        model.map(IntoProto::into_proto)
    }
}

impl<Model, Proto> FromProto<Vec<Proto>> for Vec<Model>
where
    Model: FromProto<Proto>,
{
    type Error = <Model as FromProto<Proto>>::Error;

    fn from_proto(proto: Vec<Proto>) -> Result<Self, Self::Error> {
        proto
            .into_iter()
            .map(IntoModel::into_model)
            .collect::<Result<Vec<_>, _>>()
    }
}

impl<Model, Proto> FromModel<Vec<Model>> for Vec<Proto>
where
    Proto: FromModel<Model>,
{
    fn from_model(model: Vec<Model>) -> Self {
        model
            .into_iter()
            .map(IntoProto::into_proto)
            .collect::<Vec<_>>()
    }
}

#[cfg(feature = "from-str")]
mod from_str {
    use crate::traits::FromProto;
    use std::str::FromStr;

    impl<Model> FromProto<String> for Model
    where
        Model: FromStr,
    {
        type Error = <Model as FromStr>::Err;

        fn from_proto(proto: String) -> Result<Self, Self::Error> {
            proto.parse()
        }
    }
}

#[cfg(feature = "to-str")]
mod to_str {
    use crate::traits::FromModel;
    use std::convert::Infallible;
    use std::fmt::Display;

    impl<Model> FromModel<Model> for String
    where
        Model: Display,
    {
        type Error = Infallible;

        fn from_model(model: Model) -> Result<Self, Self::Error> {
            Ok(model.to_string())
        }
    }
}

/// this module defines infallible self conversions for the case that the model and protobuf types
/// are actually identical
mod infallible {
    macro_rules! infallible_conversion {
        (
            $(
                $ty:ty;
            )+
        ) => {
            $(
                impl crate::traits::FromProto<$ty> for $ty {
                    type Error = std::convert::Infallible;

                    fn from_proto(value: $ty) -> Result<Self, Self::Error> {
                        Ok(value)
                    }
                }

                impl crate::traits::FromModel<$ty> for $ty {
                    fn from_model(value: $ty) -> Self {
                        value
                    }
                }
            )+
        };

    }

    // based on the list of Prost type mappings:
    // https://github.com/tokio-rs/prost?tab=readme-ov-file#scalar-values
    infallible_conversion! {
        i32;
        u32;
        i64;
        u64;
        f32;
        f64;
        bool;
        String;
        Vec<u8>;
    }
}

mod std {
    use crate::protobuf::{Error, FromWire, IntoWire, WireType, WireTypeView};

    impl FromWire for std::net::Ipv4Addr {
        fn from_wire(wire: WireTypeView) -> Result<Self, Error>
        where
            Self: Sized,
        {
            let n = u32::from_wire(wire)?;
            Ok(n.into())
        }
    }

    impl IntoWire for std::net::Ipv4Addr {
        fn into_wire(self) -> WireType {
            let n: u32 = self.into();
            n.into_wire()
        }

        fn size_hint(&self, tag: u32) -> usize {
            let n: u32 = (*self).into();
            n.size_hint(tag)
        }
    }
}