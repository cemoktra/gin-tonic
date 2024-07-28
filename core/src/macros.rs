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
