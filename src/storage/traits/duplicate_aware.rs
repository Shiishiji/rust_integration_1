use crate::storage::models::Laptop;

pub trait DuplicateAware<T: PartialEq> {
    fn is_duplicate(&self, other: &T) -> bool;
}

impl DuplicateAware<Laptop> for Vec<Laptop> {
    fn is_duplicate(&self, other: &Laptop) -> bool {
        for l in self.iter() {
            if l == other {
                return true;
            }
        }

        false
    }
}
