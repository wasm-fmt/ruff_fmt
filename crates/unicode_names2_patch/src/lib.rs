/// This is not correct, but the formatter does not change the output of Unicode.
/// Additionally, we do not require users to input the correct name.
/// However, this optimization reduces our wasm file size by a substantial 700kb, which is a significant benefit.
pub fn character(_name: &str) -> Option<char> {
    Some('a')
}
