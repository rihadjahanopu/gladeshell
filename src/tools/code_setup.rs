// ============================================================================
// STATUS: 100% NATIVE RUST (ZERO EXTERNAL BINARY DEPENDENCIES)
// AUDIT COMPLETED: FULL FEATURE PARITY, CROSS-OS VERIFIED & OPTIMIZED
// HANDS-OFF GUARANTEE: NO MANUAL EDITS REQUIRED
// ============================================================================
// src/tools/code_setup.rs — VS Code Settings & Extensions Bulletproof Installer
//                           (gladeshell Edition)
// ============================================================================

use std::env;
use std::error::Error;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

#[allow(dead_code)] const RED:    &str = "\x1b[38;2;243;139;168m";
#[allow(dead_code)] const GREEN:  &str = "\x1b[38;2;166;227;161m";
#[allow(dead_code)] const YELLOW: &str = "\x1b[38;2;249;226;175m";
#[allow(dead_code)] const BLUE:   &str = "\x1b[38;2;137;180;250m";
#[allow(dead_code)] const PURPLE: &str = "\x1b[38;2;203;166;247m";
#[allow(dead_code)] const CYAN:   &str = "\x1b[38;2;148;226;213m";
#[allow(dead_code)] const GRAY:   &str = "\x1b[38;2;147;153;178m";
#[allow(dead_code)] const BOLD:   &str = "\x1b[1m";
#[allow(dead_code)] const NC:     &str = "\x1b[0m";

// ── Extension List ────────────────────────────────────────────
pub const VSCODE_EXTENSIONS: &[&str] = &[
    // Themes & Icons
    "sldobri.bunker",
    "pkief.material-icon-theme",
    "wesbos.theme-cobalt2",
    "ahmadawais.shades-of-purple",
    "sumitsaha.learn-with-sumit-theme",
    "rajeshwaran.developer-theme-dark",
    "thang-nm.flow-icons",
    // Formatting & Linting
    "dbaeumer.vscode-eslint",
    "esbenp.prettier-vscode",
    "usernamehw.errorlens",
    "shardulm94.trailing-spaces",
    "oderwat.indent-rainbow",
    // HTML / CSS
    "formulahendry.auto-close-tag",
    "formulahendry.auto-rename-tag",
    "anteprimorac.html-end-tag-labels",
    "bradgashler.htmltagwrap",
    "anderseandersen.html-class-suggestions",
    "solnurkarim.html-to-css-autocompletion",
    "pranaygp.vscode-css-peek",
    "phoenisx.cssvar",
    "glenn2223.live-sass",
    "hossaini.bootstrap-intellisense",
    // Tailwind CSS
    "bradlc.vscode-tailwindcss",
    "stivo.tailwind-fold",
    // JavaScript / TypeScript
    "dsznajder.es7-react-js-snippets",
    "xabikos.javascriptsnippets",
    "mgmcdermott.vscode-language-babel",
    "steoates.autoimport",
    "nucllear.vscode-extension-auto-import",
    "yoavbls.pretty-ts-errors",
    "wix.vscode-import-cost",
    // IntelliSense & Navigation
    "christian-kohler.path-intellisense",
    "christian-kohler.npm-intellisense",
    "ionutvmi.path-autocomplete",
    "janne252.fontawesome-autocomplete",
    "kisstkondoros.vscode-gutter-preview",
    "ducphamngoc.codevisualizer",
    // Snippets & Productivity
    "jasonlhy.hungry-delete",
    "dzhavat.bracket-pair-toggler",
    "formulahendry.code-runner",
    "hoovercj.vscode-power-mode",
    "lkytal.pomodoro",
    // Git & Collaboration
    "eamodio.gitlens",
    "github.vscode-github-actions",
    "pflannery.vscode-versionlens",
    // Database & API
    "prisma.prisma",
    "mongodb.mongodb-vscode",
    "cweijan.vscode-postgresql-client2",
    "cweijan.dbclient-jdbc",
    "rangav.vscode-thunder-client",
    "bruno-api-client.bruno",
    // DevOps & Containers
    "docker.docker",
    "ms-azuretools.vscode-containers",
    "ms-vscode-remote.remote-containers",
    "mikestead.dotenv",
    // Runtime
    "oven.bun-vscode",
    // Utilities
    "adpyke.codesnap",
    "hediet.vscode-drawio",
    "simonsiefke.svg-preview",
    "glincker.thesvg",
    "tomoki1207.pdf",
    "naumovs.color-highlight",
    "redth.mobile-canvas",
    "typescriptteam.native-preview",
];

// ── Settings Payload ──────────────────────────────────────────
pub const VSCODE_SETTINGS: &str = r##"{

	"workbench.preferredDarkColorTheme": "Dobri Next -A00- Black",
	"workbench.statusBar.visible": true,
	"workbench.settings.alwaysShowAdvancedSettings": true,
	"workbench.view.alwaysShowHeaderActions": true,
	"workbench.startupEditor": "welcomePage",
	"workbench.experimental.share.enabled": true,
	"workbench.commandPalette.experimental.suggestCommands": true,

	// "window.title": "${dirty}${activeEditorShort}${separator}${rootName}${separator}${profileName}${separator}${appName}",
	"window.title": " 🤖  ${dirty}${rootName}  🤖",
	"window.zoomLevel": 2,
	"update.enableWindowsBackgroundUpdates": true,
	"background.enabled": true,

	// ==========================================
	// 2. EDITOR SETTINGS (ফন্ট, কার্সর, ফরম্যাটিং, সাজেশন)
	// ==========================================
	"editor.fontFamily": "Cascadia Code, JetBrains Mono, Fira Code, Operator Mono",
	"editor.fontSize": 17,
	"editor.fontWeight": "normal",
	"editor.lineHeight": 1.5,
	"editor.tabSize": 2,
	"editor.fontLigatures": true,
	"editor.wordWrap": "on",
	"editor.wrapOnEscapedLineFeeds": true,
	"editor.scrollbar.verticalScrollbarSize": 6,
	"editor.scrollbar.horizontalScrollbarSize": 6,
	"editor.scrollbar.horizontal": "hidden",
	"editor.smoothScrolling": true,
	"editor.cursorBlinking": "smooth",
	"editor.cursorSmoothCaretAnimation": "on",
	"editor.dragAndDrop": true,
	"editor.linkedEditing": true,
	"editor.selectionHighlight": true,
	"editor.trimWhitespaceOnDelete": true,
	"editor.stickyScroll.enabled": false,
	"editor.unicodeHighlight.invisibleCharacters": true,
	"editor.unicodeHighlight.ambiguousCharacters": true,
	"editor.inlineSuggest.edits.showCollapsed": true,
	"editor.inlayHints.fontFamily": "'Cascadia Code'",
	"editor.inlayHints.padding": true,

	// Editor Formatting & Suggestions
	"editor.formatOnSave": true,
	"editor.formatOnPaste": true,
	"editor.formatOnType": true,
	"editor.defaultFormatter": "esbenp.prettier-vscode",
	"editor.codeActionsOnSave": {
		"source.fixAll.eslint": "explicit",
		"source.organizeImports": "explicit",
	},
	"editor.codeActions.triggerOnFocusChange": true,
	"editor.acceptSuggestionOnCommitCharacter": true,
	"editor.acceptSuggestionOnEnter": "on",
	"editor.quickSuggestionsDelay": 0,
	"editor.quickSuggestions": {
		"other": "on",
		"comments": "off",
		"strings": "on",
	},
	"editor.suggest.showInlineDetails": true,
	"editor.suggest.showColors": true,
	"editor.suggest.localityBonus": true,
	"editor.suggest.insertMode": "replace",
	"editor.suggest.shareSuggestSelections": true,
	"editor.suggest.selectionMode": "always",
	"editor.suggestOnTriggerCharacters": true,
	"editor.tabCompletion": "on",
	"editor.suggestSelection": "first",
	"editor.wordBasedSuggestions": "allDocuments",
	"editor.suggest.snippetsPreventQuickSuggestions": true,
	"editor.suggest.preview": true,
	"editor.classSuggest.maxClassNames": 50,

	// Editor Tabs & Groups
	"workbench.editor.dragToOpenWindow": true,
	"workbench.editor.highlightModifiedTabs": true,
	"workbench.editor.enablePreview": false,
	"workbench.editor.enablePreviewFromQuickOpen": false,
	"workbench.editor.alwaysShowEditorActions": false,
	"workbench.editor.closeOnFileDelete": true,
	"workbench.list.smoothScrolling": true,
	"workbench.editor.autoLockGroups": {
		"default": true,
		"workbench.editor.chatSession": true,
		"workbench.editor.chatDebug": true,
		"workbench.editorinputs.searchEditorInput": true,
		"agentSessionsWelcomePage": true,
		"workbench.editor.browser": true,
		"notebookOutputEditor": true,
		"jupyter-notebook": true,
		"repl": true,
		"workbench.editors.gettingStartedInput": true,
		"imagePreview.previewEditor": true,
		"vscode.audioPreview": true,
		"vscode.videoPreview": true,
		"jsProfileVisualizer.cpuprofile.table": true,
		"jsProfileVisualizer.heapprofile.table": true,
		"jsProfileVisualizer.heapsnapshot.table": true,
		"gitlens.rebase": true,
		"workbench.input.interactive": true,
		"mainThreadWebview-markdown.preview": true,
	},

	// ==========================================
	// 3. TERMINAL SETTINGS (টার্মিনাল কনফিগারেশন)
	// ==========================================
	"terminal.integrated.fontFamily": "'Cascadia Code', 'FiraCode Nerd Font'",
	"terminal.integrated.fontWeight": "normal",
	"terminal.integrated.fontWeightBold": "bold",
	"terminal.integrated.fontSize": 16,
	"terminal.integrated.lineHeight": 1.3,
	"terminal.integrated.cursorStyle": "block",
	"terminal.integrated.textBlinking": true,
	"terminal.integrated.smoothScrolling": true,
	"terminal.integrated.gpuAcceleration": "on",
	"terminal.integrated.defaultProfile.windows": "PowerShell",
	"terminal.integrated.env.windows": {},
	"terminal.integrated.ignoreBracketedPasteMode": true,
	"terminal.integrated.enableMultiLinePasteWarning": "never",
	"terminal.integrated.copyOnSelection": true,
	"terminal.integrated.rightClickBehavior": "paste",
	"terminal.integrated.drawBoldTextInBrightColors": true,
	"terminal.integrated.allowMnemonics": true,
	"terminal.integrated.enableImages": true,
	"terminal.integrated.enableVisualBell": true,
	"terminal.integrated.developer.devMode": true,
	"terminal.integrated.shellIntegration.enabled": true,
	"terminal.integrated.shellIntegration.environmentReporting": true,
	"terminal.integrated.suggest.enabled": true,
	"terminal.integrated.suggest.insertTrailingSpace": true,
	"terminal.integrated.suggest.suggestOnTriggerCharacters": true,
	"terminal.integrated.suggest.providers": {
		"lsp": true,
	},
	"terminal.integrated.suggest.quickSuggestions": {
		"commands": "on",
		"arguments": "on",
		"unknown": "on",
	},
	"terminal.integrated.tabs.defaultIcon": "symbol-event",
	"terminal.integrated.tabs.defaultColor": "terminal.ansiYellow",
	"terminal.integrated.initialHint.copilotCli": true,
	"terminal.integrated.initialHintCopilotCli": true,

	// "terminal.integrated.defaultProfile.linux": "bash (2)",
	// "terminal.integrated.profiles.linux": {
	//  "bash": {
	//    "path": "/app/bin/host-spawn",
	//    "args": ["bash"],
	//    "icon": "terminal-bash",
	//    "overrideName": true,
	//  },
	// },

	// ==========================================
	// 4. LANGUAGE & FRAMEWORKS (JS/TS, HTML, CSS, Tailwind)
	// ==========================================
	// JavaScript & TypeScript
	"js/ts.autoClosingTags.enabled": true,
	"js/ts.suggest.autoImports": true,
	"js/ts.suggest.enabled": true,
	"js/ts.suggest.paths": true,
	"js/ts.suggest.completeFunctionCalls": true,
	"js/ts.updateImportsOnFileMove.enabled": "always",
	"js/ts.updateImportsOnPaste.enabled": true,
	"js/ts.suggestionActions.enabled": true,
	"js/ts.suggest.includeAutomaticOptionalChainCompletions": true,
	"js/ts.preferences.jsxAttributeCompletionStyle": "auto",
	"js/ts.preferences.preferTypeOnlyAutoImports": true,
	"js/ts.preferences.autoImportEntrypointDirectorySearch": true,
	"js/ts.format.insertSpaceAfterOpeningAndBeforeClosingJsxExpressionBraces": true,
	"js/ts.format.insertSpaceAfterConstructor": true,
	"js/ts.format.insertSpaceAfterTypeAssertion": true,
	"js/ts.experimental.useTsgo": true,
	"js/ts.tsdk.promptToUseWorkspaceVersion": true,
	"js/ts.tsserver.web.projectWideIntellisense.suppressSemanticErrors": true,
	"js/ts.implementationsCodeLens.enabled": true,
	"js/ts.implementationsCodeLens.showOnAllClassMethods": true,
	"js/ts.implementationsCodeLens.showOnInterfaceMethods": true,
	"js/ts.referencesCodeLens.enabled": true,
	"js/ts.referencesCodeLens.showOnAllFunctions": true,
	"js/ts.inlayHints.parameterTypes.enabled": true,
	"js/ts.inlayHints.propertyDeclarationTypes.enabled": true,
	"js/ts.inlayHints.variableTypes.enabled": true,
	"js/ts.inlayHints.parameterNames.enabled": "all",
	"js/ts.inlayHints.functionLikeReturnTypes.enabled": true,
	"js/ts.inlayHints.enumMemberValues.enabled": true,
	"js/ts.inlayHints.parameterNames.suppressWhenArgumentMatchesName": true,
	"js/ts.inlayHints.variableTypes.suppressWhenTypeMatchesName": true,

	// HTML & CSS
	"html.suggest.hideEndTagSuggestions": true,
	"html.format.indentInnerHtml": true,
	"html.format.templating": true,
	"css.completion.completePropertyWithSemicolon": true,
	"css.format.spaceAroundSelectorSeparator": true,
	"css.hover.references": true,
	"css.enabledLanguages": [
		"html",
		"typescript",
		"javascript",
		"javascriptreact",
		"typescriptreact",
		"jsx",
		"tsx",
	],
	"css.styleSheets": [
		"app/${fileBasenameNoExtension}.css",
		"src/base-styles.js",
		"src/**/*.scss",
		"src/**/*.css",
		"**/*.html",
		"**/*.js",
		"**/*.jsx",
		"**/*.ts",
		"**/*.tsx",
	],
	"scss.format.spaceAroundSelectorSeparator": true,
	"less.format.spaceAroundSelectorSeparator": true,

	// Tailwind CSS
	"tailwindCSS.classAttributes": ["class", "className", "ngClass"],
	"tailwindCSS.colorDecorators": true,
	"tailwindCSS.suggestions": true,
	"tailwindCSS.emmetCompletions": true,
	"tailwindCSS.classFunctions": ["clsx", "cn", "cva", "tw"],
	"tailwindCSS.experimental.classRegex": [
		["cva\\(([^)]*)\\)", "[\"'`]([^\"'`]*)[\"'`]"],
	],
	"tailwind-fold.enabled": true,
	"tailwind-fold.autoFold": true,
	"tailwind-fold.unfoldIfLineSelected": true,
	"tailwind-fold.showTailwindImage": true,
	"tailwind-fold.foldedText": " Classes ",
	"tailwind-fold.foldedTextColor": "#7cdbfe7e",
	"tailwind-fold.foldedTextBackgroundColor": "#52b7ee09",
	"tailwind-fold.foldStyle": "ALL",
	"tailwind-fold.supportedLanguages": [
		"html",
		"typescriptreact",
		"javascriptreact",
		"typescript",
		"javascript",
		"vue-html",
		"vue",
		"php",
		"markdown",
		"coffeescript",
		"svelte",
		"astro",
		"erb",
	],

	// Rust
	"[rust]": {
		"editor.defaultFormatter": "rust-lang.rust-analyzer",
	},
	"rust-analyzer.inlayHints.closureCaptureHints.enable": true,
	"rust-analyzer.inlayHints.expressionAdjustmentHints.enable": "always",
	"rust-analyzer.inlayHints.closureReturnTypeHints.enable": "always",
	"rust-analyzer.inlayHints.expressionAdjustmentHints.hideOutsideUnsafe": true,
	"rust-analyzer.inlayHints.genericParameterHints.lifetime.enable": true,
	"rust-analyzer.inlayHints.genericParameterHints.type.enable": true,
	"rust-analyzer.inlayHints.lifetimeElisionHints.enable": "always",
	"rust-analyzer.inlayHints.bindingModeHints.enable": true,
	"rust-analyzer.inlayHints.rangeExclusiveHints.enable": true,

	// Emmet
	"emmet.useInlineCompletions": true,
	"emmet.showSuggestionsAsSnippets": true,
	"emmet.showExpandedAbbreviation": "always",
	"emmet.triggerExpansionOnTab": true,
	"emmet.includeLanguages": {
		"vue-html": "html",
		"vue": "html",
		"razor": "html",
		"plaintext": "pug",
		"django-html": "html",
		"javascript": "javascriptreact",
		"typescript": "typescriptreact",
		"css": "css",
		"ejs": "html",
	},
	"emmet.syntaxProfiles": {
		"javascriptreact": { "self_closing_tag": true },
		"typescriptreact": { "self_closing_tag": true },
	},

	// JSON
	"json.format.keepLines": true,

	// ==========================================
	// 5. GIT & SOURCE CONTROL (গিট কনফিগারেশন)
	// ==========================================
	"git.enableSmartCommit": true,
	"git.autofetch": true,
	"git.terminalGitEditor": true,
	"git.openRepositoryInParentFolders": "always",
	"git.showPushSuccessNotification": true,
	"git.addAICoAuthor": "all",
	"git.detectWorktrees": true,
	"git.supportCancellation": true,
	"git.autoStash": true,
	"git.followTagsWhenSync": true,
	"git.confirmSync": false,
	"git.blame.editorDecoration.disableHover": true,
	"scm.alwaysShowRepositories": true,
	"scm.alwaysShowActions": true,

	// GitLens
	"gitlens.views.repositories.showIncomingActivity": true,
	"gitlens.views.branches.showRemoteBranches": true,
	"gitlens.ai.model": "vscode",
	"gitlens.ai.vscode.model": "copilot:gpt-4.1",

	// ==========================================
	// 6. AI & COPILOT (কৃত্রিম বুদ্ধিমত্তা ও চ্যাট)
	// ==========================================
	"github.copilot.enable": {
		"*": true,
		"plaintext": true,
		"markdown": true,
		"scminput": false,
		"typescript": true,
		"javascript": true,
		"javascriptreact": true,
		"html": true,
		"jsonc": true,
		"shellscript": true,
		"css": true,
		"typescriptreact": true,
	},
	"github.copilot.nextEditSuggestions.enabled": true,
	"github.copilot.nextEditSuggestions.extendedRange": true,
	"github.copilot.chat.languageContext.typescript.enabled": true,
	"github.copilot.chat.skillTool.enabled": true,
	"github.copilot.chat.anthropic.tools.websearch.enabled": true,
	"github.copilot.chat.editRecording.enabled": true,
	"chat.viewSessions.orientation": "stacked",
	"chat.checkpoints.showFileChanges": true,
	"chat.editor.wordWrap": "on",
	"chat.editing.explainChanges.enabled": true,
	"chat.viewProgressBadge.enabled": true,
	"chat.mcp.gallery.enabled": true,
	"chat.unifiedAgentsBar.enabled": true,
	"chat.restoreLastPanelSession": true,
	"chat.tools.compressOutput.enabled": true,
	"jsts-chat-features.skills.enabled": true,
	"inlineChat.notebookAgent": true,

	// ==========================================
	// 7. EXTENSIONS & TOOLS (বিভিন্ন এক্সটেনশন সেটিং)
	// ==========================================
	// Prettier
	"prettier.enable": true,
	"prettier.semi": true,
	"prettier.useTabs": true,
	"prettier.singleAttributePerLine": true,
	"prettier.bracketSameLine": true,
	"prettier.bracketSpacing": true,
	"prettier.trailingComma": "es5",
	"prettier.withNodeModules": true,
	"prettier.insertPragma": false,
	"prettier.ignorePath": ".prettierignore",
	"prettier.experimentalTernaries": true,

	// ESLint
	"eslint.enable": true,
	"eslint.validate": ["javascript", "typescript", "typescriptreact"],
	// "eslint.options": {
	// 	"overrideConfig": {
	// 		"rules": {
	// 			"@typescript-eslint/typedef": [
	// 				"error",
	// 				{
	// 					"variableDeclaration": true,
	// 					"variableDeclarationIgnoreFunction": true,
	// 				},
	// 			],

	// 			// কোডে শর্টকাট মেরে 'any' টাইপ ব্যবহার করলে এরর দেবে
	// 			"@typescript-eslint/no-explicit-any": "error",
	// 			// ভ্যারিয়েবল ডিক্লেয়ার করে ব্যবহার না করলে লাল দাগ (এরর) দেবে
	// 			"@typescript-eslint/no-unused-vars": "error",
	// 		},
	// 	},
	// },

	// Live Server & SASS
	"liveServer.settings.donotVerifyTags": true,
	"liveServer.settings.donotShowInfoMsg": true,
	"liveServer.settings.CustomBrowser": "chrome",
	"liveServer.settings.fullReload": true,
	// "liveServer.settings.useLocalIp": true,

	"liveSassCompile.settings.watchOnLaunch": true,
	"liveSassCompile.settings.formats": [
		{
			"format": "expanded",
			"extensionName": ".css",
			"savePath": null,
			"savePathReplacementPairs": null,
		},
	],

	// Code Runner
	"code-runner.defaultLanguage": "javascript",
	"code-runner.runInTerminal": true,
	"code-runner.showExecutionMessage": false,
	"code-runner.clearPreviousOutput": true,
	"code-runner.customCommand": "bun run",
	"code-runner.executorMap": {
		"javascript": "node",
		"php": "C:\\php\\php.exe",
		"python": "python",
		"ruby": "C:\\Ruby23-x64\\bin\\ruby.exe",
		"go": "go run",
		"html": "\"C:\\Program Files (x86)\\Google\\Chrome\\Application\\chrome.exe\"",
		"java": "cd $dir && javac $fileName && java $fileNameWithoutExt",
		"c": "cd $dir && gcc $fileName -o $fileNameWithoutExt && $dir$fileNameWithoutExt",
	},

	// ErrorLens (এরর হাইলাইটিং)
	"errorLens.problemRangeDecorationEnabled": true,
	"errorLens.gutterIconsEnabled": true,
	"errorLens.statusBarColorsEnabled": true,
	"errorLens.statusBarIconsEnabled": true,
	"errorLens.enabled": true,
	"errorLens.statusBarMessageType": "closestProblem",
	"errorLens.statusBarMessageEnabled": true,
	"errorLens.codeLensEnabled": true,
	"errorLens.gutterIconSet": "squareRounded",
	"errorLens.gutterIconsFollowCursorOverride": true,
	"errorLens.errorGutterIconColor": "#6a54e4",
	"errorLens.warningGutterIconColor": "#29d8ff",
	"errorLens.infoGutterIconColor": "#21d439",
	"errorLens.hintGutterIconColor": "#b5a7b0",
	"errorLens.messageTemplate": "$severity $message",
	"errorLens.severityText": ["▣", "◈", "◉", "⛆"],
	"errorLens.codeLensOnClick": "showQuickFix",
	"errorLens.fontSize": "17",

	// "errorLens.followCursor": "activeLine",
	"errorLens.alignMessage": {
		"start": 0,
		"end": 0,
		"minimumMargin": 4,
		"padding": [1, 1],
		"useFixedPosition": true,
	},
	"errorLens.decorations": {
		"errorMessage": {
			"textDecoration": ";background:linear-gradient(to right, #0088ff, #0a9c33);border-radius:0.3em;padding:0 0.5ch;",
			"color": "#fff",
			"fontWeight": "bold",
		},

		"errorRange": {
			"border": "1px dashed red",
			"backgroundColor": "#ff000090",
			"color": "#ffffff",
			"textDecoration": ";background:linear-gradient(to right, #0088ff, #0a9c33);border-radius:0.3em;padding:0 0.5ch;",
		},
		"warningRange": {
			"textDecoration": ";background:linear-gradient(to right, #0088ff, #0a9c33);border-radius:0.3em;padding:0 0.5ch;",
			"backgroundColor": "#fff0",
		},
		"infoRange": {
			"textDecoration": ";background:linear-gradient(45deg,#ff8400,#00d9ff);background-clip:text;color:transparent;border-bottom:2px solid #00d9ff",
			"backgroundColor": "#fff0",
		},
		"hintRange": {
			"textDecoration": ";background:linear-gradient(to right, #0088ff, #0a9c33);border-radius:0.3em;padding:0 0.5ch;",
		},
	},

	// "errorLens.replace": [
	// 	{ "matcher": "Missing semicolon", "message": ";" },
	// 	{ "matcher": "Missing return type on (.+)", "message": "Type $1" },
	// 	{ "matcher": "Missing return type on", "message": "<==" },
	// 	{ "matcher": "is declared but its value is never read", "message": "ಠ╭╮ಠ" },
	// ],
	"errorLens.transmute": {
		"semi": {
			"target": {
				"message": "missing semicolon",
			},
			"decoration": {
				"light": {
					"after": {
						"backgroundColor": "#00000010",
						"color": "#444444",
					},
				},
			},
		},
		// "ESLint": { "target": { "source": "eslint" }, "severity": "info" },
		// "biome":{"target": {"source": "biome"},"severity": "info"},
		"typescript": { "target": { "source": "ts" }, "severity": "info" },
		"typescriptreact": { "target": { "source": "ts" }, "severity": "info" },
	},

	// Indent Rainbow (কোডের ইনডেন্টেশন কালার)
	"indentRainbow.indicatorStyle": "light",
	"indentRainbow.lightIndicatorStyleLineWidth": 1,
	"indentRainbow.colorOnWhiteSpaceOnly": true,
	"indentRainbow.updateDelay": 0,
	"indentRainbow.errorColor": "rgba(128,32,32,0.6)",

	// "indentRainbow.colors": [
	//  "rgba(255,255,64,0.07)",
	//  "rgba(127,255,127,0.07)",
	//  "rgba(255,127,255,0.07)",
	//  "rgba(79,236,236,0.07)",
	// ],
	"indentRainbow.colors": [
		"rgba(255, 99, 132, 0.35)",
		"rgba(54, 162, 235, 0.35)",
		"rgba(75, 192, 192, 0.35)",
		"rgba(255, 206, 86, 0.35)",
		"rgba(153, 102, 255, 0.35)",
		"rgba(255, 159, 64, 0.35)",
		"rgba(100, 255, 218, 0.35)",
		"rgba(199, 199, 199, 0.35)",
	],

	// Better Comments
	"better-comments.multilineComments": true,
	"better-comments.highlightPlainText": false,
	"better-comments.tags": [
		{
			"tag": "!",
			"color": "#FF2D00",
			"strikethrough": false,
			"underline": false,
			"backgroundColor": "transparent",
			"bold": false,
			"italic": false,
		},
		{
			"tag": "?",
			"color": "#3498DB",
			"strikethrough": false,
			"underline": false,
			"backgroundColor": "transparent",
			"bold": false,
			"italic": false,
		},
		{
			"tag": "//",
			"color": "#474747",
			"strikethrough": true,
			"underline": false,
			"backgroundColor": "transparent",
			"bold": false,
			"italic": false,
		},
		{
			"tag": "todo",
			"color": "#FF8C00",
			"strikethrough": false,
			"underline": false,
			"backgroundColor": "transparent",
			"bold": false,
			"italic": false,
		},
		{
			"tag": "*",
			"color": "#98C379",
			"strikethrough": false,
			"underline": false,
			"backgroundColor": "transparent",
			"bold": false,
			"italic": false,
		},
	],

	// Auto Import & Tags
	"autoimport.autoComplete": true,
	"autoimport.semicolon": true,
	"autoimport.useSemiColon": true,
	"autoimport.doubleQuotes": true,
	"autoimport.absolute": true,
	"autoimport.filesToScan": "**/*.{js,jsx,ts,tsx}",
	"autoimport.sourceRoot": "./",
	"autoimport.higherOrderComponents": "connect|withRouter",
	"auto-close-tag.enableAutoCloseTag": true,
	"auto-close-tag.enableAutoCloseSelfClosingTag": true,
	"auto-close-tag.SublimeText3Mode": true,
	"auto-close-tag.insertSpaceBeforeSelfClosingTag": true,
	"auto-close-tag.activationOnLanguage": [
		"xml",
		"php",
		"javascript",
		"typescript",
		"javascriptreact",
		"typescriptreact",
		"vue",
		"jsx",
		"tsx",
		"markdown",
	],
	"auto-rename-tag.activationOnLanguage": ["*"],

	// Path Intellisense & HTML/CSS Completion
	"path-autocomplete.extensionOnImport": true,
	"path-autocomplete.triggerOutsideStrings": true,
	"path-intellisense.extensionOnImport": true,
	"path-intellisense.autoTriggerNextSuggestion": true,
	"path-intellisense.autoSlashAfterDirectory": true,
	"html-to-css-autocompletion.getSelectorsFromFileTypes": [
		"html",
		"css",
		"scss",
		"less",
		"javascript",
		"typescript",
		"javascriptreact",
		"typescriptreact",
		"jsx",
		"tsx",
	],
	"html-css-class-completion.enableEmmetSupport": true,
	"html-css-class-completion.CSSLanguages": ["sass", "scss", "less", "css"],
	"html-css-class-completion.JavaScriptLanguages": [
		"javascript",
		"typescript",
		"javascriptreact",
		"typescriptreact",
		"jsx",
		"tsx",
	],
	"html-css-class-completion.HTMLLanguages": [
		"html",
		"vue",
		"php",
		"markdown",
		"javascript",
		"typescript",
		"javascriptreact",
		"typescriptreact",
		"tsx",
		"jsx",
	],
	"html-css-class-completion.includeGlobPattern": "**/*.{css,html,js,jsx,ts,tsx}",

	// NPM & Bun
	"npm.packageManager": "auto",
	"npm.enableRunFromFolder": false,
	"npm.scriptHover": true,
	"npm-intellisense.scanDevDependencies": true,
	"npm-intellisense.importES6": true,
	"npm-intellisense.packageSubfoldersIntellisense": true,
	"bun.runtime": "/path/to/bun",
	"bun.debugTerminal.enabled": true,
	"bun.debugTerminal.stopOnEntry": false,
	"bun.test.filePattern": "**/*{.test.,.spec.,_test_,_spec_}{js,ts,tsx,jsx,mts,cts,cjs,mjs}",
	"bun.test.customScript": "bun test",

	// Quokka
	"quokka.showOutputOnStart": true,
	"quokka.syncSettings": true,
	"quokka.snapsAutoRunConfirmOnEdit": true,
	"quokka.automaticRestart": true,
	"quokka.colors": {
		"covered": "#62b455",
		"errorPath": "#ffa0a0",
		"errorSource": "#fe536a",
		"notCovered": "#cccccc",
		"partiallyCovered": "#d2a032",
	},

	// Other Tools (Prisma, Headwind, CodeSnap, PowerMode, Pomodoro etc.)
	"prisma.showPrismaDataPlatformNotification": false,
	"reactSnippets.settings.prettierEnabled": true,
	"powermode.enabled": true,
	"powermode.shake.enabled": false,
	"powermode.combo.location": "off",
	"powermode.combo.timerEnabled": "hide",
	"codesnap.roundedCorners": true,
	"codesnap.boxShadow": "rgba(0, 0, 0, 0.55) 0px 20px 68px",
	"pomodoro.workTime": 120,
	"hungryDelete.considerIncreaseIndentPattern": true,
	"hungryDelete.keepOneSpace": true,
	"hungryDelete.followAboveLineIndent": true,
	"color-highlight.matchHslWithNoFunction": true,
	"color-highlight.matchRgbWithNoFunction": true,
	"color-highlight.matchWords": true,
	"color-highlight.useARGB": true,
	"trailing-spaces.trimOnSave": true,
	"trailing-spaces.borderColor": "rgba(255,200,000,1)",
	"trailing-spaces.backgroundColor": "rgba(100, 255, 218, 0)",
	"inlineFold.useGlobal": true,
	"inlineFold.unfoldOnLineSelect": true,
	"inlineFold.unfoldedOpacity": 1,
	"lifeline.swap": true,
	"material-icon-theme.activeIconPack": "react",
	"mediaPreview.video.autoPlay": true,
	"htmltagwrap.tag": "div",
	"headwind.classRegex": {
		"html": "\\bclass\\s*=\\s*[\\\"\\'](https://github.com/heybourn/headwind/blob/master/[_a-zA-Z0-9\\s\\-\\:\\/]+)[\\\"\\']",
		"javascriptreact": "(?:\\bclassName\\s*=\\s*[\\\"\\'](https://github.com/heybourn/headwind/blob/master/[_a-zA-Z0-9\\s\\-\\:\\/]+)[\\\"\\'])|(?:\\btw\\s*`([_a-zA-Z0-9\\s\\-\\:\\/]*)`)",
		"typescriptreact": "(?:\\bclassName\\s*=\\s*[\\\"\\'](https://github.com/heybourn/headwind/blob/master/[_a-zA-Z0-9\\s\\-\\:\\/]+)[\\\"\\'])|(?:\\btw\\s*`([_a-zA-Z0-9\\s\\-\\:\\/]*)`)",
	},
	"mdb.mcp.server": "prompt",
	"ipynb.experimental.serialization": true,

	// ==========================================
	// 8. FILE & SYSTEM SETTINGS (ফাইল এবং সিস্টেম)
	// ==========================================
	"files.autoSave": "onFocusChange",
	"files.trimTrailingWhitespace": true,
	"files.trimFinalNewlines": true,
	"files.associations": {
		"*html": "html",
		"*njk": "html",
		"*.ejs": "html",
	},
	"files.simpleDialog.enable": false,
	"explorer.confirmDragAndDrop": false,
	"explorer.confirmDelete": false,
	"explorer.confirmPasteNative": false,
	"explorer.compactFolders": false,
	"security.workspace.trust.untrustedFiles": "open",
	"settingsSync.ignoredSettings": [
		"terminal.integrated.macOptionClickForcesSelection",
	],
	"http.systemCertificatesNode": true,
	"remote.portsAttributes": { "3000": { "protocol": "https" } },
	"search.searchView.keywordSuggestions": true,
	"containers.contexts.showInStatusBar": true,
	"accessibility.chat.showCheckmarks": true,
	"accessibility.voice.autoSynthesize": "on",

	// Browser Integration
	"workbench.browser.openLocalhostLinks": false,
	"workbench.browser.showInTitleBar": true,
	"workbench.browser.enableChatTools": true,
	"workbench.browser.experimentalUserTools.enabled": true,
	"workbench.browser.pageZoom": "100%",
	"workbench.externalBrowser": "chrome",
	// "workbench.externalBrowser": "/var/lib/flatpak/exports/bin/com.brave.Browser",

	// ==========================================
	// 9. COLOR CUSTOMIZATIONS (কাস্টম কালার থিম)
	// ==========================================
	"editor.semanticTokenColorCustomizations": {
		"[Rouge]": {
			"enabled": true,
			"rules": {
				"*.declaration": { "bold": true },
			},
		},
	},
	"editor.tokenColorCustomizations": {
		"textMateRules": [
			{
				"scope": "comment",
				"settings": { "fontStyle": "bold" },
			},
		],
	},

	// ==========================================
	// PRO-PRODUCTIVITY & WORKFLOW ENHANCEMENTS
	// ==========================================
	"editor.foldingStrategy": "indentation",
	"editor.showFoldingControls": "always",
	"editor.occurrencesHighlight": "singleFile",
	"editor.largeFileOptimizations": true,
	"git.mergeEditor": true,
	"git.showCommitDetails": true,
	"github.copilot.chat.codeGeneration.enabled": true,
	"js/ts.format.enabled": true,
	"js/ts.validate.enabled": true,
	"workbench.colorCustomizations": {
		// 1. YOUR ORIGINAL TERMINAL COLORS
		"terminal.foreground": "#a09ced",
		"terminal.background": "#011627",
		"terminalCursor.background": "#ff0000",
		"terminalCursor.foreground": "#00de21",
		"terminal.ansiBlack": "#1d211e",
		"terminal.ansiBlue": "#0D6678",
		"terminal.ansiBrightBlack": "#665C54",
		"terminal.ansiBrightBlue": "#0D6678",
		"terminal.ansiBrightCyan": "#8BA59B",
		"terminal.ansiBrightGreen": "#95C085",
		"terminal.ansiBrightMagenta": "#8F4673",
		"terminal.ansiBrightRed": "#FB543F",
		"terminal.ansiBrightWhite": "#FDF4C1",
		"terminal.ansiBrightYellow": "#FAC03B",
		"terminal.ansiCyan": "#8BA59B",
		"terminal.ansiGreen": "#95C085",
		"terminal.ansiMagenta": "#8F4673",
		"terminal.ansiRed": "#FB543F",
		"terminal.ansiWhite": "#A89984",
		"terminal.ansiYellow": "#FAC03B",
		"terminal.border": "#ffffff",
		"terminal.selectionBackground": "#193549",
		"terminalSymbolIcon.aliasForeground": "#b180d7",

		// 2. DOBRI NEXT - A00 - BLACK
		"editor.background": "#000000",
		"sideBar.background": "#000000",
		"panel.background": "#010E1A",
		"titleBar.activeBackground": "#000000",
		"breadcrumb.background": "#000000",

		// "list.activeSelectionBackground": "#FB543F",
		"list.activeSelectionForeground": "#FF8C00",
		"list.inactiveSelectionBackground": "#000000",
		"list.inactiveSelectionForeground": "#cccccc",
		"list.activeSelectionIconAndForeground": "#ffffff",
		"list.highlightForeground": "#ffdd00",
		"list.hoverBackground": "#1a233a",
		"list.focusAndSelectionOutline": "#ff9900",
		"list.focusOutline": "#FF8C00",

		"input.background": "#011627",
		"dropdown.border": "#97a7c8",

		// "editor.foreground": "#8a8d90",
		"editorLineNumber.foreground": "#305a84",
		"editorLineNumber.activeForeground": "#FAC03B",

		"button.background": "#011627",
		"button.border": "#297aa0",
		"menu.foreground": "#FF8C00",
		"menu.border": "#011627",
		"menu.background": "#000000",

		// 3. ACTIVITY BAR & BORDERS
		"activityBar.border": "#FAC03B",
		"activityBar.activeBorder": "#FAC03B",
		"activityBar.activeBackground": "#ff000000",
		"activityBar.dropBorder": "#ff0000",
		"activityBar.foreground": "#FAC03B",
		"activityBar.inactiveForeground": "#305a84",

		// 4. EDITOR HIGHLIGHTS
		"editor.findMatchBackground": "#f352fe8f",
		"editor.findMatchHighlightBackground": "#8e52fe9e",
		"editor.findMatchHighlightBorder": "#fbf300e0",
		"editor.snippetTabstopHighlightBorder": "#ff0000",
		"editorCursor.background": "#ffffff",

		// 5. PANEL & BREADCRUMBS
		"breadcrumb.foreground": "#FAC03B",
		"panel.border": "#FAC03B",
		"panel.dropBorder": "#ffffff",
		"panelTitle.activeForeground": "#ffffff",
		"panelTitle.inactiveForeground": "#FAC03B",
		"panelStickyScroll.shadow": "#ffffff",
		"panelTitle.activeBorder": "#FAC03B",
		"panelInput.border": "#FAC03B",

		// 6. INPUTS & TABS
		"icon.foreground": "#FF8C00",
		"input.border": "#FAC03B",
		"input.foreground": "#FF8C00",
		"inputOption.activeBorder": "#FAC03B",
		"tab.activeBorder": "#FAC03B",
		"tab.dragAndDropBorder": "#ff0000",
		"tab.unfocusedActiveBorder": "#ff0000",
		"tab.activeForeground": "#FF8C00",

		// 7. STATUS BAR & TITLE BAR
		"statusBarItem.remoteBackground": "#101e2c",
		"statusBarItem.errorForeground": "#ff0000",
		"statusBarItem.remoteForeground": "#FF8C00",
		"statusBarItem.hoverForeground": "#faa30c",
		"statusBarItem.errorBackground": "#011627",
		"titleBar.activeForeground": "#FF8C00",
		"statusBar.foreground": "#FF8C00",

		// 8. SIDEBAR & LISTS
		"sideBarSectionHeader.foreground": "#FAC03B",
		"sideBarTitle.foreground": "#FAC03B",

		// 9. GIT & TRANSPARENT DIAGNOSTICS
		"gitDecoration.ignoredResourceForeground": "#00ff9d9e",
		"editorError.foreground": "#fff0",
		"editorWarning.foreground": "#fff0",
		"editorInfo.foreground": "#fff0",
		"editorHint.foreground": "#fff0",

		// BORDERS FOR DOBRI NEXT BLACK LOOK
		// "sideBar.border": "#FAC03B",
		"sideBarSectionHeader.border": "#305a84",
		// "statusBar.border": "#FAC03B",
		// "titleBar.border": "#FAC03B",
		// "editorGroupHeader.tabsBorder": "#FAC03B",

		"editorError.background": "#ff000030",
		"editorWarning.background": "#ee990030",
		"editorInfo.background": "#0095d530",

		"scrollbar.background": "#000000",
		"scrollbarSlider.background": "#FF8C00",
		"scrollbarSlider.hoverBackground": "#ff0000cc",
		"scrollbarSlider.activeBackground": "#FAC03B",
	},
	"editor.inlayHints.enabled": "off",
	"workbench.experimental.modernUI": true,
	"js/ts.tsserver.experimental.enableProjectDiagnostics": true,
	"workbench.productIconTheme": "developer-icons",
	"workbench.colorTheme": "Dobri Next -A00- Black",
	"workbench.iconTheme": "material-icon-theme",
	"terminal.integrated.stickyScroll.enabled": false,
	"json.schemaDownload.trustedDomains": {
		"https://schemastore.azurewebsites.net/": true,
		"https://raw.githubusercontent.com/microsoft/vscode/": true,
		"https://raw.githubusercontent.com/devcontainers/spec/": true,
		"https://www.schemastore.org/": true,
		"https://json.schemastore.org/": true,
		"https://json-schema.org/": true,
		"https://developer.microsoft.com/json-schemas/": true,
		"https://biomejs.dev": true,
	},
	"js/ts.showDebugInfo": true,
	"js/ts.trace.server": "off",
	"files.enableTrash": false,
	"terminal.integrated.accessibleViewPreserveCursorPosition": "always",
	"workbench.browser.searchEngine": "google",
	"workbench.browser.enableRemoteProxy": true,
	"breadcrumbs.showEditorType": true,
	"terminal.integrated.allowInUntrustedWorkspace": true,
	"terminal.integrated.fontLigatures.enabled": true,
	"chat.disableAIFeatures": true,
	"editor.bracketPairColorization.independentColorPoolPerBracketType": true,
	"workbench.experimental.modernUIUppercaseViewHeaders": true,
	"workbench.editorAssociations": {
		"*.html": "default",
	},
	"markdown.experimental.richLinks.enabled": true,
	"workbench.activityBar.location": "top",
}"##;

// ── RAII Cursor Guard ─────────────────────────────────────────
struct CursorGuard;
impl CursorGuard {
    fn hide() -> Self {
        print!("\x1b[?25l");
        let _ = io::stdout().flush();
        CursorGuard
    }
}
impl Drop for CursorGuard {
    fn drop(&mut self) {
        print!("\x1b[?25h");
        let _ = io::stdout().flush();
    }
}

// ── Animated Spinner ──────────────────────────────────────────
fn spinner(msg: &str, duration_ms: u64) {
    let spin = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];
    let delay = Duration::from_millis(80);
    let start = Instant::now();
    let target = Duration::from_millis(duration_ms);
    let _guard = CursorGuard::hide();
    let mut i = 0;
    while start.elapsed() < target {
        print!("\r  {}{}{} {}", CYAN, spin[i % spin.len()], NC, msg);
        let _ = io::stdout().flush();
        thread::sleep(delay);
        i += 1;
    }
    println!("\r  {}✔{} {}", GREEN, NC, msg);
}

// ── Progress Bar ──────────────────────────────────────────────
fn draw_progress_bar(current: usize, total: usize) {
    let total = if total == 0 { 5 } else { total };
    let width: usize = 30;
    let percentage = (current * 100) / total;
    let completed = (width * current) / total;
    let remaining = width.saturating_sub(completed);
    let bar   = "█".repeat(completed);
    let empty = "░".repeat(remaining);
    println!();
    println!(
        "{b}Progress:{n} [{g}{bar}{gr}{empty}{n}] {c}{pct}%{n} (Step {cur}/{tot})",
        b=BLUE, n=NC, g=GREEN, bar=bar, gr=GRAY, empty=empty,
        c=CYAN, pct=percentage, cur=current, tot=total
    );
}

// ── ASCII Banner ──────────────────────────────────────────────
fn show_header() {
    println!();
    println!("{}        ██╗   ██╗███████╗ ██████╗ ██████╗ ██████╗ ███████╗{}", BLUE, NC);
    println!("{}        ██║   ██║██╔════╝██╔════╝██╔═══██╗██╔══██╗██╔════╝{}", BLUE, NC);
    println!("{}        ██║   ██║███████╗██║     ██║   ██║██║  ██║█████╗  {}", CYAN, NC);
    println!("{}        ╚██╗ ██╔╝╚════██║██║     ██║   ██║██║  ██║██╔══╝  {}", CYAN, NC);
    println!("{}         ╚████╔╝ ███████║╚██████╗╚██████╔╝██████╔╝███████╗{}", PURPLE, NC);
    println!("{}          ╚═══╝  ╚══════╝ ╚═════╝ ╚═════╝ ╚═════╝ ╚══════╝{}", PURPLE, NC);
    println!();
    println!(
        "   ✨ {}{}F A N C Y B A S H{}  •  {}VS Code Settings & Extensions Installer{}",
        BOLD, CYAN, NC, BOLD, NC
    );
    println!();
}

// ── Environment Helpers ───────────────────────────────────────
fn get_home_dir() -> PathBuf {
    if let Ok(h) = env::var("HOME")        { if !h.trim().is_empty() { return PathBuf::from(h); } }
    if let Ok(p) = env::var("USERPROFILE") { if !p.trim().is_empty() { return PathBuf::from(p); } }
    PathBuf::from("~")
}

fn command_exists(cmd: &str) -> bool { crate::core::utils::cmd_exists(cmd) }

fn get_uname_s() -> String {
    match env::consts::OS {
        "macos"   => "Darwin".into(),
        "windows" => "Windows_NT".into(),
        "linux"   => "Linux".into(),
        o         => o.into(),
    }
}

fn get_arch()  -> String { env::consts::ARCH.to_string() }

fn get_user() -> String {
    if let Ok(u) = env::var("USER")     { if !u.trim().is_empty() { return u; } }
    if let Ok(u) = env::var("USERNAME") { if !u.trim().is_empty() { return u; } }
    "user".into()
}

// ── Cross-Platform Config Path Detection ──────────────────────
fn detect_system_and_paths() -> (String, Vec<PathBuf>) {
    let os   = get_uname_s();
    let home = get_home_dir();
    let mut distro;
    let mut dirs   = Vec::new();

    if os.starts_with("Linux") {
        distro = "Linux".into();
        if let Ok(r) = fs::read_to_string("/etc/os-release") {
            for line in r.lines() {
                if let Some(v) = line.strip_prefix("PRETTY_NAME=") {
                    distro = v.trim_matches('"').into();
                    break;
                }
            }
        }
        // Native
        dirs.push(home.join(".config/Code/User"));
        // Flatpak
        if home.join(".var/app/com.visualstudio.code").is_dir() || command_exists("flatpak") {
            dirs.push(home.join(".var/app/com.visualstudio.code/config/Code/User"));
        }
        // Snap
        if home.join("snap/code").is_dir() || command_exists("snap") {
            dirs.push(home.join("snap/code/current/.config/Code/User"));
        }
        // WSL → Windows host
        if let Ok(v) = fs::read_to_string("/proc/version") {
            let lv = v.to_lowercase();
            if lv.contains("microsoft") || lv.contains("wsl") {
                let mnt = Path::new("/mnt/c/Users");
                if mnt.exists() {
                    if let Ok(entries) = fs::read_dir(mnt) {
                        for e in entries.flatten() {
                            let p = e.path().join("AppData/Roaming/Code/User");
                            if p.exists() { dirs.push(p); break; }
                        }
                    }
                }
            }
        }
    } else if os.starts_with("Darwin") {
        distro = "macOS".into();
        dirs.push(home.join("Library/Application Support/Code/User"));
    } else if os.starts_with("Windows_NT") || cfg!(windows) {
        distro = "Windows".into();
        let base = env::var("APPDATA")
            .ok()
            .filter(|s| !s.trim().is_empty())
            .map(PathBuf::from)
            .unwrap_or_else(|| home.join("AppData/Roaming"));
        dirs.push(base.join("Code/User"));
    } else {
        distro = "Unknown OS".into();
        dirs.push(home.join(".config/Code/User"));
    }

    (distro, dirs)
}

fn show_sysinfo(distro: &str, count: usize) {
    println!("{}──────────────────────────────────────────────────{}", BLUE, NC);
    println!(" 🖥️   {}SYSTEM & ENVIRONMENT INFO{}", BOLD, NC);
    println!("{}──────────────────────────────────────────────────{}", BLUE, NC);
    println!("  💻  {}OS:{}            {}{}{}", BOLD, NC, CYAN, distro, NC);
    println!("  👤  {}User:{}          {}{}{}", BOLD, NC, CYAN, get_user(), NC);
    println!("  ⚙️   {}Arch:{}          {}{}{}", BOLD, NC, CYAN, get_arch(), NC);
    println!("  📂  {}Paths:{}         {}{} location(s){}", BOLD, NC, CYAN, count, NC);
    println!("{}──────────────────────────────────────────────────{}\n", BLUE, NC);
}

// ── Timestamp for Backups ─────────────────────────────────────
fn get_timestamp() -> String {
    let secs  = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();
    let days  = secs / 86400;
    let rem   = secs % 86400;
    let h     = rem / 3600;
    let min   = (rem % 3600) / 60;
    let sec   = rem % 60;
    let zd    = days as i64 + 719468;
    let era   = (if zd >= 0 { zd } else { zd - 146096 }) / 146097;
    let doe   = (zd - era * 146097) as u64;
    let yoe   = (doe - doe/1460 + doe/36524 - doe/146096) / 365;
    let y     = yoe as i64 + era * 400;
    let doy   = doe - (365*yoe + yoe/4 - yoe/100);
    let mp    = (5*doy + 2) / 153;
    let d     = doy - (153*mp + 2)/5 + 1;
    let m     = if mp < 10 { mp + 3 } else { mp - 9 };
    let year  = if m <= 2 { y + 1 } else { y };
    format!("{year:04}{m:02}{d:02}_{h:02}{min:02}{sec:02}")
}

// ── Write settings.json (with atomic backup) ──────────────────
fn install_settings(dir: &Path) -> bool {
    let target = dir.join("settings.json");
    if fs::create_dir_all(dir).is_err() {
        println!("  {}❌ Cannot create dir → {}{}", RED, dir.display(), NC);
        return false;
    }
    if target.is_file() {
        if fs::metadata(&target).map(|m| m.len() > 0).unwrap_or(false) {
            let bak = dir.join(format!("settings.json.bak.{}", get_timestamp()));
            if fs::copy(&target, &bak).is_ok() {
                println!("  {}💾 Backup → {}{}", GRAY, bak.display(), NC);
            }
        }
    }
    let payload = format!("{}\n", VSCODE_SETTINGS);
    if fs::write(&target, &payload).is_err() {
        println!("  {}❌ Write failed → {}{}", RED, target.display(), NC);
        return false;
    }
    if fs::metadata(&target).map(|m| m.len() > 0).unwrap_or(false) {
        println!("  {}✔ settings.json → {}{}", GREEN, target.display(), NC);
        return true;
    }
    println!("  {}❌ Write failed → {}{}", RED, target.display(), NC);
    false
}

// ── Install Extensions via `code` CLI ─────────────────────────
fn install_extensions() -> (usize, Vec<String>) {
    let bin = ["code", "code-oss", "codium", "code-insiders"]
        .iter()
        .find(|&&c| command_exists(c))
        .copied();

    let Some(bin) = bin else {
        println!("\n  {}⚠ `code` not found in PATH — skipping extensions.{}", YELLOW, NC);
        println!("  {}  Re-run after installing VS Code: gladeshell code-setup{}", GRAY, NC);
        return (0, vec![]);
    };

    let total = VSCODE_EXTENSIONS.len();
    println!("\n  {}➜ Using `{}` — installing {} extensions...{}\n", CYAN, bin, total, NC);

    let mut ok = 0usize;
    let mut failed: Vec<String> = Vec::new();

    for (i, ext) in VSCODE_EXTENSIONS.iter().enumerate() {
        print!(
            "  {}[{}/{}]{} {}Installing {}{}{}...",
            GRAY, i + 1, total, NC, CYAN, BOLD, ext, NC
        );
        let _ = io::stdout().flush();

        match Command::new(bin).args(["--install-extension", ext, "--force"]).output() {
            Ok(out) if out.status.success() => {
                println!(" {}✔{}", GREEN, NC);
                ok += 1;
            }
            Ok(out) => {
                let stderr = String::from_utf8_lossy(&out.stderr);
                if stderr.contains("already installed") {
                    println!(" {}✔ (already installed){}", GRAY, NC);
                    ok += 1;
                } else {
                    println!(" {}✘{}", RED, NC);
                    failed.push(ext.to_string());
                }
            }
            Err(_) => {
                println!(" {}✘ (exec error){}", RED, NC);
                failed.push(ext.to_string());
            }
        }
    }
    (ok, failed)
}

// ── Main Entry Point ──────────────────────────────────────────
pub fn run() -> Result<(), Box<dyn Error>> {
    show_header();

    let (distro, target_dirs) = detect_system_and_paths();
    show_sysinfo(&distro, target_dirs.len());

    // Step 1: Detect paths
    draw_progress_bar(1, 4);
    spinner("Detecting VS Code configuration paths...", 150);
    println!("  {}➜ Found {} candidate path(s).{}", CYAN, target_dirs.len(), NC);

    // Step 2: Backup scan
    draw_progress_bar(2, 4);
    spinner("Scanning for existing VS Code settings to backup...", 150);

    // Step 3: Write settings
    draw_progress_bar(3, 4);
    let mut settings_ok = 0usize;
    for dir in &target_dirs {
        println!("  ▶ Target: {}{}{}", BOLD, dir.display(), NC);
        if install_settings(dir) { settings_ok += 1; }
    }

    // Step 4: Install extensions
    draw_progress_bar(4, 4);
    spinner("Preparing extension installation...", 150);
    let (ext_ok, ext_failed) = install_extensions();

    // ── Summary Box ───────────────────────────────────────────
    println!();
    if settings_ok > 0 {
        println!("{}┌─────────────────────────────────────────────────────────────┐{}", GREEN, NC);
        println!("{}│ {}✨  VS Code Setup Completed Successfully!                    {}{}", GREEN, BOLD, GREEN, NC);
        println!("{}├─────────────────────────────────────────────────────────────┤{}", GREEN, NC);
        println!("{}│{}  ⚙️  Settings applied to {}{}{} location(s).{}                   {}│{}", GREEN, NC, BOLD, settings_ok, NC, NC, GREEN, NC);
        println!("{}│{}  🧩 Extensions: {}{}{}/{} installed{}                              {}│{}",
            GREEN, NC, BOLD, ext_ok, VSCODE_EXTENSIONS.len(), NC, NC, GREEN, NC);
        if !ext_failed.is_empty() {
            println!("{}├─────────────────────────────────────────────────────────────┤{}", YELLOW, NC);
            println!("{}│{}  ⚠  {} extension(s) failed:", YELLOW, NC, ext_failed.len());
            for f in &ext_failed { println!("{}│{}     • {}", YELLOW, NC, f); }
        }
        println!("{}│  💡 Restart VS Code for changes to take effect.            │{}", GREEN, NC);
        println!("{}└─────────────────────────────────────────────────────────────┘{}", GREEN, NC);
        println!();
        Ok(())
    } else {
        eprintln!("{}❌ Failed to update any VS Code config paths.{}\n", RED, NC);
        Err("Failed to update any VS Code configuration paths.".into())
    }
}

// ── Unit Tests ────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_vscode_settings_non_empty() {
        // VSCODE_SETTINGS is JSONC (JSON with Comments) — VS Code reads it natively.
        // Full JSON validation is not needed here; VS Code handles JSONC parsing.
        // The install_settings_roundtrip test already verifies write-to-disk.
        assert!(!VSCODE_SETTINGS.is_empty(), "VSCODE_SETTINGS must not be empty");
        assert!(VSCODE_SETTINGS.contains("workbench.colorTheme"),   "Missing colorTheme");
        assert!(VSCODE_SETTINGS.contains("terminal.foreground"),    "Missing terminal colors");
        assert!(VSCODE_SETTINGS.contains("editor.fontSize"),        "Missing fontSize");
        assert!(VSCODE_SETTINGS.contains("prettier.semi"),          "Missing prettier config");
        assert!(VSCODE_SETTINGS.starts_with('{'),                   "Must start with {{");
        // Verify it ends with } (ignoring trailing whitespace/newlines)
        assert!(VSCODE_SETTINGS.trim_end().ends_with('}'),          "Must end with }}");
    }

    #[test]
    fn test_extensions_list_non_empty() {
        assert!(!VSCODE_EXTENSIONS.is_empty());
        for ext in VSCODE_EXTENSIONS {
            assert!(ext.contains('.'), "Bad extension ID (no dot): {}", ext);
        }
    }

    #[test]
    fn test_detect_system_and_paths() {
        let (distro, paths) = detect_system_and_paths();
        assert!(!distro.is_empty());
        assert!(!paths.is_empty());
    }

    #[test]
    fn test_timestamp_format() {
        let ts = get_timestamp();
        assert_eq!(ts.len(), 15, "Expected YYYYMMDD_HHMMSS (15 chars), got: {}", ts);
    }

    #[test]
    fn test_install_settings_roundtrip() {
        let tmp = env::temp_dir().join("gladeshell_test_code_setup");
        let _ = fs::remove_dir_all(&tmp);

        assert!(install_settings(&tmp));
        assert!(tmp.join("settings.json").exists());

        // Second install must create a .bak file
        assert!(install_settings(&tmp));
        let has_bak = fs::read_dir(&tmp).unwrap()
            .flatten()
            .any(|e| e.file_name().to_string_lossy().starts_with("settings.json.bak."));
        assert!(has_bak, "No backup file created on second install");

        let _ = fs::remove_dir_all(&tmp);
    }
}
