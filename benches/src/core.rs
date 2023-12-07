use benches::load_command_handler;
use cl_core::command::Command;
use criterion::{criterion_group, criterion_main};

fn load_command_file() -> Vec<Command> {
    load_command_handler!().load().unwrap()
}

mod core {
    use crate::load_command_file;
    use benches::build_command;
    use cl_core::commands::Commands;
    use criterion::{Bencher, Criterion};

    pub fn load_commands(c: &mut Criterion) {
        c.bench_function("load commands", |b: &mut Bencher| {
            b.iter(|| load_command_file());
        });
    }

    pub fn init_commands(c: &mut Criterion) {
        let command_vec = load_command_file();

        c.bench_function("init commands", |b: &mut Bencher| {
            b.iter(|| Commands::init(command_vec.to_owned()));
        });
    }

    pub fn find_command(c: &mut Criterion) {
        let command_vec = load_command_file();
        let commands = Commands::init(command_vec);

        c.bench_function("find command", |b: &mut Bencher| {
            b.iter(|| commands.find_command("lc".to_string(), None));
        });

        c.bench_function("find command with namespace", |b: &mut Bencher| {
            b.iter(|| commands.find_command("cl".to_string(), Some("bash".to_string())));
        });
    }

    pub fn update_command(c: &mut Criterion) {
        let command = build_command!();

        let mut command_vec = load_command_file();
        command_vec.push(command.to_owned());
        let mut commands = Commands::init(command_vec);

        let edited_command = build_command!(alias => "updated";);

        c.bench_function("update command (alias)", |b: &mut Bencher| {
            b.iter(|| commands.add_edited_command(&edited_command, &command));
        });

        let edited_command = build_command!(namespace => "updated";);

        c.bench_function("update command (namespace)", |b: &mut Bencher| {
            b.iter(|| commands.add_edited_command(&edited_command, &command));
        });
    }
}

criterion_group!(
    benches,
    core::load_commands,
    core::init_commands,
    core::find_command,
    core::update_command
);
criterion_main!(benches);
