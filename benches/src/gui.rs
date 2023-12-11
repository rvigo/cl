// use criterion::{criterion_group, criterion_main};

mod command_context {
    // use benches::load_command_handler;
    // use cl_gui::entities::contexts::commands_context::CommandsContext;
    // use criterion::{Bencher, Criterion};

    // pub fn init_command_context(c: &mut Criterion) {
    //     c.bench_function("init command context", |b: &mut Bencher| {
    //         let commands_file_handler = load_command_handler!(); // duplicate cause borrow checker stuff
    //         let commands = commands_file_handler.load().unwrap();

    //         b.iter(|| CommandsContext::new(commands.to_owned(), load_command_handler!()));
    //     });
    // }
}

// criterion_group!(
//     benches,
//     //  command_context::init_command_context
// );
// criterion_main!(benches);
