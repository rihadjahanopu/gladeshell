/* ============================================================
   FANCYBASH — Professional Advanced Documentation Scripts
   ============================================================ */

document.addEventListener('DOMContentLoaded', () => {
  initNavAutocomplete(); // ← Trie-based navbar search
  initThemePicker();
  initCopyButtons();
  initIntersectionObserver();
  initCommandExplorer();
  initTerminalPlayground();
  initInstallerBuilder();
  initSearchModal();
  initMobileSidebar();
  initBenchmark();      // ← Advanced benchmark section
});

/* ============================================================
   0. TRIE DATA STRUCTURE — Prefix Dictionary Algorithm
      Used for O(k) lookup autocomplete in the navbar.
   ============================================================ */

class TrieNode {
  constructor() {
    this.children = {}; // char → TrieNode
    this.records = []; // entries that end/pass through here
  }
}

class Trie {
  constructor() {
    this.root = new TrieNode();
  }

  /**
   * Insert a record object.
   * @param {string} key   - the searchable string (lowercased)
   * @param {object} record - the payload stored at this key
   */
  insert(key, record) {
    let node = this.root;
    for (const ch of key) {
      if (!node.children[ch]) node.children[ch] = new TrieNode();
      node = node.children[ch];
      // store at every intermediate node so prefix-search works
      if (node.records.length < 8) node.records.push(record);
    }
  }

  /**
   * Collect up to `limit` unique records whose keys share the given prefix.
   * @param {string} prefix  - lowercased query
   * @param {number} limit   - max results
   * @returns {object[]}
   */
  search(prefix, limit = 8) {
    let node = this.root;
    for (const ch of prefix) {
      if (!node.children[ch]) return [];
      node = node.children[ch];
    }
    // Deduplicate by record id
    const seen = new Set();
    const results = [];
    for (const r of node.records) {
      if (!seen.has(r.id)) {
        seen.add(r.id);
        results.push(r);
        if (results.length >= limit) break;
      }
    }
    return results;
  }
}

/* ============================================================
   AUTOCOMPLETE DICTIONARY
   Each entry has: id (anchor), title, sub (subtitle), type, icon
   ============================================================ */
const AUTOCOMPLETE_DICTIONARY = [
  // ── Sections ──────────────────────────────────────────
  {
    id: 'overview',
    title: 'Overview & Philosophy',
    sub: 'What is fancybash?',
    type: 'section',
    icon: '📖',
  },
  {
    id: 'requirements',
    title: 'System Requirements',
    sub: 'Supported OS & shells',
    type: 'section',
    icon: '💻',
  },
  {
    id: 'universal-install',
    title: 'Universal Installer',
    sub: 'One-line curl install',
    type: 'section',
    icon: '🚀',
  },
  {
    id: 'shell-installers',
    title: 'Shell-Specific Installers',
    sub: 'Bash / Zsh / Fish / PowerShell',
    type: 'section',
    icon: '🐚',
  },
  {
    id: 'manual-install',
    title: 'Manual Clone & Source',
    sub: 'Git clone + cat config',
    type: 'section',
    icon: '📂',
  },
  {
    id: 'verification',
    title: 'Verification & Reload',
    sub: 'source ~/.bashrc',
    type: 'section',
    icon: '✅',
  },
  {
    id: 'update-uninstall',
    title: 'Updating & Uninstalling',
    sub: 'uup / u.sh uninstaller',
    type: 'section',
    icon: '🔄',
  },
  {
    id: 'zsh-setup',
    title: 'Zsh Setup & Plugins',
    sub: 'Zero-framework plugin guide',
    type: 'section',
    icon: '🐚',
  },
  {
    id: 'fish-setup',
    title: 'Fish Shell Integration',
    sub: 'config.fish setup',
    type: 'section',
    icon: '🐟',
  },
  {
    id: 'powershell-setup',
    title: 'PowerShell Setup',
    sub: '$PROFILE / i.ps1',
    type: 'section',
    icon: '💠',
  },
  {
    id: 'font-setup',
    title: 'Font & Emoji Setup',
    sub: 'Noto Color Emoji + Fira Code',
    type: 'section',
    icon: '🔤',
  },
  {
    id: 'prompt-architecture',
    title: 'Smart Prompt Architecture',
    sub: 'Two-line contextual prompt',
    type: 'section',
    icon: '📟',
  },
  {
    id: 'folder-emojis',
    title: 'Folder Emojis & Badges',
    sub: 'node / bun / python / docker',
    type: 'section',
    icon: '📁',
  },
  {
    id: 'health-metrics',
    title: 'CPU Temp & System Metrics',
    sub: 'Colour-coded health indicators',
    type: 'section',
    icon: '🌡️',
  },
  {
    id: 'customizing-prompt',
    title: 'Customizing Prompt Behaviour',
    sub: 'ENV vars for prompt tuning',
    type: 'section',
    icon: '⚙️',
  },
  {
    id: 'command-explorer',
    title: 'Interactive Command Explorer',
    sub: 'Live filterable alias grid',
    type: 'section',
    icon: '⚡',
  },
  {
    id: 'terminal-simulator',
    title: 'Terminal Simulator',
    sub: 'Click chips to demo commands',
    type: 'section',
    icon: '🛠️',
  },
  {
    id: 'config-builder',
    title: 'Custom Installer Builder',
    sub: 'Pick shell + features, copy cmd',
    type: 'section',
    icon: '🏗️',
  },
  {
    id: 'nav-commands',
    title: 'Navigation & Movement Aliases',
    sub: '.. / ... / h / d / desk / proj',
    type: 'section',
    icon: '📂',
  },
  {
    id: 'pkg-commands',
    title: 'Package Managers (NPM & Bun)',
    sub: 'ni / bi / brd / nr / nid',
    type: 'section',
    icon: '📦',
  },
  {
    id: 'git-commands',
    title: 'Git Version Control',
    sub: 'gs / ga / gc / gp / glog',
    type: 'section',
    icon: '🌿',
  },
  {
    id: 'sys-commands',
    title: 'System & Maintenance',
    sub: 'uup / uu / sysinfo / gen / ex',
    type: 'section',
    icon: '⚙️',
  },
  {
    id: 'interactive-utilities',
    title: 'Interactive Tools',
    sub: 'todo / notes (gum + fzf)',
    type: 'section',
    icon: '🎯',
  },
  {
    id: 'ffmedia-suite',
    title: 'FFmedia Multimedia Suite',
    sub: 'Video compress / trim / convert',
    type: 'section',
    icon: '🎬',
  },
  {
    id: 'vault-suite',
    title: 'Hardened Multi-Vault Manager',
    sub: 'AES-256 PBKDF2 encryption & RAM guard',
    type: 'section',
    icon: '🔐',
  },
  {
    id: 'docker-suite',
    title: 'Docker Container Suite',
    sub: 'dps / dlogs / dstart / dclean',
    type: 'section',
    icon: '🐳',
  },
  {
    id: 'postgres-prisma',
    title: 'PostgreSQL & Prisma ORM',
    sub: 'pgstart / pdp / pds / pdm',
    type: 'section',
    icon: '🐘',
  },
  {
    id: 'architecture-safety',
    title: 'Architecture & Safety Specs',
    sub: 'Boundary-marker safe injection',
    type: 'section',
    icon: '🏗️',
  },
  {
    id: 'benchmarks',
    title: 'Performance Benchmarks',
    sub: '12ms vs 145ms Oh-My-Zsh',
    type: 'section',
    icon: '📊',
  },
  {
    id: 'troubleshooting-faq',
    title: 'Troubleshooting & FAQ',
    sub: 'Broken emoji / backup recovery',
    type: 'section',
    icon: '❓',
  },
  {
    id: 'contributing-license',
    title: 'Contributing & License',
    sub: 'MIT open-source contributions',
    type: 'section',
    icon: '🤝',
  },

  // ── Commands (aliases) ────────────────────────────────
  { id: 'nav-commands', title: '..', sub: 'cd .. — go up 1 level', type: 'cmd', icon: '📂' },
  { id: 'nav-commands', title: '...', sub: 'cd ../.. — go up 2 levels', type: 'cmd', icon: '📂' },
  { id: 'nav-commands', title: 'c', sub: 'clear — clear terminal screen', type: 'cmd', icon: '🧹' },
  { id: 'nav-commands', title: 'h', sub: 'cd ~ — jump to home', type: 'cmd', icon: '🏠' },
  { id: 'nav-commands', title: 'd', sub: 'cd ~/Downloads', type: 'cmd', icon: '📥' },
  { id: 'nav-commands', title: 'desk', sub: 'cd ~/Desktop', type: 'cmd', icon: '🖥️' },
  { id: 'nav-commands', title: 'proj', sub: 'cd ~/Projects', type: 'cmd', icon: '📁' },
  { id: 'pkg-commands', title: 'ni', sub: 'npm install', type: 'cmd', icon: '📦' },
  { id: 'pkg-commands', title: 'nid', sub: 'npm install -D', type: 'cmd', icon: '📦' },
  { id: 'pkg-commands', title: 'nr', sub: 'npm run', type: 'cmd', icon: '▶️' },
  { id: 'pkg-commands', title: 'ns', sub: 'npm start', type: 'cmd', icon: '🚀' },
  { id: 'pkg-commands', title: 'bi', sub: 'bun install', type: 'cmd', icon: '🥐' },
  { id: 'pkg-commands', title: 'brd', sub: 'bun run dev', type: 'cmd', icon: '🥐' },
  { id: 'pkg-commands', title: 'brb', sub: 'bun run build', type: 'cmd', icon: '🥐' },
  { id: 'pkg-commands', title: 'brs', sub: 'bun run start', type: 'cmd', icon: '🥐' },
  { id: 'pkg-commands', title: 'px', sub: 'npx — execute via npx', type: 'cmd', icon: '⚡' },
  { id: 'git-commands', title: 'gs', sub: 'git status', type: 'cmd', icon: '🌿' },
  { id: 'git-commands', title: 'ga', sub: 'git add .', type: 'cmd', icon: '🌿' },
  { id: 'git-commands', title: 'gc', sub: 'git commit -m', type: 'cmd', icon: '🌿' },
  { id: 'git-commands', title: 'gcm', sub: 'git add . && git commit -m', type: 'cmd', icon: '🌿' },
  { id: 'git-commands', title: 'gp', sub: 'git push', type: 'cmd', icon: '🌿' },
  { id: 'git-commands', title: 'gpl', sub: 'git pull', type: 'cmd', icon: '🌿' },
  { id: 'git-commands', title: 'gco', sub: 'git checkout', type: 'cmd', icon: '🌿' },
  {
    id: 'git-commands',
    title: 'gcb',
    sub: 'git checkout -b (new branch)',
    type: 'cmd',
    icon: '🌿',
  },
  { id: 'git-commands', title: 'gb', sub: 'git branch — list branches', type: 'cmd', icon: '🌿' },
  { id: 'git-commands', title: 'glog', sub: 'git log --oneline --graph', type: 'cmd', icon: '🌿' },
  { id: 'git-commands', title: 'gd', sub: 'git diff', type: 'cmd', icon: '🌿' },
  {
    id: 'sys-commands',
    title: 'uup',
    sub: 'Mega updater — OS + Snap + Flatpak + Bun + Node',
    type: 'cmd',
    icon: '🔄',
  },
  { id: 'sys-commands', title: 'uu', sub: 'Universal uninstaller (fzf)', type: 'cmd', icon: '🗑️' },
  { id: 'sys-commands', title: 'sysinfo', sub: 'Full system specs', type: 'cmd', icon: '💻' },
  {
    id: 'sys-commands',
    title: 'ports',
    sub: 'List active listening ports',
    type: 'cmd',
    icon: '🌐',
  },
  { id: 'sys-commands', title: 'myip', sub: 'Show public + local IP', type: 'cmd', icon: '🌐' },
  {
    id: 'sys-commands',
    title: 'gen',
    sub: 'gen [N] — cryptographic key generator',
    type: 'cmd',
    icon: '🔑',
  },
  {
    id: 'sys-commands',
    title: 'ex',
    sub: 'Universal archive extractor (.zip .tar .7z)',
    type: 'cmd',
    icon: '🗜️',
  },
  {
    id: 'interactive-utilities',
    title: 'todo',
    sub: 'Interactive terminal task manager',
    type: 'cmd',
    icon: '📋',
  },
  {
    id: 'interactive-utilities',
    title: 'notes',
    sub: 'Fuzzy terminal notepad',
    type: 'cmd',
    icon: '📝',
  },
  {
    id: 'ffmedia-suite',
    title: 'ffmedia',
    sub: 'FFmpeg multimedia suite',
    type: 'cmd',
    icon: '🎬',
  },
  {
    id: 'vault-suite',
    title: 'vault',
    sub: 'Hardened AES-256 directory vault manager',
    type: 'cmd',
    icon: '🔐',
  },
  {
    id: 'docker-suite',
    title: 'dps',
    sub: 'docker ps — formatted container list',
    type: 'cmd',
    icon: '🐳',
  },
  {
    id: 'docker-suite',
    title: 'dlogs',
    sub: 'docker logs -f [container]',
    type: 'cmd',
    icon: '🐳',
  },
  {
    id: 'docker-suite',
    title: 'dstart',
    sub: 'Start container interactively (fzf)',
    type: 'cmd',
    icon: '🐳',
  },
  {
    id: 'docker-suite',
    title: 'dstop',
    sub: 'Stop container interactively (fzf)',
    type: 'cmd',
    icon: '🐳',
  },
  {
    id: 'docker-suite',
    title: 'dclean',
    sub: 'docker system prune -a --volumes',
    type: 'cmd',
    icon: '🐳',
  },
  {
    id: 'postgres-prisma',
    title: 'pgstart',
    sub: 'Start PostgreSQL service',
    type: 'cmd',
    icon: '🐘',
  },
  {
    id: 'postgres-prisma',
    title: 'pgstop',
    sub: 'Stop PostgreSQL service',
    type: 'cmd',
    icon: '🐘',
  },
  { id: 'postgres-prisma', title: 'pdp', sub: 'npx prisma db push', type: 'cmd', icon: '🐘' },
  { id: 'postgres-prisma', title: 'pds', sub: 'npx prisma studio', type: 'cmd', icon: '🐘' },
  { id: 'postgres-prisma', title: 'pdm', sub: 'npx prisma migrate dev', type: 'cmd', icon: '🐘' },
];

/* Extract all 200+ commands from HTML tables dynamically */
function extractDomCommands() {
  const domEntries = [];
  let itemIdx = 2000;

  document.querySelectorAll('.docs-table tbody tr').forEach((tr) => {
    const tds = tr.querySelectorAll('td');
    if (!tds || tds.length < 2) return;
    if (tds[0].hasAttribute('colspan')) return;

    const codeEl = tds[0].querySelector('code');
    const title = codeEl ? codeEl.textContent.trim() : tds[0].textContent.trim();
    if (!title || title.length > 60) return;

    const sub = tds[1].textContent.trim().replace(/\s+/g, ' ');

    // Find anchor ID from parent section or preceding heading
    let anchorId = 'nav-commands';
    const section = tr.closest('section') || tr.closest('.docs-section');
    if (section) {
      const heading = section.querySelector('h2[id], h3[id]');
      if (heading && heading.id) anchorId = heading.id;
    }

    let icon = '⚡';
    if (title.startsWith('git') || title.startsWith('g')) icon = '🌿';
    else if (title.startsWith('d')) icon = '🐳';
    else if (title.startsWith('pg')) icon = '🐘';
    else if (title.startsWith('np') || title.startsWith('bp')) icon = '💎';
    else if (title.startsWith('ff')) icon = '🎬';
    else if (title.startsWith('todo') || title.startsWith('notes')) icon = '📋';

    domEntries.push({
      id: anchorId,
      title: title,
      sub: sub,
      type: 'cmd',
      icon: icon,
      _idx: itemIdx++,
    });
  });

  return domEntries;
}

/* Build the global Trie index on load */
function buildDocsTrie(dictionary) {
  const trie = new Trie();
  dictionary.forEach((entry, idx) => {
    const uniqueEntry = { ...entry, _idx: entry._idx !== undefined ? entry._idx : idx };
    // Index by every word in title + sub + id so fuzzy-word search works
    const tokens = `${entry.title} ${entry.sub} ${entry.id}`.toLowerCase().split(/\s+/);
    const seen = new Set();
    tokens.forEach((token) => {
      if (token.length < 1) return;
      // Index all prefixes of each token
      for (let len = 1; len <= token.length; len++) {
        const prefix = token.slice(0, len);
        if (!seen.has(prefix)) {
          trie.insert(prefix, uniqueEntry);
          seen.add(prefix);
        }
      }
    });
  });
  return trie;
}

/* ============================================================
   NAVBAR AUTOCOMPLETE CONTROLLER
   ============================================================ */
function initNavAutocomplete() {
  const input = document.getElementById('nav-search-input');
  const dropdown = document.getElementById('nav-autocomplete-list');
  if (!input || !dropdown) return;

  const fullDictionary = [...AUTOCOMPLETE_DICTIONARY, ...extractDomCommands()];
  const trie = buildDocsTrie(fullDictionary);
  let activeIdx = -1; // keyboard-selected item index
  let debounceTimer = null;

  /* ── Helpers ──────────────────────────────────────────── */
  function getItems() {
    return [...dropdown.querySelectorAll('.autocomplete-item')];
  }

  function highlightMatch(text, query) {
    if (!query) return escapeHtml(text);
    const escaped = escapeHtml(text);
    const escapedQ = escapeHtml(query);
    const re = new RegExp(`(${escapedQ.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')})`, 'gi');
    return escaped.replace(re, '<mark>$1</mark>');
  }

  function buildDropdown(results, query) {
    if (!results.length) {
      dropdown.innerHTML = `<div class="autocomplete-empty">No results for "<strong>${escapeHtml(query)}</strong>"</div>`;
      dropdown.hidden = false;
      input.setAttribute('aria-expanded', 'true');
      return;
    }

    // Group by type: sections first, then commands
    const sections = results.filter((r) => r.type === 'section');
    const cmds = results.filter((r) => r.type === 'cmd');

    let html = '';
    if (sections.length) {
      html += `<div class="autocomplete-section-label">📖 Sections</div>`;
      html += sections
        .map(
          (r, i) => `
        <div class="autocomplete-item" role="option" data-href="#${r.id}" tabindex="-1">
          <span class="autocomplete-item-icon">${r.icon}</span>
          <div class="autocomplete-item-body">
            <div class="autocomplete-item-title">${highlightMatch(r.title, query)}</div>
            <div class="autocomplete-item-sub">${escapeHtml(r.sub)}</div>
          </div>
          <span class="autocomplete-item-tag">section</span>
        </div>`
        )
        .join('');
    }
    if (cmds.length) {
      html += `<div class="autocomplete-section-label">⚡ Commands</div>`;
      html += cmds
        .map(
          (r) => `
        <div class="autocomplete-item" role="option" data-href="#${r.id}" tabindex="-1">
          <span class="autocomplete-item-icon">${r.icon}</span>
          <div class="autocomplete-item-body">
            <div class="autocomplete-item-title"><code style="color:var(--docs-cyan);font-family:var(--docs-font-mono)">${highlightMatch(r.title, query)}</code></div>
            <div class="autocomplete-item-sub">${escapeHtml(r.sub)}</div>
          </div>
          <span class="autocomplete-item-tag">alias</span>
        </div>`
        )
        .join('');
    }

    dropdown.innerHTML = html;
    dropdown.hidden = false;
    input.setAttribute('aria-expanded', 'true');
    activeIdx = -1;

    // Touch & Click handlers for autocomplete items
    dropdown.querySelectorAll('.autocomplete-item').forEach((item) => {
      const handleSelect = (e) => {
        e.preventDefault();
        e.stopPropagation();
        navigateTo(item);
      };
      item.addEventListener('click', handleSelect);
      item.addEventListener('touchend', handleSelect);
    });
  }

  function navigateTo(item) {
    const href = item.getAttribute('data-href');
    if (!href) return;
    closeDropdown();
    input.value = '';
    input.blur(); // collapse mobile soft keyboard
    const wrap = document.getElementById('nav-search-wrap');
    if (wrap) wrap.classList.remove('active');

    // Smooth scroll to anchor
    const target = document.querySelector(href);
    if (target) {
      target.scrollIntoView({ behavior: 'smooth', block: 'start' });
      target.focus && target.focus({ preventScroll: true });
    }
  }

  function closeDropdown() {
    dropdown.hidden = true;
    input.setAttribute('aria-expanded', 'false');
    activeIdx = -1;
  }

  /* ── Wrapper tap to focus on mobile ──────────────────── */
  const wrap = document.getElementById('nav-search-wrap');
  if (wrap) {
    wrap.addEventListener('click', (e) => {
      if (e.target !== input) {
        input.focus();
        wrap.classList.add('active');
      }
    });
  }

  input.addEventListener('focus', () => {
    if (wrap) wrap.classList.add('active');
  });

  input.addEventListener('blur', () => {
    setTimeout(() => {
      if (wrap) wrap.classList.remove('active');
    }, 250);
  });

  /* ── Input handler (debounced, Trie query) ──────────── */
  input.addEventListener('input', () => {
    clearTimeout(debounceTimer);
    const q = input.value.trim().toLowerCase();
    if (!q) {
      closeDropdown();
      return;
    }

    debounceTimer = setTimeout(() => {
      // Multi-word: search each token and intersect on score
      const tokens = q.split(/\s+/).filter(Boolean);
      const primary = trie.search(tokens[0], 20);

      // If multiple words, filter primary results to also match later tokens
      const filtered =
        tokens.length > 1
          ? primary.filter((r) => {
              const haystack = `${r.title} ${r.sub} ${r.id}`.toLowerCase();
              return tokens.slice(1).every((t) => haystack.includes(t));
            })
          : primary;

      // Deduplicate by unique _idx
      const seen = new Set();
      const unique = filtered.filter((r) => {
        if (seen.has(r._idx)) return false;
        seen.add(r._idx);
        return true;
      });

      buildDropdown(unique.slice(0, 10), tokens[0]);
    }, 80); // 80ms debounce
  });

  /* ── Keyboard navigation ─────────────────────────────── */
  input.addEventListener('keydown', (e) => {
    const items = getItems();
    if (!items.length) return;

    if (e.key === 'ArrowDown') {
      e.preventDefault();
      activeIdx = Math.min(activeIdx + 1, items.length - 1);
      items.forEach((it, i) =>
        it.setAttribute('aria-selected', i === activeIdx ? 'true' : 'false')
      );
      items[activeIdx]?.scrollIntoView({ block: 'nearest' });
    } else if (e.key === 'ArrowUp') {
      e.preventDefault();
      activeIdx = Math.max(activeIdx - 1, 0);
      items.forEach((it, i) =>
        it.setAttribute('aria-selected', i === activeIdx ? 'true' : 'false')
      );
      items[activeIdx]?.scrollIntoView({ block: 'nearest' });
    } else if (e.key === 'Enter') {
      e.preventDefault();
      if (activeIdx >= 0 && items[activeIdx]) {
        navigateTo(items[activeIdx]);
      } else if (items[0]) {
        navigateTo(items[0]);
      }
    } else if (e.key === 'Escape') {
      closeDropdown();
      input.blur();
    } else if (e.key === 'Tab') {
      // Tab-complete: fill input with first suggestion title
      if (!dropdown.hidden && items[0]) {
        e.preventDefault();
        const title = items[0].querySelector('.autocomplete-item-title')?.textContent?.trim();
        if (title) {
          input.value = title;
          input.dispatchEvent(new Event('input'));
        }
      }
    }
  });

  /* ── Close on outside click ──────────────────────────── */
  document.addEventListener('click', (e) => {
    if (!e.target.closest('#nav-search-wrap')) closeDropdown();
  });

  /* ── Ctrl+K / ⌘K focus ──────────────────────────────── */
  document.addEventListener('keydown', (e) => {
    if ((e.ctrlKey || e.metaKey) && e.key === 'k') {
      e.preventDefault();
      input.focus();
      input.select();
    }
  });
}

/* ============================================================
   1. THEME PICKER
   ============================================================ */
function initThemePicker() {
  const dots = document.querySelectorAll('.theme-dot');
  const savedTheme = localStorage.getItem('fancybash_docs_theme') || 'default';

  applyTheme(savedTheme);

  dots.forEach((dot) => {
    dot.addEventListener('click', () => {
      const theme = dot.getAttribute('data-t');
      applyTheme(theme);
      localStorage.setItem('fancybash_docs_theme', theme);
    });
  });

  function applyTheme(theme) {
    if (theme === 'default') {
      document.body.removeAttribute('data-theme');
    } else {
      document.body.setAttribute('data-theme', theme);
    }
    dots.forEach((d) => {
      if (d.getAttribute('data-t') === theme) {
        d.classList.add('active');
      } else {
        d.classList.remove('active');
      }
    });
  }
}

/* ============================================================
   2. COPY TO CLIPBOARD & TOAST NOTIFICATION
   ============================================================ */
function initCopyButtons() {
  document.addEventListener('click', (e) => {
    const copyBtn = e.target.closest('.copy-btn');
    if (!copyBtn) return;

    const targetId = copyBtn.getAttribute('data-copy-target');
    let textToCopy = '';

    if (targetId) {
      const targetEl = document.getElementById(targetId);
      if (targetEl) textToCopy = targetEl.innerText.trim();
    } else {
      const codeWrapper = copyBtn.closest('.code-wrapper');
      if (codeWrapper) {
        const pre = codeWrapper.querySelector('pre');
        if (pre) textToCopy = pre.innerText.trim();
      }
    }

    if (!textToCopy) return;

    navigator.clipboard
      .writeText(textToCopy)
      .then(() => {
        showToast('Copied to clipboard! 🚀');
        const originalText = copyBtn.innerHTML;
        copyBtn.innerHTML = `<span>✓</span> Copied`;
        setTimeout(() => {
          copyBtn.innerHTML = originalText;
        }, 2000);
      })
      .catch((err) => {
        showToast('Failed to copy: ' + err);
      });
  });
}

function showToast(message) {
  let container = document.querySelector('.toast-container');
  if (!container) {
    container = document.createElement('div');
    container.className = 'toast-container';
    document.body.appendChild(container);
  }

  const toast = document.createElement('div');
  toast.className = 'toast';
  toast.innerHTML = `<span>⚡</span> ${message}`;
  container.appendChild(toast);

  setTimeout(() => {
    toast.style.opacity = '0';
    toast.style.transform = 'translateY(10px)';
    toast.style.transition = 'all 0.3s ease';
    setTimeout(() => toast.remove(), 300);
  }, 2500);
}

/* ============================================================
   3. INTERSECTION OBSERVER FOR SIDEBAR HIGHLIGHTING
   ============================================================ */
function initIntersectionObserver() {
  const headings = document.querySelectorAll('.docs-content h2[id], .docs-content h3[id]');
  const sidebarLinks = document.querySelectorAll('.sidebar-links a');
  const tocLinks = document.querySelectorAll('.toc-links a');

  if (!headings.length) return;

  const observerOptions = {
    root: null,
    rootMargin: '-80px 0px -60% 0px',
    threshold: 0,
  };

  const observer = new IntersectionObserver((entries) => {
    entries.forEach((entry) => {
      if (entry.isIntersecting) {
        const id = entry.target.getAttribute('id');
        setActiveLink(id);
      }
    });
  }, observerOptions);

  headings.forEach((h) => observer.observe(h));

  function setActiveLink(id) {
    sidebarLinks.forEach((link) => {
      if (link.getAttribute('href') === `#${id}`) {
        link.classList.add('active');
      } else {
        link.classList.remove('active');
      }
    });

    tocLinks.forEach((link) => {
      if (link.getAttribute('href') === `#${id}`) {
        link.classList.add('active');
      } else {
        link.classList.remove('active');
      }
    });
  }
}

/* ============================================================
   4. INTERACTIVE COMMAND EXPLORER
   ============================================================ */
const COMMAND_DATABASE = [
  // Navigation
  { name: '..', desc: 'Go up 1 directory level', exp: 'cd ..', cat: 'nav' },
  { name: '...', desc: 'Go up 2 directory levels', exp: 'cd ../..', cat: 'nav' },
  { name: '....', desc: 'Go up 3 directory levels', exp: 'cd ../../..', cat: 'nav' },
  { name: 'c', desc: 'Clear terminal screen', exp: 'clear', cat: 'nav' },
  { name: 'h', desc: 'Jump to Home directory', exp: 'cd ~', cat: 'nav' },
  { name: 'd', desc: 'Jump to Downloads folder', exp: 'cd ~/Downloads', cat: 'nav' },
  { name: 'desk', desc: 'Jump to Desktop folder', exp: 'cd ~/Desktop', cat: 'nav' },
  { name: 'proj', desc: 'Jump to Projects folder', exp: 'cd ~/Projects', cat: 'nav' },

  // Package Managers
  { name: 'ni', desc: 'Install npm packages / dependencies', exp: 'npm install', cat: 'bun' },
  { name: 'nid', desc: 'Install npm dev dependency', exp: 'npm install -D', cat: 'bun' },
  { name: 'nr', desc: 'Run npm script interactively', exp: 'npm run', cat: 'bun' },
  { name: 'ns', desc: 'Start dev server', exp: 'npm start', cat: 'bun' },
  { name: 'bi', desc: 'Install package with Bun', exp: 'bun install', cat: 'bun' },
  { name: 'brd', desc: 'Run dev script with Bun', exp: 'bun run dev', cat: 'bun' },
  { name: 'brb', desc: 'Run build script with Bun', exp: 'bun run build', cat: 'bun' },
  { name: 'brs', desc: 'Run start script with Bun', exp: 'bun run start', cat: 'bun' },
  { name: 'px', desc: 'Execute package via npx', exp: 'npx', cat: 'bun' },

  // Git Shortcuts
  { name: 'g', desc: 'Git status / shortcut', exp: 'git', cat: 'git' },
  { name: 'gs', desc: 'Show detailed git status', exp: 'git status', cat: 'git' },
  { name: 'ga', desc: 'Stage all modified files', exp: 'git add .', cat: 'git' },
  { name: 'gc', desc: 'Commit with message prompt', exp: 'git commit -m', cat: 'git' },
  {
    name: 'gcm',
    desc: 'Stage all and commit with message',
    exp: 'git add . && git commit -m',
    cat: 'git',
  },
  { name: 'gp', desc: 'Push commits to remote repository', exp: 'git push', cat: 'git' },
  { name: 'gpl', desc: 'Pull latest changes from remote', exp: 'git pull', cat: 'git' },
  { name: 'gco', desc: 'Switch branch / checkout', exp: 'git checkout', cat: 'git' },
  { name: 'gcb', desc: 'Create and switch to new branch', exp: 'git checkout -b', cat: 'git' },
  { name: 'gb', desc: 'List local branches', exp: 'git branch', cat: 'git' },
  {
    name: 'glog',
    desc: 'Beautiful compact graph git log',
    exp: 'git log --oneline --graph --decorate',
    cat: 'git',
  },
  { name: 'gd', desc: 'Show git diff changes', exp: 'git diff', cat: 'git' },
  { name: 'grh', desc: 'Hard reset repository state', exp: 'git reset --hard', cat: 'git' },

  // System & Maintenance
  {
    name: 'uup',
    desc: 'Mega Updater: OS + Snap + Flatpak + Bun + Node',
    exp: 'sudo apt update && apt upgrade...',
    cat: 'system',
  },
  {
    name: 'uu',
    desc: 'Universal Uninstaller: fzf app remover',
    exp: 'interactive package cleanup',
    cat: 'system',
  },
  {
    name: 'sysinfo',
    desc: 'Display complete system specs & hardware',
    exp: 'neofetch / fastfetch / lsb_release',
    cat: 'system',
  },
  {
    name: 'ports',
    desc: 'List active listening network ports',
    exp: 'sudo netstat -tulpn / ss -tulpn',
    cat: 'system',
  },
  {
    name: 'myip',
    desc: 'Display public and local IP addresses',
    exp: 'curl ifconfig.me / ip route',
    cat: 'system',
  },
  {
    name: 'gen',
    desc: 'Generate cryptographically secure key',
    exp: 'openssl rand -base64 [length]',
    cat: 'system',
  },
  {
    name: 'ex',
    desc: 'Universal archive extractor (.zip, .tar, .7z)',
    exp: 'auto tar / unzip / 7z extract',
    cat: 'system',
  },

  // Utilities & Interactive
  {
    name: 'todo',
    desc: 'Interactive terminal task manager',
    exp: 'gum task list & status selector',
    cat: 'utility',
  },
  {
    name: 'notes',
    desc: 'Quick terminal notepad & search',
    exp: 'fzf note launcher & previewer',
    cat: 'utility',
  },
  {
    name: 'ffmedia',
    desc: 'FFmpeg video conversion suite',
    exp: 'interactive video compress/trim/convert',
    cat: 'utility',
  },
  {
    name: 'vault',
    desc: 'AES-256 multi-vault directory manager',
    exp: 'memory-guarded directory lock & unlock',
    cat: 'utility',
  },

  // Docker Suite
  {
    name: 'dps',
    desc: 'List running containers with clean format',
    exp: 'docker ps --format ...',
    cat: 'docker',
  },
  {
    name: 'dlogs',
    desc: 'Tail logs of selected container',
    exp: 'docker logs -f [container]',
    cat: 'docker',
  },
  {
    name: 'dstart',
    desc: 'Interactively start containers with fzf',
    exp: 'docker start $(docker ps -a...)',
    cat: 'docker',
  },
  {
    name: 'dstop',
    desc: 'Interactively stop containers with fzf',
    exp: 'docker stop $(docker ps...)',
    cat: 'docker',
  },
  {
    name: 'dclean',
    desc: 'Prune stopped containers & dangling images',
    exp: 'docker system prune -a --volumes',
    cat: 'docker',
  },

  // PostgreSQL & Prisma
  {
    name: 'pgstart',
    desc: 'Start local PostgreSQL service',
    exp: 'sudo systemctl start postgresql',
    cat: 'postgres',
  },
  {
    name: 'pgstop',
    desc: 'Stop local PostgreSQL service',
    exp: 'sudo systemctl stop postgresql',
    cat: 'postgres',
  },
  {
    name: 'pdp',
    desc: 'Push Prisma schema to database',
    exp: 'npx prisma db push',
    cat: 'postgres',
  },
  {
    name: 'pds',
    desc: 'Open interactive Prisma Studio UI',
    exp: 'npx prisma studio',
    cat: 'postgres',
  },
  {
    name: 'pdm',
    desc: 'Generate & apply Prisma migration',
    exp: 'npx prisma migrate dev',
    cat: 'postgres',
  },
];

function initCommandExplorer() {
  const grid = document.getElementById('explorer-grid');
  const searchInput = document.getElementById('explorer-search-input');
  const filterChips = document.querySelectorAll('.explorer-filters .filter-chip');

  if (!grid) return;

  let activeCategory = 'all';
  let searchQuery = '';

  renderExplorer();

  if (searchInput) {
    searchInput.addEventListener('input', (e) => {
      searchQuery = e.target.value.toLowerCase().trim();
      renderExplorer();
    });
  }

  filterChips.forEach((chip) => {
    chip.addEventListener('click', () => {
      filterChips.forEach((c) => c.classList.remove('active'));
      chip.classList.add('active');
      activeCategory = chip.getAttribute('data-cat');
      renderExplorer();
    });
  });

  function renderExplorer() {
    const filtered = COMMAND_DATABASE.filter((cmd) => {
      const matchCat = activeCategory === 'all' || cmd.cat === activeCategory;
      const matchQuery =
        !searchQuery ||
        cmd.name.toLowerCase().includes(searchQuery) ||
        cmd.desc.toLowerCase().includes(searchQuery) ||
        cmd.exp.toLowerCase().includes(searchQuery);
      return matchCat && matchQuery;
    });

    if (!filtered.length) {
      grid.innerHTML = `
        <div style="grid-column: 1/-1; text-align: center; padding: 32px; color: var(--docs-text-dim);">
          No matching commands found for "${searchQuery}".
        </div>
      `;
      return;
    }

    grid.innerHTML = filtered
      .map(
        (cmd) => `
      <div class="cmd-card">
        <div>
          <div class="cmd-name">
            <span>${escapeHtml(cmd.name)}</span>
            <span class="cmd-cat-badge">${escapeHtml(cmd.cat)}</span>
          </div>
          <div class="cmd-desc">${escapeHtml(cmd.desc)}</div>
        </div>
        <div class="cmd-expansion">${escapeHtml(cmd.exp)}</div>
      </div>
    `
      )
      .join('');
  }
}

/* ============================================================
   5. INTERACTIVE TERMINAL PLAYGROUND
   ============================================================ */
function initTerminalPlayground() {
  const body = document.getElementById('term-playground-body');
  const chips = document.querySelectorAll('.terminal-quick-chips .quick-chip');

  if (!body) return;

  const COMMAND_SIMULATIONS = {
    'gen 32': [
      { text: '🔑 Generating cryptographically secure 32-byte secret key...', class: 'cyan' },
      {
        text: 'd8f92a10b471c5e6f39e8a71239c09a82f7e61c5a9b83f21a0c4e5f67b89012d',
        class: 'success',
      },
      { text: '✓ Copied to clipboard buffer.', class: 'warning' },
    ],
    uup: [
      { text: '🔄 Launching fancybash Mega-Updater (uup)...', class: 'cyan' },
      { text: '[1/5] Updating APT package repositories & upgrades... Done.', class: 'success' },
      { text: '[2/5] Updating Snap packages... All snaps up to date.', class: 'success' },
      { text: '[3/5] Updating Flatpak runtimes... 2 packages updated.', class: 'success' },
      { text: '[4/5] Upgrading Bun & Node environment... Bun 1.1.20 ready.', class: 'success' },
      {
        text: '[5/5] Checking fancybash git repository... Latest version active!',
        class: 'success',
      },
      { text: '✨ System update completed in 4.2s!', class: 'warning' },
    ],
    sysinfo: [
      { text: '💻 fancybash System Summary:', class: 'cyan' },
      { text: 'OS: Ubuntu 24.04 LTS (x86_64)', class: '' },
      { text: 'Kernel: Linux 6.8.0-40-generic', class: '' },
      { text: 'CPU: AMD Ryzen 7 7840HS @ 3.80GHz (16 cores)', class: '' },
      { text: 'Memory: 7.8 GB / 32.0 GB (24% used)', class: '' },
      { text: 'Disk /: 45.2 GB free of 500 GB (12% used)', class: '' },
      { text: 'Shell: fancybash v2.0 (Bash 5.2.21)', class: 'success' },
    ],
    'ex demo.tar.gz': [
      { text: '🗜️ Detecting archive type: Gzip Compressed Tar', class: 'cyan' },
      { text: 'Extracting demo.tar.gz → ./demo/', class: '' },
      { text: '  - ./demo/package.json', class: '' },
      { text: '  - ./demo/src/main.ts', class: '' },
      { text: '  - ./demo/README.md', class: '' },
      { text: '✓ Extraction finished successfully.', class: 'success' },
    ],
    dps: [
      {
        text: 'CONTAINER ID   IMAGE          COMMAND                 STATUS         PORTS',
        class: 'cyan',
      },
      {
        text: 'a1b2c3d4e5f6   postgres:16    "docker-entrypoint.s…"  Up 4 hours     0.0.0.0:5432->5432/tcp',
        class: '',
      },
      {
        text: 'f9e8d7c6b5a4   redis:alpine   "docker-entrypoint.s…"  Up 4 hours     0.0.0.0:6379->6379/tcp',
        class: '',
      },
      {
        text: '123456789abc   nginx:latest   "/docker-entrypoint…"   Up 2 hours     0.0.0.0:80->80/tcp',
        class: '',
      },
    ],
    glog: [
      {
        text: '* 8f9a2b1 (HEAD -> main, origin/main) feat: add interactive docs.html',
        class: 'cyan',
      },
      { text: '* 4c3d2e1 docs: update roadmap and setup guide', class: '' },
      { text: '* 1a2b3c4 fix: resolve zsh prompt color bleeding', class: '' },
      { text: '* 9f8e7d6 v2.0 initial release', class: 'warning' },
    ],
  };

  chips.forEach((chip) => {
    chip.addEventListener('click', () => {
      const cmd = chip.getAttribute('data-cmd') || chip.innerText.trim();
      runSimulation(cmd);
    });
  });

  function runSimulation(cmd) {
    const line = document.createElement('div');
    line.className = 'term-line';
    line.innerHTML = `<span class="term-prompt">🚀 fancybash ❯ </span><span class="term-input">${escapeHtml(cmd)}</span>`;
    body.appendChild(line);

    const sim = COMMAND_SIMULATIONS[cmd] || [
      { text: `Executing fancybash alias: ${cmd}...`, class: 'cyan' },
      { text: '✓ Command finished successfully.', class: 'success' },
    ];

    sim.forEach((item, index) => {
      setTimeout(
        () => {
          const outLine = document.createElement('div');
          outLine.className = `term-out ${item.class || ''}`;
          outLine.innerText = item.text;
          body.appendChild(outLine);
          body.scrollTop = body.scrollHeight;
        },
        (index + 1) * 200
      );
    });
  }
}

/* ============================================================
   6. INTERACTIVE INSTALLER BUILDER
   ============================================================ */
function initInstallerBuilder() {
  const shellRadios = document.querySelectorAll('input[name="builder-shell"]');
  const featCheckboxes = document.querySelectorAll('.builder-feat-check');
  const outputCmd = document.getElementById('builder-output-cmd');
  const outputCode = document.getElementById('builder-output-code');

  if (!outputCmd) return;

  const INSTALL_COMMANDS = {
    bash: 'curl -fsSL https://fancybash.netlify.app/public/install.sh | bash',
    zsh: 'curl -fsSL https://fancybash.netlify.app/public/install.zsh | zsh',
    fish: 'curl -fsSL https://fancybash.netlify.app/public/install.fish | fish',
    powershell:
      'irm https://fancybash.netlify.app/public/i.ps1 | iex',
  };

  function updateBuilder() {
    let selectedShell = 'bash';
    shellRadios.forEach((r) => {
      if (r.checked) selectedShell = r.value;
    });

    let activeFeats = [];
    featCheckboxes.forEach((cb) => {
      if (cb.checked) activeFeats.push(cb.value);
    });

    outputCmd.innerText = INSTALL_COMMANDS[selectedShell] || INSTALL_COMMANDS.bash;

    let configText = `# fancybash custom configuration for ${selectedShell.toUpperCase()}\n`;
    if (activeFeats.includes('cputemp')) configText += `export FANCY_SHOW_CPU_TEMP=true\n`;
    if (activeFeats.includes('gitdirty')) configText += `export FANCY_SHOW_GIT_DIRTY=true\n`;
    if (activeFeats.includes('bun')) configText += `export FANCY_ENABLE_BUN_ALIASES=true\n`;
    if (activeFeats.includes('docker')) configText += `export FANCY_ENABLE_DOCKER_ALIASES=true\n`;
    if (activeFeats.includes('timer')) configText += `export FANCY_SHOW_EXEC_TIMER=true\n`;

    outputCode.innerText = configText;
  }

  shellRadios.forEach((r) => r.addEventListener('change', updateBuilder));
  featCheckboxes.forEach((cb) => cb.addEventListener('change', updateBuilder));
  updateBuilder();
}

/* ============================================================
   7. GLOBAL SEARCH MODAL (Ctrl+K)
   ============================================================ */
function initSearchModal() {
  const backdrop = document.getElementById('search-modal-backdrop');
  const modalInput = document.getElementById('modal-search-input');
  const modalResults = document.getElementById('modal-search-results');

  if (!backdrop) return;

  // '/' key opens the big modal; Ctrl+K is now owned by navbar Trie search
  document.addEventListener('keydown', (e) => {
    if (
      e.key === '/' &&
      document.activeElement.tagName !== 'INPUT' &&
      document.activeElement.tagName !== 'TEXTAREA'
    ) {
      e.preventDefault();
      openSearchModal();
    } else if (e.key === 'Escape' && backdrop.classList.contains('open')) {
      closeSearchModal();
    }
  });

  backdrop.addEventListener('click', (e) => {
    if (e.target === backdrop) closeSearchModal();
  });

  if (modalInput) {
    modalInput.addEventListener('input', () => {
      renderSearchModalResults(modalInput.value.toLowerCase().trim());
    });
  }

  function openSearchModal() {
    backdrop.classList.add('open');
    if (modalInput) {
      modalInput.value = '';
      modalInput.focus();
      renderSearchModalResults('');
    }
  }

  function closeSearchModal() {
    backdrop.classList.remove('open');
  }

  function renderSearchModalResults(q) {
    if (!modalResults) return;

    // Index all h2, h3 and commands
    const sections = Array.from(
      document.querySelectorAll('.docs-content h2[id], .docs-content h3[id]')
    ).map((el) => ({
      title: el.innerText.replace(/^[^\w]+/, '').trim(),
      id: el.getAttribute('id'),
      sub: 'Documentation Section',
    }));

    const cmds = COMMAND_DATABASE.map((c) => ({
      title: `${c.name} — ${c.desc}`,
      id: 'command-reference',
      sub: `Command Alias (${c.cat})`,
    }));

    const allItems = [...sections, ...cmds];

    const filtered = allItems.filter(
      (item) => !q || item.title.toLowerCase().includes(q) || item.sub.toLowerCase().includes(q)
    );

    if (!filtered.length) {
      modalResults.innerHTML = `<div style="padding:20px; text-align:center; color:var(--docs-text-dim)">No results found for "${q}"</div>`;
      return;
    }

    modalResults.innerHTML = filtered
      .slice(0, 10)
      .map(
        (item, idx) => `
      <a href="#${item.id}" class="search-res-item ${idx === 0 ? 'selected' : ''}" onclick="document.getElementById('search-modal-backdrop').classList.remove('open')">
        <div>
          <div class="search-res-title">${escapeHtml(item.title)}</div>
          <div class="search-res-sub">${escapeHtml(item.sub)}</div>
        </div>
        <span style="font-size:0.8rem; color:var(--docs-cyan)">↵</span>
      </a>
    `
      )
      .join('');
  }
}

/* ============================================================
   8. MOBILE SIDEBAR DRAWER
   ============================================================ */
function initMobileSidebar() {
  const btn = document.getElementById('mobile-menu-btn');
  const sidebar = document.getElementById('docs-sidebar');

  if (!btn || !sidebar) return;

  // Create overlay if not present
  let overlay = document.querySelector('.sidebar-overlay');
  if (!overlay) {
    overlay = document.createElement('div');
    overlay.className = 'sidebar-overlay';
    document.body.appendChild(overlay);
  }

  function openSidebar() {
    sidebar.classList.add('open');
    overlay.classList.add('active');
    document.body.style.overflow = 'hidden'; // prevent background body scroll
  }

  function closeSidebar() {
    sidebar.classList.remove('open');
    overlay.classList.remove('active');
    document.body.style.overflow = '';
  }

  const closeBtn = document.getElementById('mobile-sidebar-close');
  if (closeBtn) {
    closeBtn.addEventListener('click', closeSidebar);
  }

  btn.addEventListener('click', (e) => {
    e.stopPropagation();
    if (sidebar.classList.contains('open')) {
      closeSidebar();
    } else {
      openSidebar();
    }
  });

  overlay.addEventListener('click', closeSidebar);

  // Close sidebar when clicking any link inside sidebar
  sidebar.querySelectorAll('.sidebar-links a').forEach((link) => {
    link.addEventListener('click', () => {
      if (window.innerWidth <= 992) {
        closeSidebar();
      }
    });
  });

  // Handle window resize
  window.addEventListener('resize', () => {
    if (window.innerWidth > 992) {
      closeSidebar();
    }
  });
}

/* Helper to prevent XSS */
function escapeHtml(str) {
  return String(str)
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/"/g, '&quot;');
}

/* ============================================================
   ADVANCED BENCHMARK SECTION
   Tab switching · Animated bar fills · Canvas radar chart
   ============================================================ */

function initBenchmark() {
  /* ── 1. Tab Switcher ── */
  const tabBtns  = document.querySelectorAll('.bench-tab-btn');
  const panels   = document.querySelectorAll('.bench-panel');

  if (!tabBtns.length) return;

  tabBtns.forEach(btn => {
    btn.addEventListener('click', () => {
      const target = btn.dataset.benchTab;

      // Update buttons
      tabBtns.forEach(b => {
        b.classList.remove('active');
        b.setAttribute('aria-selected', 'false');
      });
      btn.classList.add('active');
      btn.setAttribute('aria-selected', 'true');

      // Update panels
      panels.forEach(p => p.classList.remove('active'));
      const activePanel = document.getElementById('bpanel-' + target);
      if (activePanel) {
        activePanel.classList.add('active');
        // Trigger bar animations for newly visible panel
        triggerBarsInPanel(activePanel);
        // Draw radar if overview tab
        if (target === 'overview') {
          drawRadar();
        }
      }
    });
  });

  /* ── 2. Animated Bar Fills via IntersectionObserver ── */
  function triggerBarsInPanel(panel) {
    const fills = panel.querySelectorAll('.bench-bar-fill[data-pct]');
    fills.forEach(fill => {
      const pct = parseFloat(fill.dataset.pct) || 0;
      // Use requestAnimationFrame to ensure layout is computed before animating
      requestAnimationFrame(() => {
        fill.style.width = Math.max(pct, pct === 0 ? 0 : 1) + '%';
      });
    });
  }

  // Observe the benchmark hero to trigger the initially visible panel's bars
  const hero = document.querySelector('.bench-hero');
  if (hero && 'IntersectionObserver' in window) {
    const obs = new IntersectionObserver((entries) => {
      entries.forEach(entry => {
        if (entry.isIntersecting) {
          // Animate the currently active panel
          const activePanel = document.querySelector('.bench-panel.active');
          if (activePanel) triggerBarsInPanel(activePanel);
          obs.disconnect();
        }
      });
    }, { threshold: 0.15 });
    obs.observe(hero);
  } else {
    // Fallback: trigger immediately
    const activePanel = document.querySelector('.bench-panel.active');
    if (activePanel) triggerBarsInPanel(activePanel);
  }

  /* ── 3. Canvas Radar Chart ── */
  function drawRadar() {
    const canvas = document.getElementById('bench-radar');
    if (!canvas) return;
    const ctx = canvas.getContext('2d');
    const W = canvas.width;
    const H = canvas.height;
    const cx = W / 2;
    const cy = H / 2;
    const R  = Math.min(W, H) / 2 - 32;

    // Axes labels
    const axes = [
      'Startup\nSpeed',
      'Prompt\nSpeed',
      'Memory\nEfficiency',
      'Feature\nCount',
      'Theme\nRichness',
      'POSIX\nCompat'
    ];
    const N = axes.length;

    // Shell scores [0-10] per axis: Startup, Prompt, Memory, Features, Themes, POSIX
    const shells = [
      { name: 'FancyBash', color: '#22d3ee',  fill: 'rgba(34,211,238,0.15)',  scores: [10, 10, 9.8, 8.5, 9, 9] },
      { name: 'Bash',      color: '#22c55e',  fill: 'rgba(34,197,94,0.10)',   scores: [9.7, 5, 9.9, 4, 1, 10] },
      { name: 'Fish',      color: '#0891b2',  fill: 'rgba(8,145,178,0.10)',   scores: [8.5, 7, 8.5, 7, 3, 0] },
      { name: 'PS7',       color: '#3b82f6',  fill: 'rgba(59,130,246,0.10)',  scores: [2, 4, 1, 7, 3, 0] },
      { name: 'Oh My Zsh', color: '#ef4444',  fill: 'rgba(239,68,68,0.10)',   scores: [1, 2, 1, 9, 10, 10] },
      { name: 'Oh My Posh',color: '#f97316',  fill: 'rgba(249,115,22,0.10)',  scores: [4, 1, 4, 7, 10, 5] },
    ];

    // Clear
    ctx.clearRect(0, 0, W, H);

    // Helper: polar to cartesian
    function polar(angle, r) {
      return {
        x: cx + r * Math.cos(angle - Math.PI / 2),
        y: cy + r * Math.sin(angle - Math.PI / 2),
      };
    }

    // Draw grid rings
    const rings = 5;
    for (let r = 1; r <= rings; r++) {
      const radius = (R / rings) * r;
      ctx.beginPath();
      for (let i = 0; i < N; i++) {
        const angle = (2 * Math.PI / N) * i;
        const pt = polar(angle, radius);
        i === 0 ? ctx.moveTo(pt.x, pt.y) : ctx.lineTo(pt.x, pt.y);
      }
      ctx.closePath();
      ctx.strokeStyle = 'rgba(255,255,255,0.07)';
      ctx.lineWidth = 1;
      ctx.stroke();
    }

    // Draw axis spokes
    for (let i = 0; i < N; i++) {
      const angle = (2 * Math.PI / N) * i;
      const outer = polar(angle, R);
      ctx.beginPath();
      ctx.moveTo(cx, cy);
      ctx.lineTo(outer.x, outer.y);
      ctx.strokeStyle = 'rgba(255,255,255,0.1)';
      ctx.lineWidth = 1;
      ctx.stroke();
    }

    // Draw axis labels
    ctx.font = '10px Inter, sans-serif';
    ctx.fillStyle = 'rgba(148,163,184,0.9)';
    ctx.textAlign = 'center';
    ctx.textBaseline = 'middle';
    for (let i = 0; i < N; i++) {
      const angle = (2 * Math.PI / N) * i;
      const labelR = R + 22;
      const pt = polar(angle, labelR);
      const lines = axes[i].split('\n');
      lines.forEach((line, li) => {
        ctx.fillText(line, pt.x, pt.y + (li - (lines.length - 1) / 2) * 13);
      });
    }

    // Draw each shell polygon
    shells.forEach(shell => {
      ctx.beginPath();
      for (let i = 0; i < N; i++) {
        const angle = (2 * Math.PI / N) * i;
        const val   = shell.scores[i] / 10;
        const pt    = polar(angle, R * val);
        i === 0 ? ctx.moveTo(pt.x, pt.y) : ctx.lineTo(pt.x, pt.y);
      }
      ctx.closePath();
      ctx.fillStyle   = shell.fill;
      ctx.fill();
      ctx.strokeStyle = shell.color;
      ctx.lineWidth   = 1.8;
      ctx.stroke();

      // Dot markers on each axis
      for (let i = 0; i < N; i++) {
        const angle = (2 * Math.PI / N) * i;
        const val   = shell.scores[i] / 10;
        const pt    = polar(angle, R * val);
        ctx.beginPath();
        ctx.arc(pt.x, pt.y, 3, 0, Math.PI * 2);
        ctx.fillStyle = shell.color;
        ctx.fill();
      }
    });

    // Centre dot
    ctx.beginPath();
    ctx.arc(cx, cy, 3, 0, Math.PI * 2);
    ctx.fillStyle = 'rgba(255,255,255,0.2)';
    ctx.fill();
  }
}
