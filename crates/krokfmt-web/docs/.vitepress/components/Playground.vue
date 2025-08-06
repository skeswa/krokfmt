<template>
  <div class="playground-container">
    <h1 class="playground-title">🐊 krokfmt Playground</h1>
    
    <div class="editors">
      <div class="editor-panel">
        <div class="editor-header">Input (TypeScript/TSX)</div>
        <div ref="inputEditorEl" class="monaco-editor"></div>
      </div>
      
      <div class="editor-panel">
        <div class="editor-header">Output (Formatted)</div>
        <div ref="outputEditorEl" class="monaco-editor"></div>
      </div>
    </div>
    
    <div class="controls">
      <button 
        @click="formatCode" 
        :disabled="loading"
        class="format-btn"
      >
        <span v-if="loading">⏳ Formatting...</span>
        <span v-else>Format Code</span>
      </button>
      <button @click="copyOutput" class="btn-secondary">Copy Output</button>
      <button @click="clearInput" class="btn-secondary">Clear</button>
    </div>
    
    <div v-if="error" class="error-container">
      {{ error }}
    </div>
    
    <div v-if="status" class="status-message" :class="statusType">
      {{ status }}
    </div>
    
    <div class="tips">
      <p>💡 Tip: Press <kbd>Cmd</kbd>+<kbd>Enter</kbd> (Mac) or <kbd>Ctrl</kbd>+<kbd>Enter</kbd> (Windows/Linux) to format</p>
      <p>🚀 Formatting runs entirely in your browser using WebAssembly</p>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted } from 'vue'

const inputEditorEl = ref(null)
const outputEditorEl = ref(null)
const loading = ref(false)
const error = ref('')
const status = ref('')
const statusType = ref('info')

let inputEditor = null
let outputEditor = null
let wasmModule = null
let monaco = null

const exampleCode = `// Example TypeScript code - try formatting it!
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
};`

async function loadMonaco() {
  return new Promise((resolve) => {
    if (window.monaco) {
      resolve(window.monaco)
      return
    }

    const script = document.createElement('script')
    script.src = 'https://cdnjs.cloudflare.com/ajax/libs/monaco-editor/0.45.0/min/vs/loader.min.js'
    script.onload = () => {
      window.require.config({ 
        paths: { 
          'vs': 'https://cdnjs.cloudflare.com/ajax/libs/monaco-editor/0.45.0/min/vs' 
        } 
      })
      
      window.require(['vs/editor/editor.main'], () => {
        resolve(window.monaco)
      })
    }
    document.head.appendChild(script)
  })
}

async function initializeWASM() {
  try {
    showStatus('Loading WASM formatter...', 'info')
    
    // Dynamically load the WASM module
    const script = document.createElement('script')
    script.type = 'module'
    
    const moduleCode = `
      import init, { format_typescript, init_panic_hook } from '/wasm/krokfmt_playground.js';
      
      await init('/wasm/krokfmt_playground_bg.wasm');
      init_panic_hook();
      
      window.wasmFormatter = { format_typescript };
    `
    
    script.textContent = moduleCode
    document.head.appendChild(script)
    
    // Wait for the module to load
    await new Promise((resolve) => {
      const checkInterval = setInterval(() => {
        if (window.wasmFormatter) {
          clearInterval(checkInterval)
          wasmModule = window.wasmFormatter
          resolve()
        }
      }, 100)
      
      // Timeout after 5 seconds
      setTimeout(() => {
        clearInterval(checkInterval)
        resolve()
      }, 5000)
    })
    
    if (wasmModule) {
      clearStatus()
      console.log('WASM formatter loaded successfully')
    } else {
      throw new Error('WASM module failed to load')
    }
  } catch (err) {
    console.error('Failed to load WASM module:', err)
    showStatus('Using server-side formatting (WASM unavailable)', 'warning')
    setTimeout(clearStatus, 3000)
  }
}

async function initializeEditors() {
  monaco = await loadMonaco()
  
  // Configure TypeScript options
  monaco.languages.typescript.typescriptDefaults.setDiagnosticsOptions({
    noSemanticValidation: true,
    noSyntaxValidation: false,
  })
  
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
  })
  
  // Create input editor
  inputEditor = monaco.editor.create(inputEditorEl.value, {
    value: exampleCode,
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
  })
  
  // Create output editor (read-only)
  outputEditor = monaco.editor.create(outputEditorEl.value, {
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
  })
  
  // Add keyboard shortcut for formatting
  inputEditor.addCommand(
    monaco.KeyMod.CtrlCmd | monaco.KeyCode.Enter,
    formatCode
  )
}

async function formatCode() {
  const code = inputEditor.getValue().trim()
  
  if (!code) {
    showError('Please enter some TypeScript code to format')
    return
  }
  
  loading.value = true
  clearError()
  
  try {
    if (wasmModule && wasmModule.format_typescript) {
      // Use WASM formatter
      const resultJson = wasmModule.format_typescript(code)
      const result = JSON.parse(resultJson)
      
      if (result.success) {
        outputEditor.setValue(result.formatted)
        highlightOutput()
      } else {
        showError(result.error || 'Failed to format code')
      }
    } else {
      // WASM not loaded
      showError('WASM formatter is not loaded. Please refresh the page and try again.')
    }
  } catch (err) {
    console.error('Formatting error:', err)
    showError(`Error: ${err.message}`)
  } finally {
    loading.value = false
  }
}

function copyOutput() {
  const output = outputEditor.getValue()
  
  if (!output) {
    showError('No formatted code to copy')
    return
  }
  
  navigator.clipboard.writeText(output).then(() => {
    showStatus('Copied to clipboard!', 'success')
    setTimeout(clearStatus, 2000)
  }).catch(() => {
    showError('Failed to copy to clipboard')
  })
}

function clearInput() {
  inputEditor.setValue('')
  outputEditor.setValue('')
  clearError()
}

function highlightOutput() {
  // Flash the output editor to show it updated
  const container = outputEditorEl.value
  if (container) {
    container.style.transition = 'opacity 0.2s'
    container.style.opacity = '0.7'
    setTimeout(() => {
      container.style.opacity = '1'
    }, 200)
  }
}

function showError(message) {
  error.value = message
}

function clearError() {
  error.value = ''
}

function showStatus(message, type = 'info') {
  status.value = message
  statusType.value = type
}

function clearStatus() {
  status.value = ''
}

onMounted(async () => {
  await initializeEditors()
  await initializeWASM()
})

onUnmounted(() => {
  if (inputEditor) inputEditor.dispose()
  if (outputEditor) outputEditor.dispose()
})
</script>

<style scoped>
.playground-container {
  max-width: 1400px;
  margin: 0 auto;
  padding: 2rem;
}

.playground-title {
  text-align: center;
  margin-bottom: 2rem;
  font-size: 2.5rem;
}

.editors {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 1rem;
  margin-bottom: 2rem;
}

@media (max-width: 768px) {
  .editors {
    grid-template-columns: 1fr;
  }
}

.editor-panel {
  display: flex;
  flex-direction: column;
  border: 1px solid var(--vp-c-divider);
  border-radius: 8px;
  overflow: hidden;
}

.editor-header {
  background: var(--vp-c-bg-soft);
  padding: 0.75rem 1rem;
  font-weight: 600;
  border-bottom: 1px solid var(--vp-c-divider);
}

.monaco-editor {
  height: 500px;
  width: 100%;
}

.controls {
  display: flex;
  gap: 1rem;
  justify-content: center;
  margin-bottom: 2rem;
}

.format-btn {
  background: var(--vp-c-brand);
  color: white;
  border: none;
  padding: 0.75rem 2rem;
  border-radius: 8px;
  font-size: 1rem;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s;
}

.format-btn:hover:not(:disabled) {
  background: var(--vp-c-brand-dark);
  transform: translateY(-2px);
}

.format-btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.btn-secondary {
  background: var(--vp-c-bg-soft);
  color: var(--vp-c-text-1);
  border: 1px solid var(--vp-c-divider);
  padding: 0.75rem 1.5rem;
  border-radius: 8px;
  font-size: 1rem;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-secondary:hover {
  background: var(--vp-c-bg-soft-up);
  border-color: var(--vp-c-brand);
}

.error-container {
  background: var(--vp-c-danger-soft);
  color: var(--vp-c-danger);
  padding: 1rem;
  border-radius: 8px;
  margin-bottom: 1rem;
}

.status-message {
  padding: 1rem;
  border-radius: 8px;
  margin-bottom: 1rem;
  text-align: center;
}

.status-message.info {
  background: var(--vp-c-info-soft);
  color: var(--vp-c-info);
}

.status-message.success {
  background: var(--vp-c-success-soft);
  color: var(--vp-c-success);
}

.status-message.warning {
  background: var(--vp-c-warning-soft);
  color: var(--vp-c-warning);
}

.tips {
  text-align: center;
  color: var(--vp-c-text-2);
  font-size: 0.9rem;
}

.tips p {
  margin: 0.5rem 0;
}

kbd {
  background: var(--vp-c-bg-soft);
  border: 1px solid var(--vp-c-divider);
  padding: 0.2rem 0.4rem;
  border-radius: 4px;
  font-size: 0.85em;
  font-family: var(--vp-font-family-mono);
}
</style>