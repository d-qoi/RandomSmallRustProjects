
enum CallReply {
    Reply()
}

enum Timeout {
    Infinite,
    Milli(u64),
    Secs(u64)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
