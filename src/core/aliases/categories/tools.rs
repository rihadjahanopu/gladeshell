// src/core/aliases/categories/tools.rs

use crate::core::aliases::{AliasEntry, AliasGroup};

pub fn group() -> AliasGroup {
    AliasGroup {
        name: "fancybash Native Tools".to_string(),
        only_shells: vec![],
        aliases: vec![
            AliasEntry { key: "ffmedia".into(), value: "fancybash ffmedia".into(), description: "Interactive 24-in-1 FFmpeg multimedia suite".into(), only_shells: vec![] },
            AliasEntry { key: "ut".into(),      value: "fancybash ut".into(),      description: "PC Arsenal - interactive CLI tool installer & optimizer".into(), only_shells: vec![] },
            AliasEntry { key: "uu".into(),      value: "fancybash uu".into(),      description: "Interactive universal app uninstaller".into(), only_shells: vec![] },
            AliasEntry { key: "uup".into(),     value: "fancybash uup".into(),     description: "Mega system updater with interactive menu".into(), only_shells: vec![] },
            AliasEntry { key: "fkill".into(),   value: "fancybash fkill".into(),   description: "Interactive process killer".into(), only_shells: vec![] },
            AliasEntry { key: "gwip".into(),    value: "fancybash gwip".into(),    description: "Interactive Git stage, commit & push".into(), only_shells: vec![] },
            AliasEntry { key: "dman".into(),    value: "fancybash dman".into(),    description: "Interactive Docker TUI Manager".into(), only_shells: vec![] },
            AliasEntry { key: "gbranch".into(), value: "fancybash gbranch".into(), description: "Interactive Git branch manager".into(), only_shells: vec![] },
            AliasEntry { key: "fh".into(),      value: "fancybash fh".into(),      description: "Interactive fuzzy history search".into(), only_shells: vec![] },
            AliasEntry { key: "todo".into(),    value: "fancybash todo".into(),    description: "Interactive 3-tier task manager".into(), only_shells: vec![] },
            AliasEntry { key: "notes".into(),   value: "fancybash notes".into(),   description: "Plain-text markdown notes manager".into(), only_shells: vec![] },
            AliasEntry { key: "vault".into(),   value: "fancybash vault".into(),   description: "Hardened AES-256 Multi-Vault Manager".into(), only_shells: vec![] },
            AliasEntry { key: "ex".into(),      value: "fancybash ex".into(),      description: "Universal archive extractor".into(), only_shells: vec![] },
            AliasEntry { key: "makecpp".into(), value: "fancybash makecpp".into(), description: "C++ project boilerplate generator".into(), only_shells: vec![] },
            AliasEntry { key: "run".into(),     value: "fancybash run".into(),     description: "Interactive Bun JS/TS file runner".into(), only_shells: vec![] },
            AliasEntry { key: "v".into(),       value: "fancybash v".into(),       description: "Interactive video search & player".into(), only_shells: vec![] },
            AliasEntry { key: "uc".into(),      value: "fancybash uc".into(),      description: "Universal system cleaner & optimizer".into(), only_shells: vec![] },
            AliasEntry { key: "rt".into(),      value: "fancybash rt".into(),      description: "Interactive JS runtime & NVM installer".into(), only_shells: vec![] },
            AliasEntry { key: "rn".into(),      value: "fancybash rn".into(),      description: "Smart batch file renamer".into(), only_shells: vec![] },
            AliasEntry { key: "pg".into(),      value: "fancybash pg".into(),      description: "Universal package converter".into(), only_shells: vec![] },
            AliasEntry { key: "drive".into(),   value: "fancybash drive".into(),   description: "Smart external media drive jumper".into(), only_shells: vec![] },
            // cf is a shell function (not alias) — defined in render_cf_wrapper() so it can `cd`
            AliasEntry { key: "kp".into(),      value: "fancybash kp".into(),      description: "Kill process on port".into(), only_shells: vec![] },
            AliasEntry { key: "project".into(), value: "fancybash project".into(), description: "Interactive Project Setup & Tool Hub TUI".into(), only_shells: vec![] },
            AliasEntry { key: "ii".into(),      value: "fancybash ii".into(),      description: "Interactive project setup".into(), only_shells: vec![] },
            AliasEntry { key: "vite".into(),    value: "fancybash vite".into(),    description: "Interactive Vite project generator with Tailwind v4".into(), only_shells: vec![] },
            AliasEntry { key: "next".into(),    value: "fancybash next".into(),    description: "Interactive Next.js project generator".into(), only_shells: vec![] },
            AliasEntry { key: "ui".into(),      value: "fancybash ui".into(),      description: "Interactive Shadcn UI setup & path patcher".into(), only_shells: vec![] },
            AliasEntry { key: "css".into(),     value: "fancybash css".into(),     description: "Tailwind CSS v4 auto-installer".into(), only_shells: vec![] },
            AliasEntry { key: "html".into(),    value: "fancybash html".into(),    description: "Serve / run index.html with Bun or browser".into(), only_shells: vec![] },
            AliasEntry { key: "gen".into(),     value: "fancybash gen".into(),     description: "Cryptographically-secure secret key generator".into(), only_shells: vec![] },
            AliasEntry { key: "sysmon".into(),  value: "fancybash sysmon".into(),  description: "Interactive system performance monitor".into(), only_shells: vec![] },
            AliasEntry { key: "t".into(),       value: "fancybash t".into(),       description: "Smart file creation helper".into(), only_shells: vec![] },
            AliasEntry { key: "fancy".into(),   value: "fancybash theme".into(),   description: "Interactive TUI theme picker & switcher".into(), only_shells: vec![] },
            AliasEntry { key: "theme".into(),   value: "fancybash theme".into(),   description: "Interactive TUI theme picker & switcher".into(), only_shells: vec![] },
        ],
    }
}
