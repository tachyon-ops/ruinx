pub enum AppMode {
    EDITOR,
    GAME,
}

fn variant_eq<T>(a: &T, b: &T) -> bool {
    std::mem::discriminant(a) == std::mem::discriminant(b)
}

impl PartialEq for AppMode {
    fn eq(&self, other: &AppMode) -> bool {
        variant_eq(self, other)
    }
}
