/**
 * UniLang IDE - Output Terminal Component
 *
 * Displays build/run output with color support, timestamps,
 * auto-scroll, and a clear function.
 */

class Terminal {
  constructor() {
    this.container = document.getElementById('terminal-output');
    this.clearButton = document.getElementById('btn-clear-terminal');
    this._autoScroll = true;

    this.clearButton.addEventListener('click', () => this.clear());

    // Track if user has scrolled up (disable auto-scroll)
    this.container.addEventListener('scroll', () => {
      const { scrollTop, scrollHeight, clientHeight } = this.container;
      this._autoScroll = (scrollHeight - scrollTop - clientHeight) < 30;
    });
  }

  _getTimestamp() {
    const now = new Date();
    const h = String(now.getHours()).padStart(2, '0');
    const m = String(now.getMinutes()).padStart(2, '0');
    const s = String(now.getSeconds()).padStart(2, '0');
    return `${h}:${m}:${s}`;
  }

  _appendLine(text, className) {
    const lines = text.split('\n');
    for (const line of lines) {
      if (line === '' && lines.length > 1) continue; // skip trailing empty
      const div = document.createElement('div');
      div.className = `terminal-line ${className}`;

      const timestamp = document.createElement('span');
      timestamp.className = 'timestamp';
      timestamp.textContent = `[${this._getTimestamp()}]`;

      const content = document.createElement('span');
      content.textContent = line;

      div.appendChild(timestamp);
      div.appendChild(content);
      this.container.appendChild(div);
    }

    if (this._autoScroll) {
      this.container.scrollTop = this.container.scrollHeight;
    }
  }

  writeStdout(text) {
    this._appendLine(text, 'stdout');
  }

  writeStderr(text) {
    this._appendLine(text, 'stderr');
  }

  writeInfo(text) {
    this._appendLine(text, 'info');
  }

  writeSuccess(text) {
    this._appendLine(text, 'success');
  }

  writeSystem(text) {
    this._appendLine(text, 'system');
  }

  clear() {
    this.container.innerHTML = '';
    this._autoScroll = true;
  }
}
