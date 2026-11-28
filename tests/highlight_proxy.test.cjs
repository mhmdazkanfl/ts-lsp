"use strict";

const assert = require("node:assert/strict");
const { buffer } = require("node:stream/consumers");
const { test } = require("node:test");
const { rewriteRequests } = require("../src/launch/highlight_proxy.cjs");

const highlightMethod = "textDocument/documentHighlight";
const inputUri = "file:///Work/Démo/(test)/%6Dain.ts";
const normalizedUri = "file:///Work/D%C3%A9mo/%28test%29/main.ts";
const contentType = "Content-Type: application/vscode-jsonrpc; charset=utf-8";

function highlightRequest(uri = inputUri) {
    return {
        jsonrpc: "2.0",
        id: "request-α",
        method: highlightMethod,
        params: {
            textDocument: { uri },
            position: { line: 1, character: 2 },
        },
    };
}

function frameBody(body, extraHeaders = []) {
    const headers = ["Content-Length: " + Buffer.byteLength(body), ...extraHeaders];
    return Buffer.from(headers.join("\r\n") + "\r\n\r\n" + body);
}

function frame(message) {
    return frameBody(JSON.stringify(message));
}

function readMessages(output) {
    const messages = [];
    let start = 0;
    while (start < output.length) {
        const headerEnd = output.indexOf("\r\n\r\n", start);
        assert.notEqual(headerEnd, -1, "each output message must include a complete header");
        const headerLines = output.toString("ascii", start, headerEnd).split("\r\n");
        const lengths = headerLines.filter((line) => line.toLowerCase().startsWith("content-length:"));
        assert.equal(lengths.length, 1, "each output message must include one length");
        const length = Number(lengths[0].split(":")[1].trim());
        const bodyStart = headerEnd + 4;
        const bodyEnd = bodyStart + length;
        assert.ok(Number.isSafeInteger(length) && length > 0 && bodyEnd <= output.length);
        messages.push(JSON.parse(output.toString("utf8", bodyStart, bodyEnd)));
        start = bodyEnd;
    }
    return messages;
}

const rewrittenAddresses = [
    ["Unicode, parentheses, and unnecessary escaping", inputUri, normalizedUri],
    ["punctuation and a literal percent sign", "file:///Work/!'()*.%25.ts", "file:///Work/%21%27%28%29%2A.%25.ts"],
    ["an uppercase Windows drive letter", "file:///Z:/Work/main.ts", "file:///z%3A/Work/main.ts"],
    ["a lowercase escape on a Windows drive", "file:///z%3a/Work/main.ts", "file:///z%3A/Work/main.ts"],
    ["a network share with a host", "file://server/share/Démo/!%25.ts", "file://server/share/D%C3%A9mo/%21%25.ts"],
    ["a network share with the host in the path", "file:////server/share/Démo/!%25.ts", "file://server/share/D%C3%A9mo/%21%25.ts"],
    ["a backslash in a Linux filename", "file:///Work/a\\b.ts", "file:///Work/a%5Cb.ts"],
    ["an encoded backslash in a Linux filename", "file:///Work/a%5cb.ts", "file:///Work/a%5Cb.ts"],
];

for (const [name, originalUri, expectedUri] of rewrittenAddresses) {
    test("a highlight request uses TypeScript's address for " + name, async () => {
        const input = frame(highlightRequest(originalUri));

        const output = await buffer(rewriteRequests([input], "linux"));

        assert.deepEqual(readMessages(output), [highlightRequest(expectedUri)]);
    });
}

const preservedAddresses = [
    ["an already normalized address", normalizedUri],
    ["an address outside the file system", "untitled:Untitled-1"],
    ["a file address without a path", "file://server"],
    ["a file address with a query", "file:///path/(main).ts?revision=1"],
    ["a file address with a fragment", "file:///path/(main).ts#L1"],
    ["a malformed percent escape", "file:///path/(main)%.ts"],
    ["an invalid UTF-8 escape", "file:///path/(main)%FF.ts"],
    ["an encoded slash", "file:///path/(main)%2Finner.ts"],
    ["an encoded null character", "file:///path/(main)%00.ts"],
    ["a network path with an ambiguous host", "file:////some%20dir/main.ts"],
    ["a network path without a share", "file:////server/"],
    ["a literal Windows path separator", "file:///path/(main)\\inner.ts", "win32"],
    ["an encoded Windows path separator", "file:///path/(main)%5Cinner.ts", "win32"],
];

for (const [name, uri, platform = "linux"] of preservedAddresses) {
    test("a highlight request keeps the original bytes for " + name, async () => {
        const input = frameBody(JSON.stringify(highlightRequest(uri), null, 2), [contentType]);

        const output = await buffer(rewriteRequests([input], platform));

        assert.deepEqual(output, input);
    });
}

const otherMessages = [
    ["a hover request", { ...highlightRequest(), method: "textDocument/hover" }],
    ["a multi-file highlight request", { ...highlightRequest(), method: "custom/textDocument/multiDocumentHighlight" }],
    ["a semantic color request", { ...highlightRequest(), method: "textDocument/semanticTokens/full" }],
    ["a server reply from the editor", { jsonrpc: "2.0", id: 4, result: [] }],
    ["a highlight request without parameters", { jsonrpc: "2.0", id: 4, method: highlightMethod }],
    ["a highlight request without a file address", { ...highlightRequest(), params: { textDocument: {} } }],
    ["a highlight request with a non-string address", highlightRequest(123)],
];

for (const [name, message] of otherMessages) {
    test("the original message bytes are forwarded for " + name, async () => {
        const input = frameBody(JSON.stringify(message, null, 2), [contentType]);

        const output = await buffer(rewriteRequests([input], "linux"));

        assert.deepEqual(output, input);
    });
}

test("rewriting a file address preserves the cursor, request ID, and extra fields", async () => {
    const input = frame({
        ...highlightRequest(),
        trace: "editor",
        params: {
            textDocument: { uri: inputUri, version: 7 },
            position: { line: 4, character: 8 },
            workDoneToken: "work-🧪",
        },
    });

    const output = await buffer(rewriteRequests([input], "linux"));

    assert.deepEqual(readMessages(output), [{
        jsonrpc: "2.0",
        id: "request-α",
        method: highlightMethod,
        trace: "editor",
        params: {
            textDocument: { uri: normalizedUri, version: 7 },
            position: { line: 4, character: 8 },
            workDoneToken: "work-🧪",
        },
    }]);
});

test("messages stay in order across split headers and multibyte text", async () => {
    const hover = frameBody(JSON.stringify({ ...highlightRequest(), method: "textDocument/hover" }, null, 2), [contentType]);
    const first = frame(highlightRequest());
    const second = frame(highlightRequest("file:///Work/(next).ts"));
    const headerSplit = first.indexOf("\r\n\r\n") + 3;
    const unicodeSplit = first.indexOf(Buffer.from("é")) + 1;
    const chunks = [
        Buffer.concat([hover, first.subarray(0, headerSplit)]),
        first.subarray(headerSplit, unicodeSplit),
        Buffer.concat([first.subarray(unicodeSplit), second]),
    ];

    const output = await buffer(rewriteRequests(chunks, "linux"));

    assert.deepEqual(output.subarray(0, hover.length), hover);
    assert.deepEqual(readMessages(output.subarray(hover.length)), [
        highlightRequest(normalizedUri),
        highlightRequest("file:///Work/%28next%29.ts"),
    ]);
});

test("a frame accepts a case-insensitive length header and horizontal whitespace", async () => {
    const body = JSON.stringify(highlightRequest());
    const input = Buffer.from(contentType + "\r\ncOnTeNt-LeNgTh:\t" + Buffer.byteLength(body) + " \t\r\n\r\n" + body);

    const output = await buffer(rewriteRequests([input], "linux"));

    assert.deepEqual(readMessages(output), [highlightRequest(normalizedUri)]);
});

test("an empty input stream produces no output", async () => {
    const output = await buffer(rewriteRequests([], "linux"));

    assert.deepEqual(output, Buffer.alloc(0));
});

const invalidLengths = [
    ["a missing length", contentType],
    ["a zero length", "Content-Length: 0"],
    ["a negative length", "Content-Length: -2"],
    ["a non-integer length", "Content-Length: 2.5"],
    ["a length outside the safe integer range", "Content-Length: 9007199254740992"],
    ["two lengths that disagree", "Content-Length: 2\r\ncontent-length: 3"],
    ["a valid length alongside a malformed one", "Content-Length: invalid\r\ncontent-length: 2"],
];

for (const [name, headers] of invalidLengths) {
    test("the stream rejects a message with " + name, async () => {
        const input = Buffer.from(headers + "\r\n\r\n{}");

        const output = buffer(rewriteRequests([input], "linux"));

        await assert.rejects(output, /Invalid LSP Content-Length/);
    });
}

for (const [name, bytes] of [
    ["the header", Buffer.from("Content-Length: 10\r\n")],
    ["the body", Buffer.from('Content-Length: 10\r\n\r\n{"id":1')],
]) {
    test("the stream rejects a message when input ends inside " + name, async () => {
        const output = buffer(rewriteRequests([bytes], "linux"));

        await assert.rejects(output, /Incomplete LSP message/);
    });
}

test("the stream rejects a complete frame with invalid JSON", async () => {
    const input = frameBody("{no}");

    const output = buffer(rewriteRequests([input], "linux"));

    await assert.rejects(output, SyntaxError);
});
