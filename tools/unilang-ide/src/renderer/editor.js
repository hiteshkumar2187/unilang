/**
 * UniLang IDE - Code Editor Component
 *
 * Textarea-based editor with syntax highlighting overlay,
 * line numbers, auto-indent, bracket matching, and
 * current-line highlight.
 */

class Editor {
  constructor() {
    this.textarea = document.getElementById('code-editor');
    this.overlay = document.getElementById('highlight-overlay');
    this.lineNumbers = document.getElementById('line-numbers');
    this.editorWrapper = document.getElementById('editor-wrapper');
    this.statusPosition = document.getElementById('status-position');

    this.currentFilePath = null;
    this.isModified = false;
    this._onChangeCallbacks = [];
    this._onCursorCallbacks = [];

    this._initKeywords();
    this._bindEvents();
    this._updateHighlight();
    this._updateLineNumbers();
  }

  _initKeywords() {
    // Python keywords
    this.pythonKeywords = new Set([
      'and', 'as', 'assert', 'async', 'await', 'break', 'class',
      'continue', 'def', 'del', 'elif', 'else', 'except', 'finally',
      'for', 'from', 'global', 'if', 'import', 'in', 'is', 'lambda',
      'nonlocal', 'not', 'or', 'pass', 'raise', 'return', 'try',
      'while', 'with', 'yield'
    ]);

    // Java keywords
    this.javaKeywords = new Set([
      'abstract', 'boolean', 'byte', 'case', 'catch', 'char',
      'const', 'default', 'do', 'double', 'enum', 'extends',
      'final', 'float', 'implements', 'instanceof', 'int',
      'interface', 'long', 'native', 'new', 'package', 'private',
      'protected', 'public', 'short', 'static', 'strictfp', 'super',
      'switch', 'synchronized', 'this', 'throw', 'throws',
      'transient', 'void', 'volatile'
    ]);

    // Control-flow keywords (highlighted differently)
    this.controlKeywords = new Set([
      'if', 'else', 'elif', 'for', 'while', 'do', 'switch', 'case',
      'break', 'continue', 'return', 'try', 'catch', 'except',
      'finally', 'throw', 'raise', 'with', 'async', 'await', 'yield'
    ]);

    // Type keywords
    this.typeKeywords = new Set([
      'int', 'float', 'double', 'long', 'short', 'byte', 'char',
      'boolean', 'void', 'string', 'String', 'list', 'List', 'dict',
      'Dict', 'set', 'Set', 'tuple', 'Tuple'
    ]);

    // Boolean / special constants
    this.booleans = new Set([
      'True', 'False', 'None', 'true', 'false', 'null'
    ]);

    // Bracket pairs for matching
    this.bracketPairs = {
      '(': ')', ')': '(',
      '[': ']', ']': '[',
      '{': '}', '}': '{'
    };
    this.openBrackets = new Set(['(', '[', '{']);
    this.closeBrackets = new Set([')', ']', '}']);
  }

  _bindEvents() {
    this.textarea.addEventListener('input', () => {
      this._updateHighlight();
      this._updateLineNumbers();
      this._markModified();
      this._fireChange();
    });

    this.textarea.addEventListener('scroll', () => {
      this._syncScroll();
    });

    this.textarea.addEventListener('keydown', (e) => {
      this._handleKeyDown(e);
    });

    this.textarea.addEventListener('click', () => {
      this._updateCursorPosition();
    });

    this.textarea.addEventListener('keyup', () => {
      this._updateCursorPosition();
    });

    this.textarea.addEventListener('select', () => {
      this._updateCursorPosition();
    });

    // Keep overlay sized to match content
    const resizeObserver = new ResizeObserver(() => {
      this._syncScroll();
    });
    resizeObserver.observe(this.editorWrapper);
  }

  _handleKeyDown(e) {
    const ta = this.textarea;

    // Tab key: insert 4 spaces
    if (e.key === 'Tab' && !e.shiftKey) {
      e.preventDefault();
      const start = ta.selectionStart;
      const end = ta.selectionEnd;
      const value = ta.value;
      ta.value = value.substring(0, start) + '    ' + value.substring(end);
      ta.selectionStart = ta.selectionEnd = start + 4;
      this._updateHighlight();
      this._updateLineNumbers();
      this._markModified();
      this._fireChange();
      return;
    }

    // Shift+Tab: remove up to 4 leading spaces
    if (e.key === 'Tab' && e.shiftKey) {
      e.preventDefault();
      const start = ta.selectionStart;
      const value = ta.value;
      const lineStart = value.lastIndexOf('\n', start - 1) + 1;
      const linePrefix = value.substring(lineStart, start);
      const spacesToRemove = Math.min(4, linePrefix.length - linePrefix.trimStart().length);
      if (spacesToRemove > 0) {
        ta.value = value.substring(0, lineStart) + value.substring(lineStart + spacesToRemove);
        ta.selectionStart = ta.selectionEnd = start - spacesToRemove;
        this._updateHighlight();
        this._updateLineNumbers();
        this._markModified();
        this._fireChange();
      }
      return;
    }

    // Enter key: auto-indent
    if (e.key === 'Enter') {
      e.preventDefault();
      const start = ta.selectionStart;
      const value = ta.value;
      const lineStart = value.lastIndexOf('\n', start - 1) + 1;
      const currentLine = value.substring(lineStart, start);
      const indent = currentLine.match(/^(\s*)/)[1];
      const trimmedLine = currentLine.trimEnd();
      const lastChar = trimmedLine.charAt(trimmedLine.length - 1);

      let newIndent = indent;
      if (lastChar === ':' || lastChar === '{') {
        newIndent = indent + '    ';
      }

      const insertion = '\n' + newIndent;
      ta.value = value.substring(0, start) + insertion + value.substring(ta.selectionEnd);
      ta.selectionStart = ta.selectionEnd = start + insertion.length;
      this._updateHighlight();
      this._updateLineNumbers();
      this._markModified();
      this._fireChange();
      return;
    }

    // Auto-close brackets
    const bracketMap = { '(': ')', '[': ']', '{': '}', "'": "'", '"': '"' };
    if (bracketMap[e.key]) {
      const start = ta.selectionStart;
      const end = ta.selectionEnd;
      const value = ta.value;

      // Only auto-close if nothing is selected and next char is whitespace/end/closing bracket
      if (start === end) {
        const nextChar = value.charAt(start);
        if (!nextChar || /[\s\)\]\},;]/.test(nextChar)) {
          // For quotes, only close if we're not inside the same quote
          if (e.key === "'" || e.key === '"') {
            const before = value.substring(0, start);
            const quoteCount = (before.match(new RegExp(e.key === "'" ? "'" : '"', 'g')) || []).length;
            if (quoteCount % 2 !== 0) return; // inside a string, don't auto-close
          }
          e.preventDefault();
          const closing = bracketMap[e.key];
          ta.value = value.substring(0, start) + e.key + closing + value.substring(end);
          ta.selectionStart = ta.selectionEnd = start + 1;
          this._updateHighlight();
          this._markModified();
          this._fireChange();
        }
      }
    }
  }

  // ----- Syntax Highlighting via Tokenizer -----

  _tokenize(code) {
    const tokens = [];
    let i = 0;
    const len = code.length;

    while (i < len) {
      // Single-line comment: # or //
      if (code[i] === '#' || (code[i] === '/' && code[i + 1] === '/')) {
        const start = i;
        while (i < len && code[i] !== '\n') i++;
        tokens.push({ type: 'comment', value: code.substring(start, i) });
        continue;
      }

      // Multi-line comment: /* ... */
      if (code[i] === '/' && code[i + 1] === '*') {
        const start = i;
        i += 2;
        while (i < len && !(code[i] === '*' && code[i + 1] === '/')) i++;
        if (i < len) i += 2;
        tokens.push({ type: 'comment', value: code.substring(start, i) });
        continue;
      }

      // Decorator: @word
      if (code[i] === '@' && (i === 0 || code[i - 1] === '\n' || /\s/.test(code[i - 1]))) {
        const start = i;
        i++;
        while (i < len && /[\w.]/.test(code[i])) i++;
        tokens.push({ type: 'decorator', value: code.substring(start, i) });
        continue;
      }

      // Triple-quoted string
      if ((code[i] === '"' && code[i + 1] === '"' && code[i + 2] === '"') ||
          (code[i] === "'" && code[i + 1] === "'" && code[i + 2] === "'")) {
        const quote = code.substring(i, i + 3);
        const start = i;
        i += 3;
        while (i < len) {
          if (code[i] === '\\') { i += 2; continue; }
          if (code.substring(i, i + 3) === quote) { i += 3; break; }
          i++;
        }
        tokens.push({ type: 'string', value: code.substring(start, i) });
        continue;
      }

      // String: single or double quoted
      if (code[i] === '"' || code[i] === "'") {
        const quote = code[i];
        const start = i;
        i++;
        while (i < len && code[i] !== quote && code[i] !== '\n') {
          if (code[i] === '\\') i++;
          i++;
        }
        if (i < len && code[i] === quote) i++;
        tokens.push({ type: 'string', value: code.substring(start, i) });
        continue;
      }

      // Numbers
      if (/\d/.test(code[i]) || (code[i] === '.' && i + 1 < len && /\d/.test(code[i + 1]))) {
        const start = i;
        if (code[i] === '0' && i + 1 < len && (code[i + 1] === 'x' || code[i + 1] === 'X')) {
          i += 2;
          while (i < len && /[\da-fA-F_]/.test(code[i])) i++;
        } else if (code[i] === '0' && i + 1 < len && (code[i + 1] === 'b' || code[i + 1] === 'B')) {
          i += 2;
          while (i < len && /[01_]/.test(code[i])) i++;
        } else {
          while (i < len && /[\d_]/.test(code[i])) i++;
          if (i < len && code[i] === '.') {
            i++;
            while (i < len && /[\d_]/.test(code[i])) i++;
          }
          if (i < len && (code[i] === 'e' || code[i] === 'E')) {
            i++;
            if (i < len && (code[i] === '+' || code[i] === '-')) i++;
            while (i < len && /\d/.test(code[i])) i++;
          }
        }
        tokens.push({ type: 'number', value: code.substring(start, i) });
        continue;
      }

      // Words: keywords, identifiers, etc.
      if (/[a-zA-Z_]/.test(code[i])) {
        const start = i;
        while (i < len && /[\w]/.test(code[i])) i++;
        const word = code.substring(start, i);

        if (this.booleans.has(word)) {
          tokens.push({ type: 'boolean', value: word });
        } else if (this.typeKeywords.has(word)) {
          tokens.push({ type: 'type', value: word });
        } else if (this.controlKeywords.has(word)) {
          tokens.push({ type: 'keyword-control', value: word });
        } else if (this.pythonKeywords.has(word) || this.javaKeywords.has(word)) {
          tokens.push({ type: 'keyword', value: word });
        } else if (i < len && code[i] === '(') {
          tokens.push({ type: 'function', value: word });
        } else {
          tokens.push({ type: 'identifier', value: word });
        }
        continue;
      }

      // Brackets
      if ('()[]{}' .includes(code[i])) {
        tokens.push({ type: 'bracket', value: code[i] });
        i++;
        continue;
      }

      // Operators
      if ('+-*/%=<>!&|^~?.,:;'.includes(code[i])) {
        const start = i;
        i++;
        // Consume multi-char operators
        if (i < len && '=<>&|+-'.includes(code[i])) i++;
        if (i < len && code[i] === '=') i++;
        tokens.push({ type: 'operator', value: code.substring(start, i) });
        continue;
      }

      // Whitespace and newlines
      if (/\s/.test(code[i])) {
        const start = i;
        while (i < len && /\s/.test(code[i])) i++;
        tokens.push({ type: 'whitespace', value: code.substring(start, i) });
        continue;
      }

      // Anything else
      tokens.push({ type: 'other', value: code[i] });
      i++;
    }

    return tokens;
  }

  _escapeHtml(text) {
    return text
      .replace(/&/g, '&amp;')
      .replace(/</g, '&lt;')
      .replace(/>/g, '&gt;');
  }

  _updateHighlight() {
    const code = this.textarea.value;
    const tokens = this._tokenize(code);

    let html = '';
    for (const token of tokens) {
      const escaped = this._escapeHtml(token.value);
      switch (token.type) {
        case 'keyword':
          html += `<span class="syn-keyword">${escaped}</span>`;
          break;
        case 'keyword-control':
          html += `<span class="syn-keyword-control">${escaped}</span>`;
          break;
        case 'string':
          html += `<span class="syn-string">${escaped}</span>`;
          break;
        case 'number':
          html += `<span class="syn-number">${escaped}</span>`;
          break;
        case 'comment':
          html += `<span class="syn-comment">${escaped}</span>`;
          break;
        case 'function':
          html += `<span class="syn-function">${escaped}</span>`;
          break;
        case 'type':
          html += `<span class="syn-type">${escaped}</span>`;
          break;
        case 'boolean':
          html += `<span class="syn-boolean">${escaped}</span>`;
          break;
        case 'decorator':
          html += `<span class="syn-decorator">${escaped}</span>`;
          break;
        case 'bracket':
          html += `<span class="syn-bracket">${escaped}</span>`;
          break;
        case 'operator':
          html += `<span class="syn-operator">${escaped}</span>`;
          break;
        default:
          html += escaped;
      }
    }

    // Ensure overlay always ends with a newline so it matches textarea sizing
    if (!html.endsWith('\n')) html += '\n';

    this.overlay.innerHTML = html;
    this._syncScroll();
  }

  _updateLineNumbers() {
    const lines = this.textarea.value.split('\n');
    const count = lines.length;
    const cursorLine = this._getCurrentLine();

    let html = '';
    for (let i = 1; i <= count; i++) {
      const cls = i === cursorLine ? ' current' : '';
      html += `<div class="line-number${cls}">${i}</div>`;
    }
    this.lineNumbers.innerHTML = html;
  }

  _syncScroll() {
    this.overlay.scrollTop = this.textarea.scrollTop;
    this.overlay.scrollLeft = this.textarea.scrollLeft;
    this.lineNumbers.scrollTop = this.textarea.scrollTop;
  }

  _getCurrentLine() {
    const value = this.textarea.value;
    const pos = this.textarea.selectionStart;
    const before = value.substring(0, pos);
    return (before.match(/\n/g) || []).length + 1;
  }

  _getCurrentCol() {
    const value = this.textarea.value;
    const pos = this.textarea.selectionStart;
    const lastNewline = value.lastIndexOf('\n', pos - 1);
    return pos - lastNewline;
  }

  _updateCursorPosition() {
    const line = this._getCurrentLine();
    const col = this._getCurrentCol();
    this.statusPosition.textContent = `Ln ${line}, Col ${col}`;
    this._updateLineNumbers(); // refresh current line highlight
  }

  _markModified() {
    if (!this.isModified) {
      this.isModified = true;
      const tab = document.querySelector('.editor-tab.active');
      if (tab) tab.classList.add('modified');
    }
  }

  markSaved() {
    this.isModified = false;
    const tab = document.querySelector('.editor-tab.active');
    if (tab) tab.classList.remove('modified');
  }

  // ----- Public API -----

  getValue() {
    return this.textarea.value;
  }

  setValue(content) {
    this.textarea.value = content;
    this._updateHighlight();
    this._updateLineNumbers();
    this.textarea.scrollTop = 0;
    this.isModified = false;
  }

  setFilePath(filePath) {
    this.currentFilePath = filePath;
  }

  getFilePath() {
    return this.currentFilePath;
  }

  focus() {
    this.textarea.focus();
  }

  onChange(callback) {
    this._onChangeCallbacks.push(callback);
  }

  _fireChange() {
    for (const cb of this._onChangeCallbacks) cb(this.textarea.value);
  }
}
