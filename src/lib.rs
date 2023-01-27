use std::process::Command;
use std::vec::Vec;

/*
 * macro for generating git commands
 */
macro_rules! add_git_command {
    ($a:ident, $b:expr) => {
        pub fn $a(&mut self) -> &mut GitCommand {
            if !self.git_cmd_started {
                self.git_cmd_started = true;
                for substr in $b.split("_") {
                    self.git_cmd.push(String::from(substr));
                }
            }
            self
        }
    };
    ($a:ident, $b:expr, false) => {
        pub fn $a(&mut self) -> &mut GitCommand {
            for substr in $b.split("_") {
                self.git_cmd.push(String::from(substr));
            }
            self
        }
    };
}

macro_rules! add_extra_git_text {
    ($a:ident, $func:expr) => {
        pub fn $a(&mut self, arg: &str) -> &mut GitCommand {
            self.git_cmd
                .push(String::from($func(self, String::from(arg))));
            self
        }
    };
}

pub fn find_repo_path() -> String {
    GitCommand::create(false)
        .rev_parse()
        .options()
        .double(String::from("show-toplevel"), None, None)
        .done()
        .run(false)
        .unwrap_or(String::from(""))
}

pub fn find_repo_name() -> String {
    let unparsed = find_repo_path();
    let res = unparsed.split("/").collect::<Vec<&str>>();
    String::from(res[res.len() - 1])
}

pub struct GitOptions<'a> {
    parent: &'a mut GitCommand,
    single_dash: String,
    double_dash: Vec<(String, String, bool)>,
}

impl<'a> GitOptions<'a> {
    fn new(parent: &'a mut GitCommand) -> GitOptions<'a> {
        GitOptions {
            parent,
            single_dash: String::from(""),
            double_dash: Vec::new(),
        }
    }

    pub fn single(&mut self, c: char) -> &'a mut GitOptions {
        self.single_dash.push(c);
        self
    }

    pub fn double(
        &mut self,
        name: String,
        value: Option<String>,
        equals: Option<bool>,
    ) -> &'a mut GitOptions {
        self.double_dash.push((
            name,
            value.unwrap_or(String::from("")).clone(),
            equals.unwrap_or(false),
        ));
        self
    }

    fn __options(&self) -> Vec<String> {
        let mut ret = Vec::new();
        if self.single_dash.len() != 0 {
            ret.push(String::from("-") + self.single_dash.as_str());
        }
        for (k, v, equals) in &self.double_dash {
            ret.push(String::from("--") + k);
            if !v.is_empty() {
                ret.push(
                    if equals.to_owned() {
                        String::from("=")
                    } else {
                        String::from("")
                    } + v.as_str(),
                );
            }
        }
        ret
    }

    pub fn done(&mut self) -> &mut GitCommand {
        self.parent.git_cmd.extend(self.__options());
        &mut self.parent
    }
}

pub struct GitCommand {
    repo_name: Option<String>,
    find_root: bool,
    git_cmd: Vec<String>,
    git_cmd_started: bool,
}

impl<'a> GitCommand {
    pub fn create(find_root: bool) -> GitCommand {
        let mut git = GitCommand {
            repo_name: None,
            find_root,
            git_cmd_started: false,
            git_cmd: Vec::new(),
        };
        if git.find_root {
            git.repo_name = Some(find_repo_name());
        }
        git
    }

    fn sanitize(&self, a: String) -> String {
        if self.repo_name.is_some() {
            a.replace("%%repo_name%%", self.repo_name.clone().unwrap().as_str())
        } else {
            a
        }
    }

    fn _reset(&mut self) {
        self.git_cmd = vec![String::from("git")];
        self.git_cmd_started = false;
    }

    fn command_list(&self) -> Vec<String> {
        self.git_cmd.clone()
    }

    fn command(&self) -> String {
        let added = String::from("git ") + self.git_cmd.join(" ").as_str();
        String::from(added)
    }

    pub fn run(&self, debug: bool) -> Option<String> {
        if debug {
            println!("{}", self.command());
            return None;
        }
        let output = Command::new("git")
            .args(self.command_list())
            .output()
            .expect("Failed to run git command");
        println!(
            "{:?}",
            Command::new("git").args(self.command_list()).get_args()
        );
        Some(String::from_utf8(output.stdout).unwrap())
    }

    pub fn options(&mut self) -> GitOptions {
        GitOptions::new(self)
    }

    // main commands
    // status
    add_git_command!(status, "status");
    // reset
    add_git_command!(reset, "reset");
    // add
    add_git_command!(add, "add");
    // revParse
    add_git_command!(rev_parse, "rev-parse");
    // init
    add_git_command!(init, "init");
    // log
    add_git_command!(log, "log");
    // checkout
    add_git_command!(checkout, "checkout");
    // branch
    add_git_command!(branch, "branch");
    // clone
    add_git_command!(clone, "clone");
    // commit
    add_git_command!(commit, "commit");
    // config
    add_git_command!(config, "config");
    // submodule
    add_git_command!(submodule, "submodule");
    // fetch
    add_git_command!(fetch, "fetch");
    // merge
    add_git_command!(merge, "merge");
    // mv
    add_git_command!(mv, "mv");
    // pull
    add_git_command!(pull, "pull");
    // pull_origin
    add_git_command!(pull_origin, "pull_origin");
    // push_origin
    add_git_command!(push_origin, "push_origin");
    // rebase
    add_git_command!(rebase, "rebase");
    // remote
    add_git_command!(remote, "remote");
    // rm
    add_git_command!(rm, "rm");
    // restore
    add_git_command!(restore, "restore");
    // show
    add_git_command!(show, "show");
    // switch
    add_git_command!(switch, "switch");
    // tag
    add_git_command!(tag, "tag");
    // worktree
    add_git_command!(worktree, "worktree");
    // master
    add_git_command!(master, "master", false);
    // upstream
    add_git_command!(upstream, "upstream", false);

    // text appended to the git command
    // branch_name
    add_extra_git_text!(branch_name, GitCommand::sanitize);
    // url
    add_extra_git_text!(url, GitCommand::sanitize);
    // text
    add_extra_git_text!(text, GitCommand::sanitize);
}
