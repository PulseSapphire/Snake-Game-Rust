pub trait Layout {
    fn getLayout<T: Sized, const R: usize>() -> [T; R];
}