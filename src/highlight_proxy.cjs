"use strict";

const { spawn } = require("node:child_process");
const { pipeline } = require("node:stream/promises");

const headerTerminator = "\r\n\r\n";

function encode(message) {
    const body = Buffer.from(JSON.stringify(message));
    const header = Buffer.from(`Content-Length: ${body.length}${headerTerminator}`);
    return Buffer.concat([header, body]);
}

function normalizeFileUri(uri) {
    const match = /^file:\/\/([^/?#]*)(\/[^?#]*)$/.exec(uri);
    if (!match) {
        return uri;
    }
    const [, authority, pathname] = match;
    try {
        const segments = pathname.split("/").map(decodeURIComponent);
        if (segments.some((segment) => segment.includes("/") || segment.includes("\0")
            || (process.platform === "win32" && segment.includes("\\")))) {
            return uri;
        }
        if (authority === "" && /^[a-z]:/i.test(segments[1])) {
            segments[1] = segments[1][0].toLowerCase() + segments[1].slice(1);
        }
        if (authority === "" && segments[1] === "" && segments[2]) {
            if (!segments[3] || !/^[a-z0-9._-]+$/i.test(segments[2])) {
                return uri;
            }
            // TypeScript emits UNC paths as file://server/share, even if given file:////server/share.
            segments.splice(0, 2);
        }
        // TypeScript escapes all characters outside the URI unreserved set in each path segment.
        const escaped = segments.map((segment) => encodeURIComponent(segment).replace(
            /[!'()*]/g, (character) => "%" + character.charCodeAt(0).toString(16).toUpperCase(),
        ));
        return `file://${authority}${escaped.join("/")}`;
    } catch {
        return uri;
    }
}

function normalizeHighlightRequest(message) {
    const uri = message?.params?.textDocument?.uri;
    if (message?.method !== "textDocument/documentHighlight" || typeof uri !== "string") {
        return;
    }
    const normalized = normalizeFileUri(uri);
    if (normalized !== uri) {
        return {
            ...message,
            params: { ...message.params, textDocument: { ...message.params.textDocument, uri: normalized } },
        };
    }
}

async function* rewriteRequests(input) {
    let header = Buffer.alloc(0);
    let remaining;
    let bodyParts = [];

    for await (const chunk of input) {
        let offset = 0;
        while (offset < chunk.length) {
            if (remaining === undefined) {
                const next = chunk.subarray(offset);
                const data = header.length === 0 ? next : Buffer.concat([header, next]);
                const end = data.indexOf(headerTerminator);
                if (end === -1) {
                    header = data;
                    break;
                }
                const field = /^Content-Length:[ \t]*(\d+)[ \t]*\r?$/im.exec(
                    data.toString("ascii", 0, end),
                );
                remaining = Number(field?.[1]);
                if (!Number.isSafeInteger(remaining) || remaining <= 0) {
                    throw new Error("Invalid LSP Content-Length");
                }
                offset += end + headerTerminator.length - header.length;
                header = data.subarray(0, end + headerTerminator.length);
            }

            const part = chunk.subarray(offset, offset + remaining);
            bodyParts.push(part);
            offset += part.length;
            remaining -= part.length;
            if (remaining > 0) {
                continue;
            }

            const body = bodyParts.length === 1 ? bodyParts[0] : Buffer.concat(bodyParts);
            const message = JSON.parse(body.toString("utf8"));
            const replacement = normalizeHighlightRequest(message);
            yield replacement === undefined ? Buffer.concat([header, body]) : encode(replacement);
            header = Buffer.alloc(0);
            remaining = undefined;
            bodyParts = [];
        }
    }

    if (header.length > 0) {
        throw new Error("Incomplete LSP message");
    }
}

function run(binary, args) {
    const server = spawn(binary, args, {
        stdio: ["pipe", "pipe", "inherit"],
        windowsHide: true,
    });
    let exited = false;
    let failed = false;

    function fail(error) {
        if (failed) {
            return;
        }
        failed = true;
        console.error(`TypeScript highlight proxy: ${error.message}`);
        process.exitCode = 1;
        process.stdin.destroy();
        server.kill();
    }

    server.on("error", fail);
    server.once("exit", (code) => {
        exited = true;
        if (!failed) {
            process.exitCode = code ?? 1;
        }
        process.stdin.destroy();
    });
    pipeline(process.stdin, rewriteRequests, server.stdin).catch(
        (error) => {
            if (!exited) {
                fail(error);
            }
        },
    );
    pipeline(server.stdout, process.stdout).catch(fail);
}

const [binary, ...args] = process.argv.slice(2);
run(binary, args);
