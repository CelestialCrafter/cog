use std::iter::once;

pub fn partition_n<T, B, F, const N: usize>(iter: impl Iterator<Item = T>, mut f: F) -> [B; N]
where
    B: Default + Extend<T>,
    F: FnMut(&T) -> usize,
{
    let mut collections = [(); N].map(|_| B::default());

    for item in iter {
        let index = f(&item);
        collections[index].extend(once(item));
    }

    collections
}
