"use strict";

const assert = require("node:assert/strict");
const { spawn } = require("node:child_process");
const { once } = require("node:events");
const path = require("node:path");
const { test } = require("node:test");
const { setImmediate } = require("node:timers/promises");

const highlight = "textDocument/documentHighlight";
const windows = process.platform === "win32";
const uri = windows ? "file:///C:/Work/Démo/(test)/main.ts" : "file:///Work/Démo/(test)/%6Dain.ts";
const normalized = `file:///${windows ? "c%3A/" : ""}Work/D%C3%A9mo/%28test%29/main.ts`;

function encode(fields, extraHeader = "") {
    const body = JSON.stringify({ jsonrpc: "2.0", ...fields });
    return Buffer.from(`Content-Length: ${Buffer.byteLength(body)}\r\n${extraHeader}\r\n${body}`);
}

async function* messages(stream) {
    let buffer = Buffer.alloc(0);
    for await (const chunk of stream) {
        buffer = Buffer.concat([buffer, chunk]);
        for (;;) {
            const headerEnd = buffer.indexOf("\r\n\r\n");
            if (headerEnd < 0) {
                break;
            }
            const length = Number(/^Content-Length: (\d+)/.exec(buffer.toString("ascii", 0, headerEnd))[1]);
            const end = headerEnd + 4 + length;
            if (end > buffer.length) {
                break;
            }
            yield {
                bytes: buffer.subarray(0, end),
                message: JSON.parse(buffer.toString("utf8", headerEnd + 4, end)),
            };
            buffer = buffer.subarray(end);
        }
    }
    assert.equal(buffer.length, 0);
}

test("normalizes only the file URI on standard highlight requests", { timeout: 5000 }, async (t) => {
    const proxy = path.join(__dirname, "../src/launch/highlight_proxy.cjs");
    const child = spawn(process.execPath, [proxy, process.execPath, "-e", "process.stdin.pipe(process.stdout)"], {
        stdio: ["pipe", "pipe", "inherit"],
    });
    t.after(() => child.kill());

    const results = messages(child.stdout);
    async function read() {
        const message = await results.next();
        assert.equal(message.done, false);
        return message.value;
    }

    const params = { textDocument: { uri }, position: { line: 1, character: 2 } };
    const request = encode({ id: "first", method: highlight, params });
    const utf8Offset = request.indexOf(Buffer.from("é"));
    child.stdin.write(request.subarray(0, 5));
    await setImmediate();
    child.stdin.write(request.subarray(5, utf8Offset + 1));
    await setImmediate();
    child.stdin.write(request.subarray(utf8Offset + 1));

    assert.deepEqual((await read()).message, {
        jsonrpc: "2.0",
        id: "first",
        method: highlight,
        params: { ...params, textDocument: { uri: normalized } },
    });

    // Two frames can arrive in one chunk. The hover frame must survive byte for byte.
    const hover = encode(
        { id: 2, method: "textDocument/hover", params },
        "Content-Type: application/vscode-jsonrpc; charset=utf-8\r\n",
    );
    child.stdin.write(Buffer.concat([hover, request]));
    assert.deepEqual((await read()).bytes, hover);
    assert.equal((await read()).message.params.textDocument.uri, normalized);

    const cases = [
        ["file:///Z%3a/Work/main.ts", "file:///z%3A/Work/main.ts"],
        ["file://server/share/Démo/!%25.ts", "file://server/share/D%C3%A9mo/%21%25.ts"],
        ["file:////server/share/Démo/!%25.ts", "file://server/share/D%C3%A9mo/%21%25.ts"],
        ["file:////some%20dir/main.ts", "file:////some%20dir/main.ts"],
        ["file:///path/main%2Finner.ts", "file:///path/main%2Finner.ts"],
        ["file:///path/main%.ts", "file:///path/main%.ts"],
        ["untitled:Untitled-1", "untitled:Untitled-1"],
        [normalized, normalized],
    ];
    for (const [input, expected] of cases) {
        const inputParams = { ...params, textDocument: { uri: input } };
        child.stdin.write(encode({ id: 3, method: highlight, params: inputParams }));
        assert.deepEqual((await read()).message, {
            jsonrpc: "2.0",
            id: 3,
            method: highlight,
            params: { ...inputParams, textDocument: { uri: expected } },
        });
    }

    const exit = once(child, "exit");
    child.stdin.end();
    assert.deepEqual(await exit, [0, null]);
});
