//! This modules defines traits for converting between protobuf and model types in way that is
//! comparable with the std From/Into traits

/// the "Into" trait for converting any model type into a protobuf type
pub trait IntoProto<Proto>: Sized {
    fn into_proto(self) -> Proto;
}

/// the "From" trait for converting any protobuf type into a model type
pub trait FromProto<Proto>: Sized {
    type Error;

    fn from_proto(proto: Proto) -> Result<Self, Self::Error>;
}

impl<Model, Proto> IntoProto<Proto> for Model
where
    Proto: FromModel<Model>,
{
    fn into_proto(self) -> Proto {
        Proto::from_model(self)
    }
}

/// the "Into" trait for converting any protobuf type into a model type
pub trait IntoModel<Model>: Sized {
    type Error;

    fn into_model(self) -> Result<Model, Self::Error>;
}

/// the "From" trait for converting any model type into a protobuf type
pub trait FromModel<Model>: Sized {
    fn from_model(model: Model) -> Self;
}

impl<Proto, Model> IntoModel<Model> for Proto
where
    Model: FromProto<Proto>,
{
    type Error = Model::Error;

    fn into_model(self) -> Result<Model, Self::Error> {
        Model::from_proto(self)
    }
}
