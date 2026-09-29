//! The shell leg's parse: which paths a `Bash` or `PowerShell` command writes.
//!
//! A path is a write target only where the command's grammar puts a write (record
//! `hook-enforcement-field-review` D5, as amended at S12): the right side of a redirect, every
//! argument of `tee`, the destination of `cp` and `install`, every argument of `mv` (a source is a
//! removal from its home), the file operands of an in-place `sed` or `perl`, and `dd of=`. Every
//! other position is a read. The scan this replaces fired a command's arm on any word, read a `-i`
//! anywhere later in the line as in-place, and left a separator glued to a word (`file;`) unsplit,
//! so `sed -n … <home>; grep -i …` was denied as an edit. That was 13 of the 21 shell denies across
//! two real implement runs (record F3).
//!
//! # A scan, not a parser
//!
//! The command text is scanned best-effort, and the deny reason says so: the design the prior
//! record ruled (`hook-enforced-artifact-schema` D1c). The pre-code ladder stops at the codebase
//! rung, because the tokenizer this module grew from was already here. The shelf was weighed and
//! passed over: `shlex` and `shell-words` split words but know no operators, which is where the bug
//! was; `brush-parser` and `yash-syntax` are strict parsers that return an error on text they
//! cannot parse, where the gate must answer any text and fail open, and neither reads PowerShell.
//!
//! # What the scan cannot see
//!
//! A target that opens with `$`, `{}` or `~` is an expansion the scan cannot resolve, so it is
//! dropped rather than guessed at. A `cd` or `pushd` whose one operand is an absolute literal
//! directory is followed: a relative target after it is joined to that directory, and inside
//! `( … )` the `cd` holds only to its `)`. After a `cd` to an expansion, a relative target whose
//! first segment is `.mochiko` is kept as written and any other is dropped; after any other `cd`
//! the directory is unknown, and every relative target is dropped. So a `cd $X` that points outside
//! the tree, followed by a `.mochiko/…` write, is joined to the payload's `cwd` all the same: a
//! false deny, accepted. Three shapes stay open, disclosed: a `cd` into a home by a relative or
//! expanded operand before a relative write, a path held in a variable, and a command substitution
//! inside double quotes, which is one quoted word and is never split. A `cd` in a pipeline or in
//! the background is read as though it held. A relative write from a working directory inside a
//! home is the hook's to close, by joining the target to the payload's `cwd`.
//!
//! `((` is scanned twice, and a target either reading names is a write target. The exact reading
//! takes `((` as bash and zsh do: arithmetic when the `)` closing its inner `(` is followed at once
//! by the outer `)`, two subshells otherwise. After 64 non-arithmetic answers anywhere in one scan
//! (each process substitution scans with its own budget), every further `((` reads as two subshells
//! without a look ahead. The arithmetic reading is the scan's before G1 R2: every `((` is one word,
//! to the `)` that closes its first `(`. Each alone can miss a write the other sees. The arithmetic
//! reading swallows `((echo a); tee x)` whole. The exact reading, where it takes an arithmetic `((`
//! for subshells, reads a `<<` shift inside it as a heredoc that swallows every later line: past
//! the budget, or where `closing_paren` misjudges the inner close, since it knows no backticks, no
//! `case` pattern and no `\"` inside double quotes (G3 B1). So no write either reading sees is
//! missed, and none the scan before G1 R2 saw. A write both readings miss stays open: real
//! subshells that `closing_paren` misjudges as arithmetic are one. The errors are the union of both
//! readings' false denies, such as an arithmetic `>` read as a redirect past the budget.

use std::collections::HashSet;

/// One lexical unit of a command line.
#[derive(Debug, PartialEq)]
enum Token {
    /// A word, its quotes removed.
    Word(String),
    /// Anything that ends a simple command: `;` `|` `||` `&&` `&` `|&`, a newline.
    Separator,
    /// A POSIX `(`: it ends a simple command, and opens a subshell whose `cd` holds only inside it.
    Open,
    /// A POSIX `)`: it ends a simple command, and closes the subshell.
    Close,
    /// A redirect whose operand is written: `>` `>>` `>|` `>!` `>>!` `&>` `&>>`.
    Write,
    /// `>&`: a file when its operand is a word, a descriptor copy when it is a number or `-`.
    Duplicate,
    /// A redirect whose operand is read: `<` `<&` `<<<`.
    Read,
    /// A process substitution, `>(…)` or `<(…)`: a command of its own, tokenized apart, which
    /// leaves the word run around it intact (`tee >(cat) <home>` still reaches `<home>`).
    Substitution(Vec<Token>),
}

/// The two quoting dialects.
///
/// PowerShell escapes with a backtick, so a backslash is literal there (a Windows path would not
/// survive POSIX escaping), and it has no heredocs. Its parentheses are sub-expressions, not
/// separators: splitting on them would cut a cmdlet's run short of its `-Path`.
#[derive(Clone, Copy, PartialEq)]
enum Dialect {
    Posix,
    PowerShell,
}

/// How a POSIX scan reads `((` (see the module doc).
#[derive(Clone, Copy)]
enum DoubleParen {
    /// As bash and zsh do, within the look-ahead budget of [`arithmetic_close`].
    Exact,
    /// Always arithmetic: one word, to the `)` that closes its first `(`.
    Arithmetic,
}

/// Every path this command text writes to: the targets of the exact reading, then each target of
/// the arithmetic reading that the exact one does not name.
pub fn write_targets(command: &str) -> Vec<String> {
    let mut targets = read_targets(command, DoubleParen::Exact);
    let named: HashSet<String> = targets.iter().cloned().collect();
    targets.extend(
        read_targets(command, DoubleParen::Arithmetic)
            .into_iter()
            .filter(|target| !named.contains(target)),
    );
    targets
}

/// Every path one reading of this command text writes to.
fn read_targets(command: &str, parens: DoubleParen) -> Vec<String> {
    let mut targets = Vec::new();
    // The directory each open subshell writes from, the outermost first.
    let mut scopes = vec![Directory::Start];
    for simple in simple_commands(tokenize(command, Dialect::Posix, parens, 0)) {
        for _ in 0..simple.opens {
            let inherited = scopes.last().cloned().unwrap_or(Directory::Start);
            scopes.push(inherited);
        }
        if let Some(directory) = scopes.last_mut() {
            let written = simple
                .redirected
                .into_iter()
                .chain(arm_targets(&simple.words, 0));
            targets.extend(
                written
                    .filter(|t| resolvable(t))
                    .filter_map(|t| directory.join(t)),
            );
            if let Some(moved) = change_directory(&simple.words) {
                *directory = moved;
            }
        }
        for _ in 0..simple.closes {
            if scopes.len() > 1 {
                scopes.pop();
            }
        }
    }
    targets
}

/// Every path this PowerShell command text writes to.
///
/// The vocabulary is the write cmdlets a seat reaches for after a `Write` deny — `Set-Content`,
/// `Out-File`, `Add-Content`, `New-Item`, `Tee-Object`, and the `Copy-Item`/`Move-Item` pair — plus
/// the redirects PowerShell shares with the POSIX shells. A cmdlet is matched at any position of
/// its command, case-insensitively, because PowerShell is.
pub fn powershell_write_targets(command: &str) -> Vec<String> {
    let mut targets = Vec::new();
    for simple in simple_commands(tokenize(
        command,
        Dialect::PowerShell,
        DoubleParen::Exact,
        0,
    )) {
        targets.extend(simple.redirected);
        for (index, word) in simple.words.iter().enumerate() {
            // How many *positional* paths this cmdlet takes. `Copy-Item`/`Move-Item` take two,
            // because a move out of a home names the home as its source, as with POSIX `mv`.
            let positionals = match bare_command(word).to_ascii_lowercase().as_str() {
                "set-content" | "out-file" | "add-content" | "new-item" | "tee-object" => 1,
                "copy-item" | "move-item" => 2,
                _ => 0,
            };
            if positionals > 0 {
                targets.extend(cmdlet_targets(&simple.words[index + 1..], positionals));
            }
        }
    }
    targets.retain(|t| resolvable(t));
    targets
}

/// A target the scan can name.
///
/// An expansion (`$VAR`, a `find` or `xargs` `{}` placeholder, `~`) resolves only when the shell
/// runs, so it is dropped rather than guessed at. The old scan resolved none of them to a home
/// either, and under the closed world a guess would deny from a working directory inside
/// `.mochiko/`.
fn resolvable(target: &str) -> bool {
    !(target.is_empty()
        || target.starts_with('$')
        || target.starts_with("{}")
        || target.starts_with('~'))
}

// ---------------------------------------------------------------------------
// the working directory
// ---------------------------------------------------------------------------

/// Where a relative target is written from.
#[derive(Clone)]
enum Directory {
    /// The payload's `cwd`: the target is returned as written, and the hook joins it.
    Start,
    /// An absolute directory a `cd` named literally.
    At(String),
    /// A directory named by an expansion (`$X`, `${X}`, `~`). The field shape is
    /// `cd $MAIN && cat >> .mochiko/…`, the variable naming the repository, so a target whose first
    /// segment is `.mochiko` is kept as written for the hook to join; any other is dropped.
    Expanded,
    /// A directory the scan cannot name, so a relative target is dropped.
    Unknown,
}

impl Directory {
    /// Where `target` is written, or `None` when the scan cannot say.
    fn join(&self, target: String) -> Option<String> {
        if target.starts_with('/') {
            return Some(target);
        }
        match self {
            Directory::Start => Some(target),
            Directory::At(directory) => {
                Some(format!("{}/{target}", directory.trim_end_matches('/')))
            }
            Directory::Expanded => {
                let first = target.trim_start_matches("./").split('/').next();
                (first == Some(crate::home::HOME_TREE)).then_some(target)
            }
            Directory::Unknown => None,
        }
    }
}

/// The directory a `cd`, `pushd` or `popd` leaves the shell in, or `None` when `words` is another
/// command.
///
/// Only a single operand is read. An absolute literal is followed, and an expansion is marked as
/// such. A relative literal, none, `-`, a flag or a glob leaves the directory unknown, and so does
/// `popd`, which returns to a directory the scan did not track.
fn change_directory(words: &[String]) -> Option<Directory> {
    let start = command_word(words)?;
    let name = bare_command(&words[start]);
    if !matches!(name, "cd" | "pushd" | "popd") {
        return None;
    }
    Some(match &words[start + 1..] {
        [operand] if name != "popd" => {
            if operand.contains(['*', '?', '[']) {
                Directory::Unknown
            } else if operand.starts_with('~') || operand.contains(['$', '`']) {
                Directory::Expanded
            } else if operand.starts_with('/') && !operand.contains('{') {
                Directory::At(operand.clone())
            } else {
                Directory::Unknown
            }
        }
        _ => Directory::Unknown,
    })
}

// ---------------------------------------------------------------------------
// the POSIX arms
// ---------------------------------------------------------------------------

/// How deep a `find -exec` command or a process substitution is followed.
///
/// Past it, the nested command names no target: crafted input cannot exhaust the stack, and a
/// command the scan does not follow fails open, as every other shape it cannot see does.
const MAX_DEPTH: usize = 64;

/// The paths the command in `words` writes through its own arguments, `depth` levels down.
fn arm_targets(words: &[String], depth: usize) -> Vec<String> {
    let Some(start) = command_word(words) else {
        return Vec::new();
    };
    let args = &words[start + 1..];
    match bare_command(&words[start]) {
        "tee" => operands(args),
        "cp" | "install" => target_directory(args)
            .or_else(|| operands(args).pop())
            .into_iter()
            .collect(),
        "mv" => mv_targets(args),
        "git" => git_mv_targets(args),
        "sed" => sed_files(args),
        "perl" => perl_files(args),
        "dd" => args
            .iter()
            .filter_map(|arg| arg.strip_prefix("of="))
            .map(str::to_string)
            .collect(),
        "find" => find_exec_targets(args, depth),
        _ => Vec::new(),
    }
}

/// Words that open a compound command. The command word follows them.
const RESERVED: [&str; 9] = [
    "{", "!", "if", "then", "do", "else", "elif", "while", "until",
];

/// The flags of a wrapper that take the next word as their value, or `None` when `name` is not a
/// wrapper.
///
/// A wrapper runs the command after it, so the arms look past it; the value of one of its own
/// flags (`sudo -u root`) must not be read as that command. A value glued to its flag (`-uroot`,
/// `-I{}`) needs no skip. A long option with its value as a separate word is not modelled.
fn wrapper_value_flags(name: &str) -> Option<&'static [&'static str]> {
    Some(match name {
        "sudo" => &["-u", "-g", "-C", "-D", "-h", "-p", "-r", "-t", "-U", "-T"],
        "doas" => &["-u", "-C", "-a"],
        "env" => &["-u", "-C", "-S"],
        "exec" => &["-a"],
        "time" => &["-f", "-o"],
        "timeout" => &["-s", "-k"],
        "nice" => &["-n"],
        "stdbuf" => &["-i", "-o", "-e"],
        "xargs" => &["-I", "-L", "-n", "-P", "-s", "-d", "-E", "-a"],
        "command" | "builtin" | "nohup" => &[],
        _ => return None,
    })
}

/// The index of the command word: past leading assignments, reserved words and wrappers.
fn command_word(words: &[String]) -> Option<usize> {
    let mut index = 0;
    while let Some(word) = words.get(index) {
        if is_assignment(word) || RESERVED.contains(&word.as_str()) {
            index += 1;
            continue;
        }
        let name = bare_command(word);
        let Some(value_flags) = wrapper_value_flags(name) else {
            return Some(index);
        };
        index += 1;
        while let Some(next) = words.get(index) {
            if next.starts_with('-') && next.len() > 1 {
                index += if value_flags.contains(&next.as_str()) {
                    2
                } else {
                    1
                };
            } else if name == "env" && is_assignment(next) {
                index += 1;
            } else {
                break;
            }
        }
        // `timeout` takes its duration as a positional ahead of the command.
        if name == "timeout" {
            index += 1;
        }
    }
    None
}

/// `NAME=value`, the shape of a variable assignment ahead of a command.
fn is_assignment(word: &str) -> bool {
    word.split_once('=').is_some_and(|(name, _)| {
        name.starts_with(|c: char| c == '_' || c.is_ascii_alphabetic())
            && name.chars().all(|c| c == '_' || c.is_ascii_alphanumeric())
    })
}

/// A command word's bare name, without a leading path.
fn bare_command(word: &str) -> &str {
    word.rsplit('/').next().unwrap_or(word)
}

/// The arguments of a command that are not flags.
fn operands(args: &[String]) -> Vec<String> {
    args.iter()
        .filter(|arg| !arg.starts_with('-'))
        .cloned()
        .collect()
}

/// The destination a `-t` or `--target-directory` flag names, when one does.
///
/// A `-t` may sit inside a cluster (`-rt <dir>`, `-vt<dir>`): the rest of the cluster, or when that
/// is empty the next word, is the directory. A flag that takes a value (`install`'s `-m` `-o` `-g`,
/// the backup suffix `-S`) ends the scan first, so its value is never read as the directory.
fn target_directory(args: &[String]) -> Option<String> {
    for (index, arg) in args.iter().enumerate() {
        if let Some(long) = arg.strip_prefix("--") {
            if long == "target-directory" {
                return args.get(index + 1).cloned();
            }
            if let Some(directory) = long.strip_prefix("target-directory=") {
                return Some(directory.to_string());
            }
            continue;
        }
        let Some(cluster) = arg.strip_prefix('-') else {
            continue;
        };
        for (at, flag) in cluster.char_indices() {
            match flag {
                't' => {
                    let rest = &cluster[at + 1..];
                    return if rest.is_empty() {
                        args.get(index + 1).cloned()
                    } else {
                        Some(rest.to_string())
                    };
                }
                'm' | 'o' | 'g' | 'S' => break,
                _ => {}
            }
        }
    }
    None
}

/// Every argument of a `mv`: the destination is written, and each source is removed from where it
/// was, so a move out of a home is a write to it. A re-home under the gate is done with Write.
fn mv_targets(args: &[String]) -> Vec<String> {
    let mut targets = operands(args);
    if let Some(directory) = target_directory(args) {
        if !targets.contains(&directory) {
            targets.push(directory);
        }
    }
    targets
}

/// `git mv` is a `mv`; every other `git` command reads the tree it names.
fn git_mv_targets(args: &[String]) -> Vec<String> {
    let mut index = 0;
    while let Some(arg) = args.get(index) {
        if !arg.starts_with('-') {
            break;
        }
        index += if matches!(arg.as_str(), "-C" | "-c") {
            2
        } else {
            1
        };
    }
    match args.get(index) {
        Some(sub) if sub == "mv" => mv_targets(&args[index + 1..]),
        _ => Vec::new(),
    }
}

/// `find -exec` and `-execdir` run the words up to `;` or `+` as a command of their own.
fn find_exec_targets(args: &[String], depth: usize) -> Vec<String> {
    if depth >= MAX_DEPTH {
        return Vec::new();
    }
    let mut targets = Vec::new();
    let mut index = 0;
    while index < args.len() {
        if matches!(args[index].as_str(), "-exec" | "-execdir") {
            let start = index + 1;
            let end = args[start..]
                .iter()
                .position(|arg| arg == ";" || arg == "+")
                .map_or(args.len(), |at| start + at);
            targets.extend(arm_targets(&args[start..end], depth + 1));
            index = end;
        }
        index += 1;
    }
    targets
}

/// The files an in-place `sed` edits: its operands less the script. Nothing when not in place.
///
/// The script is the value of `-e`/`-f`, or with neither, the first operand. The in-place flag is
/// `-i` or BSD's `-I`, alone or in a cluster (`-Ei`). A bare one takes the next word as its backup
/// suffix when that word is `''` or a `.` with no `/` — a word with a `/` is always a file.
fn sed_files(args: &[String]) -> Vec<String> {
    let mut in_place = false;
    let mut script_given = false;
    let mut files = Vec::new();
    let mut index = 0;
    while let Some(arg) = args.get(index) {
        index += 1;
        if let Some(long) = arg.strip_prefix("--") {
            if long.starts_with("in-place") {
                in_place = true;
            } else if long.starts_with("expression") || long.starts_with("file") {
                script_given = true;
                if !long.contains('=') {
                    index += 1;
                }
            }
            continue;
        }
        let Some(cluster) = arg.strip_prefix('-').filter(|c| !c.is_empty()) else {
            files.push(arg.clone());
            continue;
        };
        for (at, flag) in cluster.char_indices() {
            let glued = at + flag.len_utf8() < cluster.len();
            match flag {
                'e' | 'f' => {
                    script_given = true;
                    if !glued {
                        index += 1;
                    }
                    break;
                }
                'l' => {
                    if !glued {
                        index += 1;
                    }
                    break;
                }
                'i' | 'I' => {
                    in_place = true;
                    if !glued && args.get(index).is_some_and(|next| is_backup_suffix(next)) {
                        index += 1;
                    }
                    break;
                }
                _ => {}
            }
        }
    }
    if !in_place {
        return Vec::new();
    }
    if !script_given && !files.is_empty() {
        files.remove(0);
    }
    files
}

/// A BSD `sed -i` backup suffix: empty, or a `.` and no `/`.
fn is_backup_suffix(word: &str) -> bool {
    word.is_empty() || (word.starts_with('.') && !word.contains('/'))
}

/// The files an in-place `perl` edits: its operands less the program. Nothing when not in place.
///
/// The program is the `-e`/`-E` code, or with neither, the first operand (a program file). A
/// cluster is scanned up to the first switch that takes the rest of it as a value, so `-pi -e` is
/// in place and `-Mstrict -ne` is not.
fn perl_files(args: &[String]) -> Vec<String> {
    let mut in_place = false;
    let mut code_given = false;
    let mut files = Vec::new();
    let mut index = 0;
    while let Some(arg) = args.get(index) {
        index += 1;
        let Some(cluster) = arg
            .strip_prefix('-')
            .filter(|c| !c.is_empty() && !c.starts_with('-'))
        else {
            if !arg.starts_with('-') {
                files.push(arg.clone());
            }
            continue;
        };
        let mut switches = cluster.char_indices().peekable();
        while let Some((at, switch)) = switches.next() {
            let glued = at + switch.len_utf8() < cluster.len();
            match switch {
                'e' | 'E' => {
                    code_given = true;
                    if !glued {
                        index += 1;
                    }
                    break;
                }
                'i' => {
                    in_place = true;
                    break;
                }
                'I' | 'M' | 'm' => {
                    if !glued {
                        index += 1;
                    }
                    break;
                }
                // `-l` and `-0` take only digits, so the scan goes on past them (`-lpi`).
                'l' | '0' => while switches.next_if(|(_, c)| c.is_ascii_digit()).is_some() {},
                'x' | 'C' | 'd' | 'D' => break,
                _ => {}
            }
        }
    }
    if !in_place {
        return Vec::new();
    }
    if !code_given && !files.is_empty() {
        files.remove(0);
    }
    files
}

// ---------------------------------------------------------------------------
// the PowerShell arm
// ---------------------------------------------------------------------------

/// The path arguments of one cmdlet's argument run.
///
/// Named path parameters name their target directly. A parameter known to take a non-path value
/// has that value skipped, so `-Value "<some text>"` cannot be read as a path — the one shape that
/// would otherwise deny an innocent write elsewhere. Every other `-Switch` is treated as taking no
/// value, which costs at most a missed source on `Copy-Item -Force <src> <dst>` and never a false
/// deny.
fn cmdlet_targets(words: &[String], positionals: usize) -> Vec<String> {
    const PATH_PARAMETERS: [&str; 4] = ["path", "filepath", "literalpath", "destination"];
    const VALUE_PARAMETERS: [&str; 7] = [
        "value", "itemtype", "encoding", "filter", "include", "exclude", "name",
    ];
    let mut out = Vec::new();
    let mut taken = 0;
    let mut index = 0;
    while let Some(word) = words.get(index) {
        if let Some(name) = word.strip_prefix('-') {
            let name = name.to_ascii_lowercase();
            let value = words.get(index + 1).filter(|v| !v.starts_with('-'));
            if PATH_PARAMETERS.contains(&name.as_str()) {
                if let Some(value) = value {
                    out.push(value.clone());
                    index += 2;
                    continue;
                }
            } else if VALUE_PARAMETERS.contains(&name.as_str()) && value.is_some() {
                index += 2;
                continue;
            }
            index += 1;
            continue;
        }
        if taken < positionals {
            out.push(word.clone());
            taken += 1;
        }
        index += 1;
    }
    out
}

// ---------------------------------------------------------------------------
// the tokenizer
// ---------------------------------------------------------------------------

/// A simple command: its words with the redirects taken out, and the paths its redirects write.
#[derive(Default)]
struct Simple {
    words: Vec<String>,
    redirected: Vec<String>,
    /// The subshells that open just ahead of this command.
    opens: usize,
    /// The subshells that close just after it.
    closes: usize,
}

/// Split a token stream at its separators, pulling each redirect's operand out of the words. The
/// commands of a process substitution follow the command it sits in, as a subshell of their own.
fn simple_commands(tokens: Vec<Token>) -> Vec<Simple> {
    let mut commands = vec![Simple::default()];
    let mut nested = Vec::new();
    let mut tokens = tokens.into_iter().peekable();
    while let Some(token) = tokens.next() {
        match token {
            Token::Word(word) => push_word(&mut commands, word),
            Token::Separator => end_command(&mut commands, &mut nested, false),
            Token::Open => {
                end_command(&mut commands, &mut nested, false);
                if let Some(next) = commands.last_mut() {
                    next.opens += 1;
                }
            }
            Token::Close => end_command(&mut commands, &mut nested, true),
            Token::Substitution(inner) => {
                let mut group = simple_commands(inner);
                if let Some(first) = group.first_mut() {
                    first.opens += 1;
                }
                if let Some(last) = group.last_mut() {
                    last.closes += 1;
                }
                nested.extend(group);
            }
            redirect => {
                let Some(Token::Word(operand)) =
                    tokens.next_if(|next| matches!(next, Token::Word(_)))
                else {
                    continue;
                };
                let descriptor = operand == "-" || operand.chars().all(|c| c.is_ascii_digit());
                let written = match redirect {
                    Token::Write => true,
                    Token::Duplicate => !descriptor,
                    _ => false,
                };
                if written {
                    if let Some(current) = commands.last_mut() {
                        current.redirected.push(operand);
                    }
                }
            }
        }
    }
    commands.append(&mut nested);
    commands
}

/// End the current command, place the process substitutions it held after it, and start the next.
/// `closes` marks a `)` right after them.
fn end_command(commands: &mut Vec<Simple>, nested: &mut Vec<Simple>, closes: bool) {
    commands.append(nested);
    if closes {
        if let Some(last) = commands.last_mut() {
            last.closes += 1;
        }
    }
    commands.push(Simple::default());
}

fn push_word(commands: &mut [Simple], word: String) {
    if let Some(current) = commands.last_mut() {
        current.words.push(word);
    }
}

/// Split a command into tokens, honouring quotes, escapes and heredocs.
///
/// Quotes are stripped, which is the point: the wave-0 probe's line quoted its target, and a
/// scanner that kept the quotes would not have matched a home. A quoted operator stays a word. An
/// operator is its own token even when glued to a word (`x>file`, `file;`), which is the S12 fix.
/// `parens` is how a POSIX `((` is read; `depth` counts the process substitutions this text sits
/// inside (see [`MAX_DEPTH`]).
fn tokenize(command: &str, dialect: Dialect, parens: DoubleParen, depth: usize) -> Vec<Token> {
    let posix = dialect == Dialect::Posix;
    let chars: Vec<char> = command.chars().collect();
    let mut tokens = Vec::new();
    // `Some("")` is a real word: an empty quoted string, such as the suffix in BSD `sed -i ''`.
    let mut word: Option<String> = None;
    let mut heredocs: Vec<(String, bool)> = Vec::new();
    // Subshell answers to `((` this scan may still look ahead for (see [`arithmetic_close`]).
    let mut lookaheads = MAX_DEPTH;
    let mut i = 0;
    while i < chars.len() {
        let next = chars.get(i + 1).copied();
        match chars[i] {
            '\\' if posix => {
                // An escaped character is literal; an escaped newline continues the line.
                match next {
                    Some('\n') => {}
                    Some(escaped) => word.get_or_insert_with(String::new).push(escaped),
                    None => word.get_or_insert_with(String::new).push('\\'),
                }
                i += 1;
            }
            '\'' => {
                let body = word.get_or_insert_with(String::new);
                i += 1;
                while i < chars.len() && chars[i] != '\'' {
                    body.push(chars[i]);
                    i += 1;
                }
            }
            '"' => {
                let body = word.get_or_insert_with(String::new);
                i += 1;
                while i < chars.len() && chars[i] != '"' {
                    let escaped = chars.get(i + 1).copied();
                    if posix
                        && chars[i] == '\\'
                        && matches!(escaped, Some('"' | '\\' | '$' | '`' | '\n'))
                    {
                        if escaped != Some('\n') {
                            body.extend(escaped);
                        }
                        i += 2;
                        continue;
                    }
                    body.push(chars[i]);
                    i += 1;
                }
            }
            ' ' | '\t' | '\r' => flush(&mut word, &mut tokens),
            // A comment runs to the end of the line; its newline still ends the command.
            '#' if posix && word.is_none() => {
                while i < chars.len() && chars[i] != '\n' {
                    i += 1;
                }
                continue;
            }
            '\n' => {
                flush(&mut word, &mut tokens);
                tokens.push(Token::Separator);
                if posix && !heredocs.is_empty() {
                    i = skip_heredoc_bodies(&chars, i + 1, &mut heredocs);
                    continue;
                }
            }
            ';' => {
                flush(&mut word, &mut tokens);
                tokens.push(Token::Separator);
            }
            // `((…))` and `$((…))` are arithmetic: one word, whose `>` compares and `<<` shifts —
            // unless the `((` is two subshells, which open like any other `(`.
            '(' if posix && next == Some('(') => {
                let close = match parens {
                    DoubleParen::Exact => arithmetic_close(&chars, i, &mut lookaheads),
                    DoubleParen::Arithmetic => Some(closing_paren(&chars, i)),
                };
                match close {
                    Some(close) => {
                        word.get_or_insert_with(String::new)
                            .extend(&chars[i..(close + 1).min(chars.len())]);
                        i = close;
                    }
                    None => {
                        flush(&mut word, &mut tokens);
                        tokens.push(Token::Open);
                    }
                }
            }
            '(' | ')' if posix => {
                flush(&mut word, &mut tokens);
                tokens.push(if chars[i] == '(' {
                    Token::Open
                } else {
                    Token::Close
                });
            }
            '|' => {
                // `|`, `||` and `|&` all end the command.
                flush(&mut word, &mut tokens);
                tokens.push(Token::Separator);
                if matches!(next, Some('|' | '&')) {
                    i += 1;
                }
            }
            '&' => {
                flush(&mut word, &mut tokens);
                if next == Some('>') {
                    // `&>` and `&>>` write both streams to one file.
                    tokens.push(Token::Write);
                    i += if chars.get(i + 2) == Some(&'>') { 2 } else { 1 };
                } else {
                    tokens.push(Token::Separator);
                    if next == Some('&') {
                        i += 1;
                    }
                }
            }
            '>' | '<' if posix && next == Some('(') => {
                flush(&mut word, &mut tokens);
                let close = closing_paren(&chars, i + 1);
                if depth < MAX_DEPTH {
                    let inner: String = chars[i + 2..close.max(i + 2)].iter().collect();
                    tokens.push(Token::Substitution(tokenize(
                        &inner,
                        dialect,
                        parens,
                        depth + 1,
                    )));
                }
                i = close;
            }
            '>' => {
                fold_descriptor(&mut word);
                flush(&mut word, &mut tokens);
                if next == Some('&') {
                    tokens.push(Token::Duplicate);
                    i += 1;
                } else {
                    if next == Some('>') {
                        i += 1;
                    }
                    // The clobber forms: `>|`, and zsh's `>!` and `>>!`.
                    if matches!(chars.get(i + 1), Some('|' | '!')) {
                        i += 1;
                    }
                    tokens.push(Token::Write);
                }
            }
            '<' => {
                fold_descriptor(&mut word);
                flush(&mut word, &mut tokens);
                if next == Some('<') && chars.get(i + 2) == Some(&'<') {
                    tokens.push(Token::Read);
                    i += 2;
                } else if next == Some('<') && posix {
                    i = read_heredoc_delimiter(&chars, i + 2, &mut heredocs);
                    continue;
                } else {
                    tokens.push(Token::Read);
                    if matches!(next, Some('<' | '&')) {
                        i += 1;
                    }
                }
            }
            other => word.get_or_insert_with(String::new).push(other),
        }
        i += 1;
    }
    flush(&mut word, &mut tokens);
    tokens
}

fn flush(word: &mut Option<String>, tokens: &mut Vec<Token>) {
    if let Some(done) = word.take() {
        tokens.push(Token::Word(done));
    }
}

/// Drop a descriptor number glued to a redirect (`2>`), which is not a word of the command.
fn fold_descriptor(word: &mut Option<String>) {
    if word
        .as_deref()
        .is_some_and(|w| !w.is_empty() && w.chars().all(|c| c.is_ascii_digit()))
    {
        *word = None;
    }
}

/// Where the `((` at `open` ends when it is arithmetic, or `None` when it is two subshells.
///
/// Bash and zsh take `((` as arithmetic only when the `)` closing its inner `(` is followed at once
/// by the `)` closing the outer; `((echo a); tee x)` is a subshell inside a subshell (G1 R2). An
/// inner `(` that never closes stays one word, swallowing a line bash refuses anyway.
///
/// Only the inner span is scanned, and the caller consumes an arithmetic answer's span, so only a
/// subshell answer can leave text to be scanned again. `budget` bounds those answers at
/// [`MAX_DEPTH`] per [`tokenize`] call; once it is spent every `((` reads as two subshells with no
/// scan at all. That reading can false-deny an arithmetic comparison, and can miss a write behind a
/// `<<` shift it takes for a heredoc; [`write_targets`] adds the arithmetic reading, which sees
/// that write (G3 B1).
fn arithmetic_close(chars: &[char], open: usize, budget: &mut usize) -> Option<usize> {
    if *budget == 0 {
        return None;
    }
    let inner = closing_paren(chars, open + 1);
    if inner >= chars.len() {
        return Some(chars.len());
    }
    if chars.get(inner + 1) == Some(&')') {
        return Some(inner + 1);
    }
    *budget -= 1;
    None
}

/// The index of the `)` that closes the `(` at `open`, or the end of the text when none does.
/// A parenthesis inside quotes or after a backslash does not count.
fn closing_paren(chars: &[char], open: usize) -> usize {
    let mut depth = 0usize;
    let mut i = open;
    while i < chars.len() {
        match chars[i] {
            '(' => depth += 1,
            ')' => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    return i;
                }
            }
            quote @ ('\'' | '"') => {
                i += 1;
                while i < chars.len() && chars[i] != quote {
                    i += 1;
                }
            }
            '\\' => i += 1,
            _ => {}
        }
        i += 1;
    }
    chars.len()
}

/// Read a heredoc's delimiter after `<<` (or `<<-`, which strips leading tabs from the body) and
/// queue it; the body starts at the next unquoted newline. Returns the index after the delimiter.
fn read_heredoc_delimiter(
    chars: &[char],
    mut i: usize,
    heredocs: &mut Vec<(String, bool)>,
) -> usize {
    let strip_tabs = chars.get(i) == Some(&'-');
    if strip_tabs {
        i += 1;
    }
    while matches!(chars.get(i), Some(' ' | '\t')) {
        i += 1;
    }
    let mut delimiter = String::new();
    while let Some(&c) = chars.get(i) {
        match c {
            '\'' | '"' => {
                i += 1;
                while let Some(&inner) = chars.get(i).filter(|&&inner| inner != c) {
                    delimiter.push(inner);
                    i += 1;
                }
            }
            '\\' => {
                i += 1;
                delimiter.extend(chars.get(i));
            }
            c if c.is_whitespace() || ";|&<>()".contains(c) => break,
            c => delimiter.push(c),
        }
        i += 1;
    }
    if !delimiter.is_empty() {
        heredocs.push((delimiter, strip_tabs));
    }
    i
}

/// Skip the bodies of every queued heredoc, in order, from `start`. A body is data: its quotes and
/// its words never reach the scan, so an apostrophe in it cannot swallow the command after it.
fn skip_heredoc_bodies(chars: &[char], start: usize, heredocs: &mut Vec<(String, bool)>) -> usize {
    let mut i = start;
    for (delimiter, strip_tabs) in heredocs.drain(..) {
        while i < chars.len() {
            let end = chars[i..]
                .iter()
                .position(|&c| c == '\n')
                .map_or(chars.len(), |at| i + at);
            let line: String = chars[i..end].iter().collect();
            i = (end + 1).min(chars.len());
            let candidate = if strip_tabs {
                line.trim_start_matches('\t')
            } else {
                line.as_str()
            };
            if candidate == delimiter {
                break;
            }
        }
    }
    i
}
