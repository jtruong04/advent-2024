use crate::Problem;

pub struct Solution;

impl Problem for Solution {
    fn part_one(&self, _test:bool) -> String {
        "Day 02a solution".to_string()
    }

    fn part_two(&self, _test:bool) -> String {
        "Day 02b solution".to_string()
    }

    fn add_to_registry(self, registry: &mut crate::Registry) {
        registry.register(2, Box::new(self));
    }
}
