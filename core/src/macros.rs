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
macro_rules! decode_map {
    ($var:expr, $wire_type:expr, $decoder:expr, $decode_key_fn:path, $decode_value_fn:path) => {
        if WIRE_TYPE_LENGTH_ENCODED == $wire_type {
            if let Some((key, value)) =
                $decoder.decode_map_element($decode_key_fn, $decode_value_fn)?
            {
                $var.insert(key, value);
            }
        } else {
            return Err(DecodeError::UnexpectedWireType(
                WIRE_TYPE_LENGTH_ENCODED,
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
        $encoder.encode_packed($var, $encode_fn, $encode_fn);
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

// #[macro_export]
// macro_rules! encode_map {
//     ($field_number:expr, $prototy:ty, $var:expr, $encoder:expr, $encode_fn:path) => {
//         for (key, value) in $var {
//             $encoder.encode_uint32(u32::from_parts($field_number, WIRE_TYPE_LENGTH_ENCODED));
//             $encoder.encode_map_element(

//             )?
//         }
//     };
// }
