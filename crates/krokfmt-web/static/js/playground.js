// Playground functionality for krokfmt using WebAssembly and Monaco Editor
class KrokfmtPlayground {
    constructor() {
        this.inputEditor = null;
        this.outputEditor = null;
        this.formatBtn = document.getElementById('format-btn');
        this.copyBtn = document.getElementById('copy-btn');
        this.clearBtn = document.getElementById('clear-btn');
        this.errorContainer = document.getElementById('error-container');
        this.loadingSpinner = document.getElementById('loading-spinner');
        this.wasmModule = null;
        this.useWasm = true; // Try WASM first, fallback to API
        
        this.initializeMonacoEditors();
        this.initializeWASM();
        this.initializeEventListeners();
    }
    
    initializeMonacoEditors() {
        // Configure Monaco Editor
        monaco.languages.typescript.typescriptDefaults.setDiagnosticsOptions({
            noSemanticValidation: true,
            noSyntaxValidation: false,
        });
        
        monaco.languages.typescript.typescriptDefaults.setCompilerOptions({
            target: monaco.languages.typescript.ScriptTarget.Latest,
            allowNonTsExtensions: true,
            moduleResolution: monaco.languages.typescript.ModuleResolutionKind.NodeJs,
            module: monaco.languages.typescript.ModuleKind.CommonJS,
            noEmit: true,
            jsx: monaco.languages.typescript.JsxEmit.React,
            reactNamespace: "React",
            allowJs: true,
            typeRoots: ["node_modules/@types"],
        });
        
        // Create input editor
        this.inputEditor = monaco.editor.create(document.getElementById('input-editor'), {
            value: this.getExampleCode(),
            language: 'typescript',
            theme: 'vs-dark',
            minimap: { enabled: false },
            automaticLayout: true,
            fontSize: 14,
            tabSize: 2,
            wordWrap: 'on',
            scrollBeyondLastLine: false,
            formatOnPaste: false,
            formatOnType: false,
        });
        
        // Create output editor (read-only)
        this.outputEditor = monaco.editor.create(document.getElementById('output-editor'), {
            value: '',
            language: 'typescript',
            theme: 'vs-dark',
            minimap: { enabled: false },
            automaticLayout: true,
            fontSize: 14,
            tabSize: 2,
            wordWrap: 'on',
            scrollBeyondLastLine: false,
            readOnly: true,
        });
    }
    
    async initializeWASM() {
        try {
            // Show loading indicator
            this.showStatus('Loading WASM formatter...');
            
            // Import the WASM module
            const wasmModule = await import('/wasm/krokfmt_playground.js');
            await wasmModule.default('/wasm/krokfmt_playground_bg.wasm');
            
            // Initialize panic hook for better error messages
            wasmModule.init_panic_hook();
            
            // Store the module
            this.wasmModule = wasmModule;
            this.useWasm = true;
            
            this.clearStatus();
            console.log('WASM formatter loaded successfully');
        } catch (error) {
            console.error('Failed to load WASM module:', error);
            this.useWasm = false;
            this.showStatus('Using server-side formatting (WASM unavailable)', 'warning');
            setTimeout(() => this.clearStatus(), 3000);
        }
    }
    
    initializeEventListeners() {
        if (this.formatBtn) {
            this.formatBtn.addEventListener('click', () => this.formatCode());
        }
        
        if (this.copyBtn) {
            this.copyBtn.addEventListener('click', () => this.copyOutput());
        }
        
        if (this.clearBtn) {
            this.clearBtn.addEventListener('click', () => this.clearInput());
        }
        
        // Format on Cmd/Ctrl + Enter
        this.inputEditor.addCommand(
            monaco.KeyMod.CtrlCmd | monaco.KeyCode.Enter,
            () => this.formatCode()
        );
    }
    
    getExampleCode() {
        return `// Example TypeScript code - try formatting it!
import { useState } from 'react';
import axios from 'axios';
import { Button } from './components/Button';
import type { User } from '../types';
import './styles.css';

interface Props {
    title: string;
    users: User[];
    onUpdate: (id: number) => void;
    isLoading?: boolean;
}

export const UserList: React.FC<Props> = ({
    title,
    users,
    onUpdate,
    isLoading = false
}) => {
    const [filter, setFilter] = useState('');
    
    // Filter users based on search term
    const filteredUsers = users.filter(user => 
        user.name.toLowerCase().includes(filter.toLowerCase())
    );
    
    return (
        <div className="user-list">
            <h2>{title}</h2>
            <input
                type="text"
                placeholder="Search users..."
                value={filter}
                onChange={(e) => setFilter(e.target.value)}
            />
            {isLoading ? (
                <div>Loading...</div>
            ) : (
                <ul>
                    {filteredUsers.map(user => (
                        <li key={user.id}>
                            <span>{user.name}</span>
                            <Button onClick={() => onUpdate(user.id)}>
                                Update
                            </Button>
                        </li>
                    ))}
                </ul>
            )}
        </div>
    );
};`;
    }
    
    async formatCode() {
        const code = this.inputEditor.getValue().trim();
        
        if (!code) {
            this.showError('Please enter some TypeScript code to format');
            return;
        }
        
        this.setLoading(true);
        this.clearError();
        
        try {
            if (this.useWasm && this.wasmModule) {
                // Use WASM formatter
                const resultJson = this.wasmModule.format_typescript(code);
                const result = JSON.parse(resultJson);
                
                if (result.success) {
                    this.outputEditor.setValue(result.formatted);
                    this.highlightOutput();
                } else {
                    this.showError(result.error || 'Failed to format code');
                }
            } else {
                // Fallback to server API
                const response = await fetch('/api/format', {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json',
                    },
                    body: JSON.stringify({ code }),
                });
                
                const result = await response.json();
                
                if (result.success) {
                    this.outputEditor.setValue(result.formatted);
                    this.highlightOutput();
                } else {
                    this.showError(result.error || 'Failed to format code');
                }
            }
        } catch (error) {
            console.error('Formatting error:', error);
            this.showError(`Error: ${error.message}`);
        } finally {
            this.setLoading(false);
        }
    }
    
    copyOutput() {
        const output = this.outputEditor.getValue();
        
        if (!output) {
            this.showError('No formatted code to copy');
            return;
        }
        
        navigator.clipboard.writeText(output).then(() => {
            const originalText = this.copyBtn.textContent;
            this.copyBtn.textContent = 'Copied!';
            this.copyBtn.classList.add('success');
            
            setTimeout(() => {
                this.copyBtn.textContent = originalText;
                this.copyBtn.classList.remove('success');
            }, 2000);
        }).catch(() => {
            this.showError('Failed to copy to clipboard');
        });
    }
    
    clearInput() {
        this.inputEditor.setValue('');
        this.outputEditor.setValue('');
        this.clearError();
    }
    
    highlightOutput() {
        // Flash the output editor to show it updated
        const outputContainer = document.getElementById('output-editor');
        if (outputContainer) {
            outputContainer.style.transition = 'opacity 0.2s';
            outputContainer.style.opacity = '0.7';
            setTimeout(() => {
                outputContainer.style.opacity = '1';
            }, 200);
        }
    }
    
    showError(message) {
        if (this.errorContainer) {
            this.errorContainer.innerHTML = `<div class="error-message">${message}</div>`;
            this.errorContainer.style.display = 'block';
        }
    }
    
    clearError() {
        if (this.errorContainer) {
            this.errorContainer.innerHTML = '';
            this.errorContainer.style.display = 'none';
        }
    }
    
    showStatus(message, type = 'info') {
        const statusEl = document.getElementById('status-message');
        if (statusEl) {
            statusEl.textContent = message;
            statusEl.className = `status-message ${type}`;
            statusEl.style.display = 'block';
        }
    }
    
    clearStatus() {
        const statusEl = document.getElementById('status-message');
        if (statusEl) {
            statusEl.style.display = 'none';
        }
    }
    
    setLoading(isLoading) {
        if (this.formatBtn) {
            this.formatBtn.disabled = isLoading;
            this.formatBtn.textContent = isLoading ? 'Formatting...' : 'Format Code';
        }
        
        if (this.loadingSpinner) {
            this.loadingSpinner.style.display = isLoading ? 'inline-block' : 'none';
        }
    }
}

// Initialize playground when Monaco is ready
window.addEventListener('load', () => {
    // Ensure Monaco is loaded
    if (typeof monaco !== 'undefined') {
        new KrokfmtPlayground();
    } else {
        console.error('Monaco Editor failed to load');
    }
});