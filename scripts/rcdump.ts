import { cp, rm, symlink } from 'node:fs/promises';
import { basename } from 'node:path';

const rust_dump = '/disk2/dump-files/rust-target-dump/';
await rm(`${rust_dump}${basename(process.cwd())}`, { recursive: true, force: true });
await cp('./target', `${rust_dump}${basename(process.cwd())}`, { recursive: true });
await rm('./target', { recursive: true, force: true });
await symlink(`${rust_dump}${basename(process.cwd())}`, './target');
