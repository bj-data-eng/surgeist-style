use surgeist_style::{SubgridLineNameComponent, SubgridLineNameRepeatCount};

fn main() {
    let _subgrid = SubgridLineNameComponent::Repeat {
        count: SubgridLineNameRepeatCount::Count(1),
        line_name_sets: vec![],
    };
}
