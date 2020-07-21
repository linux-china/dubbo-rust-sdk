struct DubboFrame<'a> {
    magic_header: u16,
    flag: u8,
    status: u8,
    message_id: u64,
    message_length: u32,
    payload: &'a [u8],
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
