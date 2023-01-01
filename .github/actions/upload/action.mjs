import { createReadStream, createWriteStream } from 'node:fs';
import process, { stdout } from 'node:process';
import { Readable, Writable } from 'node:stream';

let artifact = process.env.artifact;
let binary   = process.env.binary;
let location = process.env.location;
let manifest = process.env.manifest;

await copy(location, binary);
await copy(manifest, "manifest.yaml");

await output({
    binary:   binary,
    manifest: "manifest.yaml",
}, process.env.GITHUB_OUTPUT);

async function copy(src, dst) {
    let reader = Readable.toWeb(createReadStream(src));
    let writer = Writable.toWeb(createWriteStream(dst));
    await reader.pipeTo(writer);
}

export async function output(outputs, path) {
    let create = () => createWriteStream(path, { flags: 'a' });
    let stream = Writable.toWeb(path ? create() : stdout);

    let data = Object.entries(outputs).flatMap(([name, value]) => (
        value ? [name, value].join('=') : []
    )).join('\n');

    await stream.getWriter().write(data + '\n');
}
