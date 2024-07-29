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
macro_rules! decode_vector {
    ($prototy:ty, $vec:expr, $wire_type:expr, $decoder:expr, $decode_fn:path) => {
        if <$prototy>::WIRE_TYPE == $wire_type {
            $vec.push($decode_fn($decoder)?);
        } else if WIRE_TYPE_LENGTH_ENCODED == $wire_type {
            $decoder.decode_packed($vec, $decode_fn)?;
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
macro_rules! encode_vector_packed {
    ($field_number:expr, $var:expr, $encoder:expr, $encode_fn:path) => {
        $encoder.encode_uint32(u32::from_parts($field_number, WIRE_TYPE_LENGTH_ENCODED));
        $encoder.encode_packed($var, $encode_fn);
    };
}

#[macro_export]
macro_rules! encode_vector_unpacked {
    ($field_number:expr, $prototy:ty, $var:expr, $encoder:expr, $encode_fn:path) => {
        for item in $var {
            $encoder.encode_uint32(u32::from_parts($field_number, <$prototy>::WIRE_TYPE));
            $encode_fn($encoder, *item);
        }
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

#[macro_export]
macro_rules! size_hint_repeated_packed {
    ($field_number:expr, $var:expr) => {
        PbType::size_hint(&UInt32(u32::from_parts(
            $field_number,
            WIRE_TYPE_LENGTH_ENCODED,
        ))) + $var.iter().map(|item| (*item).size_hint()).sum::<usize>()
    };
}

#[macro_export]
macro_rules! size_hint_repeated_packed_wrapped {
    ($field_number:expr, $protoex:expr, $var:expr) => {
        PbType::size_hint(&UInt32(u32::from_parts(
            $field_number,
            WIRE_TYPE_LENGTH_ENCODED,
        ))) + PbType::size_hint(&UInt32(u32::from_parts(
            $var.len() as u32,
            WIRE_TYPE_LENGTH_ENCODED,
        ))) + $var
            .iter()
            .map(|item| $protoex(*item).size_hint())
            .sum::<usize>()
    };
}

#[macro_export]
macro_rules! size_hint_repeated {
    ($field_number:expr, $prototy:ty, $protoex:expr, $var:expr) => {
        $var.iter()
            .map(|item| size_hint_wrapped!($field_number, $prototy, $protoex, *item))
            .sum::<usize>()
    };
}
