pub(crate) fn encode_length(namespace: &[u8]) -> [u8; 2] {
    if namespace.len() > 0xFFFF {
        panic!("only supports namespaces up to length 0xFFFF")
    }
    let length_bytes = (namespace.len() as u32).to_be_bytes();
    [length_bytes[2], length_bytes[3]]
}

pub(crate) fn namespaces_with_key(namespaces: &[&[u8]], key: &[u8]) -> Vec<u8> {
    let mut size = key.len();
    for &namespace in namespaces {
        size += namespace.len() + 2;
    }

    let mut out = Vec::with_capacity(size);
    for &namespace in namespaces {
        out.extend_from_slice(&encode_length(namespace));
        out.extend_from_slice(namespace);
    }
    out.extend_from_slice(key);
    out
}
