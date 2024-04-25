// The module 'vscode' contains the VS Code extensibility API
// Import the module and reference it with the alias vscode in your code below
import * as vscode from "vscode";
import * as move_formatter from "move-formatter-rs";

// This method is called when your extension is activated
// Your extension is activated the very first time the command is executed
export function activate(_context: vscode.ExtensionContext) {
    vscode.languages.registerDocumentFormattingEditProvider("move", {
        provideDocumentFormattingEdits(document: vscode.TextDocument): vscode.TextEdit[] {
            const newText = move_formatter.formatString(document.getText());
            return [
                new vscode.TextEdit(
                    new vscode.Range(
                        new vscode.Position(0, 0),
                        new vscode.Position(document.lineCount, document.lineAt(document.lineCount - 1).text.length)
                    ),
                    newText
                ),
            ];
        },
    });
}

// This method is called when your extension is deactivated
export function deactivate() {}
