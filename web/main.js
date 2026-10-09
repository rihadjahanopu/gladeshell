/* ============================================================
   GLADESHELL — Main JavaScript
   ============================================================ */

// ─── Navbar scroll effect ──────────────────────────────────
const navbar = document.getElementById('navbar');
const onScroll = () => {
  navbar.classList.toggle('scrolled', window.scrollY > 20);
};
window.addEventListener('scroll', onScroll, { passive: true });
onScroll();

// ─── Mobile nav toggle ────────────────────────────────────
const navToggle = document.getElementById('nav-toggle');
const navMobile = document.getElementById('nav-links-mobile');

function openNav() {
  navMobile.classList.add('is-open');
  navToggle.classList.add('is-open');
  navToggle.setAttribute('aria-expanded', 'true');
  navMobile.setAttribute('aria-hidden', 'false');
}

function closeNav() {
  navMobile.classList.remove('is-open');
  navToggle.classList.remove('is-open');
  navToggle.setAttribute('aria-expanded', 'false');
  navMobile.setAttribute('aria-hidden', 'true');
}

navToggle?.addEventListener('click', (e) => {
  e.stopPropagation();
  navMobile.classList.contains('is-open') ? closeNav() : openNav();
});

// Close on outside click
document.addEventListener('click', (e) => {
  if (
    navMobile.classList.contains('is-open') &&
    !navMobile.contains(e.target) &&
    !navToggle.contains(e.target)
  ) {
    closeNav();
  }
});

// Close when a nav link is tapped
navMobile?.querySelectorAll('a').forEach((link) => {
  link.addEventListener('click', closeNav);
});

// Close when resizing back to desktop width
window.addEventListener(
  'resize',
  () => {
    if (window.innerWidth > 768) closeNav();
  },
  { passive: true }
);

// ─── Install tab switching ────────────────────────────────
const tabBtns = document.querySelectorAll('.tab-btn');
const tabPanels = document.querySelectorAll('.tab-panel');

tabBtns.forEach((btn) => {
  btn.addEventListener('click', (e) => {
    if (btn.dataset.disabled === 'true') return;
    const targetTab = btn.dataset.tab;

    tabBtns.forEach((b) => {
      b.classList.remove('active');
      b.setAttribute('aria-selected', 'false');
    });
    tabPanels.forEach((p) => {
      p.classList.remove('active');
      p.hidden = true;
    });

    btn.classList.add('active');
    btn.setAttribute('aria-selected', 'true');

    const panel = document.getElementById(`tab-${targetTab}`);
    if (panel) {
      panel.classList.add('active');
      panel.hidden = false;
    }
  });
});

// ─── Uninstall tab switching ────────────────────────────────
const utabBtns = document.querySelectorAll('.utab-btn');
const utabPanels = document.querySelectorAll('.utab-panel');

utabBtns.forEach((btn) => {
  btn.addEventListener('click', (e) => {
    if (btn.dataset.disabled === 'true') return;
    const targetTab = btn.dataset.utab;

    utabBtns.forEach((b) => {
      b.classList.remove('active');
      b.setAttribute('aria-selected', 'false');
    });
    utabPanels.forEach((p) => {
      p.classList.remove('active');
      p.hidden = true;
    });

    btn.classList.add('active');
    btn.setAttribute('aria-selected', 'true');

    const panel = document.getElementById(`utab-${targetTab}`);
    if (panel) {
      panel.classList.add('active');
      panel.hidden = false;
    }
  });
});

// ─── Setup tab switching ──────────────────────────────────
const setupTabBtns = document.querySelectorAll('.setup-tab-btn');
const setupTabPanels = document.querySelectorAll('.setup-tab-panel');

setupTabBtns.forEach((btn) => {
  btn.addEventListener('click', (e) => {
    if (btn.dataset.disabled === 'true') return;
    const targetTab = btn.dataset.setupTab;

    setupTabBtns.forEach((b) => {
      b.classList.remove('active');
      b.setAttribute('aria-selected', 'false');
    });
    setupTabPanels.forEach((p) => {
      p.classList.remove('active');
      p.hidden = true;
    });

    btn.classList.add('active');
    btn.setAttribute('aria-selected', 'true');

    const panel = document.getElementById(`setup-tab-${targetTab}`);
    if (panel) {
      panel.classList.add('active');
      panel.hidden = false;
    }
  });
});

// ─── Command category tabs ────────────────────────────────
const cmdCatBtns = document.querySelectorAll('.cmd-cat-btn');
const cmdPanels = document.querySelectorAll('.cmd-panel');

cmdCatBtns.forEach((btn) => {
  btn.addEventListener('click', () => {
    const cat = btn.dataset.cat;

    cmdCatBtns.forEach((b) => {
      b.classList.remove('active');
      b.setAttribute('aria-selected', 'false');
    });
    cmdPanels.forEach((p) => {
      p.classList.remove('active');
      p.hidden = true;
    });

    btn.classList.add('active');
    btn.setAttribute('aria-selected', 'true');

    const panel = document.querySelector(`.cmd-panel[data-panel="${cat}"]`);
    if (panel) {
      panel.classList.add('active');
      panel.hidden = false;
    }
  });
});

// ─── Copy buttons ─────────────────────────────────────────
document.querySelectorAll('.copy-btn').forEach((btn) => {
  btn.addEventListener('click', async () => {
    const text = btn.dataset.copy;
    if (!text) return;

    try {
      await navigator.clipboard.writeText(text);
      const original = btn.innerHTML;
      btn.classList.add('copied');
      btn.innerHTML = `<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="20 6 9 17 4 12"></polyline></svg> Copied!`;

      setTimeout(() => {
        btn.innerHTML = original;
        btn.classList.remove('copied');
      }, 2000);
    } catch {
      // Fallback for older browsers
      const textarea = document.createElement('textarea');
      textarea.value = text;
      textarea.style.position = 'fixed';
      textarea.style.opacity = '0';
      document.body.appendChild(textarea);
      textarea.select();
      document.execCommand('copy');
      document.body.removeChild(textarea);
    }
  });
});

// ─── Toast Notification Helper ───────────────────────────
function showToast(message) {
  let toast = document.getElementById('global-cmd-toast');
  if (!toast) {
    toast = document.createElement('div');
    toast.id = 'global-cmd-toast';
    toast.className = 'cmd-toast';
    document.body.appendChild(toast);
  }
  toast.innerHTML = `<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="#22c55e" stroke-width="2.5"><polyline points="20 6 9 17 4 12"></polyline></svg> <span>${message}</span>`;
  toast.classList.add('show');

  clearTimeout(toast.timer);
  toast.timer = setTimeout(() => {
    toast.classList.remove('show');
  }, 2200);
}

// ─── Dynamic Copy Icons for Command Tables ──────────────────
function initTableCopyButtons() {
  const codeEls = document.querySelectorAll('.cmd-table td code');
  codeEls.forEach((code) => {
    // Skip section header rows or empty codes
    if (code.closest('.section-header-row')) return;
    if (code.closest('.code-copy-inline') || code.parentElement.querySelector('.td-copy-btn'))
      return;

    const copyText = code.dataset.copy || code.textContent.trim();
    if (!copyText) return;

    // Wrap code and copy button in a inline container
    const wrapper = document.createElement('span');
    wrapper.className = 'code-copy-inline';

    const btn = document.createElement('button');
    btn.type = 'button';
    btn.className = 'td-copy-btn';
    btn.setAttribute('title', `Copy "${copyText}"`);
    btn.setAttribute('aria-label', `Copy ${copyText}`);
    btn.innerHTML = `<svg class="copy-icon" width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2" stroke-linecap="round" stroke-linejoin="round"><rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect><path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path></svg>`;

    code.parentNode.insertBefore(wrapper, code);
    wrapper.appendChild(code);
    wrapper.appendChild(btn);

    const performCopy = async (e) => {
      e.stopPropagation();
      try {
        await navigator.clipboard.writeText(copyText);
      } catch {
        const textarea = document.createElement('textarea');
        textarea.value = copyText;
        textarea.style.position = 'fixed';
        textarea.style.opacity = '0';
        document.body.appendChild(textarea);
        textarea.select();
        document.execCommand('copy');
        document.body.removeChild(textarea);
      }

      btn.classList.add('copied');
      btn.innerHTML = `<svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><polyline points="20 6 9 17 4 12"></polyline></svg>`;
      code.classList.add('copied-code');
      showToast(`Copied <code>${copyText}</code> to clipboard!`);

      setTimeout(() => {
        btn.classList.remove('copied');
        code.classList.remove('copied-code');
        btn.innerHTML = `<svg class="copy-icon" width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2" stroke-linecap="round" stroke-linejoin="round"><rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect><path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path></svg>`;
      }, 1800);
    };

    btn.addEventListener('click', performCopy);
    code.addEventListener('click', performCopy);
  });
}

if (document.readyState === 'loading') {
  document.addEventListener('DOMContentLoaded', initTableCopyButtons);
} else {
  initTableCopyButtons();
}

// ─── Terminal typewriter animation ────────────────────────
const typedEl = document.getElementById('typed-text');
const outputEl = document.getElementById('terminal-output');

const demos = [
  {
    cmd: 'gwip "feat: add auth middleware"',
    output:
      '<span style="color:#22c55e">✓</span> Staged all files<br><span style="color:#22c55e">✓</span> Committed: feat: add auth middleware<br><span style="color:#22d3ee">→</span> Pushed to main',
  },
  {
    cmd: 'uup',
    output:
      '<span style="color:#a855f7">⚡ Opening fzf maintenance menu...</span><br><span style="color:#64748b">0. ALL_MAINTENANCE_TASKS</span><br><span style="color:#64748b">1. Core_System_Update ...</span>',
  },
  {
    cmd: 'gen 32',
    output:
      '<span style="color:#f59e0b">🔑</span> <span style="color:#22d3ee">a7f3d9c2e8b14056...</span>',
  },
  {
    cmd: 'vite',
    output:
      '<span style="color:#a855f7">⚡ Setup Vite with Bun + Tailwind CSS v4</span><br><span style="color:#22c55e">✓</span> Project ready! Run <span style="color:#22d3ee">brd</span> to start.',
  },
  {
    cmd: 'uu',
    output:
      '<span style="color:#a855f7">📦 Scanning apt + snap + flatpak + AppImage...</span><br><span style="color:#64748b">TAB to multi-select, ENTER to remove</span>',
  },
];

let demoIdx = 0;
let charIdx = 0;
let isTyping = false;
let typingTimer = null;

function typeChar() {
  const demo = demos[demoIdx];
  if (charIdx < demo.cmd.length) {
    typedEl.textContent += demo.cmd[charIdx];
    charIdx++;
    const delay = 40 + Math.random() * 30;
    typingTimer = setTimeout(typeChar, delay);
  } else {
    // Show output after brief pause
    typingTimer = setTimeout(() => {
      outputEl.innerHTML = demo.output;
      // Clear and move to next demo
      typingTimer = setTimeout(nextDemo, 2500);
    }, 400);
  }
}

function nextDemo() {
  demoIdx = (demoIdx + 1) % demos.length;
  charIdx = 0;
  typedEl.textContent = '';
  outputEl.innerHTML = '';
  typingTimer = setTimeout(typeChar, 600);
}

// Start after a short delay
typingTimer = setTimeout(typeChar, 1200);

// ─── Counter animation ────────────────────────────────────
function animateCounter(el, target, duration = 1200) {
  const start = performance.now();
  const step = (now) => {
    const progress = Math.min((now - start) / duration, 1);
    const eased = 1 - Math.pow(1 - progress, 3);
    el.textContent = Math.round(eased * target);
    if (progress < 1) requestAnimationFrame(step);
  };
  requestAnimationFrame(step);
}

const counterEls = document.querySelectorAll('[data-target]');
const counterObserver = new IntersectionObserver(
  (entries) => {
    entries.forEach((entry) => {
      if (entry.isIntersecting) {
        const el = entry.target;
        const target = parseInt(el.dataset.target, 10);
        animateCounter(el, target);
        counterObserver.unobserve(el);
      }
    });
  },
  { threshold: 0.5 }
);

counterEls.forEach((el) => counterObserver.observe(el));

// ─── Fallback scroll reveal (for browsers without scroll-driven animations) ─
if (!CSS.supports('(animation-timeline: view()) and (animation-range: entry)')) {
  const revealEls = document.querySelectorAll('.feature-card, .pe-item, .step-item, .stat-item');

  const revealObserver = new IntersectionObserver(
    (entries) => {
      entries.forEach((entry, i) => {
        if (entry.isIntersecting) {
          setTimeout(() => {
            entry.target.classList.add('visible');
          }, i * 80);
          revealObserver.unobserve(entry.target);
        }
      });
    },
    { threshold: 0.1, rootMargin: '0px 0px -40px 0px' }
  );

  revealEls.forEach((el) => revealObserver.observe(el));
}

// ─── Smooth anchor scroll with offset ────────────────────
document.querySelectorAll('a[href^="#"]').forEach((anchor) => {
  anchor.addEventListener('click', (e) => {
    const target = document.querySelector(anchor.getAttribute('href'));
    if (target) {
      e.preventDefault();
      target.scrollIntoView({ behavior: 'smooth', block: 'start' });
    }
  });
});

// ─── Disabled button tooltips ─────────────────────────────
document.querySelectorAll('[data-disabled="true"]').forEach((btn) => {
  btn.addEventListener('click', (e) => {
    e.preventDefault();
    e.stopPropagation();

    // Check if tooltip already exists
    let tooltip = btn.querySelector('.custom-tooltip');
    if (!tooltip) {
      tooltip = document.createElement('div');
      tooltip.className = 'custom-tooltip';
      tooltip.textContent = 'Coming soon';
      btn.appendChild(tooltip);
    }

    // Show tooltip
    // Use requestAnimationFrame to ensure the display transition happens
    requestAnimationFrame(() => {
      tooltip.classList.add('show');
    });

    // Hide tooltip after 2 seconds
    setTimeout(() => {
      tooltip.classList.remove('show');
    }, 2000);
  });
});

/* ============================================================
   HOME PAGE BENCHMARK SECTION LOGIC
   ============================================================ */
function initBenchmarkHome() {
  const tabs = document.querySelectorAll('.bh-tab');
  const panels = document.querySelectorAll('.bh-panel');
  if (!tabs.length) return;

  tabs.forEach((tab) => {
    tab.addEventListener('click', () => {
      const target = tab.dataset.bhTab;

      tabs.forEach((t) => {
        t.classList.remove('active');
        t.setAttribute('aria-selected', 'false');
      });
      tab.classList.add('active');
      tab.setAttribute('aria-selected', 'true');

      panels.forEach((p) => p.classList.remove('active'));
      const activePanel = document.getElementById('bhpanel-' + target);
      if (activePanel) {
        activePanel.classList.add('active');
        triggerHomeBars(activePanel);
        if (target === 'matrix') {
          drawHomeRadar();
        }
      }
    });
  });

  function triggerHomeBars(panel) {
    const fills = panel.querySelectorAll('.bh-bar-fill[data-pct]');
    fills.forEach((fill) => {
      const pct = parseFloat(fill.dataset.pct) || 0;
      requestAnimationFrame(() => {
        fill.style.width = Math.max(pct, pct === 0 ? 0 : 1) + '%';
      });
    });
  }

  const section = document.getElementById('benchmark');
  if (section && 'IntersectionObserver' in window) {
    const obs = new IntersectionObserver(
      (entries) => {
        entries.forEach((entry) => {
          if (entry.isIntersecting) {
            const activePanel = document.querySelector('.bh-panel.active');
            if (activePanel) triggerHomeBars(activePanel);
            obs.disconnect();
          }
        });
      },
      { threshold: 0.1 }
    );
    obs.observe(section);
  } else {
    const activePanel = document.querySelector('.bh-panel.active');
    if (activePanel) triggerHomeBars(activePanel);
  }

  function drawHomeRadar() {
    const canvas = document.getElementById('bh-radar');
    if (!canvas) return;
    const ctx = canvas.getContext('2d');
    const W = canvas.width;
    const H = canvas.height;
    const cx = W / 2;
    const cy = H / 2;
    const R = Math.min(W, H) / 2 - 32;

    const axes = [
      'Startup\nSpeed',
      'Prompt\nSpeed',
      'Memory\nEfficiency',
      'Feature\nCount',
      'Theme\nRichness',
      'POSIX\nCompat',
    ];
    const N = axes.length;

    const shells = [
      {
        name: 'GladeShell',
        color: '#22d3ee',
        fill: 'rgba(34,211,238,0.18)',
        scores: [10, 10, 9.8, 9.0, 9.0, 10],
      },
      {
        name: 'Starship',
        color: '#eab308',
        fill: 'rgba(234,179,8,0.12)',
        scores: [8.0, 7.5, 7.5, 8.0, 7.5, 9.5],
      },
      {
        name: 'Oh My Posh',
        color: '#f97316',
        fill: 'rgba(249,115,22,0.12)',
        scores: [6.5, 6.0, 6.0, 8.5, 9.5, 8.0],
      },
      {
        name: 'Oh My Zsh',
        color: '#ef4444',
        fill: 'rgba(239,68,68,0.12)',
        scores: [2.5, 3.5, 3.0, 9.0, 10, 9.0],
      },
    ];

    ctx.clearRect(0, 0, W, H);

    function polar(angle, r) {
      return {
        x: cx + r * Math.cos(angle - Math.PI / 2),
        y: cy + r * Math.sin(angle - Math.PI / 2),
      };
    }

    const rings = 5;
    for (let r = 1; r <= rings; r++) {
      const radius = (R / rings) * r;
      ctx.beginPath();
      for (let i = 0; i < N; i++) {
        const angle = ((2 * Math.PI) / N) * i;
        const pt = polar(angle, radius);
        i === 0 ? ctx.moveTo(pt.x, pt.y) : ctx.lineTo(pt.x, pt.y);
      }
      ctx.closePath();
      ctx.strokeStyle = 'rgba(255,255,255,0.07)';
      ctx.lineWidth = 1;
      ctx.stroke();
    }

    for (let i = 0; i < N; i++) {
      const angle = ((2 * Math.PI) / N) * i;
      const outer = polar(angle, R);
      ctx.beginPath();
      ctx.moveTo(cx, cy);
      ctx.lineTo(outer.x, outer.y);
      ctx.strokeStyle = 'rgba(255,255,255,0.1)';
      ctx.lineWidth = 1;
      ctx.stroke();
    }

    ctx.font = '10px Inter, sans-serif';
    ctx.fillStyle = 'rgba(148,163,184,0.9)';
    ctx.textAlign = 'center';
    ctx.textBaseline = 'middle';
    for (let i = 0; i < N; i++) {
      const angle = ((2 * Math.PI) / N) * i;
      const labelR = R + 22;
      const pt = polar(angle, labelR);
      const lines = axes[i].split('\n');
      lines.forEach((line, li) => {
        ctx.fillText(line, pt.x, pt.y + (li - (lines.length - 1) / 2) * 13);
      });
    }

    shells.forEach((shell) => {
      ctx.beginPath();
      for (let i = 0; i < N; i++) {
        const angle = ((2 * Math.PI) / N) * i;
        const val = shell.scores[i] / 10;
        const pt = polar(angle, R * val);
        i === 0 ? ctx.moveTo(pt.x, pt.y) : ctx.lineTo(pt.x, pt.y);
      }
      ctx.closePath();
      ctx.fillStyle = shell.fill;
      ctx.fill();
      ctx.strokeStyle = shell.color;
      ctx.lineWidth = 1.8;
      ctx.stroke();

      for (let i = 0; i < N; i++) {
        const angle = ((2 * Math.PI) / N) * i;
        const val = shell.scores[i] / 10;
        const pt = polar(angle, R * val);
        ctx.beginPath();
        ctx.arc(pt.x, pt.y, 3, 0, Math.PI * 2);
        ctx.fillStyle = shell.color;
        ctx.fill();
      }
    });

    ctx.beginPath();
    ctx.arc(cx, cy, 3, 0, Math.PI * 2);
    ctx.fillStyle = 'rgba(255,255,255,0.2)';
    ctx.fill();
  }
}

document.addEventListener('DOMContentLoaded', () => {
  initBenchmarkHome();
});

// ─── Contributor shell tab switcher ──────────────────────
document.querySelectorAll('.csh-tabs').forEach((tabGroup) => {
  const tabs   = tabGroup.querySelectorAll('.csh-tab');
  const panels = tabGroup.nextElementSibling; // .csh-panels

  tabs.forEach((tab) => {
    tab.addEventListener('click', () => {
      // Deactivate all
      tabs.forEach((t) => {
        t.classList.remove('is-active');
        t.setAttribute('aria-selected', 'false');
      });
      panels.querySelectorAll('.csh-panel').forEach((p) => {
        p.classList.remove('is-active');
        p.hidden = true;
      });
      // Activate clicked
      tab.classList.add('is-active');
      tab.setAttribute('aria-selected', 'true');
      const target = panels.querySelector('#' + tab.getAttribute('aria-controls'));
      if (target) {
        target.classList.add('is-active');
        target.hidden = false;
      }
    });
  });
});
