use crate::{copy_malloc_array, Error, Result};
use ::freefare_sys;

/// Namespace for TLV encoding and decoding helpers.
pub struct TLV;

impl TLV {
    /// Encodes a TLV object
    pub fn encode(tlv_type: u8, input_stream: &[u8], output_size: &mut usize) -> Result<Vec<u8>> {
        let input_ptr = input_stream.as_ptr();
        let input_size = input_stream.len() as u16;

        let output_ptr = unsafe {
            freefare_sys::tlv_encode(tlv_type, input_ptr, input_size, output_size as *mut usize)
        };

        if output_ptr.is_null() {
            return Err(Error::new("Failed to encode TLV."));
        }

        copy_malloc_array(output_ptr, *output_size)
    }

    /// Decodes a TLV object
    pub fn decode(input_stream: &[u8], tlv_type: &mut u8, size: &mut u16) -> Result<Vec<u8>> {
        let input_ptr = input_stream.as_ptr();

        let output_ptr =
            unsafe { freefare_sys::tlv_decode(input_ptr, tlv_type as *mut u8, size as *mut u16) };

        if output_ptr.is_null() {
            return Err(Error::new("Failed to decode TLV."));
        }

        let output_size = *size as usize;
        copy_malloc_array(output_ptr, output_size)
    }

    /// Calculates the length of a TLV record
    pub fn record_length(
        input_stream: &[u8],
        field_length_size: &mut usize,
        field_value_size: &mut usize,
    ) -> Result<usize> {
        let input_ptr = input_stream.as_ptr();

        let result = unsafe {
            freefare_sys::tlv_record_length(
                input_ptr,
                field_length_size as *mut usize,
                field_value_size as *mut usize,
            )
        };

        Ok(result)
    }

    /// Appends two TLV objects
    pub fn append(a: &[u8], b: &[u8]) -> Result<Vec<u8>> {
        let mut output = Vec::with_capacity(a.len() + b.len());
        output.extend_from_slice(a);
        output.extend_from_slice(b);
        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::TLV;

    #[test]
    fn tlv_round_trip() {
        let input = [0xAA, 0xBB, 0xCC];
        let mut encoded_size = 0;
        let encoded = TLV::encode(0x03, &input, &mut encoded_size).expect("encode");

        let mut decoded_type = 0;
        let mut decoded_size = 0;
        let decoded = TLV::decode(&encoded, &mut decoded_type, &mut decoded_size).expect("decode");

        assert_eq!(decoded_type, 0x03);
        assert_eq!(decoded_size as usize, input.len());
        assert_eq!(decoded, input);
    }

    #[test]
    fn tlv_append_preserves_both_records() {
        let mut first_len = 0;
        let first = TLV::encode(0x01, &[0x10, 0x11], &mut first_len).expect("first encode");

        let mut second_len = 0;
        let second = TLV::encode(0x02, &[0x20], &mut second_len).expect("second encode");

        let appended = TLV::append(&first, &second).expect("append");
        assert_eq!(appended.len(), first_len + second_len);
        assert_eq!(&appended[..first_len], &first);
        assert_eq!(&appended[first_len..], &second);
    }
}
