"use strict";

const assert = require("node:assert/strict");
const { spawn } = require("node:child_process");
const { once } = require("node:events");
const path = require("node:path");
const { buffer } = require("node:stream/consumers");
const { test } = require("node:test");
const { rewriteRequests } = require("../src/launch/highlight_proxy.cjs");

const proxy = path.join(__dirname, "../src/launch/highlight_proxy.cjs");
const contentType = "Content-Type: application/vscode-jsonrpc; charset=utf-8\r\n";
const uri = "file:///Work/Démo/(test)/%6Dain.ts";
const normalizedUri = "file:///Work/D%C3%A9mo/%28test%29/main.ts";
const requestId = "request-α";

function frame(message, extraHeader = "") {
    const body = JSON.stringify(message);
    return Buffer.from(`Content-Length: ${Buffer.byteLength(body)}\r\n${extraHeader}\r\n${body}`);
}

function request(uri, method = "textDocument/documentHighlight") {
    return {
        jsonrpc: "2.0",
        id: requestId,
        method,
        params: {
            textDocument: { uri },
            position: { line: 1, character: 2 },
        },
    };
}

function readFrame(bytes) {
    const headerEnd = bytes.indexOf("\r\n\r\n");
    assert.notEqual(headerEnd, -1);
    const body = bytes.subarray(headerEnd + 4);
    const length = /^Content-Length:[ \t]*(\d+)[ \t]*\r?$/im.exec(bytes.toString("ascii", 0, headerEnd));
    assert.equal(Number(length?.[1]), body.length);
    return JSON.parse(body.toString("utf8"));
}

const rewrittenUris = [
    ["a local path with Unicode, punctuation and unnecessary escaping", uri, normalizedUri],
    ["a drive letter with a lowercase escape", "file:///Z%3a/Work/main.ts", "file:///z%3A/Work/main.ts"],
    ["a UNC path with an authority", "file://server/share/Démo/!%25.ts", "file://server/share/D%C3%A9mo/%21%25.ts"],
    ["a UNC path with an empty authority", "file:////server/share/Démo/!%25.ts", "file://server/share/D%C3%A9mo/%21%25.ts"],
];

for (const [name, input, expected] of rewrittenUris) {
    test(`a highlight request normalizes ${name}`, async () => {
        const original = frame(request(input));

        const output = await buffer(rewriteRequests([original]));

        assert.deepEqual(readFrame(output), request(expected));
    });
}

const unchangedUris = [
    ["an ambiguous file path", "file:////some%20dir/main.ts"],
    ["an encoded path separator", "file:///path/(main)%2finner.ts"],
    ["an encoded NUL", "file:///path/(main)%00.ts"],
    ["an invalid percent escape", "file:///path/main%.ts"],
    ["a URI with a query", "file:///path/(main).ts?revision=1"],
    ["a non-file URI", "untitled:Untitled-1"],
    ["an already normalized URI", normalizedUri],
];

for (const [name, input] of unchangedUris) {
    test(`a highlight request preserves the original frame for ${name}`, async () => {
        const original = frame(request(input), contentType);

        const output = await buffer(rewriteRequests([original]));

        assert.deepEqual(output, original);
    });
}

test("a highlight request can span an LSP header and a multibyte character across chunks", async () => {
    const original = frame(request(uri));
    const splitCharacter = original.indexOf(Buffer.from("é")) + 1;
    const chunks = [
        original.subarray(0, 5),
        original.subarray(5, splitCharacter),
        original.subarray(splitCharacter),
    ];

    const output = await buffer(rewriteRequests(chunks));

    assert.deepEqual(readFrame(output), request(normalizedUri));
});

test("a hover request keeps its original bytes when it shares a chunk with a highlight request", async () => {
    const hover = frame(request(uri, "textDocument/hover"), contentType);
    const highlight = frame(request(uri));
    const chunk = Buffer.concat([hover, highlight]);

    const output = await buffer(rewriteRequests([chunk]));

    assert.deepEqual(output.subarray(0, hover.length), hover);
    assert.deepEqual(readFrame(output.subarray(hover.length)), request(normalizedUri));
});

test("a truncated request fails when the input closes", async () => {
    const truncated = frame(request(uri)).subarray(0, -1);

    const output = buffer(rewriteRequests([truncated]));

    await assert.rejects(output, /Incomplete LSP message/);
});

test("the stdio proxy sends a normalized request and delivers the server response verbatim", { timeout: 5000 }, async (t) => {
    const response = frame({ jsonrpc: "2.0", id: requestId, result: [] }, contentType);
    // Use the child's stderr to observe what reached its stdin.
    const server = `
        process.stdin.pipe(process.stderr, { end: false });
        process.stdin.on("end", () => process.stdout.write(process.argv[1]));
    `;
    const child = spawn(process.execPath, [
        proxy, process.execPath, "-e", server, response.toString(),
    ], { stdio: ["pipe", "pipe", "pipe"] });
    t.after(() => child.kill());
    const result = Promise.all([buffer(child.stderr), buffer(child.stdout), once(child, "close")]);

    child.stdin.end(frame(request(uri)));

    const [forwardedRequest, output, status] = await result;
    assert.deepEqual(status, [0, null]);
    assert.deepEqual(readFrame(forwardedRequest), request(normalizedUri));
    assert.deepEqual(output, response);
});
