use crate::types::{FieldElement, U256};

/// Any type where [FieldElement]s can be written into. This would typically be [Vec<FieldELement>],
/// but can also be something like a stateful hasher.
pub trait FeltWriter {
    fn write(&mut self, felt: FieldElement);
}

/// Any type that can be serialized into a series of [FieldElement]s. This trait corresponds to the
/// `serialize` function of the Cairo `Serde` trait.
pub trait AbiEncode {
    fn encode<W: FeltWriter>(&self, writer: &mut W);
}

impl FeltWriter for Vec<FieldElement> {
    fn write(&mut self, felt: FieldElement) {
        self.push(felt);
    }
}

impl AbiEncode for FieldElement {
    fn encode<W: FeltWriter>(&self, writer: &mut W) {
        writer.write(*self)
    }
}

impl AbiEncode for u8 {
    fn encode<W: FeltWriter>(&self, writer: &mut W) {
        writer.write((*self).into())
    }
}

impl AbiEncode for u16 {
    fn encode<W: FeltWriter>(&self, writer: &mut W) {
        writer.write((*self).into())
    }
}

impl AbiEncode for u32 {
    fn encode<W: FeltWriter>(&self, writer: &mut W) {
        writer.write((*self).into())
    }
}

impl AbiEncode for u64 {
    fn encode<W: FeltWriter>(&self, writer: &mut W) {
        writer.write((*self).into())
    }
}

impl AbiEncode for u128 {
    fn encode<W: FeltWriter>(&self, writer: &mut W) {
        writer.write((*self).into())
    }
}

impl AbiEncode for U256 {
    fn encode<W: FeltWriter>(&self, writer: &mut W) {
        self.low().encode(writer);
        self.high().encode(writer);
    }
}

#[cfg(test)]
mod tests {
    use core::str::FromStr;

    use super::*;

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_encode_felt() {
        let mut serialized = Vec::<FieldElement>::new();
        FieldElement::from_str("99999999999999999999999999")
            .unwrap()
            .encode(&mut serialized);
        assert_eq!(
            serialized,
            vec![FieldElement::from_str("99999999999999999999999999").unwrap()]
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_encode_u8() {
        let mut serialized = Vec::<FieldElement>::new();
        123u8.encode(&mut serialized);
        assert_eq!(serialized, vec![FieldElement::from_str("123").unwrap()]);
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_encode_u16() {
        let mut serialized = Vec::<FieldElement>::new();
        12345u16.encode(&mut serialized);
        assert_eq!(serialized, vec![FieldElement::from_str("12345").unwrap()]);
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_encode_u32() {
        let mut serialized = Vec::<FieldElement>::new();
        1234567890u32.encode(&mut serialized);
        assert_eq!(
            serialized,
            vec![FieldElement::from_str("1234567890").unwrap()]
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_encode_u64() {
        let mut serialized = Vec::<FieldElement>::new();
        12345678900000000000u64.encode(&mut serialized);
        assert_eq!(
            serialized,
            vec![FieldElement::from_str("12345678900000000000").unwrap()]
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_encode_u128() {
        let mut serialized = Vec::<FieldElement>::new();
        123456789000000000000000000000u128.encode(&mut serialized);
        assert_eq!(
            serialized,
            vec![FieldElement::from_str("123456789000000000000000000000").unwrap()]
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_encode_u256() {
        let mut serialized = Vec::<FieldElement>::new();
        U256::from_words(12345, 67890).encode(&mut serialized);
        assert_eq!(
            serialized,
            vec![
                FieldElement::from_str("12345").unwrap(),
                FieldElement::from_str("67890").unwrap()
            ]
        );
    }
}
