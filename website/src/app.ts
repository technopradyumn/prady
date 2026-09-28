import { DocItem, MethodDoc } from './types';
import { DSA_DOCS } from './dsa-docs';
import { LANGUAGE_GUIDES } from './docs-data';
import { PradyCompiler } from './prady-engine';

class DocApp {
  private allDocs: DocItem[] = [];
  private activeDocId: string = 'guide-getting-started';
  private activeMethodName: string | null = null;
  private searchTerm: string = '';

  constructor() {
    this.allDocs = [...LANGUAGE_GUIDES, ...DSA_DOCS];
    this.init();
  }

  private init() {
    this.setupTheme();
    this.renderSidebar();
    this.setupEvents();
    this.handleRouting();
  }

  private setupTheme() {
    const savedTheme = localStorage.getItem('prady-theme') || 'light';
    document.documentElement.setAttribute('data-theme', savedTheme);
  }

  private toggleTheme() {
    const current = document.documentElement.getAttribute('data-theme') || 'light';
    const next = current === 'light' ? 'dark' : 'light';
    document.documentElement.setAttribute('data-theme', next);
    localStorage.setItem('prady-theme', next);
  }

  private handleRouting() {
    const hash = window.location.hash.replace('#', '');
    if (hash) {
      const parts = hash.split('__');
      const docId = parts[0];
      const methodName = parts[1] || null;

      const exists = this.allDocs.find((d) => d.id === docId);
      if (exists) {
        this.activeDocId = docId;
        this.activeMethodName = methodName;
      }
    }
    this.renderContent();
    this.highlightActiveSidebar();
  }

  private setupEvents() {
    window.addEventListener('hashchange', () => this.handleRouting());

    const themeBtn = document.getElementById('theme-toggle-btn');
    if (themeBtn) {
      themeBtn.addEventListener('click', () => this.toggleTheme());
    }

    const searchInput = document.getElementById('search-input') as HTMLInputElement;
    if (searchInput) {
      searchInput.addEventListener('input', (e) => {
        this.searchTerm = (e.target as HTMLInputElement).value.toLowerCase().trim();
        this.renderSidebar();
      });
      document.addEventListener('keydown', (e) => {
        if (e.key === '/' && document.activeElement !== searchInput) {
          e.preventDefault();
          searchInput.focus();
        }
      });
    }

    // Playground runner
    const runBtn = document.getElementById('pg-run-btn');
    if (runBtn) {
      runBtn.addEventListener('click', () => this.runPlaygroundCode());
    }

    const resetBtn = document.getElementById('pg-reset-btn');
    if (resetBtn) {
      resetBtn.addEventListener('click', () => {
        const doc = this.allDocs.find((d) => d.id === this.activeDocId);
        if (doc && doc.example) {
          this.setPlaygroundCode(doc.example);
        }
      });
    }

    // Tab switching between Doc view and Playground view
    const tabDocs = document.getElementById('tab-btn-docs');
    const tabPlayground = document.getElementById('tab-btn-playground');
    if (tabDocs && tabPlayground) {
      tabDocs.addEventListener('click', () => this.switchView('docs'));
      tabPlayground.addEventListener('click', () => this.switchView('playground'));
    }
  }

  private switchView(view: 'docs' | 'playground') {
    const docContainer = document.getElementById('doc-article-container');
    const pgContainer = document.getElementById('playground-view');
    const tabDocs = document.getElementById('tab-btn-docs');
    const tabPlayground = document.getElementById('tab-btn-playground');

    if (view === 'docs') {
      docContainer?.classList.remove('hidden');
      pgContainer?.classList.add('hidden');
      tabDocs?.classList.add('active');
      tabPlayground?.classList.remove('active');
    } else {
      docContainer?.classList.add('hidden');
      pgContainer?.classList.remove('hidden');
      tabDocs?.classList.remove('active');
      tabPlayground?.classList.add('active');
    }
  }

  public setPlaygroundCode(code: string) {
    const editor = document.getElementById('playground-editor') as HTMLTextAreaElement;
    if (editor) {
      editor.value = code;
    }
    this.switchView('playground');
  }

  private runPlaygroundCode() {
    const editor = document.getElementById('playground-editor') as HTMLTextAreaElement;
    const outputEl = document.getElementById('playground-output');
    const timingEl = document.getElementById('playground-timing');
    if (!editor || !outputEl) return;

    outputEl.textContent = '';
    const logs: string[] = [];

    const printCb = (str: string) => {
      logs.push(str);
    };

    const result = PradyCompiler.compileAndRun(editor.value, printCb);

    if (result.success) {
      let finalOut = logs.join('\n');
      if (result.archReports && result.archReports.length > 0) {
        finalOut = result.archReports.join('\n') + '\n\n' + finalOut;
      }
      outputEl.textContent = finalOut || '(Program executed successfully with no output)';
      outputEl.className = 'stdout-output success';
    } else {
      let errText = result.errorText || result.runError || 'Execution failed';
      outputEl.textContent = errText;
      outputEl.className = 'stdout-output error';
    }

    if (timingEl) {
      timingEl.textContent = `Completed in ${result.durationMs}ms`;
    }
  }

  private renderSidebar() {
    const sidebarEl = document.getElementById('sidebar-navigation');
    if (!sidebarEl) return;

    const query = this.searchTerm;
    let html = '';

    // Group items by category
    const categories = ['Guides', 'Language Reference', 'Data Structures'];

    for (const cat of categories) {
      const items = this.allDocs.filter((d) => d.category === cat);
      if (items.length === 0) continue;

      // Filter by search query
      const matchingItems = items.filter((d) => {
        if (!query) return true;
        if (d.title.toLowerCase().includes(query)) return true;
        if (d.summary.toLowerCase().includes(query)) return true;
        if (d.methods?.some((m) => m.name.toLowerCase().includes(query))) return true;
        return false;
      });

      if (matchingItems.length === 0) continue;

      html += `<div class="sidebar-category-group">`;
      html += `<h3 class="sidebar-category-title">${cat}</h3>`;
      html += `<ul class="sidebar-list">`;

      for (const item of matchingItems) {
        const isActive = item.id === this.activeDocId;
        html += `<li class="sidebar-item ${isActive ? 'active' : ''}">`;
        html += `<a href="#${item.id}" class="sidebar-link" data-id="${item.id}">`;
        html += `<span class="link-label">${this.escapeHtml(item.title)}</span>`;
        if (item.badge) {
          html += `<span class="sidebar-badge">${this.escapeHtml(item.badge)}</span>`;
        }
        html += `</a>`;

        // If active and has methods, render sub-items for methods
        if (isActive && item.methods && item.methods.length > 0) {
          html += `<ul class="sidebar-sublist">`;
          for (const m of item.methods) {
            const isMethodActive = this.activeMethodName === m.name;
            html += `<li class="sidebar-subitem ${isMethodActive ? 'active' : ''}">`;
            html += `<a href="#${item.id}__${m.name}" class="sidebar-sublink" data-id="${item.id}" data-method="${m.name}">.${m.name}()</a>`;
            html += `</li>`;
          }
          html += `</ul>`;
        }

        html += `</li>`;
      }

      html += `</ul>`;
      html += `</div>`;
    }

    sidebarEl.innerHTML = html;
  }

  private highlightActiveSidebar() {
    this.renderSidebar();
    const activeEl = document.querySelector(`.sidebar-link[data-id="${this.activeDocId}"]`);
    if (activeEl) {
      activeEl.scrollIntoView({ block: 'nearest' });
    }
  }

  private renderContent() {
    const articleContainer = document.getElementById('doc-article-container');
    if (!articleContainer) return;

    const doc = this.allDocs.find((d) => d.id === this.activeDocId) || this.allDocs[0];
    if (!doc) return;

    if (this.activeMethodName && doc.methods) {
      const method = doc.methods.find((m) => m.name === this.activeMethodName);
      if (method) {
        articleContainer.innerHTML = this.renderMethodArticle(doc, method);
        this.attachArticleEvents();
        return;
      }
    }

    articleContainer.innerHTML = this.renderDocArticle(doc);
    this.attachArticleEvents();
  }

  private renderDocArticle(doc: DocItem): string {
    const breadcrumbHtml = doc.breadcrumbs
      .map((b, i) => {
        const isLast = i === doc.breadcrumbs.length - 1;
        return isLast ? `<span class="breadcrumb-current">${this.escapeHtml(b)}</span>` : `<span class="breadcrumb-item">${this.escapeHtml(b)}</span>`;
      })
      .join('<span class="breadcrumb-separator">/</span>');

    let html = `
      <nav class="mdn-breadcrumbs" aria-label="Breadcrumb">${breadcrumbHtml}</nav>
      <header class="article-header">
        <h1 class="article-title">${this.escapeHtml(doc.title)}</h1>
        ${doc.badge ? `<span class="article-badge">${this.escapeHtml(doc.badge)}</span>` : ''}
      </header>

      <div class="article-summary-lead">
        <p>${this.escapeHtml(doc.summary)}</p>
      </div>
    `;

    // Constructor / Syntax Section
    if (doc.constructorSyntax) {
      html += `
        <section class="doc-section">
          <h2 class="section-title">Syntax</h2>
          <div class="mdn-syntax-box">
            <pre><code>${this.escapeHtml(doc.constructorSyntax)}</code></pre>
          </div>
        </section>
      `;
    }

    // Parameters
    if (doc.constructorParameters && doc.constructorParameters.length > 0) {
      html += `
        <section class="doc-section">
          <h2 class="section-title">Parameters</h2>
          <table class="mdn-table">
            <thead>
              <tr>
                <th>Parameter</th>
                <th>Type</th>
                <th>Description</th>
              </tr>
            </thead>
            <tbody>
              ${doc.constructorParameters
                .map(
                  (p) => `
                <tr>
                  <td><code>${this.escapeHtml(p.name)}</code></td>
                  <td><code>${this.escapeHtml(p.type)}</code></td>
                  <td>${this.escapeHtml(p.description)}</td>
                </tr>
              `
                )
                .join('')}
            </tbody>
          </table>
        </section>
      `;
    }

    // Complexity
    if (doc.timeComplexity || doc.spaceComplexity) {
      html += `
        <section class="doc-section">
          <h2 class="section-title">Complexity</h2>
          <table class="mdn-table">
            <thead>
              <tr>
                <th>Operation</th>
                <th>Time Complexity</th>
                <th>Space Complexity</th>
              </tr>
            </thead>
            <tbody>
              <tr>
                <td>Summary</td>
                <td><code>${this.escapeHtml(doc.timeComplexity || 'O(1)')}</code></td>
                <td><code>${this.escapeHtml(doc.spaceComplexity || 'O(n)')}</code></td>
              </tr>
            </tbody>
          </table>
        </section>
      `;
    }

    // Overview Description
    if (doc.overview) {
      html += `
        <section class="doc-section">
          <h2 class="section-title">Description</h2>
          <div class="prose-text">${this.formatMarkdown(doc.overview)}</div>
        </section>
      `;
    }

    // Methods Table & List
    if (doc.methods && doc.methods.length > 0) {
      html += `
        <section class="doc-section">
          <h2 class="section-title">Instance Methods (${doc.methods.length})</h2>
          <p class="section-desc">Every method available on <code>${this.escapeHtml(doc.title)}</code> instances with detailed specifications:</p>
          <div class="methods-directory-table">
            <table class="mdn-table">
              <thead>
                <tr>
                  <th>Method</th>
                  <th>Signature</th>
                  <th>Time</th>
                  <th>Description</th>
                </tr>
              </thead>
              <tbody>
                ${doc.methods
                  .map(
                    (m) => `
                  <tr>
                    <td><a href="#${doc.id}__${m.name}" class="method-direct-link"><code>.${this.escapeHtml(m.name)}()</code></a></td>
                    <td><code>${this.escapeHtml(m.signature)}</code></td>
                    <td><span class="complexity-badge">${this.escapeHtml(m.timeComplexity)}</span></td>
                    <td>${this.escapeHtml(m.summary)}</td>
                  </tr>
                `
                  )
                  .join('')}
              </tbody>
            </table>
          </div>

          <div class="detailed-methods-accordion">
            <h3 class="subsection-title">Method Specifications</h3>
            ${doc.methods.map((m) => this.renderMethodCard(doc, m)).join('')}
          </div>
        </section>
      `;
    }

    // Examples
    if (doc.example) {
      html += `
        <section class="doc-section">
          <h2 class="section-title">Examples</h2>
          <div class="mdn-example-block">
            <div class="example-header">
              <span class="example-title">Interactive Example</span>
              <div class="example-actions">
                <button class="btn btn-sm btn-run-example" data-code="${this.escapeHtml(doc.example)}">Run in Playground</button>
                <button class="btn btn-sm btn-copy-code" data-code="${this.escapeHtml(doc.example)}">Copy</button>
              </div>
            </div>
            <pre class="example-code"><code>${this.escapeHtml(doc.example)}</code></pre>
          </div>
        </section>
      `;
    }

    // See Also
    if (doc.seeAlso && doc.seeAlso.length > 0) {
      html += `
        <section class="doc-section">
          <h2 class="section-title">See Also</h2>
          <ul class="mdn-links-list">
            ${doc.seeAlso
              .map(
                (sa) => `
              <li><a href="${sa.link}">${this.escapeHtml(sa.title)}</a></li>
            `
              )
              .join('')}
          </ul>
        </section>
      `;
    }

    return html;
  }

  private renderMethodCard(doc: DocItem, m: MethodDoc): string {
    return `
      <article class="method-card" id="method-${m.name}">
        <header class="method-header">
          <h4 class="method-title">
            <a href="#${doc.id}__${m.name}" class="method-anchor"><code>.${this.escapeHtml(m.name)}()</code></a>
          </h4>
          <span class="method-complexity-pill">${this.escapeHtml(m.timeComplexity)}</span>
        </header>
        <p class="method-summary">${this.escapeHtml(m.summary)}</p>

        <div class="method-signature-box">
          <pre><code>${this.escapeHtml(m.signature)}</code></pre>
        </div>

        ${
          m.parameters.length > 0
            ? `
          <div class="method-params-wrap">
            <h5>Parameters</h5>
            <ul class="param-bullets">
              ${m.parameters
                .map(
                  (p) => `
                <li>
                  <code>${this.escapeHtml(p.name)}</code> (<code>${this.escapeHtml(p.type)}</code>)${p.optional ? ' <em>[optional]</em>' : ''} &mdash; ${this.escapeHtml(p.description)}
                </li>
              `
                )
                .join('')}
            </ul>
          </div>
        `
            : ''
        }

        <div class="method-return-wrap">
          <h5>Return value</h5>
          <p><code>${this.escapeHtml(m.returnType.type)}</code> &mdash; ${this.escapeHtml(m.returnType.description)}</p>
        </div>

        ${
          m.example
            ? `
          <div class="method-example-wrap">
            <div class="example-header">
              <span class="example-title">Example</span>
              <button class="btn btn-sm btn-run-example" data-code="${this.escapeHtml(m.example)}">Run in Playground</button>
            </div>
            <pre class="example-code"><code>${this.escapeHtml(m.example)}</code></pre>
          </div>
        `
            : ''
        }
      </article>
    `;
  }

  private renderMethodArticle(doc: DocItem, method: MethodDoc): string {
    const breadcrumbs = [...doc.breadcrumbs, `${method.name}()`];
    const breadcrumbHtml = breadcrumbs
      .map((b, i) => {
        const isLast = i === breadcrumbs.length - 1;
        return isLast ? `<span class="breadcrumb-current">${this.escapeHtml(b)}</span>` : `<span class="breadcrumb-item">${this.escapeHtml(b)}</span>`;
      })
      .join('<span class="breadcrumb-separator">/</span>');

    return `
      <nav class="mdn-breadcrumbs" aria-label="Breadcrumb">${breadcrumbHtml}</nav>
      <header class="article-header">
        <h1 class="article-title">${this.escapeHtml(doc.title)}: ${this.escapeHtml(method.name)}()</h1>
        <span class="article-badge">${this.escapeHtml(method.timeComplexity)}</span>
      </header>

      <div class="article-summary-lead">
        <p>${this.escapeHtml(method.summary)}</p>
      </div>

      <section class="doc-section">
        <h2 class="section-title">Syntax</h2>
        <div class="mdn-syntax-box">
          <pre><code>${this.escapeHtml(method.signature)}</code></pre>
        </div>
      </section>

      <section class="doc-section">
        <h2 class="section-title">Parameters</h2>
        ${
          method.parameters.length === 0
            ? '<p>None.</p>'
            : `
          <table class="mdn-table">
            <thead>
              <tr>
                <th>Parameter</th>
                <th>Type</th>
                <th>Description</th>
              </tr>
            </thead>
            <tbody>
              ${method.parameters
                .map(
                  (p) => `
                <tr>
                  <td><code>${this.escapeHtml(p.name)}</code></td>
                  <td><code>${this.escapeHtml(p.type)}</code></td>
                  <td>${this.escapeHtml(p.description)}</td>
                </tr>
              `
                )
                .join('')}
            </tbody>
          </table>
        `
        }
      </section>

      <section class="doc-section">
        <h2 class="section-title">Return Value</h2>
        <p>A value of type <code>${this.escapeHtml(method.returnType.type)}</code>: ${this.escapeHtml(method.returnType.description)}</p>
      </section>

      <section class="doc-section">
        <h2 class="section-title">Complexity</h2>
        <table class="mdn-table">
          <thead>
            <tr>
              <th>Time Complexity</th>
              <th>Space Complexity</th>
            </tr>
          </thead>
          <tbody>
            <tr>
              <td><code>${this.escapeHtml(method.timeComplexity)}</code></td>
              <td><code>${this.escapeHtml(method.spaceComplexity)}</code></td>
            </tr>
          </tbody>
        </table>
      </section>

      <section class="doc-section">
        <h2 class="section-title">Description</h2>
        <p>${this.escapeHtml(method.description)}</p>
      </section>

      <section class="doc-section">
        <h2 class="section-title">Examples</h2>
        <div class="mdn-example-block">
          <div class="example-header">
            <span class="example-title">Example: using ${this.escapeHtml(method.name)}()</span>
            <div class="example-actions">
              <button class="btn btn-sm btn-run-example" data-code="${this.escapeHtml(method.example)}">Run in Playground</button>
              <button class="btn btn-sm btn-copy-code" data-code="${this.escapeHtml(method.example)}">Copy</button>
            </div>
          </div>
          <pre class="example-code"><code>${this.escapeHtml(method.example)}</code></pre>
        </div>
      </section>

      <section class="doc-section">
        <h2 class="section-title">See Also</h2>
        <ul class="mdn-links-list">
          <li><a href="#${doc.id}">Return to ${this.escapeHtml(doc.title)} Overview</a></li>
          ${
            doc.methods
              ? doc.methods
                  .filter((m) => m.name !== method.name)
                  .slice(0, 5)
                  .map((m) => `<li><a href="#${doc.id}__${m.name}"><code>${doc.title}.${m.name}()</code></a></li>`)
                  .join('')
              : ''
          }
        </ul>
      </section>
    `;
  }

  private attachArticleEvents() {
    const runButtons = document.querySelectorAll('.btn-run-example');
    runButtons.forEach((btn) => {
      btn.addEventListener('click', (e) => {
        const code = (e.currentTarget as HTMLElement).getAttribute('data-code') || '';
        this.setPlaygroundCode(code);
      });
    });

    const copyButtons = document.querySelectorAll('.btn-copy-code');
    copyButtons.forEach((btn) => {
      btn.addEventListener('click', (e) => {
        const code = (e.currentTarget as HTMLElement).getAttribute('data-code') || '';
        navigator.clipboard.writeText(code);
        const originalText = btn.textContent;
        btn.textContent = 'Copied!';
        setTimeout(() => (btn.textContent = originalText), 1500);
      });
    });
  }

  private escapeHtml(str: string): string {
    return str
      .replace(/&/g, '&amp;')
      .replace(/</g, '&lt;')
      .replace(/>/g, '&gt;')
      .replace(/"/g, '&quot;')
      .replace(/'/g, '&#039;');
  }

  private formatMarkdown(text: string): string {
    return text
      .split('\n\n')
      .map((para) => {
        if (para.startsWith('### ')) {
          return `<h4>${this.escapeHtml(para.replace('### ', ''))}</h4>`;
        }
        if (para.startsWith('## ')) {
          return `<h3>${this.escapeHtml(para.replace('## ', ''))}</h3>`;
        }
        if (para.startsWith('# ')) {
          return `<h2>${this.escapeHtml(para.replace('# ', ''))}</h2>`;
        }
        if (para.startsWith('- ')) {
          const items = para
            .split('\n')
            .map((li) => `<li>${this.escapeHtml(li.replace(/^- /, ''))}</li>`)
            .join('');
          return `<ul>${items}</ul>`;
        }
        return `<p>${this.escapeHtml(para)}</p>`;
      })
      .join('');
  }
}

// Instantiate on DOM ready
document.addEventListener('DOMContentLoaded', () => {
  (window as any).__pradyApp = new DocApp();
});
