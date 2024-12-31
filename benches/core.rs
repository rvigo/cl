use alias_sample::ALIASES;
use cl_core::{fs, Command, Commands};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::{seq::SliceRandom, thread_rng};
use std::{borrow::Cow, collections::HashMap};

pub fn init_commands(command_map: HashMap<String, Vec<Command<'static>>>) -> Commands<'static> {
    Commands::init(command_map)
}

pub fn find_command(commands: Commands, alias: &str, namespace: Option<&str>) {
    let _ = commands.find(alias, namespace);

    black_box(commands);
}

pub fn update_command(mut commands: Commands, alias: &str, namespace: Option<&str>) {
    let current = commands.find(alias, namespace);
    match current {
        Ok(cmd) => {
            let mut new = cmd.clone();
            new.description = Some(Cow::Borrowed("Updated description"));
            let _ = commands.edit(&new, &cmd);
        }
        Err(err) => {
            eprintln!("Command not found: {}", err);
        }
    }

    black_box(commands);
}

pub fn add_command(mut commands: Commands, command: Command<'static>) {
    let _ = commands.add(&command);

    black_box(commands);
}

pub fn remove_command(mut commands: Commands<'static>, command: Command<'static>) {
    let _ = commands.remove(&command);

    black_box(commands);
}

pub fn load_command_map() -> HashMap<String, Vec<Command<'static>>> {
    let command_file_path = "benches/data/sample.toml";
    let result = fs::load_from(command_file_path);
    let command_map = match result {
        Ok(map) => map,
        Err(err) => {
            eprintln!("Failed to load command file: {}", err);
            std::process::exit(1);
        }
    };

    command_map
}

fn benchmark(c: &mut Criterion) {
    let mut rng = thread_rng();
    let map = load_command_map();
    c.bench_function("init commands struct", |b| {
        b.iter(|| init_commands(map.clone()));
    });
    c.bench_function("find command", |b| {
        b.iter(|| {
            find_command(
                init_commands(map.clone()),
                ALIASES.choose(&mut rng).unwrap(),
                None,
            )
        });
    });
    c.bench_function("find command with namespace", |b| {
        b.iter(|| {
            find_command(
                init_commands(map.clone()),
                ALIASES.choose(&mut rng).unwrap(),
                ALIASES.choose(&mut rng).copied(),
            )
        });
    });
    c.bench_function("update command", |b| {
        b.iter(|| {
            update_command(
                init_commands(map.clone()),
                ALIASES.choose(&mut rng).unwrap(),
                None,
            )
        });
    });

    c.bench_function("add command", |b| {
        b.iter(|| {
            add_command(
                init_commands(map.clone()),
                Command {
                    namespace: Cow::Borrowed("test"),
                    command: Cow::Borrowed("test"),
                    description: Some(Cow::Borrowed("test")),
                    alias: Cow::Borrowed("test"),
                    tags: None,
                },
            )
        });
    });

    let to_be_removed = Command {
        namespace: Cow::Borrowed("test"),
        command: Cow::Borrowed("test"),
        description: Some(Cow::Borrowed("test")),
        alias: Cow::Borrowed("test"),
        tags: None,
    };
    c.bench_function("remove command", |b| {
        b.iter(|| {
            remove_command(init_commands(map.clone()), to_be_removed.clone());
        });
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);

mod alias_sample {
    pub const ALIASES: &[&str] = &[
        "apple",
        "banana",
        "cherry",
        "date",
        "elderberry",
        "fig",
        "grape",
        "honeydew",
        "ice",
        "jackfruit",
        "kiwi",
        "lemon",
        "mango",
        "nectarine",
        "orange",
        "papaya",
        "quince",
        "raspberry",
        "strawberry",
        "tangerine",
        "ugli",
        "vanilla",
        "watermelon",
        "xigua",
        "yam",
        "zucchini",
        "almond",
        "beet",
        "carrot",
        "dill",
        "eggplant",
        "fennel",
        "garlic",
        "hazelnut",
        "iceberg",
        "jalapeno",
        "kale",
        "leek",
        "mushroom",
        "nutmeg",
        "oregano",
        "parsley",
        "quinoa",
        "radish",
        "spinach",
        "thyme",
        "urchin",
        "vermicelli",
        "wheat",
        "xylitol",
        "yogurt",
        "acorn",
        "basil",
        "cumin",
        "dillweed",
        "eucalyptus",
        "frankincense",
        "geranium",
        "hyacinth",
        "iris",
        "juniper",
        "kumquat",
        "lavender",
        "mint",
        "nutella",
        "olive",
        "poppy",
        "rose",
        "saffron",
        "tarragon",
        "umeboshi",
        "violet",
        "walnut",
        "xanthan",
        "yarrow",
        "zinnia",
        "avocado",
        "blackberry",
        "cucumber",
        "dragonfruit",
        "endive",
        "grapefruit",
        "horseradish",
        "kohlrabi",
        "lime",
        "mustard",
        "nasturtium",
        "onion",
        "peanut",
        "rhubarb",
        "scallion",
        "tomato",
        "wasabi",
        "yuca",
        "amaranth",
        "barley",
        "cilantro",
        "dandelion",
        "elderflower",
        "ginger",
        "honey",
        "jasmine",
        "lemongrass",
        "marjoram",
        "pistachio",
        "rosemary",
        "sage",
        "zest",
        "anise",
        "coriander",
        "pear",
        "starfruit",
        "turmeric",
        "xylophone",
    ];
}
