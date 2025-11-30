use std::cmp::Ordering;

// if you have multiple PartialOrd elements, and you want an ordering with fallbacks if the
// ordering fails
pub fn fallback_ordering<T: PartialOrd, U: PartialOrd<T>>(
    vec1: &Vec<U>,
    vec2: &Vec<T>,
) -> Option<Ordering> {
    for element1 in vec1 {
        for element2 in vec2 {
            let ordering = element1.partial_cmp(element2);
            if ordering.is_some() {
                return ordering;
            }
        }
    }

    None
}
