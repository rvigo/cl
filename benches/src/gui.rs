use criterion::{criterion_group, criterion_main};

pub mod cache {
    use benches::{build_command, load_command_handler};
    use cl_gui::entities::contexts::cache_info::CacheInfo;
    use criterion::{Bencher, Criterion};

    pub fn load_entries(c: &mut Criterion) {
        c.bench_function("load entries", |b: &mut Bencher| {
            let commands = load_command_handler!().load().unwrap();

            b.iter(|| {
                CacheInfo::new(commands.to_owned());
            })
        });
    }

    pub fn add_entry(c: &mut Criterion) {
        c.bench_function("add entry", |b: &mut Bencher| {
            let command = build_command!();

            let commands = load_command_handler!().load().unwrap();

            let mut cache_info = CacheInfo::new(commands.to_owned());

            b.iter(|| cache_info.insert_entry(command.to_owned()))
        });
    }

    pub fn update_entry(c: &mut Criterion) {
        c.bench_function("update entry", |b: &mut Bencher| {
            let command = build_command!();

            let mut commands = load_command_handler!().load().unwrap();

            commands.push(command.to_owned());

            let updated_command = build_command!(alias => "updated";);

            let mut cache_info = CacheInfo::new(commands.to_owned());

            b.iter(|| cache_info.update_entry(&updated_command, &command))
        });
    }
}

criterion_group!(
    benches,
    cache::load_entries,
    cache::add_entry,
    cache::update_entry
);
criterion_main!(benches);
