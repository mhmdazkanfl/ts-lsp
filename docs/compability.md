# TypeScript 7 LSP compatibility checklist

This checklist covers all 66 methods in the pinned [`microsoft/TypeScript` main snapshot][snapshot] (`7.1.0-dev`). The content mapper request was added after [TypeScript 7.0.2][release]. [Server handlers][handlers] · [Wire types][protocol]

The extension maps TypeScript to `typescript`, TSX to `typescriptreact`, JavaScript to `javascript`, and JSX to `javascriptreact`. [Extension manifest](../extension.toml)

Use the unchecked boxes for the Zed parity audit. A **wire field** records the upstream data shape. The **negotiation** and **advertised capabilities** checklists record the behavior confirmed in the server. Optional fields and union alternatives can appear in the upstream type even when the handler leaves them unset. The reusable wire shapes are linked from each method.

[Methods](#methods) · [JSON-RPC framing](#json-rpc-framing) · [Client negotiation](#client-capability-negotiation) · [Server capabilities](#advertised-server-capabilities) · [Workspace preferences](#workspace-preferences) · [Wire shapes](#shared-wire-shapes)

## Methods

- [ ] **Completion** (`textDocument/completion`) — client → server request
  Suggests symbols, members, keywords, types, object properties, JSX tags and attributes, import paths, auto-imports, and string literal values. Includes JSDoc tags and templates, exhaustive `switch` snippets, and metadata such as commit characters, label details, insert/replace edits, and deprecation tags. [Completion implementation][completions]
  - [ ] Completion normally returns a completion list, including its item defaults when negotiated.
  - [ ] Parameters — [`CompletionParams`](#completionparams)
    - [ ] `textDocument` ([`TextDocumentIdentifier`](#textdocumentidentifier), required)
    - [ ] `position` ([`Position`](#position), required)
    - [ ] `workDoneToken` ([`IntegerOrString`](#integerorstring), optional)
    - [ ] `partialResultToken` ([`IntegerOrString`](#integerorstring), optional)
    - [ ] `context` ([`CompletionContext`](#completioncontext), optional)
  - [ ] Result — [`CompletionItemsOrListOrNull`](#completionitemsorlistornull) (or null)
    - [ ] Variant — array of [`CompletionItem`](#completionitem)
    - [ ] Variant — [`CompletionList`](#completionlist)
    - [ ] Variant — null

- [ ] **Resolve a completion** (`completionItem/resolve`) — client → server request
  Fills in signatures, documentation, and additional edits such as inserting an import.
  - [ ] Parameters — [`CompletionItem`](#completionitem)
    - [ ] `label` (string, required)
    - [ ] `labelDetails` ([`CompletionItemLabelDetails`](#completionitemlabeldetails), optional)
    - [ ] `kind` ([`CompletionItemKind`](#completionitemkind), optional)
    - [ ] `tags` (array of [`CompletionItemTag`](#completionitemtag), optional)
    - [ ] `detail` (string, optional)
    - [ ] `documentation` ([`StringOrMarkupContent`](#stringormarkupcontent), optional)
    - [ ] `deprecated` (boolean, optional)
    - [ ] `preselect` (boolean, optional)
    - [ ] `sortText` (string, optional)
    - [ ] `filterText` (string, optional)
    - [ ] `insertText` (string, optional)
    - [ ] `insertTextFormat` ([`InsertTextFormat`](#inserttextformat), optional)
    - [ ] `insertTextMode` ([`InsertTextMode`](#inserttextmode), optional)
    - [ ] `textEdit` ([`TextEditOrInsertReplaceEdit`](#texteditorinsertreplaceedit), optional)
    - [ ] `textEditText` (string, optional)
    - [ ] `additionalTextEdits` (array of [`TextEdit`](#textedit), optional)
    - [ ] `commitCharacters` (array of string, optional)
    - [ ] `command` ([`Command`](#command), optional)
    - [ ] `data` ([`CompletionItemData`](#completionitemdata), optional)
  - [ ] Result — [`CompletionItem`](#completionitem)
    - [ ] `label` (string, required)
    - [ ] `labelDetails` ([`CompletionItemLabelDetails`](#completionitemlabeldetails), optional)
    - [ ] `kind` ([`CompletionItemKind`](#completionitemkind), optional)
    - [ ] `tags` (array of [`CompletionItemTag`](#completionitemtag), optional)
    - [ ] `detail` (string, optional)
    - [ ] `documentation` ([`StringOrMarkupContent`](#stringormarkupcontent), optional)
    - [ ] `deprecated` (boolean, optional)
    - [ ] `preselect` (boolean, optional)
    - [ ] `sortText` (string, optional)
    - [ ] `filterText` (string, optional)
    - [ ] `insertText` (string, optional)
    - [ ] `insertTextFormat` ([`InsertTextFormat`](#inserttextformat), optional)
    - [ ] `insertTextMode` ([`InsertTextMode`](#inserttextmode), optional)
    - [ ] `textEdit` ([`TextEditOrInsertReplaceEdit`](#texteditorinsertreplaceedit), optional)
    - [ ] `textEditText` (string, optional)
    - [ ] `additionalTextEdits` (array of [`TextEdit`](#textedit), optional)
    - [ ] `commitCharacters` (array of string, optional)
    - [ ] `command` ([`Command`](#command), optional)
    - [ ] `data` ([`CompletionItemData`](#completionitemdata), optional)

- [ ] **Hover** (`textDocument/hover`) — client → server request
  Shows types, signatures, and documentation in Markdown or plain text. Supports expandable hover via `experimental.hoverVerbosityLevel` and Visual Studio classified text and icons via `_vs_rawContent`. [Hover implementation][hover]
  - [ ] Parameters — [`HoverParams`](#hoverparams)
    - [ ] `textDocument` ([`TextDocumentIdentifier`](#textdocumentidentifier), required)
    - [ ] `position` ([`Position`](#position), required)
    - [ ] `workDoneToken` ([`IntegerOrString`](#integerorstring), optional)
    - [ ] `verbosityLevel` (integer, optional)
  - [ ] Result — [`HoverOrNull`](#hoverornull) (or null)
    - [ ] Variant — [`Hover`](#hover)
    - [ ] Variant — null

- [ ] **Signature help** (`textDocument/signatureHelp`) — client → server request
  Shows overloads, active parameters or type parameters, and documentation. Supports Visual Studio classified labels via `_vs_colorizedLabel`. [Signature help implementation][signature-help]
  - [ ] Parameters — [`SignatureHelpParams`](#signaturehelpparams)
    - [ ] `textDocument` ([`TextDocumentIdentifier`](#textdocumentidentifier), required)
    - [ ] `position` ([`Position`](#position), required)
    - [ ] `workDoneToken` ([`IntegerOrString`](#integerorstring), optional)
    - [ ] `context` ([`SignatureHelpContext`](#signaturehelpcontext), optional)
  - [ ] Result — [`SignatureHelpOrNull`](#signaturehelpornull) (or null)
    - [ ] Variant — [`SignatureHelp`](#signaturehelp)
    - [ ] Variant — null

- [ ] **Go to definition** (`textDocument/definition`) — client → server request
  Finds a symbol’s definition. The `preferGoToSourceDefinition` preference can favor original source.
  - [ ] The implementation returns locations or location links according to `textDocument.definition.linkSupport`.
  - [ ] Parameters — [`DefinitionParams`](#definitionparams)
    - [ ] `textDocument` ([`TextDocumentIdentifier`](#textdocumentidentifier), required)
    - [ ] `position` ([`Position`](#position), required)
    - [ ] `workDoneToken` ([`IntegerOrString`](#integerorstring), optional)
    - [ ] `partialResultToken` ([`IntegerOrString`](#integerorstring), optional) — The implementation returns results in the final response.
  - [ ] Result — [`LocationOrLocationsOrDefinitionLinksOrNull`](#locationorlocationsordefinitionlinksornull) (or null)
    - [ ] Variant — [`Location`](#location)
    - [ ] Variant — array of [`Location`](#location)
    - [ ] Variant — array of [`LocationLink`](#locationlink)
    - [ ] Variant — null

- [ ] **Go to type definition** (`textDocument/typeDefinition`) — client → server request
  Finds the definition of a symbol’s type.
  - [ ] Location links depend on `textDocument.typeDefinition.linkSupport`.
  - [ ] Parameters — [`TypeDefinitionParams`](#typedefinitionparams)
    - [ ] `textDocument` ([`TextDocumentIdentifier`](#textdocumentidentifier), required)
    - [ ] `position` ([`Position`](#position), required)
    - [ ] `workDoneToken` ([`IntegerOrString`](#integerorstring), optional)
    - [ ] `partialResultToken` ([`IntegerOrString`](#integerorstring), optional)
  - [ ] Result — [`LocationOrLocationsOrDefinitionLinksOrNull`](#locationorlocationsordefinitionlinksornull) (or null)
    - [ ] Variant — [`Location`](#location)
    - [ ] Variant — array of [`Location`](#location)
    - [ ] Variant — array of [`LocationLink`](#locationlink)
    - [ ] Variant — null

- [ ] **Go to implementation** (`textDocument/implementation`) — client → server request
  Finds implementations across relevant projects.
  - [ ] Location links depend on `textDocument.implementation.linkSupport`.
  - [ ] Parameters — [`ImplementationParams`](#implementationparams)
    - [ ] `textDocument` ([`TextDocumentIdentifier`](#textdocumentidentifier), required)
    - [ ] `position` ([`Position`](#position), required)
    - [ ] `workDoneToken` ([`IntegerOrString`](#integerorstring), optional)
    - [ ] `partialResultToken` ([`IntegerOrString`](#integerorstring), optional)
  - [ ] Result — [`LocationOrLocationsOrDefinitionLinksOrNull`](#locationorlocationsordefinitionlinksornull) (or null)
    - [ ] Variant — [`Location`](#location)
    - [ ] Variant — array of [`Location`](#location)
    - [ ] Variant — array of [`LocationLink`](#locationlink)
    - [ ] Variant — null

- [ ] **Go to source definition** (`custom/textDocument/sourceDefinition`) — client → server request
  Finds original source definitions, including when navigation starts from a declaration file.
  - [ ] Parameters — [`TextDocumentPositionParams`](#textdocumentpositionparams)
    - [ ] `textDocument` ([`TextDocumentIdentifier`](#textdocumentidentifier), required)
    - [ ] `position` ([`Position`](#position), required)
  - [ ] Result — [`LocationOrLocationsOrDefinitionLinksOrNull`](#locationorlocationsordefinitionlinksornull) (or null)
    - [ ] Variant — [`Location`](#location)
    - [ ] Variant — array of [`Location`](#location)
    - [ ] Variant — array of [`LocationLink`](#locationlink)
    - [ ] Variant — null

- [ ] **Find references** (`textDocument/references`) — client → server request
  Finds symbol usages across relevant projects. The client can include the declaration.
  - [ ] Parameters — [`ReferenceParams`](#referenceparams)
    - [ ] `textDocument` ([`TextDocumentIdentifier`](#textdocumentidentifier), required)
    - [ ] `position` ([`Position`](#position), required)
    - [ ] `workDoneToken` ([`IntegerOrString`](#integerorstring), optional)
    - [ ] `partialResultToken` ([`IntegerOrString`](#integerorstring), optional)
    - [ ] `context` ([`ReferenceContext`](#referencecontext), required)
  - [ ] Result — [`LocationsOrNull`](#locationsornull) (or null)
    - [ ] Variant — array of [`Location`](#location)
    - [ ] Variant — null

- [ ] **Visual Studio grouped references** (`textDocument/_vs_references`) — client → server request
  Groups references by definition and classifies them as reads or writes.
  - [ ] Parameters — [`ReferenceParams`](#referenceparams)
    - [ ] `textDocument` ([`TextDocumentIdentifier`](#textdocumentidentifier), required)
    - [ ] `position` ([`Position`](#position), required)
    - [ ] `workDoneToken` ([`IntegerOrString`](#integerorstring), optional)
    - [ ] `partialResultToken` ([`IntegerOrString`](#integerorstring), optional)
    - [ ] `context` ([`ReferenceContext`](#referencecontext), required)
  - [ ] Result — [`VSReferenceItemsOrNull`](#vsreferenceitemsornull) (or null)
    - [ ] Variant — array of [`VSReferenceItem`](#vsreferenceitem)
    - [ ] Variant — null

- [ ] **Document highlights** (`textDocument/documentHighlight`) — client → server request
  Highlights occurrences in the current document.
  - [ ] Parameters — [`DocumentHighlightParams`](#documenthighlightparams)
    - [ ] `textDocument` ([`TextDocumentIdentifier`](#textdocumentidentifier), required)
    - [ ] `position` ([`Position`](#position), required)
    - [ ] `workDoneToken` ([`IntegerOrString`](#integerorstring), optional)
    - [ ] `partialResultToken` ([`IntegerOrString`](#integerorstring), optional)
  - [ ] Result — [`DocumentHighlightsOrNull`](#documenthighlightsornull) (or null)
    - [ ] Variant — array of [`DocumentHighlight`](#documenthighlight)
    - [ ] Variant — null

- [ ] **Multi-document highlights** (`custom/textDocument/multiDocumentHighlight`) — client → server request
  Returns highlights across the documents specified by the client.
  - [ ] Parameters — [`MultiDocumentHighlightParams`](#multidocumenthighlightparams)
    - [ ] `textDocument` ([`TextDocumentIdentifier`](#textdocumentidentifier), required)
    - [ ] `position` ([`Position`](#position), required)
    - [ ] `filesToSearch` (array of [`DocumentUri`](#documenturi), required)
  - [ ] Result — [`MultiDocumentHighlightsOrNull`](#multidocumenthighlightsornull) (or null)
    - [ ] Variant — array of [`MultiDocumentHighlight`](#multidocumenthighlight)
    - [ ] Variant — null

- [ ] **Document outline** (`textDocument/documentSymbol`) — client → server request
  Lists symbols in a document, with a hierarchy when supported.
  - [ ] Parameters — [`DocumentSymbolParams`](#documentsymbolparams)
    - [ ] `workDoneToken` ([`IntegerOrString`](#integerorstring), optional)
    - [ ] `partialResultToken` ([`IntegerOrString`](#integerorstring), optional)
    - [ ] `textDocument` ([`TextDocumentIdentifier`](#textdocumentidentifier), required)
  - [ ] Result — [`SymbolInformationsOrDocumentSymbolsOrNull`](#symbolinformationsordocumentsymbolsornull) (or null)
    - [ ] Variant — array of [`SymbolInformation`](#symbolinformation)
    - [ ] Variant — array of [`DocumentSymbol`](#documentsymbol)
    - [ ] Variant — null

- [ ] **Workspace symbol search** (`workspace/symbol`) — client → server request
  Searches loaded projects for matching declarations.
  - [ ] Parameters — [`WorkspaceSymbolParams`](#workspacesymbolparams)
    - [ ] `workDoneToken` ([`IntegerOrString`](#integerorstring), optional)
    - [ ] `partialResultToken` ([`IntegerOrString`](#integerorstring), optional)
    - [ ] `query` (string, required)
    - [ ] `textDocument` ([`TextDocumentIdentifier`](#textdocumentidentifier), optional)
  - [ ] Result — [`SymbolInformationsOrWorkspaceSymbolsOrNull`](#symbolinformationsorworkspacesymbolsornull) (or null)
    - [ ] Variant — array of [`SymbolInformation`](#symbolinformation)
    - [ ] Variant — array of [`WorkspaceSymbol`](#workspacesymbol)
    - [ ] Variant — null

- [ ] **Prepare call hierarchy** (`textDocument/prepareCallHierarchy`) — client → server request
  Identifies the callable at a document position.
  - [ ] Parameters — [`CallHierarchyPrepareParams`](#callhierarchyprepareparams)
    - [ ] `textDocument` ([`TextDocumentIdentifier`](#textdocumentidentifier), required)
    - [ ] `position` ([`Position`](#position), required)
    - [ ] `workDoneToken` ([`IntegerOrString`](#integerorstring), optional)
  - [ ] Result — [`CallHierarchyItemsOrNull`](#callhierarchyitemsornull) (or null)
    - [ ] Variant — array of [`CallHierarchyItem`](#callhierarchyitem)
    - [ ] Variant — null

- [ ] **Incoming calls** (`callHierarchy/incomingCalls`) — client → server request
  Finds callers of a call hierarchy item.
  - [ ] Parameters — [`CallHierarchyIncomingCallsParams`](#callhierarchyincomingcallsparams)
    - [ ] `workDoneToken` ([`IntegerOrString`](#integerorstring), optional)
    - [ ] `partialResultToken` ([`IntegerOrString`](#integerorstring), optional)
    - [ ] `item` ([`CallHierarchyItem`](#callhierarchyitem), required)
  - [ ] Result — [`CallHierarchyIncomingCallsOrNull`](#callhierarchyincomingcallsornull) (or null)
    - [ ] Variant — array of [`CallHierarchyIncomingCall`](#callhierarchyincomingcall)
    - [ ] Variant — null

- [ ] **Outgoing calls** (`callHierarchy/outgoingCalls`) — client → server request
  Finds calls made by a call hierarchy item.
  - [ ] Parameters — [`CallHierarchyOutgoingCallsParams`](#callhierarchyoutgoingcallsparams)
    - [ ] `workDoneToken` ([`IntegerOrString`](#integerorstring), optional)
    - [ ] `partialResultToken` ([`IntegerOrString`](#integerorstring), optional)
    - [ ] `item` ([`CallHierarchyItem`](#callhierarchyitem), required)
  - [ ] Result — [`CallHierarchyOutgoingCallsOrNull`](#callhierarchyoutgoingcallsornull) (or null)
    - [ ] Variant — array of [`CallHierarchyOutgoingCall`](#callhierarchyoutgoingcall)
    - [ ] Variant — null

- [ ] **Prepare rename** (`textDocument/prepareRename`) — client → server request
  Checks whether the target can be renamed and returns the editable range and placeholder.
  - [ ] Parameters — [`PrepareRenameParams`](#preparerenameparams)
    - [ ] `textDocument` ([`TextDocumentIdentifier`](#textdocumentidentifier), required)
    - [ ] `position` ([`Position`](#position), required)
    - [ ] `workDoneToken` ([`IntegerOrString`](#integerorstring), optional)
  - [ ] Result — [`RangeOrPrepareRenamePlaceholderOrPrepareRenameDefaultBehaviorOrNull`](#rangeorpreparerenameplaceholderorpreparerenamedefaultbehaviorornull) (or null)
    - [ ] Variant — [`Range`](#range)
    - [ ] Variant — [`PrepareRenamePlaceholder`](#preparerenameplaceholder)
    - [ ] Variant — [`PrepareRenameDefaultBehavior`](#preparerenamedefaultbehavior)
    - [ ] Variant — null

- [ ] **Rename** (`textDocument/rename`) — client → server request
  Returns edits for symbol occurrences, including across project boundaries. Renaming an import path can also return a file rename operation when supported. [Rename handlers][rename]
  - [ ] Parameters — [`RenameParams`](#renameparams)
    - [ ] `textDocument` ([`TextDocumentIdentifier`](#textdocumentidentifier), required)
    - [ ] `position` ([`Position`](#position), required)
    - [ ] `workDoneToken` ([`IntegerOrString`](#integerorstring), optional)
    - [ ] `newName` (string, required)
  - [ ] Result — [`WorkspaceEditOrNull`](#workspaceeditornull) (or null)
    - [ ] Variant — [`WorkspaceEdit`](#workspaceedit)
    - [ ] Variant — null

- [ ] **Update imports on file rename** (`workspace/willRenameFiles`) — client → server request
  Computes edits to import paths and other references before a supported file is moved or renamed.
  - [ ] Parameters — [`RenameFilesParams`](#renamefilesparams)
    - [ ] `files` (array of [`FileRename`](#filerename), required)
  - [ ] Result — [`WorkspaceEditOrNull`](#workspaceeditornull) (or null)
    - [ ] Variant — [`WorkspaceEdit`](#workspaceedit)
    - [ ] Variant — null

- [ ] **Format document** (`textDocument/formatting`) — client → server request
  Formats a whole file using editor and TypeScript formatting options.
  - [ ] Parameters — [`DocumentFormattingParams`](#documentformattingparams)
    - [ ] `workDoneToken` ([`IntegerOrString`](#integerorstring), optional)
    - [ ] `textDocument` ([`TextDocumentIdentifier`](#textdocumentidentifier), required)
    - [ ] `options` ([`FormattingOptions`](#formattingoptions), required)
  - [ ] Result — [`TextEditsOrNull`](#texteditsornull) (or null)
    - [ ] Variant — array of [`TextEdit`](#textedit)
    - [ ] Variant — null

- [ ] **Format selection** (`textDocument/rangeFormatting`) — client → server request
  Formats the requested range.
  - [ ] Parameters — [`DocumentRangeFormattingParams`](#documentrangeformattingparams)
    - [ ] `workDoneToken` ([`IntegerOrString`](#integerorstring), optional)
    - [ ] `textDocument` ([`TextDocumentIdentifier`](#textdocumentidentifier), required)
    - [ ] `range` ([`Range`](#range), required)
    - [ ] `options` ([`FormattingOptions`](#formattingoptions), required)
  - [ ] Result — [`TextEditsOrNull`](#texteditsornull) (or null)
    - [ ] Variant — array of [`TextEdit`](#textedit)
    - [ ] Variant — null

- [ ] **Format on type** (`textDocument/onTypeFormatting`) — client → server request
  Formats on `{`, `}`, `;`, and newline.
  - [ ] Parameters — [`DocumentOnTypeFormattingParams`](#documentontypeformattingparams)
    - [ ] `textDocument` ([`TextDocumentIdentifier`](#textdocumentidentifier), required)
    - [ ] `position` ([`Position`](#position), required)
    - [ ] `ch` (string, required)
    - [ ] `options` ([`FormattingOptions`](#formattingoptions), required)
  - [ ] Result — [`TextEditsOrNull`](#texteditsornull) (or null)
    - [ ] Variant — array of [`TextEdit`](#textedit)
    - [ ] Variant — null

- [ ] **Folding ranges** (`textDocument/foldingRange`) — client → server request
  Finds foldable blocks, comments, and explicit regions, with collapsed text where supported.
  - [ ] Parameters — [`FoldingRangeParams`](#foldingrangeparams)
    - [ ] `workDoneToken` ([`IntegerOrString`](#integerorstring), optional)
    - [ ] `partialResultToken` ([`IntegerOrString`](#integerorstring), optional)
    - [ ] `textDocument` ([`TextDocumentIdentifier`](#textdocumentidentifier), required)
  - [ ] Result — [`FoldingRangesOrNull`](#foldingrangesornull) (or null)
    - [ ] Variant — array of [`FoldingRange`](#foldingrange)
    - [ ] Variant — null

- [ ] **Selection ranges** (`textDocument/selectionRange`) — client → server request
  Returns nested syntax-aware ranges for expanding or shrinking a selection.
  - [ ] Parameters — [`SelectionRangeParams`](#selectionrangeparams)
    - [ ] `workDoneToken` ([`IntegerOrString`](#integerorstring), optional)
    - [ ] `partialResultToken` ([`IntegerOrString`](#integerorstring), optional)
    - [ ] `textDocument` ([`TextDocumentIdentifier`](#textdocumentidentifier), required)
    - [ ] `positions` (array of [`Position`](#position), required)
  - [ ] Result — [`SelectionRangesOrNull`](#selectionrangesornull) (or null)
    - [ ] Variant — array of [`SelectionRange`](#selectionrange)
    - [ ] Variant — null

- [ ] **Linked JSX editing** (`textDocument/linkedEditingRange`) — client → server request
  Keeps matching JSX tag names in sync.
  - [ ] Parameters — [`LinkedEditingRangeParams`](#linkededitingrangeparams)
    - [ ] `textDocument` ([`TextDocumentIdentifier`](#textdocumentidentifier), required)
    - [ ] `position` ([`Position`](#position), required)
    - [ ] `workDoneToken` ([`IntegerOrString`](#integerorstring), optional)
  - [ ] Result — [`LinkedEditingRangesOrNull`](#linkededitingrangesornull) (or null)
    - [ ] Variant — [`LinkedEditingRanges`](#linkededitingranges)
    - [ ] Variant — null

- [ ] **JSX auto-close** (`textDocument/_vs_onAutoInsert`) — client → server request
  Returns a snippet that closes an element or fragment when the user types `>`.
  - [ ] Parameters — [`VSOnAutoInsertParams`](#vsonautoinsertparams)
    - [ ] `_vs_textDocument` ([`TextDocumentIdentifier`](#textdocumentidentifier), required)
    - [ ] `_vs_position` ([`Position`](#position), required)
    - [ ] `_vs_ch` (string, required)
  - [ ] Result — [`VSOnAutoInsertResponseItemOrNull`](#vsonautoinsertresponseitemornull) (or null)
    - [ ] Variant — [`VSOnAutoInsertResponseItem`](#vsonautoinsertresponseitem)
    - [ ] Variant — null

- [ ] **Code actions** (`textDocument/codeAction`) — client → server request
  `quickfix` covers missing and type-only imports, `isolatedDeclarations` fixes, and missing interface members, with batch fixes where available. Source actions are `source.fixAll.ts`, `source.organizeImports.ts`, `source.removeUnusedImports.ts`, and `source.sortImports.ts`. [Advertised action kinds][action-kinds] · [Code fix providers][code-actions]
  - [ ] Advertised values of the action kind
    - [ ] `quickfix` — Diagnostic fixes and eligible batch fixes.
    - [ ] `source.fixAll.ts` — Applies eligible source fixes.
    - [ ] `source.organizeImports.ts` — Organizes imports and removes unused ones.
    - [ ] `source.removeUnusedImports.ts` — Removes unused imports.
    - [ ] `source.sortImports.ts` — Sorts imports and specifiers.
  - [ ] Parameters — [`CodeActionParams`](#codeactionparams)
    - [ ] `workDoneToken` ([`IntegerOrString`](#integerorstring), optional)
    - [ ] `partialResultToken` ([`IntegerOrString`](#integerorstring), optional)
    - [ ] `textDocument` ([`TextDocumentIdentifier`](#textdocumentidentifier), required)
    - [ ] `range` ([`Range`](#range), required)
    - [ ] `context` ([`CodeActionContext`](#codeactioncontext), required)
  - [ ] Result — [`CommandOrCodeActionArrayOrNull`](#commandorcodeactionarrayornull) (or null)
    - [ ] Variant — array of [`CommandOrCodeAction`](#commandorcodeaction)
    - [ ] Variant — null

- [ ] **Pull document diagnostics** (`textDocument/diagnostic`) — client → server request
  Reports syntax, semantic, suggestion, and applicable declaration-emit diagnostics. Supports inter-file dependencies, related locations, and unnecessary or deprecated tags where supported.
  - [ ] The provider emits the full-report variant and does not return an unchanged report.
  - [ ] Parameters — [`DocumentDiagnosticParams`](#documentdiagnosticparams)
    - [ ] `workDoneToken` ([`IntegerOrString`](#integerorstring), optional)
    - [ ] `partialResultToken` ([`IntegerOrString`](#integerorstring), optional)
    - [ ] `textDocument` ([`TextDocumentIdentifier`](#textdocumentidentifier), required)
    - [ ] `identifier` (string, optional) — The server advertises `typescript` as its diagnostic identifier.
    - [ ] `previousResultId` (string, optional) — The handler returns a full report.
  - [ ] Result — [`RelatedFullDocumentDiagnosticReportOrUnchangedDocumentDiagnosticReport`](#relatedfulldocumentdiagnosticreportorunchangeddocumentdiagnosticreport)
    - [ ] Variant — [`RelatedFullDocumentDiagnosticReport`](#relatedfulldocumentdiagnosticreport)
    - [ ] Variant — [`RelatedUnchangedDocumentDiagnosticReport`](#relatedunchangeddocumentdiagnosticreport)

- [ ] **Push project diagnostics** (`textDocument/publishDiagnostics`) — server → client notification
  Publishes project-level diagnostics against the config file. The client can turn this off with `initializationOptions.disablePushDiagnostics`. [Project diagnostics][project-diagnostics]
  - [ ] Parameters — [`PublishDiagnosticsParams`](#publishdiagnosticsparams)
    - [ ] `uri` ([`DocumentUri`](#documenturi), required)
    - [ ] `version` (integer, optional) — Schema only for TypeScript’s project-level push reports.
    - [ ] `diagnostics` (array of [`Diagnostic`](#diagnostic), required)

- [ ] **Refresh pull diagnostics** (`workspace/diagnostic/refresh`) — server → client request
  Asks the client to pull diagnostics again when project state changes.
  - [ ] Parameters — no parameters
  - [ ] Result — null

- [ ] **Full-document semantic highlighting** (`textDocument/semanticTokens/full`) — client → server request
  Returns semantic token types and modifiers for the whole document.
  - [ ] The server advertises full and range requests. A separate full/delta method is not registered.
  - [ ] Parameters — [`SemanticTokensParams`](#semantictokensparams)
    - [ ] `workDoneToken` ([`IntegerOrString`](#integerorstring), optional)
    - [ ] `partialResultToken` ([`IntegerOrString`](#integerorstring), optional)
    - [ ] `textDocument` ([`TextDocumentIdentifier`](#textdocumentidentifier), required)
  - [ ] Result — [`SemanticTokensOrNull`](#semantictokensornull) (or null)
    - [ ] Variant — [`SemanticTokens`](#semantictokens)
    - [ ] Variant — null

- [ ] **Range semantic highlighting** (`textDocument/semanticTokens/range`) — client → server request
  Returns semantic tokens for the requested range.
  - [ ] Parameters — [`SemanticTokensRangeParams`](#semantictokensrangeparams)
    - [ ] `workDoneToken` ([`IntegerOrString`](#integerorstring), optional)
    - [ ] `partialResultToken` ([`IntegerOrString`](#integerorstring), optional)
    - [ ] `textDocument` ([`TextDocumentIdentifier`](#textdocumentidentifier), required)
    - [ ] `range` ([`Range`](#range), required)
  - [ ] Result — [`SemanticTokensOrNull`](#semantictokensornull) (or null)
    - [ ] Variant — [`SemanticTokens`](#semantictokens)
    - [ ] Variant — null

- [ ] **Inlay hints** (`textDocument/inlayHint`) — client → server request
  Shows parameter names, inferred parameter, variable, property, and return types, and enum values. Hint labels can link to declarations where available. [Inlay hints implementation][inlay-hints]
  - [ ] The server returns label strings or label parts with locations when the inferred type refers to a declaration.
  - [ ] Parameters — [`InlayHintParams`](#inlayhintparams)
    - [ ] `workDoneToken` ([`IntegerOrString`](#integerorstring), optional)
    - [ ] `partialResultToken` ([`IntegerOrString`](#integerorstring), optional)
    - [ ] `textDocument` ([`TextDocumentIdentifier`](#textdocumentidentifier), required)
    - [ ] `range` ([`Range`](#range), required)
  - [ ] Result — [`InlayHintsOrNull`](#inlayhintsornull) (or null)
    - [ ] Variant — array of [`InlayHint`](#inlayhint)
    - [ ] Variant — null

- [ ] **Refresh inlay hints** (`workspace/inlayHint/refresh`) — server → client request
  Asks the client to refresh hints after preferences change.
  - [ ] Parameters — no parameters
  - [ ] Result — null

- [ ] **List CodeLenses** (`textDocument/codeLens`) — client → server request
  Returns CodeLenses for references and implementations.
  - [ ] Parameters — [`CodeLensParams`](#codelensparams)
    - [ ] `workDoneToken` ([`IntegerOrString`](#integerorstring), optional)
    - [ ] `partialResultToken` ([`IntegerOrString`](#integerorstring), optional)
    - [ ] `textDocument` ([`TextDocumentIdentifier`](#textdocumentidentifier), required)
  - [ ] Result — [`CodeLensesOrNull`](#codelensesornull) (or null)
    - [ ] Variant — array of [`CodeLens`](#codelens)
    - [ ] Variant — null

- [ ] **Resolve a CodeLens** (`codeLens/resolve`) — client → server request
  Fills in the reference or implementation count and the locations for the client command named by `initializationOptions.codeLensShowLocationsCommandName`. [CodeLens implementation][code-lens]
  - [ ] Parameters — [`CodeLens`](#codelens)
    - [ ] `range` ([`Range`](#range), required)
    - [ ] `command` ([`Command`](#command), optional)
    - [ ] `data` ([`CodeLensData`](#codelensdata), optional)
  - [ ] Result — [`CodeLens`](#codelens)
    - [ ] `range` ([`Range`](#range), required)
    - [ ] `command` ([`Command`](#command), optional)
    - [ ] `data` ([`CodeLensData`](#codelensdata), optional)

- [ ] **Refresh CodeLenses** (`workspace/codeLens/refresh`) — server → client request
  Asks the client to refresh CodeLenses after preferences change.
  - [ ] Parameters — no parameters
  - [ ] Result — null

- [ ] **Read workspace configuration** (`workspace/configuration`) — server → client request
  Requests `js/ts`, `typescript`, `javascript`, and `editor` settings for features such as formatting, validation, locale, and automatic type acquisition. Clients without configuration requests can use `initializationOptions.userPreferences`. [Configuration handler][configuration] · [Type acquisition][ata]
  - [ ] Array entries correspond to the requested sections in order. See [workspace preferences](#workspace-preferences) for the individual keys.
  - [ ] Parameters — [`ConfigurationParams`](#configurationparams)
    - [ ] `items` (array of [`ConfigurationItem`](#configurationitem), required)
  - [ ] Result — array of JSON value

- [ ] **Reload workspace configuration** (`workspace/didChangeConfiguration`) — client → server notification
  Reloads settings after the client reports a change.
  - [ ] An object payload is applied directly by section. A null payload leaves the settings unchanged.
  - [ ] Parameters — [`DidChangeConfigurationParams`](#didchangeconfigurationparams)
    - [ ] `settings` (JSON value, required) — An object is parsed as workspace preferences by section. Null leaves the settings unchanged.

- [ ] **Open a document** (`textDocument/didOpen`) — client → server notification
  Tracks the open document and its language ID.
  - [ ] Parameters — [`DidOpenTextDocumentParams`](#didopentextdocumentparams)
    - [ ] `textDocument` ([`TextDocumentItem`](#textdocumentitem), required)

- [ ] **Synchronize document edits** (`textDocument/didChange`) — client → server notification
  Applies incremental edits to the server’s copy of the document.
  - [ ] Parameters — [`DidChangeTextDocumentParams`](#didchangetextdocumentparams)
    - [ ] `textDocument` ([`VersionedTextDocumentIdentifier`](#versionedtextdocumentidentifier), required)
    - [ ] `contentChanges` (array of [`TextDocumentContentChangePartialOrWholeDocument`](#textdocumentcontentchangepartialorwholedocument), required)

- [ ] **Save a document** (`textDocument/didSave`) — client → server notification
  Receives save notifications for open documents.
  - [ ] Parameters — [`DidSaveTextDocumentParams`](#didsavetextdocumentparams)
    - [ ] `textDocument` ([`TextDocumentIdentifier`](#textdocumentidentifier), required)
    - [ ] `text` (string, optional) — Schema only. The save handler uses the document URI.

- [ ] **Close a document** (`textDocument/didClose`) — client → server notification
  Releases the open-document state.
  - [ ] Parameters — [`DidCloseTextDocumentParams`](#didclosetextdocumentparams)
    - [ ] `textDocument` ([`TextDocumentIdentifier`](#textdocumentidentifier), required)

- [ ] **Receive watched file changes** (`workspace/didChangeWatchedFiles`) — client → server notification
  Updates project state as files and dependencies change. A built-in watcher is available on supported platforms. [File watcher setup][workspace-init]
  - [ ] Parameters — [`DidChangeWatchedFilesParams`](#didchangewatchedfilesparams)
    - [ ] `changes` (array of [`FileEvent`](#fileevent), required)

- [ ] **Register client capabilities dynamically** (`client/registerCapability`) — server → client request
  Registers for configuration and file changes. On main, also registers sync, diagnostics, and supported editor features for content-mapped file types. [Content mapper registration][content-mapper]
  - [ ] Parameters — [`RegistrationParams`](#registrationparams)
    - [ ] `registrations` (array of [`Registration`](#registration), required)
  - [ ] Result — null

- [ ] **Unregister client capabilities dynamically** (`client/unregisterCapability`) — server → client request
  Removes file watchers or superseded content mapper registrations.
  - [ ] Parameters — [`UnregistrationParams`](#unregistrationparams)
    - [ ] `unregisterations` (array of [`Unregistration`](#unregistration), required) — This spelling is the LSP wire key.
  - [ ] Result — null

- [ ] **Create a progress indicator** (`window/workDoneProgress/create`) — server → client request
  Creates a work-done progress token when supported by the client.
  - [ ] Parameters — [`WorkDoneProgressCreateParams`](#workdoneprogresscreateparams)
    - [ ] `token` ([`IntegerOrString`](#integerorstring), required)
  - [ ] Result — null

- [ ] **Report progress** (`$/progress`) — server → client notification
  Sends project loading progress for that token. [Project loading progress][progress]
  - [ ] Parameters — [`ProgressParams`](#progressparams)
    - [ ] `token` ([`IntegerOrString`](#integerorstring), required) — The project progress reporter uses a string token.
    - [ ] `value` ([`WorkDoneProgressBeginOrReportOrEnd`](#workdoneprogressbeginorreportorend), required)

- [ ] **Get project information** (`custom/projectInfo`) — client → server request
  Returns the path to the document’s configured `tsconfig.json` or `jsconfig.json` where available.
  - [ ] Parameters — [`ProjectInfoParams`](#projectinfoparams)
    - [ ] `textDocument` ([`TextDocumentIdentifier`](#textdocumentidentifier), required)
  - [ ] Result — [`ProjectInfoResult`](#projectinforesult)
    - [ ] `configFilePath` (string, required)

- [ ] **Open a TypeScript API session** (`custom/initializeAPISession`) — client → server request
  Opens a separate IPC session backed by the language server’s project state.
  - [ ] Parameters — [`InitializeAPISessionParams`](#initializeapisessionparams)
    - [ ] `pipe` (string, optional) — The server creates a private pipe when the client leaves this empty.
  - [ ] Result — [`InitializeAPISessionResult`](#initializeapisessionresult)
    - [ ] `sessionId` (string, required)
    - [ ] `pipe` (string, required)

- [ ] **Set content mapper contributions** (`custom/setContentMapperContributions`) — client → server request
  Lets a client contribute mappings from additional file types to virtual TypeScript source. Available on the pinned main snapshot.
  - [ ] Parameters — [`SetContentMapperContributionsParams`](#setcontentmappercontributionsparams)
    - [ ] `contributions` (array of [`ContentMapperContribution`](#contentmappercontribution), required)
    - [ ] `openDocuments` (array of [`TextDocumentIdentifier`](#textdocumentidentifier), required)
  - [ ] Result — null

- [ ] **Initialize a session** (`initialize`) — client → server request
  Negotiates capabilities, initial workspace folders or root, locale, and position encoding. Defaults to UTF-16 and prefers UTF-8 when offered. Reads startup options including `initializationOptions.logVerbosity`, `initializationOptions.runExternalCode` for external content mappers, and `initializationOptions.trackFlakyDiagnostics` on main.
  - [ ] See [client capability negotiation](#client-capability-negotiation) and [advertised server capabilities](#advertised-server-capabilities).
  - [ ] Parameters — [`InitializeParams`](#initializeparams)
    - [ ] `workDoneToken` ([`IntegerOrString`](#integerorstring), optional) — TypeScript project loading uses a server-created token.
    - [ ] `processId` ([`IntegerOrNull`](#integerornull) (or null), required) — Starts the parent-process watchdog when the runtime provides one.
    - [ ] `clientInfo` ([`ClientInfo`](#clientinfo), optional)
    - [ ] `locale` (string, optional) — Sets the initial locale and the fallback for the auto preference.
    - [ ] `rootPath` ([`StringOrNull`](#stringornull) (or null), optional, nullable)
    - [ ] `rootUri` ([`DocumentUriOrNull`](#documenturiornull) (or null), required)
    - [ ] `capabilities` ([`ClientCapabilities`](#clientcapabilities), required) — See the negotiated capability checklist below.
    - [ ] `initializationOptions` ([`InitializationOptionsOrNull`](#initializationoptionsornull) (or null), optional, nullable) — See the individual startup options below.
    - [ ] `trace` ([`TraceValue`](#tracevalue), optional) — Accepted at startup. Runtime server logging uses the TypeScript verbosity option.
    - [ ] `workspaceFolders` ([`WorkspaceFoldersOrNull`](#workspacefoldersornull) (or null), optional, nullable)
  - [ ] Result — [`InitializeResult`](#initializeresult)
    - [ ] `capabilities` ([`ServerCapabilities`](#servercapabilities), required) — See the advertised capability checklist below.
    - [ ] `serverInfo` ([`ServerInfo`](#serverinfo), optional)

- [ ] **Finish initialization** (`initialized`) — client → server notification
  Starts project and configuration services after the client acknowledges initialization.
  - [ ] Parameters — [`InitializedParams`](#initializedparams)

- [ ] **Shut down a session** (`shutdown`) — client → server request
  Shuts down the language service session.
  - [ ] Parameters — no parameters
  - [ ] Result — null

- [ ] **Exit the server** (`exit`) — client → server notification
  Ends the server process.
  - [ ] Parameters — no parameters

- [ ] **Cancel a request** (`$/cancelRequest`) — client → server notification
  Cancels server work associated with the request ID.
  - [ ] Parameters — [`CancelParams`](#cancelparams)
    - [ ] `id` ([`IntegerOrString`](#integerorstring), required)

- [ ] **Receive trace settings** (`$/setTrace`) — client → server notification
  Accepts the standard trace notification as a no-op. [Trace handler][trace]
  - [ ] Parameters — [`SetTraceParams`](#settraceparams)
    - [ ] `value` ([`TraceValue`](#tracevalue), required)

- [ ] **Send log messages** (`window/logMessage`) — server → client notification
  Sends server log messages to the client. [Server logging][logging]
  - [ ] Parameters — [`LogMessageParams`](#logmessageparams)
    - [ ] `type` ([`MessageType`](#messagetype), required)
    - [ ] `message` (string, required)

- [ ] **Change log verbosity** (`custom/setLogVerbosity`) — client → server notification
  Updates the server’s log level during a session.
  - [ ] Parameters — [`SetLogVerbosityParams`](#setlogverbosityparams)
    - [ ] `verbosity` ([`LogVerbosity`](#logverbosity), required)

- [ ] **Send telemetry** (`telemetry/event`) — server → client notification
  Sends project and server events when `initializationOptions.enableTelemetry` is enabled.
  - [ ] Parameters — [`RequestFailureTelemetryEventOrPerformanceStatsTelemetryEventOrProjectInfoTelemetryEventOrNull`](#requestfailuretelemetryeventorperformancestatstelemetryeventorprojectinfotelemetryeventornull) (or null)
    - [ ] Variant — [`RequestFailureTelemetryEvent`](#requestfailuretelemetryevent)
    - [ ] Variant — [`PerformanceStatsTelemetryEvent`](#performancestatstelemetryevent)
    - [ ] Variant — [`ProjectInfoTelemetryEvent`](#projectinfotelemetryevent)
    - [ ] Variant — null

- [ ] **Force garbage collection** (`custom/runGC`) — client → server request
  Requests a server garbage collection cycle.
  - [ ] Parameters — no parameters
  - [ ] Result — null

- [ ] **Save a heap profile** (`custom/saveHeapProfile`) — client → server request
  Writes a memory heap profile and returns its path.
  - [ ] Parameters — [`ProfileParams`](#profileparams)
    - [ ] `dir` (string, required)
  - [ ] Result — [`ProfileResult`](#profileresult)
    - [ ] `file` (string, required)

- [ ] **Save an allocation profile** (`custom/saveAllocProfile`) — client → server request
  Writes an allocation profile and returns its path.
  - [ ] Parameters — [`ProfileParams`](#profileparams)
    - [ ] `dir` (string, required)
  - [ ] Result — [`ProfileResult`](#profileresult)
    - [ ] `file` (string, required)

- [ ] **Start CPU profiling** (`custom/startCPUProfile`) — client → server request
  Starts recording CPU usage.
  - [ ] Parameters — [`ProfileParams`](#profileparams)
    - [ ] `dir` (string, required)
  - [ ] Result — null

- [ ] **Stop CPU profiling** (`custom/stopCPUProfile`) — client → server request
  Stops recording and returns the saved profile’s path.
  - [ ] Parameters — no parameters
  - [ ] Result — [`ProfileResult`](#profileresult)
    - [ ] `file` (string, required)

## JSON-RPC framing

These fields apply to the stdio transport and the JSON-RPC messages above. [Framing][framing] · [Message envelope][envelope]

- [ ] LSP framing
  - [ ] `Content-Length` — Required byte length.
  - [ ] `Content-Type` — Optional LSP header. The TypeScript stdio reader skips it and the writer emits Content-Length only.
- [ ] Request envelope
  - [ ] `jsonrpc` — `2.0`
  - [ ] `id` — Integer or string request identifier.
  - [ ] `method` — Name of the LSP request.
  - [ ] `params` — Method-specific parameters where applicable.
- [ ] Notification envelope
  - [ ] `jsonrpc` — `2.0`
  - [ ] `method` — Name of the notification.
  - [ ] `params` — Method-specific parameters where applicable.
- [ ] Success response envelope
  - [ ] `jsonrpc` — `2.0`
  - [ ] `id` — Identifier of the corresponding request.
  - [ ] `result` — Method-specific result or null.
- [ ] Error response envelope
  - [ ] `jsonrpc` — `2.0`
  - [ ] `id` — Identifier of the corresponding request or null when unavailable.
  - [ ] `error.code` — Numeric JSON-RPC or LSP error code.
  - [ ] `error.message` — Human-readable text.
  - [ ] `error.data` — Optional error-specific payload.

## Client capability negotiation

Paths below are relative to `initialize.params.capabilities`. These are the client fields read by the server when choosing how to respond or register a feature. [Initialization and registrations][capabilities] · [Language service][language-service]

- [ ] General and progress
  - [ ] `general.positionEncodings` — Selects UTF-8 when offered. The fallback is UTF-16.
  - [ ] `window.workDoneProgress` — Enables project work-done progress.
  - [ ] `experimental.hoverVerbosityLevel` — Enables expandable hover output and `canIncreaseVerbosity`.
  - [ ] `_vs_supportsVisualStudioExtensions` — Enables Visual Studio-specific hover, signatures, references, and diagnostic presentation.
- [ ] Workspace
  - [ ] `workspace.configuration` — Enables configuration requests. Startup user preferences serve as a fallback.
  - [ ] `workspace.workspaceFolders` — The server can use a single client workspace folder as the session working directory.
  - [ ] `workspace.didChangeWatchedFiles.dynamicRegistration` — Enables client-side file watching.
  - [ ] `workspace.didChangeWatchedFiles.relativePatternSupport` — Lets registered file watches use relative patterns.
  - [ ] `workspace.fileOperations.dynamicRegistration` — Allows content-mapped rename registrations when the client also advertises willRename.
  - [ ] `workspace.fileOperations.willRename` — Lets file rename edits rely on the client to request updates.
  - [ ] `workspace.workspaceEdit.documentChanges` — Controls use of documentChanges in workspace edits.
  - [ ] `workspace.workspaceEdit.resourceOperations` — File rename operations require the rename resource operation.
  - [ ] `workspace.diagnostics.refreshSupport` — Allows the server to request a diagnostic refresh.
  - [ ] `workspace.inlayHint.refreshSupport` — Allows the server to request an inlay hint refresh.
  - [ ] `workspace.codeLens.refreshSupport` — Allows the server to request a CodeLens refresh.
- [ ] Text documents
  - [ ] `textDocument.hover.contentFormat` — Selects Markdown or plain text.
  - [ ] `textDocument.definition.linkSupport` — Selects location links for definition and source definition.
  - [ ] `textDocument.typeDefinition.linkSupport` — Selects location links for type definition.
  - [ ] `textDocument.implementation.linkSupport` — Selects location links for implementation.
  - [ ] `textDocument.completion.completionItem.labelDetailsSupport` — Enables secondary completion labels and descriptions.
  - [ ] `textDocument.completion.completionItem.snippetSupport` — Allows snippet edits.
  - [ ] `textDocument.completion.completionItem.commitCharactersSupport` — Enables item commit characters.
  - [ ] `textDocument.completion.completionItem.insertReplaceSupport` — Allows separate insert and replace ranges.
  - [ ] `textDocument.completion.completionItem.documentationFormat` — Selects the format for completion documentation.
  - [ ] `textDocument.completion.completionList.itemDefaults` — Enables list-level editRange and commitCharacters defaults when the client lists those keys.
  - [ ] `textDocument.signatureHelp.signatureInformation.activeParameterSupport` — Allows reporting the active parameter per signature.
  - [ ] `textDocument.signatureHelp.signatureInformation.noActiveParameterSupport` — Allows null when there is no active parameter.
  - [ ] `textDocument.signatureHelp.signatureInformation.documentationFormat` — Selects the format for signature documentation.
  - [ ] `textDocument.documentSymbol.hierarchicalDocumentSymbolSupport` — Selects a document symbol tree or a flat list.
  - [ ] `textDocument.foldingRange.lineFoldingOnly` — Uses line-only folding ranges.
  - [ ] `textDocument.foldingRange.foldingRange.collapsedText` — Allows collapsed preview text.
  - [ ] `textDocument.semanticTokens.tokenTypes` — Filters the advertised legend and token data to compatible types.
  - [ ] `textDocument.semanticTokens.tokenModifiers` — Filters the legend and token data to compatible modifiers.
  - [ ] `textDocument.diagnostic.relatedInformation` — Enables related locations on pulled diagnostics.
  - [ ] `textDocument.diagnostic.tagSupport.valueSet` — Selects supported pull diagnostics tags.
  - [ ] `textDocument.publishDiagnostics.relatedInformation` — Enables related locations on pushed diagnostics.
  - [ ] `textDocument.publishDiagnostics.tagSupport.valueSet` — Selects supported push diagnostics tags.
  - [ ] `textDocument.filters.relativePatternSupport` — Allows relative document filter patterns for content mappers.
- [ ] Content mapper dynamic registration on main
  - [ ] `textDocument.synchronization.dynamicRegistration` — Gates the corresponding registration for contributed document types.
  - [ ] `textDocument.diagnostic.dynamicRegistration` — Gates the corresponding registration for contributed document types.
  - [ ] `textDocument.hover.dynamicRegistration` — Gates the corresponding registration for contributed document types.
  - [ ] `textDocument.signatureHelp.dynamicRegistration` — Gates the corresponding registration for contributed document types.
  - [ ] `textDocument.definition.dynamicRegistration` — Gates the corresponding registration for contributed document types.
  - [ ] `textDocument.typeDefinition.dynamicRegistration` — Gates the corresponding registration for contributed document types.
  - [ ] `textDocument.implementation.dynamicRegistration` — Gates the corresponding registration for contributed document types.
  - [ ] `textDocument.references.dynamicRegistration` — Gates the corresponding registration for contributed document types.
  - [ ] `textDocument.documentHighlight.dynamicRegistration` — Gates the corresponding registration for contributed document types.
  - [ ] `textDocument.completion.dynamicRegistration` — Gates the corresponding registration for contributed document types.
  - [ ] `textDocument.rename.dynamicRegistration` — Gates the corresponding registration for contributed document types.
  - [ ] `textDocument.semanticTokens.dynamicRegistration` — Gates the corresponding registration for contributed document types.
  - [ ] `textDocument.documentSymbol.dynamicRegistration` — Gates the corresponding registration for contributed document types.
  - [ ] `textDocument.foldingRange.dynamicRegistration` — Gates the corresponding registration for contributed document types.
  - [ ] `textDocument.selectionRange.dynamicRegistration` — Gates the corresponding registration for contributed document types.
  - [ ] `textDocument.inlayHint.dynamicRegistration` — Gates the corresponding registration for contributed document types.
  - [ ] `textDocument.codeLens.dynamicRegistration` — Gates the corresponding registration for contributed document types.
  - [ ] `textDocument.codeAction.dynamicRegistration` — Gates the corresponding registration for contributed document types.
  - [ ] `textDocument.formatting.dynamicRegistration` — Gates the corresponding registration for contributed document types.
  - [ ] `textDocument.rangeFormatting.dynamicRegistration` — Gates the corresponding registration for contributed document types.
  - [ ] `textDocument.onTypeFormatting.dynamicRegistration` — Gates the corresponding registration for contributed document types.
  - [ ] `textDocument.linkedEditingRange.dynamicRegistration` — Gates the corresponding registration for contributed document types.
  - [ ] `textDocument.callHierarchy.dynamicRegistration` — Gates the corresponding registration for contributed document types.

## Advertised server capabilities

Paths below are relative to `initialize.result.capabilities`. [Initialize response][capabilities]

- [ ] `positionEncoding` — UTF-8 if offered, otherwise UTF-16.
- [ ] `hoverProvider` — True.
- [ ] `definitionProvider` — True.
- [ ] `typeDefinitionProvider` — True.
- [ ] `referencesProvider` — True.
- [ ] `implementationProvider` — True.
- [ ] `documentFormattingProvider` — True.
- [ ] `documentRangeFormattingProvider` — True.
- [ ] `workspaceSymbolProvider` — True.
- [ ] `documentSymbolProvider` — True.
- [ ] `foldingRangeProvider` — True.
- [ ] `documentHighlightProvider` — True.
- [ ] `selectionRangeProvider` — True.
- [ ] `linkedEditingRangeProvider` — True.
- [ ] `inlayHintProvider` — True.
- [ ] `callHierarchyProvider` — True.
- [ ] `_vs_referencesProvider` — True.
- [ ] `textDocumentSync`
  - [ ] `openClose` — True.
  - [ ] `change` — Incremental (`2`).
  - [ ] `save` — True.
- [ ] `diagnosticProvider`
  - [ ] `identifier` — `typescript`.
  - [ ] `interFileDependencies` — True.
  - [ ] `workspaceDiagnostics` — False.
- [ ] `completionProvider`
  - [ ] `triggerCharacters` — Period, double quote, single quote, backtick, slash, at sign, less-than, hash, space, and asterisk.
  - [ ] `resolveProvider` — True.
  - [ ] `completionItem.labelDetailsSupport` — True.
- [ ] `signatureHelpProvider`
  - [ ] `triggerCharacters` — Opening parenthesis, comma, and less-than.
  - [ ] `retriggerCharacters` — Closing parenthesis.
- [ ] `documentOnTypeFormattingProvider`
  - [ ] `firstTriggerCharacter` — Opening brace.
  - [ ] `moreTriggerCharacter` — Closing brace, semicolon, and newline.
- [ ] `renameProvider`
  - [ ] `prepareProvider` — True.
- [ ] `codeLensProvider`
  - [ ] `resolveProvider` — True.
- [ ] `codeActionProvider`
  - [ ] `codeActionKinds` — The five kinds listed under the code actions method.
- [ ] `experimental`
  - [ ] `customSourceDefinitionProvider` — True.
  - [ ] `customMultiDocumentHighlightProvider` — True.
- [ ] `_vs_onAutoInsertProvider`
  - [ ] `_vs_triggerCharacters` — Greater-than.
- [ ] `workspace`
  - [ ] `fileOperations.willRename.filters` — File-scheme paths matching TS, TSX, JS, JSX, CTS, CJS, MTS, MJS, and JSON.
- [ ] `semanticTokensProvider`
  - [ ] `legend.tokenTypes` — Intersected with the client token types.
    - [ ] `"namespace"`
    - [ ] `"class"`
    - [ ] `"enum"`
    - [ ] `"interface"`
    - [ ] `"struct"`
    - [ ] `"typeParameter"`
    - [ ] `"type"`
    - [ ] `"parameter"`
    - [ ] `"variable"`
    - [ ] `"property"`
    - [ ] `"enumMember"`
    - [ ] `"decorator"`
    - [ ] `"event"`
    - [ ] `"function"`
    - [ ] `"method"`
    - [ ] `"macro"`
    - [ ] `"label"`
    - [ ] `"comment"`
    - [ ] `"string"`
    - [ ] `"keyword"`
    - [ ] `"number"`
    - [ ] `"regexp"`
    - [ ] `"operator"`
  - [ ] `legend.tokenModifiers` — Intersected with the client token modifiers.
    - [ ] `"declaration"`
    - [ ] `"definition"`
    - [ ] `"readonly"`
    - [ ] `"static"`
    - [ ] `"deprecated"`
    - [ ] `"abstract"`
    - [ ] `"async"`
    - [ ] `"modification"`
    - [ ] `"documentation"`
    - [ ] `"defaultLibrary"`
    - [ ] `"local"`
  - [ ] `full` — True.
  - [ ] `range` — True.

## Workspace preferences

These settings are parsed from the `workspace/configuration` results in increasing precedence `editor`, `javascript`, `typescript`, and `js/ts`. In the language sections, flat fields can also appear under `unstable`. Nested keys override flat fields in the same section. The startup `userPreferences` fallback is parsed as the `js/ts` section. [Configuration parser][preference-parser] · [Settings schema][settings-schema]

- [ ] Editor defaults
  - [ ] `editor.tabSize` — Also supplies indentSize when that field is absent.
  - [ ] `editor.insertSpaces` — Supplies convertTabsToSpaces when that field is absent.
  - [ ] `editor.indentSize` — Overrides the indent-size fallback.
  - [ ] `editor.convertTabsToSpaces` — Overrides the insertSpaces fallback.
  - [ ] Other parsed flat fields use the same names as the preferences below.
- [ ] CodeLens
  - [ ] `implementationsCodeLensEnabled` — boolean. Nested or fallback key `implementationsCodeLens.enabled`.
  - [ ] `implementationsCodeLensShowOnAllClassMethods` — boolean. Nested or fallback key `implementationsCodeLens.showOnAllClassMethods`.
  - [ ] `implementationsCodeLensShowOnInterfaceMethods` — boolean. Nested or fallback key `implementationsCodeLens.showOnInterfaceMethods`.
  - [ ] `referencesCodeLensEnabled` — boolean. Nested or fallback key `referencesCodeLens.enabled`.
  - [ ] `referencesCodeLensShowOnAllFunctions` — boolean. Nested or fallback key `referencesCodeLens.showOnAllFunctions`.
- [ ] Completions
  - [ ] `includeAutomaticOptionalChainCompletions` — boolean. Nested or fallback key `suggest.includeAutomaticOptionalChainCompletions`.
  - [ ] `includeCompletionsForImportStatements` — boolean. Nested or fallback key `suggest.includeCompletionsForImportStatements`.
  - [ ] `includeCompletionsForModuleExports` — boolean. Nested or fallback key `suggest.autoImports`.
  - [ ] `includeCompletionsWithClassMemberSnippets` — boolean. Nested or fallback key `suggest.classMemberSnippets.enabled`.
  - [ ] `includeCompletionsWithObjectLiteralMethodSnippets` — boolean. Nested or fallback key `suggest.objectLiteralMethodSnippets.enabled`.
  - [ ] `jsxAttributeCompletionStyle` — string with values `"auto"`, `"braces"`, `"none"`. Nested or fallback key `preferences.jsxAttributeCompletionStyle`.
- [ ] Editor formatting defaults
  - [ ] `baseIndentSize` — integer. Nested or fallback key `format.baseIndentSize`.
  - [ ] `convertTabsToSpaces` — boolean. Nested or fallback key `format.convertTabsToSpaces`.
  - [ ] `indentSize` — integer. Nested or fallback key `format.indentSize`.
  - [ ] `indentStyle` — string with values `"none"`, `"block"`, `"smart"`, also accepts a number. Nested or fallback key `format.indentStyle`.
  - [ ] `newLineCharacter` — string. Nested or fallback key `format.newLineCharacter`.
  - [ ] `tabSize` — integer. Nested or fallback key `format.tabSize`.
  - [ ] `trimTrailingWhitespace` — boolean. Nested or fallback key `format.trimTrailingWhitespace`.
- [ ] Formatting
  - [ ] `indentMultiLineObjectLiteralBeginningOnBlankLine` — boolean. Nested or fallback key `format.indentMultiLineObjectLiteralBeginningOnBlankLine`.
  - [ ] `indentSwitchCase` — boolean. Nested or fallback key `format.indentSwitchCase`.
  - [ ] `insertSpaceAfterCommaDelimiter` — boolean. Nested or fallback key `format.insertSpaceAfterCommaDelimiter`.
  - [ ] `insertSpaceAfterConstructor` — boolean. Nested or fallback key `format.insertSpaceAfterConstructor`.
  - [ ] `insertSpaceAfterFunctionKeywordForAnonymousFunctions` — boolean. Nested or fallback key `format.insertSpaceAfterFunctionKeywordForAnonymousFunctions`.
  - [ ] `insertSpaceAfterKeywordsInControlFlowStatements` — boolean. Nested or fallback key `format.insertSpaceAfterKeywordsInControlFlowStatements`.
  - [ ] `insertSpaceAfterOpeningAndBeforeClosingEmptyBraces` — boolean. Nested or fallback key `format.insertSpaceAfterOpeningAndBeforeClosingEmptyBraces`.
  - [ ] `insertSpaceAfterOpeningAndBeforeClosingJsxExpressionBraces` — boolean. Nested or fallback key `format.insertSpaceAfterOpeningAndBeforeClosingJsxExpressionBraces`.
  - [ ] `insertSpaceAfterOpeningAndBeforeClosingNonemptyBraces` — boolean. Nested or fallback key `format.insertSpaceAfterOpeningAndBeforeClosingNonemptyBraces`.
  - [ ] `insertSpaceAfterOpeningAndBeforeClosingNonemptyBrackets` — boolean. Nested or fallback key `format.insertSpaceAfterOpeningAndBeforeClosingNonemptyBrackets`.
  - [ ] `insertSpaceAfterOpeningAndBeforeClosingNonemptyParenthesis` — boolean. Nested or fallback key `format.insertSpaceAfterOpeningAndBeforeClosingNonemptyParenthesis`.
  - [ ] `insertSpaceAfterOpeningAndBeforeClosingTemplateStringBraces` — boolean. Nested or fallback key `format.insertSpaceAfterOpeningAndBeforeClosingTemplateStringBraces`.
  - [ ] `insertSpaceAfterSemicolonInForStatements` — boolean. Nested or fallback key `format.insertSpaceAfterSemicolonInForStatements`.
  - [ ] `insertSpaceAfterTypeAssertion` — boolean. Nested or fallback key `format.insertSpaceAfterTypeAssertion`.
  - [ ] `insertSpaceBeforeAndAfterBinaryOperators` — boolean. Nested or fallback key `format.insertSpaceBeforeAndAfterBinaryOperators`.
  - [ ] `insertSpaceBeforeFunctionParenthesis` — boolean. Nested or fallback key `format.insertSpaceBeforeFunctionParenthesis`.
  - [ ] `insertSpaceBeforeTypeAnnotation` — boolean. Nested or fallback key `format.insertSpaceBeforeTypeAnnotation`.
  - [ ] `placeOpenBraceOnNewLineForControlBlocks` — boolean. Nested or fallback key `format.placeOpenBraceOnNewLineForControlBlocks`.
  - [ ] `placeOpenBraceOnNewLineForFunctions` — boolean. Nested or fallback key `format.placeOpenBraceOnNewLineForFunctions`.
  - [ ] `semicolons` — string with values `"ignore"`, `"insert"`, `"remove"`. Nested or fallback key `format.semicolons`.
- [ ] Inlay hints
  - [ ] `includeInlayEnumMemberValueHints` — boolean. Nested or fallback key `inlayHints.enumMemberValues.enabled`.
  - [ ] `includeInlayFunctionLikeReturnTypeHints` — boolean. Nested or fallback key `inlayHints.functionLikeReturnTypes.enabled`.
  - [ ] `includeInlayFunctionParameterTypeHints` — boolean. Nested or fallback key `inlayHints.parameterTypes.enabled`.
  - [ ] `includeInlayParameterNameHints` — string with values `"none"`, `"literals"`, `"all"`. Nested or fallback key `inlayHints.parameterNames.enabled`.
  - [ ] `includeInlayParameterNameHintsWhenArgumentMatchesName` — boolean. Nested or fallback key `inlayHints.parameterNames.suppressWhenArgumentMatchesName` (inverted boolean).
  - [ ] `includeInlayPropertyDeclarationTypeHints` — boolean. Nested or fallback key `inlayHints.propertyDeclarationTypes.enabled`.
  - [ ] `includeInlayVariableTypeHints` — boolean. Nested or fallback key `inlayHints.variableTypes.enabled`.
  - [ ] `includeInlayVariableTypeHintsWhenTypeMatchesName` — boolean. Nested or fallback key `inlayHints.variableTypes.suppressWhenTypeMatchesName` (inverted boolean).
- [ ] Project and language preferences
  - [ ] `allowRenameOfImportPath` — boolean.
  - [ ] `allowTextChangesInNewFiles` — boolean.
  - [ ] `autoClosingTags` — boolean. Nested or fallback key `autoClosingTags.enabled`.
  - [ ] `autoImportEntrypointDirectorySearch` — boolean. Nested or fallback key `preferences.autoImportEntrypointDirectorySearch`.
  - [ ] `autoImportFileExcludePatterns` — array of string values. Nested or fallback key `preferences.autoImportFileExcludePatterns`.
  - [ ] `autoImportSpecifierExcludeRegexes` — array of string values. Nested or fallback key `preferences.autoImportSpecifierExcludeRegexes`.
  - [ ] `automaticTypeAcquisitionEnabled` — boolean. Nested or fallback key `tsserver.automaticTypeAcquisition.enabled`.
  - [ ] `completeJSDocs` — boolean. Nested or fallback keys `suggest.jsdoc.enabled`, `suggest.completeJSDocs`.
  - [ ] `customConfigFileName` — string.
  - [ ] `disableAutomaticTypeAcquisition` — boolean. The newer automaticTypeAcquisitionEnabled preference takes precedence.
  - [ ] `disableLineTextInReferences` — boolean.
  - [ ] `disableSuggestions` — boolean.
  - [ ] `displayPartsForJSDoc` — boolean.
  - [ ] `excludeLibrarySymbolsInNavTo` — boolean. Nested or fallback key `workspaceSymbols.excludeLibrarySymbols`.
  - [ ] `formatEnabled` — boolean. Nested or fallback keys `format.enabled`, `format.enable`.
  - [ ] `generateReturnInDocTemplate` — boolean. Nested or fallback key `suggest.jsdoc.generateReturns`.
  - [ ] `importModuleSpecifierEnding` — string with values `"auto"`, `"minimal"`, `"index"`, `"js"`. Nested or fallback key `preferences.importModuleSpecifierEnding`.
  - [ ] `importModuleSpecifierPreference` — string with values `"shortest"`, `"project-relative"`, `"relative"`, `"non-relative"`. Nested or fallback key `preferences.importModuleSpecifier`.
  - [ ] `lazyConfiguredProjectsFromExternalProject` — boolean.
  - [ ] `locale` — string.
  - [ ] `maximumHoverLength` — integer.
  - [ ] `organizeImportsAccentCollation` — boolean. Nested or fallback key `preferences.organizeImports.accentCollation`.
  - [ ] `organizeImportsCaseFirst` — string with values `"default"`, `"lower"`, `"upper"`. Nested or fallback key `preferences.organizeImports.caseFirst`.
  - [ ] `organizeImportsCollation` — string with values `"ordinal"`, `"unicode"`. Nested or fallback key `preferences.organizeImports.unicodeCollation`.
  - [ ] `organizeImportsIgnoreCase` — boolean, also accepts `caseInsensitive` and `caseSensitive` for the nested key. Nested or fallback key `preferences.organizeImports.caseSensitivity`.
  - [ ] `organizeImportsLocale` — string, accepted for compatibility. Nested or fallback key `preferences.organizeImports.locale`.
  - [ ] `organizeImportsNumericCollation` — boolean. Nested or fallback key `preferences.organizeImports.numericCollation`.
  - [ ] `organizeImportsSort` — string with values `"auto"`, `"ordinal"`, `"ordinalIgnoreCase"`, `"natural"`, `"naturalIgnoreCase"`. Nested or fallback key `preferences.organizeImports.sort`.
  - [ ] `organizeImportsTypeOrder` — string with values `"auto"`, `"last"`, `"inline"`, `"first"`. Nested or fallback key `preferences.organizeImports.typeOrder`.
  - [ ] `preferGoToSourceDefinition` — boolean.
  - [ ] `preferTypeOnlyAutoImports` — boolean. Nested or fallback key `preferences.preferTypeOnlyAutoImports`.
  - [ ] `providePrefixAndSuffixTextForRename` — boolean. Nested or fallback key `preferences.useAliasesForRenames`.
  - [ ] `provideRefactorNotApplicableReason` — boolean.
  - [ ] `quotePreference` — string with values `"auto"`, `"double"`, `"single"`. Nested or fallback key `preferences.quoteStyle`.
  - [ ] `reportStyleChecksAsWarnings` — boolean.
  - [ ] `validateEnabled` — boolean. Nested or fallback keys `validate.enabled`, `validate.enable`.
  - [ ] `workspaceSymbolsScope` — string with values `"allOpenProjects"`, `"currentProject"`. Nested or fallback key `workspaceSymbols.scope`.

## Shared wire shapes

These definitions give the JSON field names and types used by the method checklists. A **variant** describes the JSON value at that position. It adds a discriminator field only when that field appears inside the variant’s object. [Generated wire types][protocol]

### `AddAsTypeOnly`

- [ ] Wire type — integer.
  - [ ] `1` — Allowed.
  - [ ] `2` — Required.
  - [ ] `4` — NotAllowed.

### `AnnotatedTextEdit`

- [ ] `range` ([`Range`](#range), required)
- [ ] `newText` (string, required)
- [ ] `annotationId` (string, required)

### `ApplyKind`

- [ ] Wire type — unsigned integer.
  - [ ] `1` — Replace.
  - [ ] `2` — Merge.

### `AutoImportFix`

- [ ] `kind` ([`AutoImportFixKind`](#autoimportfixkind), optional, nullable)
- [ ] `name` (string, optional, nullable)
- [ ] `importKind` ([`ImportKind`](#importkind), required)
- [ ] `useRequire` (boolean, optional, nullable)
- [ ] `addAsTypeOnly` ([`AddAsTypeOnly`](#addastypeonly), required)
- [ ] `moduleSpecifier` (string, optional, nullable)
- [ ] `importIndex` (integer, required)
- [ ] `usagePosition` ([`Position`](#position), optional)
- [ ] `namespacePrefix` (string, optional, nullable)

### `AutoImportFixKind`

- [ ] Wire type — integer.
  - [ ] `0` — UseNamespace.
  - [ ] `1` — JsdocTypeImport.
  - [ ] `2` — AddToExisting.
  - [ ] `3` — AddNew.
  - [ ] `4` — PromoteTypeOnly.

### `BooleanOrEmptyObject`

- [ ] Variant — boolean
- [ ] Variant — empty object

### `BooleanOrSemanticTokensFullDelta`

- [ ] Variant — boolean
- [ ] Variant — [`SemanticTokensFullDelta`](#semantictokensfulldelta)

### `CallHierarchyIncomingCall`

- [ ] `from` ([`CallHierarchyItem`](#callhierarchyitem), required)
- [ ] `fromRanges` (array of [`Range`](#range), required)

### `CallHierarchyIncomingCallsOrNull`

- [ ] Variant — array of [`CallHierarchyIncomingCall`](#callhierarchyincomingcall)
- [ ] Variant — null

### `CallHierarchyIncomingCallsParams`

- [ ] `workDoneToken` ([`IntegerOrString`](#integerorstring), optional)
- [ ] `partialResultToken` ([`IntegerOrString`](#integerorstring), optional)
- [ ] `item` ([`CallHierarchyItem`](#callhierarchyitem), required)

### `CallHierarchyItem`

- [ ] `name` (string, required)
- [ ] `kind` ([`SymbolKind`](#symbolkind), required)
- [ ] `tags` (array of [`SymbolTag`](#symboltag), optional)
- [ ] `detail` (string, optional)
- [ ] `uri` ([`DocumentUri`](#documenturi), required)
- [ ] `range` ([`Range`](#range), required)
- [ ] `selectionRange` ([`Range`](#range), required)
- [ ] `data` ([`CallHierarchyItemData`](#callhierarchyitemdata), optional)

### `CallHierarchyItemData`

- [ ] Empty object.

### `CallHierarchyItemsOrNull`

- [ ] Variant — array of [`CallHierarchyItem`](#callhierarchyitem)
- [ ] Variant — null

### `CallHierarchyOutgoingCall`

- [ ] `to` ([`CallHierarchyItem`](#callhierarchyitem), required)
- [ ] `fromRanges` (array of [`Range`](#range), required)

### `CallHierarchyOutgoingCallsOrNull`

- [ ] Variant — array of [`CallHierarchyOutgoingCall`](#callhierarchyoutgoingcall)
- [ ] Variant — null

### `CallHierarchyOutgoingCallsParams`

- [ ] `workDoneToken` ([`IntegerOrString`](#integerorstring), optional)
- [ ] `partialResultToken` ([`IntegerOrString`](#integerorstring), optional)
- [ ] `item` ([`CallHierarchyItem`](#callhierarchyitem), required)

### `CallHierarchyPrepareParams`

- [ ] `textDocument` ([`TextDocumentIdentifier`](#textdocumentidentifier), required)
- [ ] `position` ([`Position`](#position), required)
- [ ] `workDoneToken` ([`IntegerOrString`](#integerorstring), optional)

### `CallHierarchyRegistrationOptions`

- [ ] `documentSelector` ([`DocumentSelectorOrNull`](#documentselectorornull) (or null), required)
- [ ] `workDoneProgress` (boolean, optional)
- [ ] `id` (string, optional)

### `CancelParams`

- [ ] `id` ([`IntegerOrString`](#integerorstring), required)

### `ChangeAnnotation`

- [ ] `label` (string, required)
- [ ] `needsConfirmation` (boolean, optional)
- [ ] `description` (string, optional)

### `ClientCapabilities`

- [ ] [Fields used by TypeScript](#client-capability-negotiation)

### `ClientInfo`

- [ ] `name` (string, required)
- [ ] `version` (string, optional)

### `CodeAction`

- [ ] `title` (string, required)
- [ ] `kind` ([`CodeActionKind`](#codeactionkind), optional)
- [ ] `diagnostics` (array of [`Diagnostic`](#diagnostic), optional)
- [ ] `isPreferred` (boolean, optional)
- [ ] `disabled` ([`CodeActionDisabled`](#codeactiondisabled), optional)
- [ ] `edit` ([`WorkspaceEdit`](#workspaceedit), optional)
- [ ] `command` ([`Command`](#command), optional)
- [ ] `data` ([`CodeActionData`](#codeactiondata), optional)
- [ ] `tags` (array of [`CodeActionTag`](#codeactiontag), optional)

### `CodeActionContext`

- [ ] `diagnostics` (array of [`Diagnostic`](#diagnostic), required)
- [ ] `only` (array of [`CodeActionKind`](#codeactionkind), optional)
- [ ] `triggerKind` ([`CodeActionTriggerKind`](#codeactiontriggerkind), optional)

### `CodeActionData`

- [ ] Empty object.

### `CodeActionDisabled`

- [ ] `reason` (string, required)

### `CodeActionKind`

- [ ] Wire type — string.
  - [ ] `""` — Empty.
  - [ ] `"quickfix"` — QuickFix.
  - [ ] `"refactor"` — Refactor.
  - [ ] `"refactor.extract"` — RefactorExtract.
  - [ ] `"refactor.inline"` — RefactorInline.
  - [ ] `"refactor.move"` — RefactorMove.
  - [ ] `"refactor.rewrite"` — RefactorRewrite.
  - [ ] `"source"` — Source.
  - [ ] `"source.organizeImports"` — SourceOrganizeImports.
  - [ ] `"source.fixAll"` — SourceFixAll.
  - [ ] `"source.fixAll.ts"` — SourceFixAllTs.
  - [ ] `"source.organizeImports.ts"` — SourceOrganizeImportsTs.
  - [ ] `"source.removeUnusedImports.ts"` — SourceRemoveUnusedImportsTs.
  - [ ] `"source.sortImports.ts"` — SourceSortImportsTs.

### `CodeActionKindDocumentation`

- [ ] `kind` ([`CodeActionKind`](#codeactionkind), required)
- [ ] `command` ([`Command`](#command), required)

### `CodeActionParams`

- [ ] `workDoneToken` ([`IntegerOrString`](#integerorstring), optional)
- [ ] `partialResultToken` ([`IntegerOrString`](#integerorstring), optional)
- [ ] `textDocument` ([`TextDocumentIdentifier`](#textdocumentidentifier), required)
- [ ] `range` ([`Range`](#range), required)
- [ ] `context` ([`CodeActionContext`](#codeactioncontext), required)

### `CodeActionRegistrationOptions`

- [ ] `documentSelector` ([`DocumentSelectorOrNull`](#documentselectorornull) (or null), required)
- [ ] `workDoneProgress` (boolean, optional)
- [ ] `codeActionKinds` (array of [`CodeActionKind`](#codeactionkind), optional)
- [ ] `documentation` (array of [`CodeActionKindDocumentation`](#codeactionkinddocumentation), optional)
- [ ] `resolveProvider` (boolean, optional)

### `CodeActionTag`

- [ ] Wire type — unsigned integer.
  - [ ] `1` — LLMGenerated.

### `CodeActionTriggerKind`

- [ ] Wire type — unsigned integer.
  - [ ] `1` — Invoked.
  - [ ] `2` — Automatic.

### `CodeDescription`

- [ ] `href` ([`URI`](#uri), required)

### `CodeLens`

- [ ] `range` ([`Range`](#range), required)
- [ ] `command` ([`Command`](#command), optional)
- [ ] `data` ([`CodeLensData`](#codelensdata), optional)

### `CodeLensData`

- [ ] `kind` ([`CodeLensKind`](#codelenskind), required)
- [ ] `uri` ([`DocumentUri`](#documenturi), required)
- [ ] `position` (integer, required)
- [ ] `supplementalFileIndex` (integer, optional)

### `CodeLensesOrNull`

- [ ] Variant — array of [`CodeLens`](#codelens)
- [ ] Variant — null

### `CodeLensKind`

- [ ] Wire type — string.
  - [ ] `"references"` — References.
  - [ ] `"implementations"` — Implementations.

### `CodeLensParams`

- [ ] `workDoneToken` ([`IntegerOrString`](#integerorstring), optional)
- [ ] `partialResultToken` ([`IntegerOrString`](#integerorstring), optional)
- [ ] `textDocument` ([`TextDocumentIdentifier`](#textdocumentidentifier), required)

### `CodeLensRegistrationOptions`

- [ ] `documentSelector` ([`DocumentSelectorOrNull`](#documentselectorornull) (or null), required)
- [ ] `workDoneProgress` (boolean, optional)
- [ ] `resolveProvider` (boolean, optional)

### `Command`

- [ ] `title` (string, required)
- [ ] `tooltip` (string, optional)
- [ ] `command` (string, required)
- [ ] `arguments` (array of JSON value, optional)

### `CommandOrCodeAction`

- [ ] Variant — [`Command`](#command)
- [ ] Variant — [`CodeAction`](#codeaction)

### `CommandOrCodeActionArrayOrNull`

- [ ] Variant — array of [`CommandOrCodeAction`](#commandorcodeaction)
- [ ] Variant — null

### `CompletionContext`

- [ ] `triggerKind` ([`CompletionTriggerKind`](#completiontriggerkind), required)
- [ ] `triggerCharacter` (string, optional)

### `CompletionItem`

- [ ] `label` (string, required)
- [ ] `labelDetails` ([`CompletionItemLabelDetails`](#completionitemlabeldetails), optional)
- [ ] `kind` ([`CompletionItemKind`](#completionitemkind), optional)
- [ ] `tags` (array of [`CompletionItemTag`](#completionitemtag), optional)
- [ ] `detail` (string, optional)
- [ ] `documentation` ([`StringOrMarkupContent`](#stringormarkupcontent), optional)
- [ ] `deprecated` (boolean, optional)
- [ ] `preselect` (boolean, optional)
- [ ] `sortText` (string, optional)
- [ ] `filterText` (string, optional)
- [ ] `insertText` (string, optional)
- [ ] `insertTextFormat` ([`InsertTextFormat`](#inserttextformat), optional)
- [ ] `insertTextMode` ([`InsertTextMode`](#inserttextmode), optional)
- [ ] `textEdit` ([`TextEditOrInsertReplaceEdit`](#texteditorinsertreplaceedit), optional)
- [ ] `textEditText` (string, optional)
- [ ] `additionalTextEdits` (array of [`TextEdit`](#textedit), optional)
- [ ] `commitCharacters` (array of string, optional)
- [ ] `command` ([`Command`](#command), optional)
- [ ] `data` ([`CompletionItemData`](#completionitemdata), optional)

### `CompletionItemApplyKinds`

- [ ] `commitCharacters` ([`ApplyKind`](#applykind), optional)
- [ ] `data` ([`ApplyKind`](#applykind), optional)

### `CompletionItemData`

- [ ] `fileName` (string, optional, nullable)
- [ ] `position` (integer, optional, nullable)
- [ ] `supplementalFileIndex` (integer, optional)
- [ ] `source` (string, optional, nullable)
- [ ] `name` (string, optional, nullable)
- [ ] `autoImport` ([`AutoImportFix`](#autoimportfix), optional)
- [ ] `isImportStatementCompletion` (boolean, optional, nullable)

### `CompletionItemDefaults`

- [ ] `commitCharacters` (array of string, optional)
- [ ] `editRange` ([`RangeOrEditRangeWithInsertReplace`](#rangeoreditrangewithinsertreplace), optional)
- [ ] `insertTextFormat` ([`InsertTextFormat`](#inserttextformat), optional)
- [ ] `insertTextMode` ([`InsertTextMode`](#inserttextmode), optional)
- [ ] `data` ([`CompletionItemDefaultsData`](#completionitemdefaultsdata), optional)

### `CompletionItemDefaultsData`

- [ ] Empty object.

### `CompletionItemKind`

- [ ] Wire type — unsigned integer.
  - [ ] `1` — Text.
  - [ ] `2` — Method.
  - [ ] `3` — Function.
  - [ ] `4` — Constructor.
  - [ ] `5` — Field.
  - [ ] `6` — Variable.
  - [ ] `7` — Class.
  - [ ] `8` — Interface.
  - [ ] `9` — Module.
  - [ ] `10` — Property.
  - [ ] `11` — Unit.
  - [ ] `12` — Value.
  - [ ] `13` — Enum.
  - [ ] `14` — Keyword.
  - [ ] `15` — Snippet.
  - [ ] `16` — Color.
  - [ ] `17` — File.
  - [ ] `18` — Reference.
  - [ ] `19` — Folder.
  - [ ] `20` — EnumMember.
  - [ ] `21` — Constant.
  - [ ] `22` — Struct.
  - [ ] `23` — Event.
  - [ ] `24` — Operator.
  - [ ] `25` — TypeParameter.

### `CompletionItemLabelDetails`

- [ ] `detail` (string, optional)
- [ ] `description` (string, optional)

### `CompletionItemsOrListOrNull`

- [ ] Variant — array of [`CompletionItem`](#completionitem)
- [ ] Variant — [`CompletionList`](#completionlist)
- [ ] Variant — null

### `CompletionItemTag`

- [ ] Wire type — unsigned integer.
  - [ ] `1` — Deprecated.

### `CompletionList`

- [ ] `isIncomplete` (boolean, required)
- [ ] `itemDefaults` ([`CompletionItemDefaults`](#completionitemdefaults), optional)
- [ ] `applyKind` ([`CompletionItemApplyKinds`](#completionitemapplykinds), optional) — Schema only for the TypeScript completion list.
- [ ] `items` (array of [`CompletionItem`](#completionitem), required)

### `CompletionParams`

- [ ] `textDocument` ([`TextDocumentIdentifier`](#textdocumentidentifier), required)
- [ ] `position` ([`Position`](#position), required)
- [ ] `workDoneToken` ([`IntegerOrString`](#integerorstring), optional)
- [ ] `partialResultToken` ([`IntegerOrString`](#integerorstring), optional)
- [ ] `context` ([`CompletionContext`](#completioncontext), optional)

### `CompletionRegistrationOptions`

- [ ] `documentSelector` ([`DocumentSelectorOrNull`](#documentselectorornull) (or null), required)
- [ ] `workDoneProgress` (boolean, optional)
- [ ] `triggerCharacters` (array of string, optional)
- [ ] `allCommitCharacters` (array of string, optional)
- [ ] `resolveProvider` (boolean, optional)
- [ ] `completionItem` ([`ServerCompletionItemOptions`](#servercompletionitemoptions), optional)

### `CompletionTriggerKind`

- [ ] Wire type — unsigned integer.
  - [ ] `1` — Invoked.
  - [ ] `2` — TriggerCharacter.
  - [ ] `3` — TriggerForIncompleteCompletions.

### `ConfigurationItem`

- [ ] `scopeUri` ([`URI`](#uri), optional)
- [ ] `section` (string, optional)

### `ConfigurationParams`

- [ ] `items` (array of [`ConfigurationItem`](#configurationitem), required)

### `ContentMapperContribution`

- [ ] `contributorId` (string, required)
- [ ] `extensions` (array of string, optional)
- [ ] `inferredProjectContribution` ([`InferredProjectContentMapperContribution`](#inferredprojectcontentmappercontribution), optional)

### `ContentMapperManifest`

- [ ] `name` (string, required)
- [ ] `version` (string, optional)
- [ ] `exec` (array of string, required)
- [ ] `cwd` (string, optional)
- [ ] `compilerOptions` (array of string, optional)
- [ ] `dynamicConfig` (boolean, optional)
- [ ] `outputExtensions` (object keyed by string with values of string, optional)
- [ ] `extensions` (array of string, optional)

### `CreateFile`

- [ ] `kind` (`"create"`, required)
- [ ] `annotationId` (string, optional)
- [ ] `uri` ([`DocumentUri`](#documenturi), required)
- [ ] `options` ([`CreateFileOptions`](#createfileoptions), optional)

### `CreateFileOptions`

- [ ] `overwrite` (boolean, optional)
- [ ] `ignoreIfExists` (boolean, optional)

### `DefinitionParams`

- [ ] `textDocument` ([`TextDocumentIdentifier`](#textdocumentidentifier), required)
- [ ] `position` ([`Position`](#position), required)
- [ ] `workDoneToken` ([`IntegerOrString`](#integerorstring), optional)
- [ ] `partialResultToken` ([`IntegerOrString`](#integerorstring), optional) — The implementation returns results in the final response.

### `DefinitionRegistrationOptions`

- [ ] `documentSelector` ([`DocumentSelectorOrNull`](#documentselectorornull) (or null), required)
- [ ] `workDoneProgress` (boolean, optional)

### `DeleteFile`

- [ ] `kind` (`"delete"`, required)
- [ ] `annotationId` (string, optional)
- [ ] `uri` ([`DocumentUri`](#documenturi), required)
- [ ] `options` ([`DeleteFileOptions`](#deletefileoptions), optional)

### `DeleteFileOptions`

- [ ] `recursive` (boolean, optional)
- [ ] `ignoreIfNotExists` (boolean, optional)

### `Diagnostic`

- [ ] `range` ([`Range`](#range), required)
- [ ] `severity` ([`DiagnosticSeverity`](#diagnosticseverity), optional)
- [ ] `code` ([`IntegerOrString`](#integerorstring), optional)
- [ ] `codeDescription` ([`CodeDescription`](#codedescription), optional)
- [ ] `source` (string, optional)
- [ ] `message` ([`StringOrMarkupContent`](#stringormarkupcontent), required)
- [ ] `tags` (array of [`DiagnosticTag`](#diagnostictag), optional)
- [ ] `relatedInformation` (array of [`DiagnosticRelatedInformation`](#diagnosticrelatedinformation), optional)
- [ ] `data` ([`DiagnosticData`](#diagnosticdata), optional)

### `DiagnosticData`

- [ ] Empty object.

### `DiagnosticFlakeLogLevel`

- [ ] Wire type — integer.
  - [ ] `0` — Off.
  - [ ] `1` — Log.
  - [ ] `2` — Panic.

### `DiagnosticRegistrationOptions`

- [ ] `documentSelector` ([`DocumentSelectorOrNull`](#documentselectorornull) (or null), required)
- [ ] `workDoneProgress` (boolean, optional)
- [ ] `identifier` (string, optional)
- [ ] `interFileDependencies` (boolean, required)
- [ ] `workspaceDiagnostics` (boolean, required)
- [ ] `id` (string, optional)

### `DiagnosticRelatedInformation`

- [ ] `location` ([`Location`](#location), required)
- [ ] `message` (string, required)

### `DiagnosticSeverity`

- [ ] Wire type — unsigned integer.
  - [ ] `1` — Error.
  - [ ] `2` — Warning.
  - [ ] `3` — Information.
  - [ ] `4` — Hint.

### `DiagnosticTag`

- [ ] Wire type — unsigned integer.
  - [ ] `1` — Unnecessary.
  - [ ] `2` — Deprecated.

### `DidChangeConfigurationParams`

- [ ] `settings` (JSON value, required) — An object is parsed as workspace preferences by section. Null leaves the settings unchanged.

### `DidChangeConfigurationRegistrationOptions`

- [ ] `section` ([`StringOrStrings`](#stringorstrings), optional)

### `DidChangeTextDocumentParams`

- [ ] `textDocument` ([`VersionedTextDocumentIdentifier`](#versionedtextdocumentidentifier), required)
- [ ] `contentChanges` (array of [`TextDocumentContentChangePartialOrWholeDocument`](#textdocumentcontentchangepartialorwholedocument), required)

### `DidChangeWatchedFilesParams`

- [ ] `changes` (array of [`FileEvent`](#fileevent), required)

### `DidChangeWatchedFilesRegistrationOptions`

- [ ] `watchers` (array of [`FileSystemWatcher`](#filesystemwatcher), required)

### `DidCloseTextDocumentParams`

- [ ] `textDocument` ([`TextDocumentIdentifier`](#textdocumentidentifier), required)

### `DidOpenTextDocumentParams`

- [ ] `textDocument` ([`TextDocumentItem`](#textdocumentitem), required)

### `DidSaveTextDocumentParams`

- [ ] `textDocument` ([`TextDocumentIdentifier`](#textdocumentidentifier), required)
- [ ] `text` (string, optional) — Schema only. The save handler uses the document URI.

### `DocumentDiagnosticParams`

- [ ] `workDoneToken` ([`IntegerOrString`](#integerorstring), optional)
- [ ] `partialResultToken` ([`IntegerOrString`](#integerorstring), optional)
- [ ] `textDocument` ([`TextDocumentIdentifier`](#textdocumentidentifier), required)
- [ ] `identifier` (string, optional) — The server advertises `typescript` as its diagnostic identifier.
- [ ] `previousResultId` (string, optional) — The handler returns a full report.

### `DocumentFormattingParams`

- [ ] `workDoneToken` ([`IntegerOrString`](#integerorstring), optional)
- [ ] `textDocument` ([`TextDocumentIdentifier`](#textdocumentidentifier), required)
- [ ] `options` ([`FormattingOptions`](#formattingoptions), required)

### `DocumentFormattingRegistrationOptions`

- [ ] `documentSelector` ([`DocumentSelectorOrNull`](#documentselectorornull) (or null), required)
- [ ] `workDoneProgress` (boolean, optional)

### `DocumentHighlight`

- [ ] `range` ([`Range`](#range), required)
- [ ] `kind` ([`DocumentHighlightKind`](#documenthighlightkind), optional)

### `DocumentHighlightKind`

- [ ] Wire type — unsigned integer.
  - [ ] `1` — Text.
  - [ ] `2` — Read.
  - [ ] `3` — Write.

### `DocumentHighlightParams`

- [ ] `textDocument` ([`TextDocumentIdentifier`](#textdocumentidentifier), required)
- [ ] `position` ([`Position`](#position), required)
- [ ] `workDoneToken` ([`IntegerOrString`](#integerorstring), optional)
- [ ] `partialResultToken` ([`IntegerOrString`](#integerorstring), optional)

### `DocumentHighlightRegistrationOptions`

- [ ] `documentSelector` ([`DocumentSelectorOrNull`](#documentselectorornull) (or null), required)
- [ ] `workDoneProgress` (boolean, optional)

### `DocumentHighlightsOrNull`

- [ ] Variant — array of [`DocumentHighlight`](#documenthighlight)
- [ ] Variant — null

### `DocumentOnTypeFormattingParams`

- [ ] `textDocument` ([`TextDocumentIdentifier`](#textdocumentidentifier), required)
- [ ] `position` ([`Position`](#position), required)
- [ ] `ch` (string, required)
- [ ] `options` ([`FormattingOptions`](#formattingoptions), required)

### `DocumentOnTypeFormattingRegistrationOptions`

- [ ] `documentSelector` ([`DocumentSelectorOrNull`](#documentselectorornull) (or null), required)
- [ ] `firstTriggerCharacter` (string, required)
- [ ] `moreTriggerCharacter` (array of string, optional)

### `DocumentRangeFormattingParams`

- [ ] `workDoneToken` ([`IntegerOrString`](#integerorstring), optional)
- [ ] `textDocument` ([`TextDocumentIdentifier`](#textdocumentidentifier), required)
- [ ] `range` ([`Range`](#range), required)
- [ ] `options` ([`FormattingOptions`](#formattingoptions), required)

### `DocumentRangeFormattingRegistrationOptions`

- [ ] `documentSelector` ([`DocumentSelectorOrNull`](#documentselectorornull) (or null), required)
- [ ] `workDoneProgress` (boolean, optional)
- [ ] `rangesSupport` (boolean, optional)

### `DocumentSelectorOrNull`

- [ ] Variant — array of [`TextDocumentFilterLanguageOrSchemeOrPattern`](#textdocumentfilterlanguageorschemeorpattern)
- [ ] Variant — null

### `DocumentSymbol`

- [ ] `name` (string, required)
- [ ] `detail` (string, optional)
- [ ] `kind` ([`SymbolKind`](#symbolkind), required)
- [ ] `tags` (array of [`SymbolTag`](#symboltag), optional)
- [ ] `deprecated` (boolean, optional)
- [ ] `range` ([`Range`](#range), required)
- [ ] `selectionRange` ([`Range`](#range), required)
- [ ] `children` (array of [`DocumentSymbol`](#documentsymbol), optional)

### `DocumentSymbolParams`

- [ ] `workDoneToken` ([`IntegerOrString`](#integerorstring), optional)
- [ ] `partialResultToken` ([`IntegerOrString`](#integerorstring), optional)
- [ ] `textDocument` ([`TextDocumentIdentifier`](#textdocumentidentifier), required)

### `DocumentSymbolRegistrationOptions`

- [ ] `documentSelector` ([`DocumentSelectorOrNull`](#documentselectorornull) (or null), required)
- [ ] `workDoneProgress` (boolean, optional)
- [ ] `label` (string, optional)

### `DocumentUri`

- [ ] URI string.

### `DocumentUriOrNull`

- [ ] Variant — [`DocumentUri`](#documenturi)
- [ ] Variant — null

### `EditRangeWithInsertReplace`

- [ ] `insert` ([`Range`](#range), required)
- [ ] `replace` ([`Range`](#range), required)

### `FileChangeType`

- [ ] Wire type — unsigned integer.
  - [ ] `1` — Created.
  - [ ] `2` — Changed.
  - [ ] `3` — Deleted.

### `FileEvent`

- [ ] `uri` ([`DocumentUri`](#documenturi), required)
- [ ] `type` ([`FileChangeType`](#filechangetype), required)

### `FileOperationFilter`

- [ ] `scheme` (string, optional)
- [ ] `pattern` ([`FileOperationPattern`](#fileoperationpattern), required)

### `FileOperationPattern`

- [ ] `glob` (string, required)
- [ ] `matches` ([`FileOperationPatternKind`](#fileoperationpatternkind), optional)
- [ ] `options` ([`FileOperationPatternOptions`](#fileoperationpatternoptions), optional)

### `FileOperationPatternKind`

- [ ] Wire type — string.
  - [ ] `"file"` — File.
  - [ ] `"folder"` — Folder.

### `FileOperationPatternOptions`

- [ ] `ignoreCase` (boolean, optional)

### `FileOperationRegistrationOptions`

- [ ] `filters` (array of [`FileOperationFilter`](#fileoperationfilter), required)

### `FileRename`

- [ ] `oldUri` ([`DocumentUri`](#documenturi), required)
- [ ] `newUri` ([`DocumentUri`](#documenturi), required)

### `FileSystemWatcher`

- [ ] `globPattern` ([`PatternOrRelativePattern`](#patternorrelativepattern), required)
- [ ] `kind` ([`WatchKind`](#watchkind), optional)

### `FoldingRange`

- [ ] `startLine` (unsigned integer, required)
- [ ] `startCharacter` (unsigned integer, optional)
- [ ] `endLine` (unsigned integer, required)
- [ ] `endCharacter` (unsigned integer, optional)
- [ ] `kind` ([`FoldingRangeKind`](#foldingrangekind), optional)
- [ ] `collapsedText` (string, optional)

### `FoldingRangeKind`

- [ ] Wire type — string.
  - [ ] `"comment"` — Comment.
  - [ ] `"imports"` — Imports.
  - [ ] `"region"` — Region.

### `FoldingRangeParams`

- [ ] `workDoneToken` ([`IntegerOrString`](#integerorstring), optional)
- [ ] `partialResultToken` ([`IntegerOrString`](#integerorstring), optional)
- [ ] `textDocument` ([`TextDocumentIdentifier`](#textdocumentidentifier), required)

### `FoldingRangeRegistrationOptions`

- [ ] `documentSelector` ([`DocumentSelectorOrNull`](#documentselectorornull) (or null), required)
- [ ] `workDoneProgress` (boolean, optional)
- [ ] `id` (string, optional)

### `FoldingRangesOrNull`

- [ ] Variant — array of [`FoldingRange`](#foldingrange)
- [ ] Variant — null

### `FormattingOptions`

- [ ] `tabSize` (unsigned integer, required) — Overrides both tab size and indent size for the request.
- [ ] `insertSpaces` (boolean, required) — Overrides the tabs-versus-spaces preference.
- [ ] `trimTrailingWhitespace` (boolean, optional) — Overrides the preference when present.
- [ ] `insertFinalNewline` (boolean, optional) — Schema only in the request format option conversion.
- [ ] `trimFinalNewlines` (boolean, optional) — Schema only in the request format option conversion.

### `FullDocumentDiagnosticReport`

- [ ] `kind` (`"full"`, required)
- [ ] `resultId` (string, optional)
- [ ] `items` (array of [`Diagnostic`](#diagnostic), required)

### `FullDocumentDiagnosticReportOrUnchangedDocumentDiagnosticReport`

- [ ] Variant — [`FullDocumentDiagnosticReport`](#fulldocumentdiagnosticreport)
- [ ] Variant — [`UnchangedDocumentDiagnosticReport`](#unchangeddocumentdiagnosticreport)

### `Hover`

- [ ] `contents` ([`MarkupContentOrStringOrMarkedStringWithLanguageOrMarkedStrings`](#markupcontentorstringormarkedstringwithlanguageormarkedstrings), required)
- [ ] `range` ([`Range`](#range), optional)
- [ ] `canIncreaseVerbosity` (boolean, optional, nullable)
- [ ] `_vs_rawContent` ([`VSContainerElement`](#vscontainerelement), optional)

### `HoverOrNull`

- [ ] Variant — [`Hover`](#hover)
- [ ] Variant — null

### `HoverParams`

- [ ] `textDocument` ([`TextDocumentIdentifier`](#textdocumentidentifier), required)
- [ ] `position` ([`Position`](#position), required)
- [ ] `workDoneToken` ([`IntegerOrString`](#integerorstring), optional)
- [ ] `verbosityLevel` (integer, optional)

### `HoverRegistrationOptions`

- [ ] `documentSelector` ([`DocumentSelectorOrNull`](#documentselectorornull) (or null), required)
- [ ] `workDoneProgress` (boolean, optional)

### `ImplementationParams`

- [ ] `textDocument` ([`TextDocumentIdentifier`](#textdocumentidentifier), required)
- [ ] `position` ([`Position`](#position), required)
- [ ] `workDoneToken` ([`IntegerOrString`](#integerorstring), optional)
- [ ] `partialResultToken` ([`IntegerOrString`](#integerorstring), optional)

### `ImplementationRegistrationOptions`

- [ ] `documentSelector` ([`DocumentSelectorOrNull`](#documentselectorornull) (or null), required)
- [ ] `workDoneProgress` (boolean, optional)
- [ ] `id` (string, optional)

### `ImportKind`

- [ ] Wire type — integer.
  - [ ] `0` — Named.
  - [ ] `1` — Default.
  - [ ] `2` — Namespace.
  - [ ] `3` — CommonJS.

### `InferredProjectContentMapperContribution`

- [ ] `options` (object keyed by string with values of JSON value, optional)
- [ ] `manifest` ([`ContentMapperManifest`](#contentmappermanifest), required)

### `InitializationOptions`

- [ ] `disablePushDiagnostics` (boolean, optional) — Disables push publication of project diagnostics.
- [ ] `codeLensShowLocationsCommandName` (string, optional) — Names the client command used by resolved references and implementations lenses.
- [ ] `userPreferences` (JSON value, optional, nullable) — Fallback configuration object when the client does not support workspace/configuration. See [workspace preferences](#workspace-preferences).
- [ ] `enableTelemetry` (boolean, optional) — Enables outbound telemetry.
- [ ] `logVerbosity` ([`LogVerbosity`](#logverbosity), optional) — Sets server log verbosity at startup.
- [ ] `runExternalCode` (boolean, optional) — Allows external content mapper processes on main.
- [ ] `trackFlakyDiagnostics` ([`DiagnosticFlakeLogLevel`](#diagnosticflakeloglevel), optional) — Enables comparison of diagnostics around emit for debugging on main.

### `InitializationOptionsOrNull`

- [ ] Variant — [`InitializationOptions`](#initializationoptions)
- [ ] Variant — null

### `InitializeAPISessionParams`

- [ ] `pipe` (string, optional) — The server creates a private pipe when the client leaves this empty.

### `InitializeAPISessionResult`

- [ ] `sessionId` (string, required)
- [ ] `pipe` (string, required)

### `InitializedParams`

- [ ] Empty object.

### `InitializeParams`

- [ ] `workDoneToken` ([`IntegerOrString`](#integerorstring), optional) — TypeScript project loading uses a server-created token.
- [ ] `processId` ([`IntegerOrNull`](#integerornull) (or null), required) — Starts the parent-process watchdog when the runtime provides one.
- [ ] `clientInfo` ([`ClientInfo`](#clientinfo), optional)
- [ ] `locale` (string, optional) — Sets the initial locale and the fallback for the auto preference.
- [ ] `rootPath` ([`StringOrNull`](#stringornull) (or null), optional, nullable)
- [ ] `rootUri` ([`DocumentUriOrNull`](#documenturiornull) (or null), required)
- [ ] `capabilities` ([`ClientCapabilities`](#clientcapabilities), required) — See the negotiated capability checklist below.
- [ ] `initializationOptions` ([`InitializationOptionsOrNull`](#initializationoptionsornull) (or null), optional, nullable) — See the individual startup options below.
- [ ] `trace` ([`TraceValue`](#tracevalue), optional) — Accepted at startup. Runtime server logging uses the TypeScript verbosity option.
- [ ] `workspaceFolders` ([`WorkspaceFoldersOrNull`](#workspacefoldersornull) (or null), optional, nullable)

### `InitializeResult`

- [ ] `capabilities` ([`ServerCapabilities`](#servercapabilities), required) — See the advertised capability checklist below.
- [ ] `serverInfo` ([`ServerInfo`](#serverinfo), optional)

### `InlayHint`

- [ ] `position` ([`Position`](#position), required)
- [ ] `label` ([`StringOrInlayHintLabelParts`](#stringorinlayhintlabelparts), required)
- [ ] `kind` ([`InlayHintKind`](#inlayhintkind), optional)
- [ ] `textEdits` (array of [`TextEdit`](#textedit), optional)
- [ ] `tooltip` ([`StringOrMarkupContent`](#stringormarkupcontent), optional)
- [ ] `paddingLeft` (boolean, optional)
- [ ] `paddingRight` (boolean, optional)
- [ ] `data` ([`InlayHintData`](#inlayhintdata), optional)

### `InlayHintData`

- [ ] Empty object.

### `InlayHintKind`

- [ ] Wire type — unsigned integer.
  - [ ] `1` — Type.
  - [ ] `2` — Parameter.

### `InlayHintLabelPart`

- [ ] `value` (string, required)
- [ ] `tooltip` ([`StringOrMarkupContent`](#stringormarkupcontent), optional)
- [ ] `location` ([`Location`](#location), optional)
- [ ] `command` ([`Command`](#command), optional)

### `InlayHintParams`

- [ ] `workDoneToken` ([`IntegerOrString`](#integerorstring), optional)
- [ ] `partialResultToken` ([`IntegerOrString`](#integerorstring), optional)
- [ ] `textDocument` ([`TextDocumentIdentifier`](#textdocumentidentifier), required)
- [ ] `range` ([`Range`](#range), required)

### `InlayHintRegistrationOptions`

- [ ] `workDoneProgress` (boolean, optional)
- [ ] `resolveProvider` (boolean, optional)
- [ ] `documentSelector` ([`DocumentSelectorOrNull`](#documentselectorornull) (or null), required)
- [ ] `id` (string, optional)

### `InlayHintsOrNull`

- [ ] Variant — array of [`InlayHint`](#inlayhint)
- [ ] Variant — null

### `InsertReplaceEdit`

- [ ] `newText` (string, required)
- [ ] `insert` ([`Range`](#range), required)
- [ ] `replace` ([`Range`](#range), required)

### `InsertTextFormat`

- [ ] Wire type — unsigned integer.
  - [ ] `1` — PlainText.
  - [ ] `2` — Snippet.

### `InsertTextMode`

- [ ] Wire type — unsigned integer.
  - [ ] `1` — AsIs.
  - [ ] `2` — AdjustIndentation.

### `IntegerOrNull`

- [ ] Variant — integer
- [ ] Variant — null

### `IntegerOrString`

- [ ] Variant — integer
- [ ] Variant — string

### `LanguageKind`

- [ ] Wire type — string.
  - [ ] `"abap"` — ABAP.
  - [ ] `"bat"` — WindowsBat.
  - [ ] `"bibtex"` — BibTeX.
  - [ ] `"clojure"` — Clojure.
  - [ ] `"coffeescript"` — Coffeescript.
  - [ ] `"c"` — C.
  - [ ] `"cpp"` — CPP.
  - [ ] `"csharp"` — CSharp.
  - [ ] `"css"` — CSS.
  - [ ] `"d"` — D.
  - [ ] `"pascal"` — Delphi.
  - [ ] `"diff"` — Diff.
  - [ ] `"dart"` — Dart.
  - [ ] `"dockerfile"` — Dockerfile.
  - [ ] `"elixir"` — Elixir.
  - [ ] `"erlang"` — Erlang.
  - [ ] `"fsharp"` — FSharp.
  - [ ] `"git-commit"` — GitCommit.
  - [ ] `"git-rebase"` — GitRebase.
  - [ ] `"go"` — Go.
  - [ ] `"groovy"` — Groovy.
  - [ ] `"handlebars"` — Handlebars.
  - [ ] `"haskell"` — Haskell.
  - [ ] `"html"` — HTML.
  - [ ] `"ini"` — Ini.
  - [ ] `"java"` — Java.
  - [ ] `"javascript"` — JavaScript.
  - [ ] `"javascriptreact"` — JavaScriptReact.
  - [ ] `"json"` — JSON.
  - [ ] `"latex"` — LaTeX.
  - [ ] `"less"` — Less.
  - [ ] `"lua"` — Lua.
  - [ ] `"makefile"` — Makefile.
  - [ ] `"markdown"` — Markdown.
  - [ ] `"objective-c"` — ObjectiveC.
  - [ ] `"objective-cpp"` — ObjectiveCPP.
  - [ ] `"pascal"` — Pascal.
  - [ ] `"perl"` — Perl.
  - [ ] `"perl6"` — Perl6.
  - [ ] `"php"` — PHP.
  - [ ] `"plaintext"` — Plaintext.
  - [ ] `"powershell"` — Powershell.
  - [ ] `"jade"` — Pug.
  - [ ] `"python"` — Python.
  - [ ] `"r"` — R.
  - [ ] `"razor"` — Razor.
  - [ ] `"ruby"` — Ruby.
  - [ ] `"rust"` — Rust.
  - [ ] `"scss"` — SCSS.
  - [ ] `"sass"` — SASS.
  - [ ] `"scala"` — Scala.
  - [ ] `"shaderlab"` — ShaderLab.
  - [ ] `"shellscript"` — ShellScript.
  - [ ] `"sql"` — SQL.
  - [ ] `"swift"` — Swift.
  - [ ] `"typescript"` — TypeScript.
  - [ ] `"typescriptreact"` — TypeScriptReact.
  - [ ] `"tex"` — TeX.
  - [ ] `"vb"` — VisualBasic.
  - [ ] `"xml"` — XML.
  - [ ] `"xsl"` — XSL.
  - [ ] `"yaml"` — YAML.

### `LinkedEditingRangeParams`

- [ ] `textDocument` ([`TextDocumentIdentifier`](#textdocumentidentifier), required)
- [ ] `position` ([`Position`](#position), required)
- [ ] `workDoneToken` ([`IntegerOrString`](#integerorstring), optional)

### `LinkedEditingRangeRegistrationOptions`

- [ ] `documentSelector` ([`DocumentSelectorOrNull`](#documentselectorornull) (or null), required)
- [ ] `workDoneProgress` (boolean, optional)
- [ ] `id` (string, optional)

### `LinkedEditingRanges`

- [ ] `ranges` (array of [`Range`](#range), required)
- [ ] `wordPattern` (string, optional)

### `LinkedEditingRangesOrNull`

- [ ] Variant — [`LinkedEditingRanges`](#linkededitingranges)
- [ ] Variant — null

### `Location`

- [ ] `uri` ([`DocumentUri`](#documenturi), required)
- [ ] `range` ([`Range`](#range), required)

### `LocationLink`

- [ ] `originSelectionRange` ([`Range`](#range), optional)
- [ ] `targetUri` ([`DocumentUri`](#documenturi), required)
- [ ] `targetRange` ([`Range`](#range), required)
- [ ] `targetSelectionRange` ([`Range`](#range), required)

### `LocationOrLocationsOrDefinitionLinksOrNull`

- [ ] Variant — [`Location`](#location)
- [ ] Variant — array of [`Location`](#location)
- [ ] Variant — array of [`LocationLink`](#locationlink)
- [ ] Variant — null

### `LocationOrLocationUriOnly`

- [ ] Variant — [`Location`](#location)
- [ ] Variant — [`LocationUriOnly`](#locationurionly)

### `LocationsOrNull`

- [ ] Variant — array of [`Location`](#location)
- [ ] Variant — null

### `LocationUriOnly`

- [ ] `uri` ([`DocumentUri`](#documenturi), required)

### `LogMessageParams`

- [ ] `type` ([`MessageType`](#messagetype), required)
- [ ] `message` (string, required)

### `LogVerbosity`

- [ ] Wire type — integer.
  - [ ] `0` — Off.
  - [ ] `1` — Trace.
  - [ ] `2` — Debug.
  - [ ] `3` — Info.
  - [ ] `4` — Warning.
  - [ ] `5` — Error.

### `MarkedStringWithLanguage`

- [ ] `language` (string, required)
- [ ] `value` (string, required)

### `MarkupContent`

- [ ] `kind` ([`MarkupKind`](#markupkind), required)
- [ ] `value` (string, required)

### `MarkupContentOrStringOrMarkedStringWithLanguageOrMarkedStrings`

- [ ] Variant — [`MarkupContent`](#markupcontent)
- [ ] Variant — string
- [ ] Variant — [`MarkedStringWithLanguage`](#markedstringwithlanguage)
- [ ] Variant — array of [`StringOrMarkedStringWithLanguage`](#stringormarkedstringwithlanguage)

### `MarkupKind`

- [ ] Wire type — string.
  - [ ] `"plaintext"` — PlainText.
  - [ ] `"markdown"` — Markdown.

### `MessageType`

- [ ] Wire type — unsigned integer.
  - [ ] `1` — Error.
  - [ ] `2` — Warning.
  - [ ] `3` — Info.
  - [ ] `4` — Log.
  - [ ] `5` — Debug.

### `MultiDocumentHighlight`

- [ ] `uri` ([`DocumentUri`](#documenturi), required)
- [ ] `highlights` (array of [`DocumentHighlight`](#documenthighlight), required)

### `MultiDocumentHighlightParams`

- [ ] `textDocument` ([`TextDocumentIdentifier`](#textdocumentidentifier), required)
- [ ] `position` ([`Position`](#position), required)
- [ ] `filesToSearch` (array of [`DocumentUri`](#documenturi), required)

### `MultiDocumentHighlightsOrNull`

- [ ] Variant — array of [`MultiDocumentHighlight`](#multidocumenthighlight)
- [ ] Variant — null

### `OptionalVersionedTextDocumentIdentifier`

- [ ] `uri` ([`DocumentUri`](#documenturi), required)
- [ ] `version` ([`IntegerOrNull`](#integerornull) (or null), required)

### `ParameterInformation`

- [ ] `label` ([`StringOrTuple`](#stringortuple), required)
- [ ] `documentation` ([`StringOrMarkupContent`](#stringormarkupcontent), optional)

### `PatternOrRelativePattern`

- [ ] Variant — string
- [ ] Variant — [`RelativePattern`](#relativepattern)

### `PerformanceStatsTelemetryEvent`

- [ ] `eventName` (`"languageServer.performanceStats"`, required)
- [ ] `telemetryPurpose` (`"usage"`, required)
- [ ] `measurements` ([`PerformanceStatsTelemetryMeasurements`](#performancestatstelemetrymeasurements), required)

### `PerformanceStatsTelemetryMeasurements`

- [ ] `openFileCount` (number, optional, nullable)
- [ ] `uptimeSeconds` (number, optional, nullable)
- [ ] `projectCount` (number, optional, nullable)
- [ ] `configCount` (number, optional, nullable)
- [ ] `cachedDiskFileCount` (number, optional, nullable)
- [ ] `memoryUsedBytes` (number, optional, nullable)
- [ ] `goMemLimit` (number, optional, nullable)
- [ ] `goGCPercent` (number, optional, nullable)
- [ ] `heapGoalBytes` (number, optional, nullable)
- [ ] `heapLiveBytes` (number, optional, nullable)
- [ ] `heapObjectCount` (number, optional, nullable)
- [ ] `heapStackBytes` (number, optional, nullable)
- [ ] `heapReleasedBytes` (number, optional, nullable)
- [ ] `heapFreeBytes` (number, optional, nullable)
- [ ] `gcScanHeapBytes` (number, optional, nullable)
- [ ] `goMaxProcs` (number, optional, nullable)
- [ ] `goroutineCount` (number, optional, nullable)
- [ ] `gcCyclesTotal` (number, optional, nullable)
- [ ] `gcCPUSeconds` (number, optional, nullable)
- [ ] `userCPUSeconds` (number, optional, nullable)
- [ ] `systemMemTotal` (number, optional, nullable)
- [ ] `systemMemUsed` (number, optional, nullable)
- [ ] `autoImportProjectBucketCount` (number, optional, nullable)
- [ ] `autoImportNodeModulesBucketCount` (number, optional, nullable)
- [ ] `autoImportUniquePackageCount` (number, optional, nullable)
- [ ] `autoImportProjectExportCount` (number, optional, nullable)
- [ ] `autoImportNodeModulesExportCount` (number, optional, nullable)
- [ ] `autoImportProjectFileCount` (number, optional, nullable)
- [ ] `autoImportNodeModulesFileCount` (number, optional, nullable)
- [ ] `autoImportNodeModulesUnfilteredBucketCount` (number, optional, nullable)

### `Position`

- [ ] `line` (unsigned integer, required)
- [ ] `character` (unsigned integer, required)

### `PrepareRenameDefaultBehavior`

- [ ] `defaultBehavior` (boolean, required)

### `PrepareRenameParams`

- [ ] `textDocument` ([`TextDocumentIdentifier`](#textdocumentidentifier), required)
- [ ] `position` ([`Position`](#position), required)
- [ ] `workDoneToken` ([`IntegerOrString`](#integerorstring), optional)

### `PrepareRenamePlaceholder`

- [ ] `range` ([`Range`](#range), required)
- [ ] `placeholder` (string, required)

### `ProfileParams`

- [ ] `dir` (string, required)

### `ProfileResult`

- [ ] `file` (string, required)

### `ProgressParams`

- [ ] `token` ([`IntegerOrString`](#integerorstring), required) — The project progress reporter uses a string token.
- [ ] `value` ([`WorkDoneProgressBeginOrReportOrEnd`](#workdoneprogressbeginorreportorend), required)

### `ProjectInfoParams`

- [ ] `textDocument` ([`TextDocumentIdentifier`](#textdocumentidentifier), required)

### `ProjectInfoResult`

- [ ] `configFilePath` (string, required)

### `ProjectInfoTelemetryEvent`

- [ ] `eventName` (`"languageServer.projectInfo"`, required)
- [ ] `telemetryPurpose` (`"usage"`, required)
- [ ] `properties` (object keyed by string with values of string, required)
- [ ] `measurements` ([`ProjectInfoTelemetryMeasurements`](#projectinfotelemetrymeasurements), required)

### `ProjectInfoTelemetryMeasurements`

- [ ] `jsFileCount` (number, optional, nullable)
- [ ] `jsFileSize` (number, optional, nullable)
- [ ] `jsxFileCount` (number, optional, nullable)
- [ ] `jsxFileSize` (number, optional, nullable)
- [ ] `tsFileCount` (number, optional, nullable)
- [ ] `tsFileSize` (number, optional, nullable)
- [ ] `tsxFileCount` (number, optional, nullable)
- [ ] `tsxFileSize` (number, optional, nullable)
- [ ] `dtsFileCount` (number, optional, nullable)
- [ ] `dtsFileSize` (number, optional, nullable)

### `PublishDiagnosticsParams`

- [ ] `uri` ([`DocumentUri`](#documenturi), required)
- [ ] `version` (integer, optional) — Schema only for TypeScript’s project-level push reports.
- [ ] `diagnostics` (array of [`Diagnostic`](#diagnostic), required)

### `Range`

- [ ] `start` ([`Position`](#position), required)
- [ ] `end` ([`Position`](#position), required)

### `RangeOrEditRangeWithInsertReplace`

- [ ] Variant — [`Range`](#range)
- [ ] Variant — [`EditRangeWithInsertReplace`](#editrangewithinsertreplace)

### `RangeOrPrepareRenamePlaceholderOrPrepareRenameDefaultBehaviorOrNull`

- [ ] Variant — [`Range`](#range)
- [ ] Variant — [`PrepareRenamePlaceholder`](#preparerenameplaceholder)
- [ ] Variant — [`PrepareRenameDefaultBehavior`](#preparerenamedefaultbehavior)
- [ ] Variant — null

### `ReferenceContext`

- [ ] `includeDeclaration` (boolean, required)

### `ReferenceParams`

- [ ] `textDocument` ([`TextDocumentIdentifier`](#textdocumentidentifier), required)
- [ ] `position` ([`Position`](#position), required)
- [ ] `workDoneToken` ([`IntegerOrString`](#integerorstring), optional)
- [ ] `partialResultToken` ([`IntegerOrString`](#integerorstring), optional)
- [ ] `context` ([`ReferenceContext`](#referencecontext), required)

### `ReferenceRegistrationOptions`

- [ ] `documentSelector` ([`DocumentSelectorOrNull`](#documentselectorornull) (or null), required)
- [ ] `workDoneProgress` (boolean, optional)

### `RegisterOptions`

- [ ] `textDocument/implementation` — [`ImplementationRegistrationOptions`](#implementationregistrationoptions)
- [ ] `textDocument/typeDefinition` — [`TypeDefinitionRegistrationOptions`](#typedefinitionregistrationoptions)
- [ ] `textDocument/foldingRange` — [`FoldingRangeRegistrationOptions`](#foldingrangeregistrationoptions)
- [ ] `textDocument/selectionRange` — [`SelectionRangeRegistrationOptions`](#selectionrangeregistrationoptions)
- [ ] `textDocument/prepareCallHierarchy` — [`CallHierarchyRegistrationOptions`](#callhierarchyregistrationoptions)
- [ ] `textDocument/semanticTokens` — [`SemanticTokensRegistrationOptions`](#semantictokensregistrationoptions)
- [ ] `textDocument/linkedEditingRange` — [`LinkedEditingRangeRegistrationOptions`](#linkededitingrangeregistrationoptions)
- [ ] `workspace/willRenameFiles` — [`FileOperationRegistrationOptions`](#fileoperationregistrationoptions)
- [ ] `textDocument/inlayHint` — [`InlayHintRegistrationOptions`](#inlayhintregistrationoptions)
- [ ] `textDocument/diagnostic` — [`DiagnosticRegistrationOptions`](#diagnosticregistrationoptions)
- [ ] `textDocument/completion` — [`CompletionRegistrationOptions`](#completionregistrationoptions)
- [ ] `textDocument/hover` — [`HoverRegistrationOptions`](#hoverregistrationoptions)
- [ ] `textDocument/signatureHelp` — [`SignatureHelpRegistrationOptions`](#signaturehelpregistrationoptions)
- [ ] `textDocument/definition` — [`DefinitionRegistrationOptions`](#definitionregistrationoptions)
- [ ] `textDocument/references` — [`ReferenceRegistrationOptions`](#referenceregistrationoptions)
- [ ] `textDocument/documentHighlight` — [`DocumentHighlightRegistrationOptions`](#documenthighlightregistrationoptions)
- [ ] `textDocument/documentSymbol` — [`DocumentSymbolRegistrationOptions`](#documentsymbolregistrationoptions)
- [ ] `textDocument/codeAction` — [`CodeActionRegistrationOptions`](#codeactionregistrationoptions)
- [ ] `textDocument/codeLens` — [`CodeLensRegistrationOptions`](#codelensregistrationoptions)
- [ ] `textDocument/formatting` — [`DocumentFormattingRegistrationOptions`](#documentformattingregistrationoptions)
- [ ] `textDocument/rangeFormatting` — [`DocumentRangeFormattingRegistrationOptions`](#documentrangeformattingregistrationoptions)
- [ ] `textDocument/onTypeFormatting` — [`DocumentOnTypeFormattingRegistrationOptions`](#documentontypeformattingregistrationoptions)
- [ ] `textDocument/rename` — [`RenameRegistrationOptions`](#renameregistrationoptions)
- [ ] `workspace/didChangeConfiguration` — [`DidChangeConfigurationRegistrationOptions`](#didchangeconfigurationregistrationoptions)
- [ ] `textDocument/didOpen` — [`TextDocumentRegistrationOptions`](#textdocumentregistrationoptions)
- [ ] `textDocument/didChange` — [`TextDocumentChangeRegistrationOptions`](#textdocumentchangeregistrationoptions)
- [ ] `textDocument/didClose` — [`TextDocumentRegistrationOptions`](#textdocumentregistrationoptions)
- [ ] `workspace/didChangeWatchedFiles` — [`DidChangeWatchedFilesRegistrationOptions`](#didchangewatchedfilesregistrationoptions)

### `Registration`

- [ ] `id` (string, required)
- [ ] `method` (string, required)
- [ ] `registerOptions` ([`RegisterOptions`](#registeroptions), required) — The value shape is selected by `method`.

### `RegistrationParams`

- [ ] `registrations` (array of [`Registration`](#registration), required)

### `RelatedFullDocumentDiagnosticReport`

- [ ] `kind` (`"full"`, required)
- [ ] `resultId` (string, optional) — Schema only for the document diagnostic provider.
- [ ] `items` (array of [`Diagnostic`](#diagnostic), required)
- [ ] `relatedDocuments` (object keyed by [`DocumentUri`](#documenturi) with values of [`FullDocumentDiagnosticReportOrUnchangedDocumentDiagnosticReport`](#fulldocumentdiagnosticreportorunchangeddocumentdiagnosticreport), optional) — Schema only for the document diagnostic provider.

### `RelatedFullDocumentDiagnosticReportOrUnchangedDocumentDiagnosticReport`

- [ ] Variant — [`RelatedFullDocumentDiagnosticReport`](#relatedfulldocumentdiagnosticreport)
- [ ] Variant — [`RelatedUnchangedDocumentDiagnosticReport`](#relatedunchangeddocumentdiagnosticreport)

### `RelatedUnchangedDocumentDiagnosticReport`

- [ ] `kind` (`"unchanged"`, required)
- [ ] `resultId` (string, required)
- [ ] `relatedDocuments` (object keyed by [`DocumentUri`](#documenturi) with values of [`FullDocumentDiagnosticReportOrUnchangedDocumentDiagnosticReport`](#fulldocumentdiagnosticreportorunchangeddocumentdiagnosticreport), optional)

### `RelativePattern`

- [ ] `baseUri` ([`WorkspaceFolderOrURI`](#workspacefolderoruri), required)
- [ ] `pattern` (string, required)

### `RenameFile`

- [ ] `kind` (`"rename"`, required)
- [ ] `annotationId` (string, optional)
- [ ] `oldUri` ([`DocumentUri`](#documenturi), required)
- [ ] `newUri` ([`DocumentUri`](#documenturi), required)
- [ ] `options` ([`RenameFileOptions`](#renamefileoptions), optional)

### `RenameFileOptions`

- [ ] `overwrite` (boolean, optional)
- [ ] `ignoreIfExists` (boolean, optional)

### `RenameFilesParams`

- [ ] `files` (array of [`FileRename`](#filerename), required)

### `RenameParams`

- [ ] `textDocument` ([`TextDocumentIdentifier`](#textdocumentidentifier), required)
- [ ] `position` ([`Position`](#position), required)
- [ ] `workDoneToken` ([`IntegerOrString`](#integerorstring), optional)
- [ ] `newName` (string, required)

### `RenameRegistrationOptions`

- [ ] `documentSelector` ([`DocumentSelectorOrNull`](#documentselectorornull) (or null), required)
- [ ] `workDoneProgress` (boolean, optional)
- [ ] `prepareProvider` (boolean, optional)

### `RequestFailureTelemetryEvent`

- [ ] `eventName` (`"languageServer.errorResponse"`, required)
- [ ] `telemetryPurpose` (`"error"`, required)
- [ ] `properties` ([`RequestFailureTelemetryProperties`](#requestfailuretelemetryproperties), required)

### `RequestFailureTelemetryEventOrPerformanceStatsTelemetryEventOrProjectInfoTelemetryEventOrNull`

- [ ] Variant — [`RequestFailureTelemetryEvent`](#requestfailuretelemetryevent)
- [ ] Variant — [`PerformanceStatsTelemetryEvent`](#performancestatstelemetryevent)
- [ ] Variant — [`ProjectInfoTelemetryEvent`](#projectinfotelemetryevent)
- [ ] Variant — null

### `RequestFailureTelemetryProperties`

- [ ] `errorCode` (string, required)
- [ ] `requestMethod` (string, required)
- [ ] `stack` (string, required)

### `SelectionRange`

- [ ] `range` ([`Range`](#range), required)
- [ ] `parent` ([`SelectionRange`](#selectionrange), optional)

### `SelectionRangeParams`

- [ ] `workDoneToken` ([`IntegerOrString`](#integerorstring), optional)
- [ ] `partialResultToken` ([`IntegerOrString`](#integerorstring), optional)
- [ ] `textDocument` ([`TextDocumentIdentifier`](#textdocumentidentifier), required)
- [ ] `positions` (array of [`Position`](#position), required)

### `SelectionRangeRegistrationOptions`

- [ ] `workDoneProgress` (boolean, optional)
- [ ] `documentSelector` ([`DocumentSelectorOrNull`](#documentselectorornull) (or null), required)
- [ ] `id` (string, optional)

### `SelectionRangesOrNull`

- [ ] Variant — array of [`SelectionRange`](#selectionrange)
- [ ] Variant — null

### `SemanticTokens`

- [ ] `resultId` (string, optional) — Schema only for this server, which does not advertise delta updates.
- [ ] `data` (array of unsigned integer, required)

### `SemanticTokensFullDelta`

- [ ] `delta` (boolean, optional)

### `SemanticTokensLegend`

- [ ] `tokenTypes` (array of string, required)
- [ ] `tokenModifiers` (array of string, required)

### `SemanticTokensOrNull`

- [ ] Variant — [`SemanticTokens`](#semantictokens)
- [ ] Variant — null

### `SemanticTokensParams`

- [ ] `workDoneToken` ([`IntegerOrString`](#integerorstring), optional)
- [ ] `partialResultToken` ([`IntegerOrString`](#integerorstring), optional)
- [ ] `textDocument` ([`TextDocumentIdentifier`](#textdocumentidentifier), required)

### `SemanticTokensRangeParams`

- [ ] `workDoneToken` ([`IntegerOrString`](#integerorstring), optional)
- [ ] `partialResultToken` ([`IntegerOrString`](#integerorstring), optional)
- [ ] `textDocument` ([`TextDocumentIdentifier`](#textdocumentidentifier), required)
- [ ] `range` ([`Range`](#range), required)

### `SemanticTokensRegistrationOptions`

- [ ] `documentSelector` ([`DocumentSelectorOrNull`](#documentselectorornull) (or null), required)
- [ ] `workDoneProgress` (boolean, optional)
- [ ] `legend` ([`SemanticTokensLegend`](#semantictokenslegend), required)
- [ ] `range` ([`BooleanOrEmptyObject`](#booleanoremptyobject), optional)
- [ ] `full` ([`BooleanOrSemanticTokensFullDelta`](#booleanorsemantictokensfulldelta), optional)
- [ ] `id` (string, optional)

### `ServerCapabilities`

- [ ] [Fields advertised by TypeScript](#advertised-server-capabilities)

### `ServerCompletionItemOptions`

- [ ] `labelDetailsSupport` (boolean, optional)

### `ServerInfo`

- [ ] `name` (string, required)
- [ ] `version` (string, optional)

### `SetContentMapperContributionsParams`

- [ ] `contributions` (array of [`ContentMapperContribution`](#contentmappercontribution), required)
- [ ] `openDocuments` (array of [`TextDocumentIdentifier`](#textdocumentidentifier), required)

### `SetLogVerbosityParams`

- [ ] `verbosity` ([`LogVerbosity`](#logverbosity), required)

### `SetTraceParams`

- [ ] `value` ([`TraceValue`](#tracevalue), required)

### `SignatureHelp`

- [ ] `signatures` (array of [`SignatureInformation`](#signatureinformation), required)
- [ ] `activeSignature` (unsigned integer, optional)
- [ ] `activeParameter` ([`UintegerOrNull`](#uintegerornull) (or null), optional, nullable)

### `SignatureHelpContext`

- [ ] `triggerKind` ([`SignatureHelpTriggerKind`](#signaturehelptriggerkind), required)
- [ ] `triggerCharacter` (string, optional)
- [ ] `isRetrigger` (boolean, required)
- [ ] `activeSignatureHelp` ([`SignatureHelp`](#signaturehelp), optional)

### `SignatureHelpOrNull`

- [ ] Variant — [`SignatureHelp`](#signaturehelp)
- [ ] Variant — null

### `SignatureHelpParams`

- [ ] `textDocument` ([`TextDocumentIdentifier`](#textdocumentidentifier), required)
- [ ] `position` ([`Position`](#position), required)
- [ ] `workDoneToken` ([`IntegerOrString`](#integerorstring), optional)
- [ ] `context` ([`SignatureHelpContext`](#signaturehelpcontext), optional)

### `SignatureHelpRegistrationOptions`

- [ ] `documentSelector` ([`DocumentSelectorOrNull`](#documentselectorornull) (or null), required)
- [ ] `workDoneProgress` (boolean, optional)
- [ ] `triggerCharacters` (array of string, optional)
- [ ] `retriggerCharacters` (array of string, optional)

### `SignatureHelpTriggerKind`

- [ ] Wire type — unsigned integer.
  - [ ] `1` — Invoked.
  - [ ] `2` — TriggerCharacter.
  - [ ] `3` — ContentChange.

### `SignatureInformation`

- [ ] `label` (string, required)
- [ ] `documentation` ([`StringOrMarkupContent`](#stringormarkupcontent), optional)
- [ ] `parameters` (array of [`ParameterInformation`](#parameterinformation), optional)
- [ ] `activeParameter` ([`UintegerOrNull`](#uintegerornull) (or null), optional, nullable)
- [ ] `_vs_colorizedLabel` ([`VSClassifiedTextElement`](#vsclassifiedtextelement), optional)

### `SnippetTextEdit`

- [ ] `range` ([`Range`](#range), required)
- [ ] `snippet` ([`StringValue`](#stringvalue), required)
- [ ] `annotationId` (string, optional)

### `StringOrInlayHintLabelParts`

- [ ] Variant — string
- [ ] Variant — array of [`InlayHintLabelPart`](#inlayhintlabelpart)

### `StringOrMarkedStringWithLanguage`

- [ ] Variant — string
- [ ] Variant — [`MarkedStringWithLanguage`](#markedstringwithlanguage)

### `StringOrMarkupContent`

- [ ] Variant — string
- [ ] Variant — [`MarkupContent`](#markupcontent)

### `StringOrNull`

- [ ] Variant — string
- [ ] Variant — null

### `StringOrStrings`

- [ ] Variant — string
- [ ] Variant — array of string

### `StringOrTuple`

- [ ] Variant — string
- [ ] Variant — array of two unsigned integers

### `StringValue`

- [ ] `kind` (`"snippet"`, required)
- [ ] `value` (string, required)

### `SymbolInformation`

- [ ] `name` (string, required)
- [ ] `kind` ([`SymbolKind`](#symbolkind), required)
- [ ] `tags` (array of [`SymbolTag`](#symboltag), optional)
- [ ] `containerName` (string, optional)
- [ ] `deprecated` (boolean, optional)
- [ ] `location` ([`Location`](#location), required)

### `SymbolInformationsOrDocumentSymbolsOrNull`

- [ ] Variant — array of [`SymbolInformation`](#symbolinformation)
- [ ] Variant — array of [`DocumentSymbol`](#documentsymbol)
- [ ] Variant — null

### `SymbolInformationsOrWorkspaceSymbolsOrNull`

- [ ] Variant — array of [`SymbolInformation`](#symbolinformation)
- [ ] Variant — array of [`WorkspaceSymbol`](#workspacesymbol)
- [ ] Variant — null

### `SymbolKind`

- [ ] Wire type — unsigned integer.
  - [ ] `1` — File.
  - [ ] `2` — Module.
  - [ ] `3` — Namespace.
  - [ ] `4` — Package.
  - [ ] `5` — Class.
  - [ ] `6` — Method.
  - [ ] `7` — Property.
  - [ ] `8` — Field.
  - [ ] `9` — Constructor.
  - [ ] `10` — Enum.
  - [ ] `11` — Interface.
  - [ ] `12` — Function.
  - [ ] `13` — Variable.
  - [ ] `14` — Constant.
  - [ ] `15` — String.
  - [ ] `16` — Number.
  - [ ] `17` — Boolean.
  - [ ] `18` — Array.
  - [ ] `19` — Object.
  - [ ] `20` — Key.
  - [ ] `21` — Null.
  - [ ] `22` — EnumMember.
  - [ ] `23` — Struct.
  - [ ] `24` — Event.
  - [ ] `25` — Operator.
  - [ ] `26` — TypeParameter.

### `SymbolTag`

- [ ] Wire type — unsigned integer.
  - [ ] `1` — Deprecated.

### `TextDocumentChangeRegistrationOptions`

- [ ] `documentSelector` ([`DocumentSelectorOrNull`](#documentselectorornull) (or null), required)
- [ ] `syncKind` ([`TextDocumentSyncKind`](#textdocumentsynckind), required)

### `TextDocumentContentChangePartial`

- [ ] `range` ([`Range`](#range), required)
- [ ] `rangeLength` (unsigned integer, optional)
- [ ] `text` (string, required)

### `TextDocumentContentChangePartialOrWholeDocument`

- [ ] Variant — [`TextDocumentContentChangePartial`](#textdocumentcontentchangepartial)
- [ ] Variant — [`TextDocumentContentChangeWholeDocument`](#textdocumentcontentchangewholedocument)

### `TextDocumentContentChangeWholeDocument`

- [ ] `text` (string, required)

### `TextDocumentEdit`

- [ ] `textDocument` ([`OptionalVersionedTextDocumentIdentifier`](#optionalversionedtextdocumentidentifier), required)
- [ ] `edits` (array of [`TextEditOrAnnotatedTextEditOrSnippetTextEdit`](#texteditorannotatedtexteditorsnippettextedit), required)

### `TextDocumentEditOrCreateFileOrRenameFileOrDeleteFile`

- [ ] Variant — [`TextDocumentEdit`](#textdocumentedit)
- [ ] Variant — [`CreateFile`](#createfile)
- [ ] Variant — [`RenameFile`](#renamefile)
- [ ] Variant — [`DeleteFile`](#deletefile)

### `TextDocumentFilterLanguage`

- [ ] `language` (string, required)
- [ ] `scheme` (string, optional)
- [ ] `pattern` ([`PatternOrRelativePattern`](#patternorrelativepattern), optional)

### `TextDocumentFilterLanguageOrSchemeOrPattern`

- [ ] Variant — [`TextDocumentFilterLanguage`](#textdocumentfilterlanguage)
- [ ] Variant — [`TextDocumentFilterScheme`](#textdocumentfilterscheme)
- [ ] Variant — [`TextDocumentFilterPattern`](#textdocumentfilterpattern)

### `TextDocumentFilterPattern`

- [ ] `language` (string, optional)
- [ ] `scheme` (string, optional)
- [ ] `pattern` ([`PatternOrRelativePattern`](#patternorrelativepattern), required)

### `TextDocumentFilterScheme`

- [ ] `language` (string, optional)
- [ ] `scheme` (string, required)
- [ ] `pattern` ([`PatternOrRelativePattern`](#patternorrelativepattern), optional)

### `TextDocumentIdentifier`

- [ ] `uri` ([`DocumentUri`](#documenturi), required)

### `TextDocumentItem`

- [ ] `uri` ([`DocumentUri`](#documenturi), required)
- [ ] `languageId` ([`LanguageKind`](#languagekind), required)
- [ ] `version` (integer, required)
- [ ] `text` (string, required)

### `TextDocumentPositionParams`

- [ ] `textDocument` ([`TextDocumentIdentifier`](#textdocumentidentifier), required)
- [ ] `position` ([`Position`](#position), required)

### `TextDocumentRegistrationOptions`

- [ ] `documentSelector` ([`DocumentSelectorOrNull`](#documentselectorornull) (or null), required)

### `TextDocumentSyncKind`

- [ ] Wire type — unsigned integer.
  - [ ] `0` — None.
  - [ ] `1` — Full.
  - [ ] `2` — Incremental.

### `TextEdit`

- [ ] `range` ([`Range`](#range), required)
- [ ] `newText` (string, required)

### `TextEditOrAnnotatedTextEditOrSnippetTextEdit`

- [ ] Variant — [`TextEdit`](#textedit)
- [ ] Variant — [`AnnotatedTextEdit`](#annotatedtextedit)
- [ ] Variant — [`SnippetTextEdit`](#snippettextedit)

### `TextEditOrInsertReplaceEdit`

- [ ] Variant — [`TextEdit`](#textedit)
- [ ] Variant — [`InsertReplaceEdit`](#insertreplaceedit)

### `TextEditsOrNull`

- [ ] Variant — array of [`TextEdit`](#textedit)
- [ ] Variant — null

### `TraceValue`

- [ ] Wire type — string.
  - [ ] `"off"` — Off.
  - [ ] `"messages"` — Messages.
  - [ ] `"verbose"` — Verbose.

### `TypeDefinitionParams`

- [ ] `textDocument` ([`TextDocumentIdentifier`](#textdocumentidentifier), required)
- [ ] `position` ([`Position`](#position), required)
- [ ] `workDoneToken` ([`IntegerOrString`](#integerorstring), optional)
- [ ] `partialResultToken` ([`IntegerOrString`](#integerorstring), optional)

### `TypeDefinitionRegistrationOptions`

- [ ] `documentSelector` ([`DocumentSelectorOrNull`](#documentselectorornull) (or null), required)
- [ ] `workDoneProgress` (boolean, optional)
- [ ] `id` (string, optional)

### `UintegerOrNull`

- [ ] Variant — unsigned integer
- [ ] Variant — null

### `UnchangedDocumentDiagnosticReport`

- [ ] `kind` (`"unchanged"`, required)
- [ ] `resultId` (string, required)

### `Unregistration`

- [ ] `id` (string, required)
- [ ] `method` (string, required)

### `UnregistrationParams`

- [ ] `unregisterations` (array of [`Unregistration`](#unregistration), required) — This spelling is the LSP wire key.

### `URI`

- [ ] URI string.

### `VersionedTextDocumentIdentifier`

- [ ] `uri` ([`DocumentUri`](#documenturi), required)
- [ ] `version` (integer, required)

### `VSClassifiedTextElement`

- [ ] `Runs` (array of [`VSClassifiedTextRun`](#vsclassifiedtextrun), required)
- [ ] `_vs_type` (`"ClassifiedTextElement"`, required)

### `VSClassifiedTextRun`

- [ ] `ClassificationTypeName` (string, required)
- [ ] `Text` (string, required)
- [ ] `MarkerTagType` (string, optional)
- [ ] `Style` (integer, optional, nullable)
- [ ] `_vs_type` (`"ClassifiedTextRun"`, required)

### `VSContainerElement`

- [ ] `Style` ([`VSContainerElementStyle`](#vscontainerelementstyle), required)
- [ ] `Elements` (array of [`VSImageElementOrClassifiedTextElementOrContainerElement`](#vsimageelementorclassifiedtextelementorcontainerelement), required)
- [ ] `_vs_type` (`"ContainerElement"`, required)

### `VSContainerElementStyle`

- [ ] Wire type — integer.
  - [ ] `0` — Wrapped.
  - [ ] `1` — Stacked.

### `VSImageElement`

- [ ] `ImageId` ([`VSImageId`](#vsimageid), required)
- [ ] `_vs_type` (`"ImageElement"`, required)

### `VSImageElementOrClassifiedTextElementOrContainerElement`

- [ ] Variant — [`VSImageElement`](#vsimageelement)
- [ ] Variant — [`VSClassifiedTextElement`](#vsclassifiedtextelement)
- [ ] Variant — [`VSContainerElement`](#vscontainerelement)

### `VSImageId`

- [ ] `Guid` (string, required)
- [ ] `Id` (integer, required)
- [ ] `_vs_type` (`"ImageId"`, required)

### `VSOnAutoInsertParams`

- [ ] `_vs_textDocument` ([`TextDocumentIdentifier`](#textdocumentidentifier), required)
- [ ] `_vs_position` ([`Position`](#position), required)
- [ ] `_vs_ch` (string, required)

### `VSOnAutoInsertResponseItem`

- [ ] `_vs_textEditFormat` ([`InsertTextFormat`](#inserttextformat), required)
- [ ] `_vs_textEdit` ([`TextEdit`](#textedit), required)

### `VSOnAutoInsertResponseItemOrNull`

- [ ] Variant — [`VSOnAutoInsertResponseItem`](#vsonautoinsertresponseitem)
- [ ] Variant — null

### `VSReferenceItem`

- [ ] `_vs_id` (integer, required)
- [ ] `_vs_definitionId` (integer, optional)
- [ ] `_vs_kind` (array of [`VSReferenceKind`](#vsreferencekind), optional)
- [ ] `_vs_location` ([`Location`](#location), required)
- [ ] `_vs_definitionText` ([`VSClassifiedTextElement`](#vsclassifiedtextelement), optional)
- [ ] `_vs_projectName` (string, optional)
- [ ] `_vs_containingType` (string, optional)

### `VSReferenceItemsOrNull`

- [ ] Variant — array of [`VSReferenceItem`](#vsreferenceitem)
- [ ] Variant — null

### `VSReferenceKind`

- [ ] Wire type — integer.
  - [ ] `0` — Inactive.
  - [ ] `1` — Comment.
  - [ ] `2` — String.
  - [ ] `3` — Read.
  - [ ] `4` — Write.
  - [ ] `5` — Reference.
  - [ ] `6` — Name.
  - [ ] `7` — Qualified.
  - [ ] `8` — TypeArgument.
  - [ ] `9` — TypeConstraint.
  - [ ] `10` — BaseType.
  - [ ] `11` — Constructor.
  - [ ] `12` — Destructor.
  - [ ] `13` — Import.
  - [ ] `14` — Declaration.
  - [ ] `15` — AddressOf.
  - [ ] `16` — NotReference.
  - [ ] `17` — Unknown.

### `WatchKind`

- [ ] Wire type — unsigned integer.
  - [ ] `1` — Create.
  - [ ] `2` — Change.
  - [ ] `4` — Delete.

### `WorkDoneProgressBegin`

- [ ] `kind` (`"begin"`, required)
- [ ] `title` (string, required)
- [ ] `cancellable` (boolean, optional) — The project progress reporter does not set this field.
- [ ] `message` (string, optional)
- [ ] `percentage` (unsigned integer, optional) — The project progress reporter does not set this field.

### `WorkDoneProgressBeginOrReportOrEnd`

- [ ] Variant — [`WorkDoneProgressBegin`](#workdoneprogressbegin)
- [ ] Variant — [`WorkDoneProgressReport`](#workdoneprogressreport)
- [ ] Variant — [`WorkDoneProgressEnd`](#workdoneprogressend)

### `WorkDoneProgressCreateParams`

- [ ] `token` ([`IntegerOrString`](#integerorstring), required)

### `WorkDoneProgressEnd`

- [ ] `kind` (`"end"`, required)
- [ ] `message` (string, optional)

### `WorkDoneProgressReport`

- [ ] `kind` (`"report"`, required)
- [ ] `cancellable` (boolean, optional) — The project progress reporter does not set this field.
- [ ] `message` (string, optional)
- [ ] `percentage` (unsigned integer, optional) — The project progress reporter does not set this field.

### `WorkspaceEdit`

- [ ] `changes` (object keyed by [`DocumentUri`](#documenturi) with values of array of [`TextEdit`](#textedit), optional)
- [ ] `documentChanges` (array of [`TextDocumentEditOrCreateFileOrRenameFileOrDeleteFile`](#textdocumenteditorcreatefileorrenamefileordeletefile), optional)
- [ ] `changeAnnotations` (object keyed by string with values of [`ChangeAnnotation`](#changeannotation), optional)

### `WorkspaceEditOrNull`

- [ ] Variant — [`WorkspaceEdit`](#workspaceedit)
- [ ] Variant — null

### `WorkspaceFolder`

- [ ] `uri` ([`URI`](#uri), required)
- [ ] `name` (string, required)

### `WorkspaceFolderOrURI`

- [ ] Variant — [`WorkspaceFolder`](#workspacefolder)
- [ ] Variant — [`URI`](#uri)

### `WorkspaceFoldersOrNull`

- [ ] Variant — array of [`WorkspaceFolder`](#workspacefolder)
- [ ] Variant — null

### `WorkspaceSymbol`

- [ ] `name` (string, required)
- [ ] `kind` ([`SymbolKind`](#symbolkind), required)
- [ ] `tags` (array of [`SymbolTag`](#symboltag), optional)
- [ ] `containerName` (string, optional)
- [ ] `location` ([`LocationOrLocationUriOnly`](#locationorlocationurionly), required)
- [ ] `data` ([`WorkspaceSymbolData`](#workspacesymboldata), optional)

### `WorkspaceSymbolData`

- [ ] Empty object.

### `WorkspaceSymbolParams`

- [ ] `workDoneToken` ([`IntegerOrString`](#integerorstring), optional)
- [ ] `partialResultToken` ([`IntegerOrString`](#integerorstring), optional)
- [ ] `query` (string, required)
- [ ] `textDocument` ([`TextDocumentIdentifier`](#textdocumentidentifier), optional)

[snapshot]: https://github.com/microsoft/TypeScript/tree/01f1a759d92b19ea7024776da02358995d56a7ff
[release]: https://github.com/microsoft/TypeScript/blob/v7.0.2/tsc/internal/lsp/server.go
[handlers]: https://github.com/microsoft/TypeScript/blob/01f1a759d92b19ea7024776da02358995d56a7ff/tsc/internal/lsp/server.go#L1302-L1370
[protocol]: https://github.com/microsoft/TypeScript/blob/01f1a759d92b19ea7024776da02358995d56a7ff/tsc/internal/lsp/lsproto/lsp_generated.go
[completions]: https://github.com/microsoft/TypeScript/blob/01f1a759d92b19ea7024776da02358995d56a7ff/tsc/internal/ls/completions.go
[hover]: https://github.com/microsoft/TypeScript/blob/01f1a759d92b19ea7024776da02358995d56a7ff/tsc/internal/ls/hover.go
[signature-help]: https://github.com/microsoft/TypeScript/blob/01f1a759d92b19ea7024776da02358995d56a7ff/tsc/internal/ls/signaturehelp.go
[rename]: https://github.com/microsoft/TypeScript/blob/01f1a759d92b19ea7024776da02358995d56a7ff/tsc/internal/lsp/server.go#L2021-L2125
[action-kinds]: https://github.com/microsoft/TypeScript/blob/01f1a759d92b19ea7024776da02358995d56a7ff/tsc/internal/lsp/server.go#L389-L397
[code-actions]: https://github.com/microsoft/TypeScript/blob/01f1a759d92b19ea7024776da02358995d56a7ff/tsc/internal/ls/codeactions.go
[project-diagnostics]: https://github.com/microsoft/TypeScript/blob/01f1a759d92b19ea7024776da02358995d56a7ff/tsc/internal/project/session.go#L1903-L2026
[inlay-hints]: https://github.com/microsoft/TypeScript/blob/01f1a759d92b19ea7024776da02358995d56a7ff/tsc/internal/ls/inlay_hints.go
[code-lens]: https://github.com/microsoft/TypeScript/blob/01f1a759d92b19ea7024776da02358995d56a7ff/tsc/internal/ls/codelens.go
[configuration]: https://github.com/microsoft/TypeScript/blob/01f1a759d92b19ea7024776da02358995d56a7ff/tsc/internal/lsp/server.go#L851-L910
[ata]: https://github.com/microsoft/TypeScript/blob/01f1a759d92b19ea7024776da02358995d56a7ff/tsc/internal/project/ata/ata.go
[workspace-init]: https://github.com/microsoft/TypeScript/blob/01f1a759d92b19ea7024776da02358995d56a7ff/tsc/internal/lsp/server.go#L1749-L1866
[content-mapper]: https://github.com/microsoft/TypeScript/blob/01f1a759d92b19ea7024776da02358995d56a7ff/tsc/internal/lsp/server.go#L473-L755
[progress]: https://github.com/microsoft/TypeScript/blob/01f1a759d92b19ea7024776da02358995d56a7ff/tsc/internal/lsp/progress.go
[trace]: https://github.com/microsoft/TypeScript/blob/01f1a759d92b19ea7024776da02358995d56a7ff/tsc/internal/lsp/server.go#L1947-L1959
[logging]: https://github.com/microsoft/TypeScript/blob/01f1a759d92b19ea7024776da02358995d56a7ff/tsc/internal/lsp/logger.go#L48-L75
[framing]: https://github.com/microsoft/TypeScript/blob/01f1a759d92b19ea7024776da02358995d56a7ff/tsc/internal/jsonrpc/baseproto.go
[envelope]: https://github.com/microsoft/TypeScript/blob/01f1a759d92b19ea7024776da02358995d56a7ff/tsc/internal/lsp/lsproto/jsonrpc.go
[capabilities]: https://github.com/microsoft/TypeScript/blob/01f1a759d92b19ea7024776da02358995d56a7ff/tsc/internal/lsp/server.go#L1573-L1747
[language-service]: https://github.com/microsoft/TypeScript/tree/01f1a759d92b19ea7024776da02358995d56a7ff/tsc/internal/ls
[preference-parser]: https://github.com/microsoft/TypeScript/blob/01f1a759d92b19ea7024776da02358995d56a7ff/tsc/internal/ls/lsutil/userpreferences.go#L219-L250
[settings-schema]: https://github.com/microsoft/TypeScript/blob/01f1a759d92b19ea7024776da02358995d56a7ff/tools/userPreferences.schema.json
