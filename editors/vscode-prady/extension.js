// Prady Language Extension for VS Code
// Professional LSP client with go-to-definition, diagnostics, hover, completion, and symbols.

const vscode = require('vscode');
const path   = require('path');
const fs     = require('fs');
const {
  LanguageClient,
  TransportKind,
} = require('vscode-languageclient/node');

let client;

/**
 * Find the prady-lsp binary next to the prady CLI binary,
 * or fall back to PATH.
 */
function findLspBinary(context) {
  const binName = process.platform === 'win32' ? 'prady-lsp.exe' : 'prady-lsp';

  // 1. Check user setting
  const cfg = vscode.workspace.getConfiguration('prady');
  const setting = cfg.get('lspServerPath', '');
  if (setting && fs.existsSync(setting)) return setting;

  // 2. Bundled inside the extension itself (VSIX install)
  const bundled = path.join(context.extensionPath, binName);
  if (fs.existsSync(bundled)) return bundled;

  // 3. Same directory as configured CLI path
  const cliPath = cfg.get('executablePath', 'prady');
  const dir = path.dirname(cliPath);
  const adjacent = path.join(dir, binName);
  if (fs.existsSync(adjacent)) return adjacent;

  // 4. Workspace-local target/debug or target/release
  const wf = vscode.workspace.workspaceFolders;
  if (wf && wf.length > 0) {
    const root = wf[0].uri.fsPath;
    for (const profile of ['debug', 'release']) {
      const candidate = path.join(root, 'target', profile, binName);
      if (fs.existsSync(candidate)) return candidate;
    }
  }

  // 5. Fallback: rely on PATH
  return binName;
}

function activate(context) {
  const lspBin = findLspBinary(context);

  const serverOptions = {
    run:   { command: lspBin, transport: TransportKind.stdio },
    debug: { command: lspBin, transport: TransportKind.stdio },
  };

  const clientOptions = {
    documentSelector: [{ scheme: 'file', language: 'prady' }],
    synchronize: {
      fileEvents: vscode.workspace.createFileSystemWatcher('**/*.pr'),
    },
    outputChannelName: 'Prady Language Server',
    traceOutputChannel: vscode.window.createOutputChannel('Prady LSP Trace'),
  };

  client = new LanguageClient(
    'prady-lsp',
    'Prady Language Server',
    serverOptions,
    clientOptions,
  );

  client.start();

  // ── Status bar item ────────────────────────────────────────────────────
  const statusBar = vscode.window.createStatusBarItem(
    vscode.StatusBarAlignment.Left, 10
  );
  statusBar.text   = '$(prady-file-icon) Prady';
  statusBar.tooltip = 'Prady Language Server is active';
  statusBar.command = 'prady.showOutput';
  statusBar.show();
  context.subscriptions.push(statusBar);

  // ── Commands ───────────────────────────────────────────────────────────

  // Run current file
  context.subscriptions.push(
    vscode.commands.registerCommand('prady.runFile', async () => {
      const editor = vscode.window.activeTextEditor;
      if (!editor) { vscode.window.showErrorMessage('No active .pr file'); return; }
      const file = editor.document.uri.fsPath;
      const cfg  = vscode.workspace.getConfiguration('prady');
      const cli  = cfg.get('executablePath', 'prady');

      const terminal = vscode.window.createTerminal({ name: 'Prady Run' });
      terminal.show();
      terminal.sendText(`"${cli}" run "${file}"`);
    })
  );

  // Check current file
  context.subscriptions.push(
    vscode.commands.registerCommand('prady.checkFile', async () => {
      const editor = vscode.window.activeTextEditor;
      if (!editor) { vscode.window.showErrorMessage('No active .pr file'); return; }
      const file = editor.document.uri.fsPath;
      const cfg  = vscode.workspace.getConfiguration('prady');
      const cli  = cfg.get('executablePath', 'prady');

      const terminal = vscode.window.createTerminal({ name: 'Prady Check' });
      terminal.show();
      terminal.sendText(`"${cli}" check "${file}"`);
    })
  );

  // Show AST
  context.subscriptions.push(
    vscode.commands.registerCommand('prady.showAst', async () => {
      const editor = vscode.window.activeTextEditor;
      if (!editor) { vscode.window.showErrorMessage('No active .pr file'); return; }
      const file = editor.document.uri.fsPath;
      const cfg  = vscode.workspace.getConfiguration('prady');
      const cli  = cfg.get('executablePath', 'prady');

      const terminal = vscode.window.createTerminal({ name: 'Prady AST' });
      terminal.show();
      terminal.sendText(`"${cli}" ast "${file}"`);
    })
  );

  // Show LSP output
  context.subscriptions.push(
    vscode.commands.registerCommand('prady.showOutput', () => {
      client.outputChannel.show();
    })
  );

  context.subscriptions.push(client);
}

function deactivate() {
  if (client) return client.stop();
}

module.exports = { activate, deactivate };
