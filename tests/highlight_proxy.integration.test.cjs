"use strict";

const assert = require("node:assert/strict");
const { spawn } = require("node:child_process");
const { once } = require("node:events");
const { mkdtempSync, readFileSync, rmSync } = require("node:fs");
const { tmpdir } = require("node:os");
const { join } = require("node:path");
const { buffer } = require("node:stream/consumers");
const { test } = require("node:test");

const proxySource = readFileSync(require.resolve("../src/launch/highlight_proxy.cjs"), "utf8") + "\nrun();";
const processTest = { timeout: 5000 };

function frame(body, extraHeaders = []) {
    return ["Content-Length: " + Buffer.byteLength(body), ...extraHeaders].join("\r\n") + "\r\n\r\n" + body;
}

function launchProxy(t, binary, args = []) {
    const sut = spawn(process.execPath, [
        "--input-type=commonjs", "--eval", proxySource, "--", binary, ...args,
    ], { stdio: ["pipe", "pipe", "pipe"] });
    t.after(() => sut.kill());
    const finished = Promise.all([once(sut, "close"), buffer(sut.stdout), buffer(sut.stderr)]);
    return { sut, finished };
}

// This runs in the server process. It replies only after seeing a valid request,
// then checks at EOF that it received exactly one request.
function replyToOneRequest() {
    const assert = require("node:assert/strict");
    const [expectedRequest, response] = process.argv.slice(1);
    let received = Buffer.alloc(0);
    let frameEnd;
    process.stdin.on("data", (chunk) => {
        received = Buffer.concat([received, chunk]);
        if (frameEnd !== undefined) {
            assert.equal(received.length, frameEnd);
            return;
        }
        const headerEnd = received.indexOf("\r\n\r\n");
        if (headerEnd === -1) {
            return;
        }
        const match = /^Content-Length: *(\d+)\r?$/im.exec(received.toString("ascii", 0, headerEnd));
        assert.ok(match);
        const bodyStart = headerEnd + 4;
        const bodyEnd = bodyStart + Number(match[1]);
        if (received.length < bodyEnd) {
            return;
        }
        frameEnd = bodyEnd;
        assert.equal(received.length, frameEnd);
        assert.deepEqual(JSON.parse(received.toString("utf8", bodyStart)), JSON.parse(expectedRequest));
        process.stdout.write(response);
    });
    process.stdin.on("end", () => assert.equal(received.length, frameEnd));
}

test("Zed receives the server's exact reply before closing its input", processTest, async (t) => {
    const request = '{"jsonrpc":"2.0","id":7,"method":"textDocument/documentHighlight","params":{"textDocument":{"uri":"file:///work/(demo).ts"},"position":{"line":0,"character":6}}}';
    const expected = '{"jsonrpc":"2.0","id":7,"method":"textDocument/documentHighlight","params":{"textDocument":{"uri":"file:///work/%28demo%29.ts"},"position":{"line":0,"character":6}}}';
    const response = frame('{ "jsonrpc": "2.0", "id": 7, "result": [] }', ["Content-Type: application/vscode-jsonrpc; charset=utf-8"]);
    const { sut, finished } = launchProxy(t, process.execPath, ["-e", "(" + replyToOneRequest + ")()", expected, response]);
    sut.stdout.once("data", () => sut.stdin.end());

    sut.stdin.write(frame(request));
    const [status, output, stderr] = await finished;

    assert.deepEqual(status, [0, null], stderr.toString());
    assert.deepEqual(output, Buffer.from(response));
});

test("Zed receives the server's exit status with its input still open", processTest, async (t) => {
    const { finished } = launchProxy(t, process.execPath, ["-e", "process.exitCode = 23;"]);

    const [status, output, stderr] = await finished;

    assert.deepEqual(status, [23, null], stderr.toString());
    assert.deepEqual(output, Buffer.alloc(0));
});

test("Zed receives the final output and status after the server closes its input", processTest, async (t) => {
    const ready = frame('{"jsonrpc":"2.0","method":"server/closing"}');
    const response = frame('{"jsonrpc":"2.0","id":7,"result":[]}');
    const server = [
        'require("node:fs").closeSync(0);',
        'process.stdout.write(process.argv[1]);',
        // Give the editor time to write to the closed pipe before the server finishes.
        'setTimeout(() => { process.stdout.write(process.argv[2]); process.exitCode = 23; }, 200);',
    ].join("\n");
    const { sut, finished } = launchProxy(t, process.execPath, ["-e", server, ready, response]);
    sut.stdout.once("data", () => {
        sut.stdin.write(frame('{"jsonrpc":"2.0","method":"initialized","params":{}}'));
    });

    const [status, output, stderr] = await finished;

    assert.deepEqual(status, [23, null], stderr.toString());
    assert.deepEqual(output, Buffer.from(ready + response));
});

test("an invalid request stops the session without forwarding the bad message", processTest, async (t) => {
    const { sut, finished } = launchProxy(t, process.execPath, ["-e", "process.stdin.pipe(process.stdout);"]);

    sut.stdin.write(frame("{invalid JSON}"));
    const [status, output, stderr] = await finished;

    assert.deepEqual(status, [1, null], stderr.toString());
    assert.deepEqual(output, Buffer.alloc(0));
});

test("Zed sees a failed start when the TypeScript executable is absent", processTest, async (t) => {
    const directory = mkdtempSync(join(tmpdir(), "highlight-proxy-"));
    t.after(() => rmSync(directory, { recursive: true, force: true }));
    const { finished } = launchProxy(t, join(directory, "missing-server"));

    const [status, output, stderr] = await finished;

    assert.deepEqual(status, [1, null], stderr.toString());
    assert.deepEqual(output, Buffer.alloc(0));
});
