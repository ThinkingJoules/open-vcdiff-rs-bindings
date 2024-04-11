use std::ptr;

mod bindings {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(unused)]

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}
pub const FORMAT_STANDARD: i32 = 0; //bindings::VCDiffFormatExtensionFlagValues_VCD_STANDARD_FORMAT;
pub const FORMAT_INTERLEAVED: i32 = 1; //bindings::VCDiffFormatExtensionFlagValues_VCD_FORMAT_INTERLEAVED;
pub const FORMAT_CHECKSUM: i32 = 2; //bindings::VCDiffFormatExtensionFlagValues_VCD_FORMAT_CHECKSUM;
pub const FORMAT_JSON: i32 = 4; // bindings::VCDiffFormatExtensionFlagValues_VCD_FORMAT_JSON;

pub fn encode(dictionary: &[u8], target: &[u8], format_extensions: i32, look_for_target_matches: bool) -> Vec<u8> {

    let mut encoded_data = ptr::null_mut();
    let mut encoded_len = 0;
    unsafe {
        bindings::vcdiff_encode(dictionary.as_ptr(),
                                dictionary.len(),
                                target.as_ptr(),
                                target.len(),
                                &mut encoded_data,
                                &mut encoded_len,
                                format_extensions,
                                look_for_target_matches);
    }

    let mut result = Vec::with_capacity(encoded_len);

    unsafe {
        ptr::copy_nonoverlapping(encoded_data, result.as_mut_ptr(), encoded_len);
        result.set_len(encoded_len);
        bindings::vcdiff_free_data(encoded_data);
    }
    result
}

pub fn decode(dictionary: &[u8], encoded: &[u8]) -> Vec<u8> {
    let mut target_data = ptr::null_mut();
    let mut target_len = 0;

    unsafe {
        bindings::vcdiff_decode(dictionary.as_ptr(),
                                dictionary.len(),
                                encoded.as_ptr(),
                                encoded.len(),
                                &mut target_data,
                                &mut target_len);
    }

    let mut result = Vec::with_capacity(target_len);

    unsafe {
        ptr::copy_nonoverlapping(target_data, result.as_mut_ptr(), target_len);
        result.set_len(target_len);
        bindings::vcdiff_free_data(target_data);
    }

    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_standard() {
        let dict: &[u8] = &[1, 2, 3];
        let target: &[u8] = &[4, 5, 6, 1, 2, 3, 4, 5, 6, 1, 2, 4];
        let encoded = encode(dict, target, FORMAT_STANDARD, false);
        let decoded = decode(dict, &encoded);

        assert_eq!(target, decoded.as_slice());
    }

    #[test]
    fn roundtrip_standard_target_matches() {
        let dict: &[u8] = &[1, 2, 3];
        let target: &[u8] = &[4, 5, 6, 1, 2, 3, 4, 5, 6, 1, 2, 4];
        let encoded = encode(dict, target, FORMAT_STANDARD, true);
        let decoded = decode(dict, &encoded);

        assert_eq!(target, decoded.as_slice());
    }

    #[test]
    fn roundtrip_interleaved() {
        let dict: &[u8] = &[1, 2, 3];
        let target: &[u8] = &[4, 5, 6, 1, 2, 3, 4, 5, 6, 1, 2, 4];
        let encoded = encode(dict, target, FORMAT_INTERLEAVED, false);
        let decoded = decode(dict, &encoded);

        assert_eq!(target, decoded.as_slice());
    }

    #[test]
    fn roundtrip_interleaved_target_matches() {
        let dict: &[u8] = &[1, 2, 3];
        let target: &[u8] = &[4, 5, 6, 1, 2, 3, 4, 5, 6, 1, 2, 4];
        let encoded = encode(dict, target, FORMAT_INTERLEAVED, true);
        let decoded = decode(dict, &encoded);

        assert_eq!(target, decoded.as_slice());
    }

    #[test]
    fn roundtrip_checksum() {
        let dict: &[u8] = &[1, 2, 3];
        let target: &[u8] = &[4, 5, 6, 1, 2, 3, 4, 5, 6, 1, 2, 4];
        let encoded = encode(dict, target, FORMAT_CHECKSUM, false);
        let decoded = decode(dict, &encoded);

        assert_eq!(target, decoded.as_slice());
    }

    #[test]
    fn roundtrip_checksum_target_matches() {
        let dict: &[u8] = &[1, 2, 3];
        let target: &[u8] = &[4, 5, 6, 1, 2, 3, 4, 5, 6, 1, 2, 4];
        let encoded = encode(dict, target, FORMAT_CHECKSUM, true);
        let decoded = decode(dict, &encoded);

        assert_eq!(target, decoded.as_slice());
    }

    #[test]
    fn roundtrip_interleaved_checksum() {
        let dict: &[u8] = &[1, 2, 3];
        let target: &[u8] = &[4, 5, 6, 1, 2, 3, 4, 5, 6, 1, 2, 4];
        let encoded = encode(dict, target, FORMAT_INTERLEAVED | FORMAT_CHECKSUM, false);
        let decoded = decode(dict, &encoded);

        assert_eq!(target, decoded.as_slice());
    }

    #[test]
    fn roundtrip_interleaved_checksum_target_matches() {
        let dict: &[u8] = &[1, 2, 3];
        let target: &[u8] = &[4, 5, 6, 1, 2, 3, 4, 5, 6, 1, 2, 4];
        let encoded = encode(dict, target, FORMAT_INTERLEAVED | FORMAT_CHECKSUM, true);
        let decoded = decode(dict, &encoded);

        assert_eq!(target, decoded.as_slice());
    }
}