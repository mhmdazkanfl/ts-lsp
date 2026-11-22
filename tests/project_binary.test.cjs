const assert = require('node:assert/strict');
const { execFileSync } = require('node:child_process');
const fs = require('node:fs');
const os = require('node:os');
const path = require('node:path');
const test = require('node:test');

const probe = fs.readFileSync(path.join(__dirname, '../src/project_binary.cjs'), 'utf8');
const platformPackage = '@typescript/typescript-test-platform';
const binaryName = process.platform === 'win32' ? 'tsc.exe' : 'tsc';

function temporaryProject(t) {
    const root = fs.mkdtempSync(path.join(os.tmpdir(), 'ts-lsp test '));
    t.after(() => fs.rmSync(root, { recursive: true, force: true }));
    return fs.realpathSync(root);
}

function writePackage(directory, manifest) {
    fs.mkdirSync(directory, { recursive: true });
    fs.writeFileSync(path.join(directory, 'package.json'), JSON.stringify(manifest));
}

function install(root, version, nativeVersion = version) {
    const modules = path.join(root, 'node_modules');
    writePackage(path.join(modules, 'typescript'), {
        name: 'typescript', version, optionalDependencies: { [platformPackage]: version },
    });
    const native = path.join(modules, platformPackage);
    writePackage(native, { name: platformPackage, version: nativeVersion });
    fs.mkdirSync(path.join(native, 'lib'));
    const binary = path.join(native, 'lib', binaryName);
    fs.writeFileSync(binary, '', { mode: 0o755 });
    return binary;
}

function resolve(root, pin = '') {
    return execFileSync(process.execPath, [
        '--no-global-search-paths', '--eval', probe, '--', root, platformPackage, binaryName, pin,
    ], { encoding: 'utf8' });
}

test('uses an ancestor workspace install, including when the worktree is a single file', (t) => {
    const root = temporaryProject(t);
    const binary = install(root, '7.0.2');
    const child = path.join(root, 'packages', 'consumer');
    const file = path.join(child, 'main.ts');
    fs.mkdirSync(child, { recursive: true });
    fs.writeFileSync(file, '');
    assert.equal(resolve(child), binary);
    assert.equal(resolve(file), binary);
});

test('uses the selected TypeScript package’s dependency through an isolated layout', (t) => {
    const root = temporaryProject(t);
    const isolated = path.join(root, 'node_modules', '.pnpm', 'typescript@7.0.2');
    const binary = install(isolated, '7.0.2');
    fs.symlinkSync(path.join(isolated, 'node_modules', 'typescript'), path.join(root, 'node_modules', 'typescript'), 'junction');
    assert.equal(resolve(root), binary);
});

test('can switch pins without reusing an incompatible project binary', (t) => {
    const root = temporaryProject(t);
    const binary = install(root, '7.0.2');
    assert.equal(resolve(root, '7.0.2'), binary);
    assert.equal(resolve(root, '7.0.1'), '');
    assert.equal(resolve(root, '7.0.2'), binary);
});

test('falls back on a legacy version, mismatched platform package, or missing executable', (t) => {
    const legacy = temporaryProject(t);
    install(legacy, '6.0.3');
    assert.equal(resolve(legacy), '');

    const mismatch = temporaryProject(t);
    install(mismatch, '7.0.2', '7.0.1');
    assert.equal(resolve(mismatch), '');

    const missing = temporaryProject(t);
    fs.unlinkSync(install(missing, '7.0.2'));
    assert.equal(resolve(missing), '');
});
