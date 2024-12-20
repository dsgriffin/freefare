use ::freefare_sys;

pub struct TLV;

impl TLV {
    /// Encodes a TLV object
    pub fn encode(
        tlv_type: u8,
        input_stream: &[u8],
        output_size: &mut usize,
    ) -> Result<Vec<u8>, String> {
        let input_ptr = input_stream.as_ptr();
        let input_size = input_stream.len() as u16;

        let output_ptr = unsafe {
            freefare_sys::tlv_encode(tlv_type, input_ptr, input_size, output_size as *mut usize)
        };

        if output_ptr.is_null() {
            return Err("Failed to encode TLV.".to_string());
        }

        let output_data = unsafe { Vec::from_raw_parts(output_ptr, *output_size, *output_size) };

        Ok(output_data)
    }

    /// Decodes a TLV object
    pub fn decode(
        input_stream: &[u8],
        tlv_type: &mut u8,
        size: &mut u16,
    ) -> Result<Vec<u8>, String> {
        let input_ptr = input_stream.as_ptr();

        let output_ptr = unsafe {
            freefare_sys::tlv_decode(input_ptr, tlv_type as *mut u8, size as *mut u16)
        };

        if output_ptr.is_null() {
            return Err("Failed to decode TLV.".to_string());
        }

        // Copy the decoded data into a Rust Vec<u8>
        let output_size = *size as usize;
        let output_data = unsafe { Vec::from_raw_parts(output_ptr, output_size, output_size) };

        Ok(output_data)
    }

    /// Calculates the length of a TLV record
    pub fn record_length(
        input_stream: &[u8],
        field_length_size: &mut usize,
        field_value_size: &mut usize,
    ) -> Result<usize, String> {
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
    pub fn append(a: &mut Vec<u8>, b: &mut Vec<u8>) -> Result<Vec<u8>, String> {
        let a_ptr = a.as_mut_ptr();
        let b_ptr = b.as_mut_ptr();

        let output_ptr = unsafe { freefare_sys::tlv_append(a_ptr, b_ptr) };

        if output_ptr.is_null() {
            return Err("Failed to append TLV objects.".to_string());
        }

        let output_size = a.len() + b.len(); // Approximation
        let output_data = unsafe { Vec::from_raw_parts(output_ptr, output_size, output_size) };

        Ok(output_data)
    }
}
