use surgeist_style::{GridTrackComponent, TrackRepeat, TrackRepeatCount};

fn main() {
    let _repeat = TrackRepeat {
        count: TrackRepeatCount::Count(1),
        components: Vec::<GridTrackComponent>::new(),
    };
}
