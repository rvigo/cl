use crate::fuzzy::Fuzzy;
use crate::state::selected_command::SelectedCommand;
use crate::state::selected_namespace::SelectedNamespace;
use anyhow::bail;
use cl_core::{
    fs, Command, CommandExec, CommandMap, CommandMapExt, CommandVec, CommandVecExt, Commands,
    Config,
};
use log::{debug, error};
use nucleo_matcher::pattern::{Atom, AtomKind, CaseMatching, Normalization};
use nucleo_matcher::Matcher;
use nucleo_matcher::{Config as MatcherConfig, Utf32Str};
use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};

pub struct State {
    commands: Commands<'static>,
    selected_command: Option<SelectedCommand>,
    selected_namespace: SelectedNamespace,
    namespaces: Vec<String>,
    config: Box<dyn Config>,

    // search
    current_query: Option<String>,
    cmd_map: CommandMap<'static>,
    current_items: HashSet<Command<'static>>,
}

const DEFAULT_NAMESPACE: &str = "All";

impl State {
    pub fn new(cfg: impl Config + 'static) -> State {
        // cmd load
        let command_map = fs::load_from(cfg.command_file_path()).unwrap();
        let commands = Commands::init(command_map);

        let cmd_map = commands.as_map().clone();
        let current_items = HashSet::from_iter(commands.as_list());

        // namespaces
        let namespaces: Vec<String> = commands.as_list().namespaces();
        let mut namespaces = append_default_namespace(namespaces);
        namespaces.sort_by_key(|a| a.to_lowercase());

        let selected = SelectedCommand::from_vec(&commands.as_list());

        Self {
            commands,
            selected_namespace: SelectedNamespace::new(0, DEFAULT_NAMESPACE.to_string()),
            selected_command: selected,
            namespaces,
            config: Box::new(cfg),
            current_query: None,
            current_items,
            cmd_map,
        }
    }

    pub fn select(&mut self, idx: usize) {
        self.selected_command = Some(SelectedCommand::new(
            self.current_items.to_vec()[idx].clone(),
            idx,
        ))
    }

    pub fn get_selected_command(&self) -> Option<SelectedCommand> {
        self.selected_command.clone()
    }

    pub fn get_all_items(&self) -> CommandVec<'static> {
        debug!("got all items");
        self.current_items.to_vec().sort_and_return()
    }

    pub fn get_all_namespaces(&self) -> Vec<String> {
        debug!("got all namespaces");
        self.namespaces.clone()
    }

    fn set_namespaces(&mut self, items: CommandVec<'static>) {
        self.namespaces = append_default_namespace(items.namespaces());
    }

    pub fn get_current_query(&self) -> String {
        self.current_query.clone().unwrap_or_default()
    }

    /// Commands based on the current namespace
    pub fn current_commands(&self) -> CommandVec<'static> {
        self.filter_command_by_namespace(&self.selected_namespace.name)
    }

    pub fn next_item(&mut self) -> Option<SelectedCommand> {
        let items = self.current_items.to_vec();
        if let Some(selected_command) = &self.selected_command {
            let current = selected_command.current_idx;
            let next = (current + 1) % items.len();

            self.select(next);
        } else {
            panic!("selected command is None");
        }

        self.selected_command.clone()
    }

    pub fn previous_item(&mut self) -> Option<SelectedCommand> {
        let items = self.current_items.to_vec();
        if let Some(selected_command) = &self.selected_command {
            let current = selected_command.current_idx;
            let previous = (current + items.len() - 1) % items.len();

            self.select(previous);
        } else {
            panic!("selected command is None");
        }

        self.selected_command.clone()
    }

    pub fn next_tab(&mut self) -> (SelectedNamespace, CommandVec<'static>) {
        let current_idx = self.selected_namespace.idx;
        let namespaces = &self.namespaces;
        if namespaces.is_empty() {
            error!("no namespaces found");
            return (self.selected_namespace.to_owned(), vec![]);
        }
        let next = (current_idx + 1) % namespaces.len();
        let next_namespace = &self.namespaces[next];

        self.selected_namespace = SelectedNamespace::new(next, next_namespace.to_string());

        let filtered_commands = &self.filter_command_by_namespace(next_namespace);

        self.current_items = HashSet::from_vec(filtered_commands.to_vec());
        self.selected_command = SelectedCommand::from_vec(filtered_commands);

        (
            self.selected_namespace.to_owned(),
            filtered_commands.to_vec().sort_and_return(),
        )
    }

    pub fn previous_tab(&mut self) -> (SelectedNamespace, CommandVec<'static>) {
        let current = self.selected_namespace.idx;
        let items = &self.namespaces;
        if items.is_empty() {
            error!("no namespaces found");
            return (self.selected_namespace.to_owned(), vec![]);
        }
        let previous = (current + items.len() - 1) % items.len();
        let previous_namespace = &self.namespaces[previous];

        self.selected_namespace = SelectedNamespace::new(previous, previous_namespace.to_string());

        let filtered_commands = &self.filter_command_by_namespace(previous_namespace);
        self.current_items = HashSet::from_vec(filtered_commands.to_vec());
        self.selected_command = SelectedCommand::from_vec(filtered_commands);

        (
            self.selected_namespace.to_owned(),
            filtered_commands.to_vec().sort_and_return(),
        )
    }

    pub fn execute(&self) {
        if let Some(selected_command) = &self.selected_command {
            let command = selected_command.value.to_owned();
            debug!("executing command: {:?}", command);
            command
                .exec(false, self.config.preferences().quiet_mode())
                .unwrap();
        }
    }

    pub fn delete_command(&mut self) -> anyhow::Result<()> {
        if let Some(selected_command) = &self.selected_command {
            let command = &selected_command.value;
            match self.commands.remove(command) {
                Ok(map) => {
                    fs::save_at(&map, self.config.command_file_path())?;
                    self.current_items =
                        HashSet::from_iter(self.commands.as_list().sort_and_return());
                    self.selected_command = if !self.current_items.is_empty() {
                        Some(SelectedCommand::new(self.current_items.first(), 0))
                    } else {
                        None
                    };
                }
                Err(e) => {
                    bail!("Error deleting command: {:?}", e);
                }
            }
        }
        debug!("command deleted");
        Ok(())
    }

    /// Filters the commands based on a query and a namespace
    ///
    /// ## Arguments
    /// * `query` - A String representing the user's query
    ///
    #[inline(always)]
    pub fn filter(&mut self, query: String) {
        if query.is_empty() {
            self.current_query = None;
            return;
        }

        //
        self.current_query = Some(query.clone());
        let current_items = self.fuzzy_find(query).to_vec();
        self.current_items = HashSet::from_iter(current_items.clone());
        self.selected_command = SelectedCommand::from_vec(&current_items);
        self.set_namespaces(current_items);
    }

    fn ff_vec(&self, query: &str, command_vec: &CommandVec<'static>) -> Vec<Command<'static>> {
        let mut matcher = Matcher::new(MatcherConfig::DEFAULT);
        let atom = Atom::new(
            &query,
            CaseMatching::Ignore,
            Normalization::Smart,
            AtomKind::Fuzzy,
            false,
        );

        let mut buf = Vec::new();
        let mut scored = command_vec
            .iter()
            .cloned()
            .filter_map(|item| {
                let score =
                    atom.score(Utf32Str::new(&item.lookup_string(), &mut buf), &mut matcher);
                if let Some(score) = score {
                    debug!("item: {}, score: {score}", item.alias);
                    return Some((item, score));
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        scored.sort_by_key(|(_, score)| Reverse(*score));
        scored.into_iter().map(|(c, _)| c.to_owned()).collect()
    }

    fn fuzzy_find(&self, query: String) -> CommandMap<'static> {
        self.cmd_map
            .iter()
            .map(|(namespace, vec)| (namespace.clone(), self.ff_vec(&query, vec)))
            .collect::<HashMap<_, _>>()
    }

    #[inline(always)]
    fn filter_command_by_namespace(&self, namespace: &str) -> CommandVec<'static> {
        debug!("filtering commands by namespace: {}", namespace);
        if namespace == DEFAULT_NAMESPACE {
            return self.cmd_map.to_vec().sort_and_return();
        }

        let result = self
            .cmd_map
            .get(namespace)
            .unwrap_or(&vec![])
            .to_vec()
            .sort_and_return();
        if let Some(query) = &self.current_query {
            self.ff_vec(query, &result)
        } else {
            result
        }
    }
}

fn append_default_namespace(namespaces: Vec<String>) -> Vec<String> {
    if !namespaces.is_empty() || namespaces.len() > 1 {
        let mut namespaces_set: HashSet<String> =
            namespaces.iter().cloned().map(|ns| ns.into()).collect();

        namespaces_set.insert(DEFAULT_NAMESPACE.to_string());

        let mut namespaces = namespaces_set.into_iter().collect::<Vec<String>>();
        namespaces.sort_by_key(|a| a.to_lowercase());

        namespaces
    } else {
        namespaces
    }
}

trait HashSetExt<T> {
    fn to_vec(&self) -> Vec<T>;

    fn first(&self) -> T;

    fn from_vec(vec: Vec<T>) -> HashSet<T>;
}

impl HashSetExt<Command<'static>> for HashSet<Command<'static>> {
    fn to_vec(&self) -> Vec<Command<'static>> {
        self.iter().cloned().collect()
    }

    fn first(&self) -> Command<'static> {
        self.to_vec().first()
    }

    fn from_vec(vec: Vec<Command<'static>>) -> HashSet<Command<'static>> {
        vec.into_iter().collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::Result;
    use cl_core::{CommandBuilder, Preferences};
    use std::fs::File;
    use std::path::PathBuf;
    use tempdir::TempDir;

    struct TestConfig {
        cfp: PathBuf,
    }

    impl TestConfig {
        pub fn new() -> Result<Self> {
            let cfp = TempDir::new("test")?.into_path().join("commands.toml");

            let _cf = File::create(cfp.clone())?;

            Ok(Self { cfp })
        }
    }

    impl Config for TestConfig {
        fn load() -> Result<Self>
        where
            Self: Sized,
        {
            todo!()
        }

        fn save(&self) -> Result<()> {
            todo!()
        }

        fn preferences(&self) -> &Preferences {
            todo!()
        }

        fn preferences_mut(&mut self) -> &mut Preferences {
            todo!()
        }

        fn command_file_path(&self) -> PathBuf {
            self.cfp.clone()
        }

        fn log_dir_path(&self) -> PathBuf {
            todo!()
        }
    }

    fn setup_state() -> Result<State> {
        let mut commands = HashMap::new();
        let command1 = CommandBuilder::default()
            .command("test")
            .namespace("test")
            .alias("test")
            .build();
        let command2 = CommandBuilder::default()
            .command("azul")
            .namespace("amarelo")
            .alias("laranja")
            .build();
        commands.insert(command1.namespace.to_string(), vec![command1]);
        commands.insert(command2.namespace.to_string(), vec![command2]);

        let cfg = TestConfig::new()?;
        fs::save_at(&commands, cfg.command_file_path())?;

        Ok(State::new(cfg))
    }

    #[test]
    fn should_filter_commands_by_namespace() -> Result<()> {
        let state = setup_state()?;

        let result = state.filter_command_by_namespace("test");

        assert_eq!(result.len(), 1);
        assert_eq!(result.first().alias, "test");

        Ok(())
    }

    #[test]
    fn should_filter_commands() -> Result<()> {
        let mut state = setup_state()?;

        state.filter("e".to_string());

        assert_eq!(state.current_items.len(), 2);

        state.filter("te".to_string());

        assert_eq!(state.current_items.len(), 1);
        assert_eq!(
            state.current_items.to_vec().first().alias.to_string(),
            "test".to_string()
        );

        state.filter("az".to_string());
        assert_eq!(state.current_items.len(), 1);
        assert_eq!(
            state.current_items.to_vec().first().alias.to_string(),
            "laranja".to_string()
        );

        Ok(())
    }

    #[test]
    fn should_append_the_default_namespace() -> Result<()> {
        let namespaces = vec!["a".to_string(), "a".to_string(), "b".to_string()];

        let result = append_default_namespace(namespaces);

        assert_eq!(result.len(), 3);
        assert_eq!(result.iter().filter(|&s| s == "a").count(), 1);
        assert!(result.contains(&"All".to_string()));

        Ok(())
    }
}
