// src/core/aliases/categories/prisma.rs

use crate::core::aliases::{AliasEntry, AliasGroup};

pub fn npx_group() -> AliasGroup {
    AliasGroup {
        name: "Prisma (npx)".to_string(),
        only_shells: vec![],
        aliases: vec![
            AliasEntry {
                key: "np".to_string(),
                value: "npx prisma".to_string(),
                description: "npx prisma CLI".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "npg".to_string(),
                value: "npx prisma generate".to_string(),
                description: "Generate Prisma client".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "nps".to_string(),
                value: "npx prisma studio".to_string(),
                description: "Open Prisma Studio".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "npmd".to_string(),
                value: "npx prisma migrate dev".to_string(),
                description: "Run development migration".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "npdp".to_string(),
                value: "npx prisma db push".to_string(),
                description: "Push schema to DB without migration".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "npf".to_string(),
                value: "npx prisma format".to_string(),
                description: "Format schema.prisma".to_string(),
                only_shells: vec![],
            },
        ],
    }
}

pub fn bunx_group() -> AliasGroup {
    AliasGroup {
        name: "Prisma (bunx)".to_string(),
        only_shells: vec![],
        aliases: vec![
            AliasEntry {
                key: "bp".to_string(),
                value: "bunx prisma".to_string(),
                description: "bunx prisma CLI".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "bpg".to_string(),
                value: "bunx prisma generate".to_string(),
                description: "Generate Prisma client via Bun".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "bps".to_string(),
                value: "bunx prisma studio".to_string(),
                description: "Open Prisma Studio via Bun".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "bpmd".to_string(),
                value: "bunx prisma migrate dev".to_string(),
                description: "Run development migration via Bun".to_string(),
                only_shells: vec![],
            },
        ],
    }
}
