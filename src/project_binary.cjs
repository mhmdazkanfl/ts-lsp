// Resolve the native package from TypeScript itself so pnpm's isolated dependencies work.
// Only read package metadata here. The project launcher and package modules are never loaded.
const fs = require('node:fs');
const path = require('node:path');
const { createRequire } = require('node:module');
const [root, platformPackage, binaryName, pinnedVersion] = process.argv.slice(1);

function readManifest(file) {
    return JSON.parse(fs.readFileSync(file, 'utf8'));
}

function projectBinary() {
    const directory = fs.statSync(root).isDirectory() ? root : path.dirname(root);
    const resolveFromProject = createRequire(path.resolve(directory, 'package.json'));
    const manifest = resolveFromProject.resolve('typescript/package.json');
    const { name, version, optionalDependencies } = readManifest(manifest);
    if (name !== 'typescript' || typeof version !== 'string' || !/^\d+\.\d+\.\d+/.test(version) || Number(version.split('.')[0]) < 7
        || (pinnedVersion && version !== pinnedVersion) || optionalDependencies?.[platformPackage] !== version) {
        return;
    }

    const resolveFromTypeScript = createRequire(manifest);
    const platformManifest = resolveFromTypeScript.resolve(`${platformPackage}/package.json`);
    const nativePackage = readManifest(platformManifest);
    if (nativePackage.name !== platformPackage || nativePackage.version !== version) {
        return;
    }
    const binary = path.join(path.dirname(platformManifest), 'lib', binaryName);
    if (fs.statSync(binary).isFile()) {
        fs.accessSync(binary, fs.constants.X_OK);
        return binary;
    }
}

try {
    const binary = projectBinary();
    if (binary) process.stdout.write(binary);
} catch (error) {
    if (!['MODULE_NOT_FOUND', 'ERR_PACKAGE_PATH_NOT_EXPORTED', 'ENOENT', 'ENOTDIR', 'EACCES'].includes(error.code)) {
        throw error;
    }
}
