// src/core/aliases/categories/prisma.rs

use crate::core::aliases::{AliasEntry, AliasGroup};

pub fn npx_group() -> AliasGroup {
    AliasGroup {
        name: "Prisma ORM (npx)".to_string(),
        only_shells: vec![],
        aliases: vec![
            AliasEntry {
                key: "np".to_string(),
                value: "npx prisma".to_string(),
                description: "npx prisma CLI".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "npi".to_string(),
                value: "npx prisma init".to_string(),
                description: "Initialize Prisma project".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "npg".to_string(),
                value: "npx prisma generate".to_string(),
                description: "Generate Prisma Client".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "nps".to_string(),
                value: "npx prisma studio".to_string(),
                description: "Open Prisma Studio GUI".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "npmd".to_string(),
                value: "npx prisma migrate dev".to_string(),
                description: "Run Prisma migrate dev".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "npmdn".to_string(),
                value: "npx prisma migrate dev --name".to_string(),
                description: "Run Prisma migrate dev with custom migration name".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "npmr".to_string(),
                value: "npx prisma migrate reset".to_string(),
                description: "Reset database and run all migrations".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "npmdp".to_string(),
                value: "npx prisma migrate deploy".to_string(),
                description: "Apply pending migrations to database".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "npms".to_string(),
                value: "npx prisma migrate status".to_string(),
                description: "Check status of database migrations".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "npdp".to_string(),
                value: "npx prisma db push".to_string(),
                description: "Push Prisma schema state directly to DB".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "npdl".to_string(),
                value: "npx prisma db pull".to_string(),
                description: "Pull database schema into Prisma schema".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "npds".to_string(),
                value: "npx prisma db seed".to_string(),
                description: "Seed database using Prisma seed script".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "npf".to_string(),
                value: "npx prisma format".to_string(),
                description: "Format schema.prisma file".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "npv".to_string(),
                value: "npx prisma version".to_string(),
                description: "Display Prisma CLI & engine versions".to_string(),
                only_shells: vec![],
            },
        ],
    }
}

pub fn bunx_group() -> AliasGroup {
    AliasGroup {
        name: "Prisma ORM (bunx)".to_string(),
        only_shells: vec![],
        aliases: vec![
            AliasEntry {
                key: "bp".to_string(),
                value: "bunx prisma".to_string(),
                description: "bunx prisma CLI".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "bpi".to_string(),
                value: "bunx prisma init".to_string(),
                description: "Initialize Prisma project via Bun".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "bpg".to_string(),
                value: "bunx prisma generate".to_string(),
                description: "Generate Prisma Client via Bun".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "bps".to_string(),
                value: "bunx prisma studio".to_string(),
                description: "Open Prisma Studio GUI via Bun".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "bpmd".to_string(),
                value: "bunx prisma migrate dev".to_string(),
                description: "Run Prisma migrate dev via Bun".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "bpmdn".to_string(),
                value: "bunx prisma migrate dev --name".to_string(),
                description: "Run Prisma migrate dev with custom name via Bun".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "bpmr".to_string(),
                value: "bunx prisma migrate reset".to_string(),
                description: "Reset database and run all migrations via Bun".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "bpmdp".to_string(),
                value: "bunx prisma migrate deploy".to_string(),
                description: "Apply pending migrations to database via Bun".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "bpms".to_string(),
                value: "bunx prisma migrate status".to_string(),
                description: "Check status of database migrations via Bun".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "bpdp".to_string(),
                value: "bunx prisma db push".to_string(),
                description: "Push Prisma schema state directly to DB via Bun".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "bpdl".to_string(),
                value: "bunx prisma db pull".to_string(),
                description: "Pull database schema into Prisma schema via Bun".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "bpds".to_string(),
                value: "bunx prisma db seed".to_string(),
                description: "Seed database using Prisma seed script via Bun".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "bpf".to_string(),
                value: "bunx prisma format".to_string(),
                description: "Format schema.prisma file via Bun".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "bpv".to_string(),
                value: "bunx prisma version".to_string(),
                description: "Display Prisma CLI & engine versions via Bun".to_string(),
                only_shells: vec![],
            },
        ],
    }
}
