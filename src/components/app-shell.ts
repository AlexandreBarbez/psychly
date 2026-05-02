import { invoke } from "@tauri-apps/api/core";

interface NavigateDetail {
  view: string;
  id?: string;
  entryId?: string;
  sessionId?: string;
  query?: string;
}

export class AppShell extends HTMLElement {
  private currentView = "list";
  private ollamaConnected = false;
  private chatMounted = false;

  connectedCallback() {
    this.render();
    this.addEventListener("navigate", ((e: CustomEvent<NavigateDetail>) => {
      this.navigateTo(e.detail);
    }) as EventListener);

    this.checkOllamaStatus();
    // Check Ollama status periodically
    setInterval(() => this.checkOllamaStatus(), 30_000);
  }

  private async checkOllamaStatus() {
    try {
      this.ollamaConnected = await invoke<boolean>("check_ollama_status");
    } catch {
      this.ollamaConnected = false;
    }
    this.updateStatusIndicator();
  }

  private updateStatusIndicator() {
    const indicator = this.querySelector("#ollama-status");
    if (indicator) {
      indicator.className = `status-indicator ${this.ollamaConnected ? "status-connected" : "status-disconnected"}`;
      indicator.textContent = this.ollamaConnected ? "● Ollama connecté" : "○ Ollama indisponible";
    }
  }

  private updateActiveNav(view: string) {
    this.querySelectorAll(".nav-item").forEach(el => el.classList.remove("active"));
    const viewToId: Record<string, string> = {
      list: "nav-journal",
      editor: "nav-new-entry",
      search: "nav-search",
      chat: "nav-chat",
      "chat-history": "nav-chat-history",
      entry: "nav-journal",
    };
    const id = viewToId[view];
    if (id) this.querySelector(`#${id}`)?.classList.add("active");
  }

  private updateBreadcrumb(view: string, hasId?: boolean) {
    const el = this.querySelector<HTMLElement>("#app-breadcrumb");
    if (!el) return;
    const paths: Record<string, string[]> = {
      list: ["journal", "entries"],
      editor: hasId ? ["journal", "entries", "edit"] : ["journal", "new-entry"],
      entry: ["journal", "entries", "view"],
      search: ["journal", "search"],
      chat: ["therapy", "chat"],
      "chat-history": ["therapy", "history"],
    };
    const segments = paths[view] ?? [view];
    el.innerHTML = `
      <span class="bc-home">~</span>
      ${segments.map((s, i) => `
        <span class="bc-sep">/</span>
        <span class="${i === segments.length - 1 ? "bc-current" : "bc-section"}">${s}</span>
      `).join("")}
    `;
  }

  private navigateTo(detail: NavigateDetail) {
    this.currentView = detail.view;
    this.updateActiveNav(detail.view);
    this.updateBreadcrumb(detail.view, !!detail.id);
    const appContent = this.querySelector<HTMLElement>("#app-content");
    const chatContainer = this.querySelector<HTMLElement>("#chat-container");
    if (!appContent || !chatContainer) return;

    if (detail.view === "chat") {
      appContent.style.display = "none";
      chatContainer.style.display = "flex";
      if (!this.chatMounted) {
        if (detail.sessionId) {
          chatContainer.innerHTML = `<chat-view session-id="${detail.sessionId}"></chat-view>`;
        } else if (detail.entryId) {
          chatContainer.innerHTML = `<chat-view entry-id="${detail.entryId}"></chat-view>`;
        } else {
          chatContainer.innerHTML = `<chat-view></chat-view>`;
        }
        this.chatMounted = true;
      }
      return;
    }

    appContent.style.display = "";
    chatContainer.style.display = "none";

    switch (detail.view) {
      case "list":
        appContent.innerHTML = `<journal-list></journal-list>`;
        break;
      case "editor":
        if (detail.id) {
          appContent.innerHTML = `<journal-editor entry-id="${detail.id}"></journal-editor>`;
        } else {
          appContent.innerHTML = `<journal-editor></journal-editor>`;
        }
        break;
      case "entry":
        appContent.innerHTML = `<journal-entry-view entry-id="${detail.id}"></journal-entry-view>`;
        break;
      case "search":
        appContent.innerHTML = `<journal-search></journal-search>`;
        break;
      case "chat-history":
        appContent.innerHTML = `<chat-session-list></chat-session-list>`;
        break;
    }
  }

  private render() {
    this.innerHTML = `
      <disclaimer-dialog></disclaimer-dialog>
      <div class="app-shell">
        <div class="app-body">
          <nav class="app-sidebar">
            <div class="sidebar-brand">
              <div class="sidebar-brand-name">ψ psychly</div>
              <div class="sidebar-brand-sub">v0.1.0 · local</div>
            </div>
            <div class="nav-section">
              <div class="nav-label">journal</div>
              <button class="nav-item active" id="nav-journal">
                <span class="nav-icon">❯</span>entries
              </button>
              <button class="nav-item" id="nav-new-entry">
                <span class="nav-icon">✦</span>new entry
              </button>
              <button class="nav-item" id="nav-search">
                <span class="nav-icon">⌕</span>search
              </button>
            </div>
            <div class="nav-section">
              <div class="nav-label">therapy</div>
              <button class="nav-item" id="nav-chat">
                <span class="nav-icon">❯</span>new chat
              </button>
              <button class="nav-item" id="nav-chat-history">
                <span class="nav-icon">↳</span>history
              </button>
            </div>
            <div class="nav-section">
              <div class="nav-label">data</div>
              <button class="nav-item" id="nav-export">
                <span class="nav-icon">↑</span>export
              </button>
            </div>
            <div class="sidebar-status">
              <div id="ollama-status" class="status-indicator status-disconnected">○ ollama indisponible</div>
            </div>
          </nav>
          <div class="main-panel">
            <div class="breadcrumb" id="app-breadcrumb">
              <span class="bc-home">~</span>
              <span class="bc-sep">/</span>
              <span class="bc-section">journal</span>
              <span class="bc-sep">/</span>
              <span class="bc-current">entries</span>
            </div>
            <main id="app-content">
              <journal-list></journal-list>
            </main>
            <div id="chat-container" style="display:none;"></div>

          </div>
        </div>
      </div>
    `;

    this.querySelector("#nav-journal")?.addEventListener("click", () => {
      this.navigateTo({ view: "list" });
    });
    this.querySelector("#nav-new-entry")?.addEventListener("click", () => {
      this.navigateTo({ view: "editor" });
    });
    this.querySelector("#nav-search")?.addEventListener("click", () => {
      this.navigateTo({ view: "search" });
    });
    this.querySelector("#nav-chat")?.addEventListener("click", () => {
      this.navigateTo({ view: "chat" });
    });
    this.querySelector("#nav-chat-history")?.addEventListener("click", () => {
      this.navigateTo({ view: "chat-history" });
    });
    this.querySelector("#nav-export")?.addEventListener("click", () => {
      if (document.querySelector("export-dialog")) return;
      const dialog = document.createElement("export-dialog");
      document.body.appendChild(dialog);
    });

    this.querySelector("#chat-container")?.addEventListener("close-chat", () => {
      const chatContainer = this.querySelector<HTMLElement>("#chat-container");
      if (chatContainer) {
        chatContainer.innerHTML = "";
        chatContainer.style.display = "none";
      }
      this.chatMounted = false;
      this.navigateTo({ view: "list" });
    });
  }
}

customElements.define("app-shell", AppShell);
