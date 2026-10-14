# Windows & PowerShell Web UI Re-enablement Playbook
**File:** `win-btn-recovery-stable.md`  
**Purpose:** Comprehensive, exact recovery guide to restore all Windows & PowerShell installation buttons, tabs, badges, and documentation once PowerShell reaches stable parity with Linux & macOS.  
**Instruction for Future Agent:** When the user says *"win-btn-recovery-stable.md dekho and kaj koro"*, follow this file step-by-step to re-enable everything and remove all "Coming Soon" restrictions.

---

## 📋 Quick Action Summary

To restore Windows to full active status across the website:
1. **`web/index.html`**:
   - Re-enable the main Install tab: remove `data-disabled="true"`, `data-tooltip`, and `<span class="coming-soon-badge">Soon</span>` from `#tab-btn-win`.
   - Re-enable the Install panel copy button: remove `data-disabled="true"`, `data-tooltip`, and the badge in `#tab-win`.
   - Re-enable the Uninstall tab: remove `data-disabled="true"`, `data-tooltip`, and badge from `#utab-btn-ps`.
   - Re-enable the Setup tab: remove `data-disabled="true"`, `data-tooltip`, and badge from `[data-setup-tab="ps"]`.
2. **`web/docs.html`**:
   - Universal 1-line PowerShell installer block: restore opacity to `1.0`, remove `coming-soon-badge`, restore standard `Copy Command` button.
   - Shell-specific table row: replace the temporary disabled button with the live code command `<code data-copy='...'>powershell -c "irm https://gladeshell.netlify.app/install.ps1 | iex"</code>`.
   - PowerShell Setup section: remove `coming-soon-badge` and restore normal script execution notes.
3. **Run Validation**:
   - Validate HTML: Python `HTMLParser`
   - Validate JS: `node --check web/main.js && node --check web/docs.js`
   - Run tests: `cargo test --tests`

---

## 🛠 File-by-File Exact Replacements

### 1. `web/index.html`

#### A. Main OS Install Tab (`#tab-btn-win`)
- **Target Location:** Around line 655
- **Current Disabled Code:**
```html
            <button
              class="tab-btn"
              role="tab"
              aria-selected="false"
              aria-controls="tab-win"
              id="tab-btn-win"
              data-tab="win"
              data-disabled="true"
              data-tooltip="Windows coming soon"
            >
              <span>🪟</span> Windows <span class="coming-soon-badge">Soon</span>
            </button>
```
- **Restore To (Enabled):**
```html
            <button
              class="tab-btn"
              role="tab"
              aria-selected="false"
              aria-controls="tab-win"
              id="tab-btn-win"
              data-tab="win"
            >
              <span>🪟</span> Windows
            </button>
```

---

#### B. Main OS Install Panel (`#tab-win`)
- **Target Location:** Around line 720
- **Current Disabled Code:**
```html
                <div class="code-header" style="background: rgba(137, 180, 250, 0.1)">
                  <span class="code-lang" style="color: #89b4fa; font-weight: 600"
                    >🪟 Universal Auto-Detect (Windows) <span class="coming-soon-badge">Coming Soon</span></span
                  >
                  <button
                    class="copy-btn"
                    aria-label="Copy universal Windows install command"
                    data-copy='powershell -c "irm https://gladeshell.netlify.app/install.ps1 | iex"'
                    data-disabled="true"
                    data-tooltip="Windows coming soon"
                  >
```
- **Restore To (Enabled):**
```html
                <div class="code-header" style="background: rgba(137, 180, 250, 0.1)">
                  <span class="code-lang" style="color: #89b4fa; font-weight: 600"
                    >🪟 Universal Auto-Detect (Windows)</span
                  >
                  <button
                    class="copy-btn"
                    aria-label="Copy universal Windows install command"
                    data-copy='powershell -c "irm https://gladeshell.netlify.app/install.ps1 | iex"'
                  >
```

---

#### C. Uninstall Tab (`#utab-btn-ps`)
- **Target Location:** Around line 1080
- **Current Disabled Code:**
```html
                <button
                  class="utab-btn"
                  role="tab"
                  aria-selected="false"
                  aria-controls="utab-ps"
                  id="utab-btn-ps"
                  data-utab="ps"
                  data-disabled="true"
                  data-tooltip="Windows coming soon"
                >
                  🪟 PowerShell <span class="coming-soon-badge">Soon</span>
                </button>
```
- **Restore To (Enabled):**
```html
                <button
                  class="utab-btn"
                  role="tab"
                  aria-selected="false"
                  aria-controls="utab-ps"
                  id="utab-btn-ps"
                  data-utab="ps"
                >
                  🪟 PowerShell
                </button>
```

---

#### D. Setup Tab (`[data-setup-tab="ps"]`)
- **Target Location:** Around line 1265
- **Current Disabled Code:**
```html
            <button
              class="setup-tab-btn"
              role="tab"
              aria-selected="false"
              aria-controls="setup-tab-ps"
              data-setup-tab="ps"
              data-disabled="true"
              data-tooltip="Windows coming soon"
            >
              🪟 PowerShell <span class="coming-soon-badge">Soon</span>
            </button>
```
- **Restore To (Enabled):**
```html
            <button
              class="setup-tab-btn"
              role="tab"
              aria-selected="false"
              aria-controls="setup-tab-ps"
              data-setup-tab="ps"
            >
              🪟 PowerShell
            </button>
```

---

### 2. `web/docs.html`

#### A. Universal 1-Line Installer PowerShell Block
- **Target Location:** Around line 445
- **Current Disabled Code:**
```html
          <div class="code-wrapper" style="opacity: 0.78; position: relative;">
            <div class="code-header">
              <span class="code-lang">PowerShell (Windows Universal) <span class="coming-soon-badge">Coming Soon</span></span>
              <button class="copy-btn" data-disabled="true" data-tooltip="Windows coming soon"><span>⏳</span> Coming Soon</button>
            </div>
            <pre><code>powershell -c "irm https://gladeshell.netlify.app/install.ps1 | iex"</code></pre>
          </div>
```
- **Restore To (Enabled):**
```html
          <div class="code-wrapper">
            <div class="code-header">
              <span class="code-lang">PowerShell (Windows Universal)</span>
              <button class="copy-btn"><span>📋</span> Copy Command</button>
            </div>
            <pre><code>powershell -c "irm https://gladeshell.netlify.app/install.ps1 | iex"</code></pre>
          </div>
```

---

#### B. Shell-Specific Installers Table Row
- **Target Location:** Around line 490
- **Current Disabled Code:**
```html
                <tr>
                  <td><strong>PowerShell</strong> <span class="coming-soon-badge">Coming Soon</span></td>
                  <td><code>$PROFILE</code></td>
                  <td>
                    <button type="button" class="table-soon-btn" data-disabled="true" data-tooltip="Windows coming soon" style="background: rgba(168, 85, 247, 0.12); border: 1px dashed rgba(168, 85, 247, 0.4); color: #c084fc; padding: 4px 10px; border-radius: 6px; font-size: 0.8rem; cursor: not-allowed; font-family: inherit; font-weight: 500;">
                      <span>⏳</span> Windows Coming Soon
                    </button>
                  </td>
                </tr>
```
- **Restore To (Enabled):**
```html
                <tr>
                  <td><strong>PowerShell</strong></td>
                  <td><code>$PROFILE</code></td>
                  <td>
                    <code
                      >powershell -c "irm https://gladeshell.netlify.app/install.ps1 | iex"</code
                    >
                  </td>
                </tr>
```

---

#### C. PowerShell Setup ($PROFILE) Section
- **Target Location:** Around line 650
- **Current Disabled Code:**
```html
          <h3 id="powershell-setup">PowerShell Setup ($PROFILE) <span class="coming-soon-badge">Coming Soon</span></h3>
          <p>PowerShell 7+ and Windows PowerShell 5.1 support is currently undergoing stabilization for parity with Linux &amp; macOS.</p>
          <div class="alert-box alert-warning">
            <div class="alert-title">⏳ Windows Support Coming Soon</div>
            <p>
              Windows native installer is temporarily paused while we polish PowerShell performance and compatibility.
              Once released, you can enable remote script execution via:
              <code>Set-ExecutionPolicy RemoteSigned -Scope CurrentUser</code>
            </p>
          </div>
```
- **Restore To (Enabled):**
```html
          <h3 id="powershell-setup">PowerShell Setup ($PROFILE)</h3>
          <p>PowerShell 7+ and Windows PowerShell 5.1 are fully supported via `install.ps1`.</p>
          <div class="alert-box alert-warning">
            <div class="alert-title">⚠️ Windows Script Execution Policy</div>
            <p>
              If PowerShell blocks remote script execution, enable execution policy first in an
              Administrator terminal:
              <code>Set-ExecutionPolicy RemoteSigned -Scope CurrentUser</code>
            </p>
          </div>
```

---

## 🔍 Validation Commands

After applying the changes above, run the following verification steps:

```bash
# 1. Validate HTML syntax
python3 -c "
from html.parser import HTMLParser
for path in ['web/index.html', 'web/docs.html']:
    with open(path, 'r', encoding='utf-8') as f:
        HTMLParser().feed(f.read())
print('HTML syntax valid!')
"

# 2. Validate JS syntax
node --check web/main.js && node --check web/docs.js && echo "JS syntax valid!"

# 3. Run full test suite
cargo test --tests
```

*(Note: CSS and JS tooltip logic in `web/style.css`, `web/docs.css`, `web/main.js`, and `web/docs.js` can remain permanently in place as a generic reusable feature for any future `[data-disabled="true"]` buttons).*

