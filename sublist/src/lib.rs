#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list == _second_list {
        return Comparison::Equal;
    }
    if _first_list.len() < _second_list.len() && is_subset(_first_list, _second_list) {
        return Comparison::Sublist;
    }
    if _first_list.len() > _second_list.len() && is_subset(_second_list, _first_list) {
        return Comparison::Superlist;
    }
    Comparison::Unequal
}

fn is_subset<T: PartialEq>(subset: &[T], superset: &[T]) -> bool {
    subset.is_empty() || superset.windows(subset.len()).any(|window| window == subset)
}
