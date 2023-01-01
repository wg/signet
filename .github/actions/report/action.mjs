import { createWriteStream } from 'node:fs';
import { readFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import process, { stdout } from 'node:process';
import { Writable } from 'node:stream';
import { fileURLToPath } from 'node:url';

let output = process.env.GITHUB_STEP_SUMMARY;
let create = () => createWriteStream(output, { flags: 'a' });
let stream = Writable.toWeb(output ? create() : stdout);

let variables = {
    artifact: process.env.artifact,
    digest:   process.env.digest,
    source:   process.env.source,
    commit:   process.env.commit,
    manifest: await readFile(process.env.manifest),
};

let location = dirname(fileURLToPath(import.meta.url));
let buffer   = await readFile(join(location, 'report.md'));
let template = buffer.toString();

let report = template.replace(/\$(\w+)/g, (_, name) => {
    return variables[name];
});

await stream.getWriter().write(report);
