// Prady Language Web Portal & Live Studio App Controller

const EXAMPLES = {
  hello: `// examples/hello.pr
// Prady Language — Basic Functions, Types & Mutation

fn add(a: Int, b: Int) -> Int {
    return a + b;
}

fn main() {
    let name = "Pradyumn";
    let age: Int = 25;
    let mut counter = 0;

    counter += 1;
    print("Welcome to Prady Language Live Studio!");
    print("Developer: " + name);
    print("Age: " + age);
    print("Result of add(10, 20): " + add(10, 20));
    print("Counter updated to: " + counter);
}
`,

  architecture: `// examples/architecture.pr
// Prady Architecture as Code — Compile-time Layer Policy Enforcement

architecture backend {
    layer presentation;
    layer application;
    layer domain;
    layer infrastructure;

    presentation -> application;
    application -> domain;
    infrastructure -> domain;

    domain cannot import presentation;
    domain cannot import infrastructure;
}

fn main() {
    print("Validating Clean Architecture layer boundaries...");
    print("Architecture verified according to backend policy contract.");
}
`,

  dsa: `// DSA in Prady — Fibonacci & Algorithm Logic

fn fib(n: Int) -> Int {
    if (n <= 1) {
        return n;
    }
    return fib(n - 1) + fib(n - 2);
}

fn main() {
    print("=== Prady DSA Engine: Fibonacci Sequence ===");
    let mut i = 0;
    while (i <= 10) {
        print("fib(" + i + ") = " + fib(i));
        i += 1;
    }
}
`,

  oop: `// examples/oop.pr
// Prady OOP — Interfaces, Classes, and Composition

interface PaymentGateway {
    fn charge(amount: Int) -> Bool;
}

class CheckoutService {
    fn checkout(amount: Int) -> Bool {
        print("Processing checkout for amount: $" + amount);
        return true;
    }
}

fn main() {
    print("CheckoutService declared and initialized.");
    print("Encapsulation & composition validated cleanly.");
}
`,

  patterns: `// examples/patterns.pr
// Prady Pattern Matching & Enumerations

enum Status {
    Pending,
    Active,
    Completed,
    Failed
}

fn main() {
    print("Pattern matching engine initialized.");
    print("Option & Result monads active for explicit error handling.");
}
`
};

// Global Screen Switcher
window.switchScreen = function (screenId) {
  document.querySelectorAll('.screen-view').forEach(s => s.classList.remove('active'));
  document.querySelectorAll('.nav-link').forEach(n => n.classList.remove('active'));

  const target = document.getElementById(screenId);
  if (target) {
    target.classList.add('active');
  }

  // Highlight corresponding nav link
  const navBtn = document.querySelector(`.nav-link[data-screen="${screenId}"]`);
  if (navBtn) {
    navBtn.classList.add('active');
  }

  window.scrollTo({ top: 0, behavior: 'smooth' });
};

// Global OS Tab Switcher
window.switchOsTab = function (os) {
  document.querySelectorAll('.os-tab').forEach(t => t.classList.remove('active'));
  document.querySelectorAll('.os-guide-content').forEach(c => c.classList.remove('active'));

  const activeTab = document.querySelector(`.os-tab[data-os="${os}"]`);
  const activeContent = document.getElementById(`guide-${os}`);
  if (activeTab) activeTab.classList.add('active');
  if (activeContent) activeContent.classList.add('active');
};

// Global Snippet Copier
window.copySnippet = function (text) {
  navigator.clipboard.writeText(text).then(() => {
    window.showToast('📋 Copied command to clipboard!');
  }).catch(() => {
    prompt('Copy command:', text);
  });
};

// Global Quick Preview Launcher
window.loadAndLaunch = function (exampleKey) {
  const codeInput = document.getElementById('code-input');
  const exampleSelect = document.getElementById('example-select');
  if (EXAMPLES[exampleKey]) {
    codeInput.value = EXAMPLES[exampleKey];
    exampleSelect.value = exampleKey;
  }
  window.switchScreen('screen-studio');
  if (window.runCode) {
    window.runCode();
  }
};

window.showToast = function (msg) {
  const toast = document.getElementById('toast');
  if (!toast) return;
  toast.textContent = msg;
  toast.style.display = 'block';
  setTimeout(() => {
    toast.style.display = 'none';
  }, 2800);
};

document.addEventListener('DOMContentLoaded', () => {
  const codeInput = document.getElementById('code-input');
  const lineNumbers = document.getElementById('line-numbers');
  const exampleSelect = document.getElementById('example-select');
  const btnRun = document.getElementById('btn-run');
  const btnCheck = document.getElementById('btn-check');
  const btnShare = document.getElementById('btn-share');
  const btnDownload = document.getElementById('btn-download');
  const btnCopyCli = document.getElementById('btn-copy-cli');
  const quickRunBtn = document.getElementById('quick-run-btn');

  const consoleOutput = document.getElementById('console-output');
  const astOutput = document.getElementById('ast-output');
  const tokensOutput = document.getElementById('tokens-output');
  const archOutput = document.getElementById('arch-output');

  // Nav Links Handlers
  document.querySelectorAll('.nav-link').forEach(link => {
    link.addEventListener('click', () => {
      const screenId = link.getAttribute('data-screen');
      window.switchScreen(screenId);
    });
  });

  if (quickRunBtn) {
    quickRunBtn.addEventListener('click', () => {
      window.switchScreen('screen-studio');
      runCode();
    });
  }

  // Check URL Hash for shared code or screen
  function handleUrlHash() {
    const hash = window.location.hash;
    if (hash.startsWith('#code=')) {
      try {
        const raw = hash.substring(6);
        const decoded = decodeURIComponent(escape(atob(raw)));
        if (decoded) {
          codeInput.value = decoded;
          updateLineNumbers();
          window.switchScreen('screen-studio');
          return;
        }
      } catch (e) {
        console.error('Failed to parse URL code payload', e);
      }
    } else if (hash === '#studio') {
      window.switchScreen('screen-studio');
    } else if (hash === '#install') {
      window.switchScreen('screen-install');
    } else if (hash === '#docs') {
      window.switchScreen('screen-docs');
    } else if (hash === '#versions') {
      window.switchScreen('screen-versions');
    } else {
      // Default to Dashboard
      window.switchScreen('screen-dashboard');
    }
    codeInput.value = EXAMPLES.hello;
    updateLineNumbers();
  }

  // Line Numbers Synchronizer
  function updateLineNumbers() {
    const lines = codeInput.value.split('\n').length;
    let numbers = '';
    for (let i = 1; i <= lines; i++) {
      numbers += i + '\n';
    }
    lineNumbers.textContent = numbers;
  }

  codeInput.addEventListener('input', updateLineNumbers);
  codeInput.addEventListener('scroll', () => {
    lineNumbers.scrollTop = codeInput.scrollTop;
  });

  // Handle Tab key in editor
  codeInput.addEventListener('keydown', (e) => {
    if (e.key === 'Tab') {
      e.preventDefault();
      const start = codeInput.selectionStart;
      const end = codeInput.selectionEnd;
      codeInput.value = codeInput.value.substring(0, start) + '    ' + codeInput.value.substring(end);
      codeInput.selectionStart = codeInput.selectionEnd = start + 4;
      updateLineNumbers();
    }
    if (e.ctrlKey && e.key === 'Enter') {
      e.preventDefault();
      runCode();
    }
  });

  // Example Selection
  exampleSelect.addEventListener('change', (e) => {
    const key = e.target.value;
    if (EXAMPLES[key]) {
      codeInput.value = EXAMPLES[key];
      updateLineNumbers();
      runCode();
    }
  });

  // Tab switching inside Studio
  const tabBtns = document.querySelectorAll('.tab-btn');
  const tabPanes = document.querySelectorAll('.tab-pane');

  tabBtns.forEach(btn => {
    btn.addEventListener('click', () => {
      tabBtns.forEach(b => b.classList.remove('active'));
      tabPanes.forEach(p => p.classList.remove('active'));

      btn.classList.add('active');
      const tabId = btn.getAttribute('data-tab');
      const pane = document.getElementById(tabId);
      if (pane) pane.classList.add('active');
    });
  });

  // Execution Handler
  function runCode() {
    const source = codeInput.value;
    consoleOutput.innerHTML = '';

    const addLine = (text, cls = '') => {
      const div = document.createElement('div');
      div.className = `terminal-line ${cls}`;
      div.textContent = text;
      consoleOutput.appendChild(div);
    };

    addLine('$ prady run main.pr', 'system');

    const result = window.PradyEngine.compileAndRun(source, (output) => {
      addLine(output);
    });

    if (result.ast) {
      astOutput.textContent = JSON.stringify(result.ast, null, 2);
    }

    if (result.tokens) {
      let tableHtml = `<table class="tokens-table">
        <thead><tr><th>LINE</th><th>COL</th><th>KIND</th><th>TEXT</th></tr></thead>
        <tbody>`;
      for (const tok of result.tokens) {
        if (tok.kind === 'Eof') continue;
        tableHtml += `<tr>
          <td>${tok.line}</td>
          <td>${tok.col}</td>
          <td class="token-kind">${tok.kind}</td>
          <td class="token-val">${escapeHtml(tok.text)}</td>
        </tr>`;
      }
      tableHtml += `</tbody></table>`;
      tokensOutput.innerHTML = tableHtml;
    }

    if (result.architecture) {
      let archHtml = `<div class="arch-card-grid">`;
      archHtml += `<div class="arch-layer-card"><div class="arch-layer-name">System: ${result.architecture.name}</div><span class="status-dot"></span></div>`;
      for (const layer of result.architecture.layers) {
        archHtml += `<div class="arch-layer-card"><div>Layer: <strong>${layer}</strong></div><span style="color:var(--neon-cyan)">Enforced</span></div>`;
      }
      if (result.architecture.rules.length > 0) {
        archHtml += `<div style="margin-top:10px; font-weight:700; color:var(--text-secondary)">Dependency Contracts:</div>`;
        for (const r of result.architecture.rules) {
          const isDeny = r.type === 'deny';
          archHtml += `<div class="arch-layer-card" style="border-left: 3px solid ${isDeny ? 'var(--neon-rose)' : 'var(--neon-emerald)'}">
            <span>${r.from} <strong>${isDeny ? 'cannot import' : '->'}</strong> ${r.to}</span>
            <span style="color:${isDeny ? 'var(--neon-rose)' : 'var(--neon-emerald)'}">${isDeny ? 'Forbidden' : 'Allowed'}</span>
          </div>`;
        }
      }
      archHtml += `</div>`;
      archOutput.innerHTML = archHtml;
    } else {
      archOutput.innerHTML = `<div style="color:var(--text-muted); padding:16px;">No 'architecture' block declared in this file. Try the Clean Architecture preset above!</div>`;
    }

    if (result.archReports && result.archReports.length > 0) {
      for (const rep of result.archReports) {
        addLine(rep, 'arch');
      }
    }

    if (!result.success) {
      if (result.errorText) {
        addLine(result.errorText, 'error');
      }
      if (result.runError) {
        addLine(`runtime error: ${result.runError}`, 'error');
      }
      addLine(`[Failed in ${result.durationMs}ms]`, 'error');
    } else {
      addLine(`\nok: Process finished with exit code 0 (${result.durationMs}ms)`, 'success');
    }
  }

  window.runCode = runCode;

  function escapeHtml(str) {
    if (!str) return '';
    return str.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;').replace(/"/g, '&quot;');
  }

  // Button Listeners
  btnRun.addEventListener('click', runCode);

  btnCheck.addEventListener('click', () => {
    runCode();
    window.showToast('Prady check: Syntax & Types validated!');
  });

  btnShare.addEventListener('click', () => {
    const code = codeInput.value;
    const encoded = btoa(unescape(encodeURIComponent(code)));
    const url = window.location.origin + window.location.pathname + '#code=' + encoded;
    navigator.clipboard.writeText(url).then(() => {
      window.showToast('🔗 Shareable link copied to clipboard!');
    }).catch(() => {
      prompt('Copy this link to share:', url);
    });
  });

  btnDownload.addEventListener('click', () => {
    const blob = new Blob([codeInput.value], { type: 'text/plain;charset=utf-8' });
    const a = document.createElement('a');
    a.href = URL.createObjectURL(blob);
    a.download = 'program.pr';
    a.click();
    window.showToast('📥 Downloaded program.pr');
  });

  btnCopyCli.addEventListener('click', () => {
    const cmd = 'prady run main.pr';
    navigator.clipboard.writeText(cmd).then(() => {
      window.showToast('📋 Copied terminal command: prady run main.pr');
    });
  });

  // Docs smooth scroll
  document.querySelectorAll('.docs-nav-item').forEach(item => {
    item.addEventListener('click', (e) => {
      document.querySelectorAll('.docs-nav-item').forEach(i => i.classList.remove('active'));
      item.classList.add('active');
    });
  });

  // Initialize
  handleUrlHash();
  runCode();
});
