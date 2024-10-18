use xilem::{
    view::{button, flex, label},
    MasonryView, Xilem,
};

fn app_logic(count: &mut i32) -> impl MasonryView<i32> {
    flex((
        button("Reset", |count| *count = 0),
        button("++", |count| *count += 1),
        button("--", |count| *count -= 1),
        label(format!("{}", count)),
    ))
}

fn main() {
    let app = Xilem::new(0, app_logic);
    app.run_windowed("Yeehoo".into()).unwrap();
}
