/**
 * UniLang IDE - File Tree Component
 *
 * Reads directories via IPC, renders a recursive tree with
 * expand/collapse, highlights .uniL files, and fires events
 * when a file is selected.
 */

class FileTree {
  constructor() {
    this.container = document.getElementById('file-tree');
    this.rootPath = null;
    this.currentFilePath = null;
    this._onFileSelectCallbacks = [];

    // Cache of expanded folders
    this._expanded = new Set();
  }

  async setRoot(dirPath) {
    this.rootPath = dirPath;
    this._expanded.clear();
    this._expanded.add(dirPath);
    await this.refresh();
  }

  async refresh() {
    if (!this.rootPath) {
      this.container.innerHTML = '<div style="padding: 12px; color: #858585; font-style: italic;">No folder open</div>';
      return;
    }
    this.container.innerHTML = '';
    await this._renderDirectory(this.rootPath, this.container, 0);
  }

  async _renderDirectory(dirPath, parentEl, depth) {
    const result = await window.electronAPI.readDirectory(dirPath);
    if (!result.success) return;

    for (const item of result.items) {
      const el = this._createTreeItem(item, depth);
      parentEl.appendChild(el);

      if (item.isDirectory) {
        const childContainer = document.createElement('div');
        childContainer.className = 'tree-children';
        if (this._expanded.has(item.path)) {
          childContainer.classList.add('expanded');
          await this._renderDirectory(item.path, childContainer, depth + 1);
        }
        parentEl.appendChild(childContainer);
      }
    }
  }

  _createTreeItem(item, depth) {
    const div = document.createElement('div');
    div.className = 'tree-item';
    div.style.paddingLeft = `${12 + depth * 16}px`;
    div.dataset.path = item.path;
    div.dataset.isDirectory = item.isDirectory;

    const iconSpan = document.createElement('span');
    iconSpan.className = 'tree-icon';

    const labelSpan = document.createElement('span');
    labelSpan.className = 'tree-label';
    labelSpan.textContent = item.name;

    if (item.isDirectory) {
      div.classList.add('tree-item-folder');
      const isExpanded = this._expanded.has(item.path);
      iconSpan.textContent = isExpanded ? '\u25BE' : '\u25B8';

      div.addEventListener('click', async () => {
        await this._toggleDirectory(item.path, div);
      });
    } else {
      const ext = item.name.split('.').pop();
      if (ext === 'uniL') {
        div.classList.add('tree-item-unilang');
        iconSpan.textContent = '\u25C6'; // diamond
      } else if (['js', 'ts', 'json'].includes(ext)) {
        iconSpan.textContent = '\u2B25'; // small diamond
      } else if (['md', 'txt', 'rst'].includes(ext)) {
        iconSpan.textContent = '\u2637'; // trigram
      } else if (['yml', 'yaml', 'toml'].includes(ext)) {
        iconSpan.textContent = '\u2699'; // gear
      } else {
        iconSpan.textContent = '\u2022'; // bullet
      }

      div.addEventListener('click', () => {
        this._selectFile(item.path, div);
      });
    }

    // Highlight if current
    if (!item.isDirectory && item.path === this.currentFilePath) {
      div.classList.add('active');
    }

    div.appendChild(iconSpan);
    div.appendChild(labelSpan);
    return div;
  }

  async _toggleDirectory(dirPath, itemEl) {
    const childContainer = itemEl.nextElementSibling;
    if (!childContainer || !childContainer.classList.contains('tree-children')) return;

    const icon = itemEl.querySelector('.tree-icon');

    if (this._expanded.has(dirPath)) {
      this._expanded.delete(dirPath);
      childContainer.classList.remove('expanded');
      childContainer.innerHTML = '';
      if (icon) icon.textContent = '\u25B8';
    } else {
      this._expanded.add(dirPath);
      childContainer.classList.add('expanded');
      const depth = Math.round((parseInt(itemEl.style.paddingLeft) - 12) / 16) + 1;
      await this._renderDirectory(dirPath, childContainer, depth);
      if (icon) icon.textContent = '\u25BE';
    }
  }

  _selectFile(filePath, itemEl) {
    // Remove previous active
    const prev = this.container.querySelector('.tree-item.active');
    if (prev) prev.classList.remove('active');

    itemEl.classList.add('active');
    this.currentFilePath = filePath;

    for (const cb of this._onFileSelectCallbacks) {
      cb(filePath);
    }
  }

  setCurrentFile(filePath) {
    this.currentFilePath = filePath;
    // Update visual highlight
    const items = this.container.querySelectorAll('.tree-item');
    items.forEach(el => {
      if (el.dataset.path === filePath) {
        el.classList.add('active');
      } else {
        el.classList.remove('active');
      }
    });
  }

  onFileSelect(callback) {
    this._onFileSelectCallbacks.push(callback);
  }
}
