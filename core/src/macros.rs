#[macro_export]
macro_rules! decode_field {
    ($prototy:ty, $var:expr, $wire_type:expr, $decoder:expr, $decode_fn:path) => {
        if <$prototy>::WIRE_TYPE == $wire_type {
            $var = Some($decode_fn($decoder)?);
        } else {
            return Err(DecodeError::UnexpectedWireType(
                <$prototy>::WIRE_TYPE,
                $wire_type,
            ));
        }
    };
}

#[macro_export]
macro_rules! encode_field {
    ($field_number:expr, $prototy:ty, $var:expr, $encoder:expr, $encode_fn:path) => {
        $encoder.encode_uint32(u32::from_parts($field_number, <$prototy>::WIRE_TYPE));
        $encode_fn($encoder, $var);
    };
}

#[macro_export]
macro_rules! size_hint {
    ($field_number:expr, $prototy:ty, $var:expr) => {
        PbType::size_hint(&UInt32(u32::from_parts(
            $field_number,
            <$prototy>::WIRE_TYPE,
        ))) + $var.size_hint()
    };
}

#[macro_export]
macro_rules! size_hint_wrapped {
    ($field_number:expr, $prototy:ty, $protoex:expr, $var:expr) => {
        PbType::size_hint(&UInt32(u32::from_parts(
            $field_number,
            <$prototy>::WIRE_TYPE,
        ))) + $protoex($var).size_hint()
    };
}
