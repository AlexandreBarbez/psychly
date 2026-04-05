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

  private navigateTo(detail: NavigateDetail) {
    this.currentView = detail.view;
    const content = this.querySelector("#app-content");
    if (!content) return;

    switch (detail.view) {
      case "list":
        content.innerHTML = `<journal-list></journal-list>`;
        break;
      case "editor":
        if (detail.id) {
          content.innerHTML = `<journal-editor entry-id="${detail.id}"></journal-editor>`;
        } else {
          content.innerHTML = `<journal-editor></journal-editor>`;
        }
        break;
      case "entry":
        content.innerHTML = `<journal-entry-view entry-id="${detail.id}"></journal-entry-view>`;
        break;
      case "search":
        content.innerHTML = `<journal-search></journal-search>`;
        break;
      case "chat":
        if (detail.sessionId) {
          content.innerHTML = `<chat-view session-id="${detail.sessionId}"></chat-view>`;
        } else if (detail.entryId) {
          content.innerHTML = `<chat-view entry-id="${detail.entryId}"></chat-view>`;
        } else {
          content.innerHTML = `<chat-view></chat-view>`;
        }
        break;
      case "chat-history":
        content.innerHTML = `<chat-session-list></chat-session-list>`;
        break;
    }
  }

  private render() {
    this.innerHTML = `
      <disclaimer-dialog></disclaimer-dialog>
      <div class="app-shell">
        <nav class="app-nav">
          <div class="nav-brand">
            <svg class="nav-logo-icon" viewBox="0 0 32 32" width="28" height="28" xmlns="http://www.w3.org/2000/svg" aria-hidden="true">
              <rect width="32" height="32" rx="7" fill="url(#navLogoGrad)"/>
              <defs>
                <linearGradient id="navLogoGrad" x1="0" y1="0" x2="0" y2="1">
                  <stop offset="0%" stop-color="#5185b2"/>
                  <stop offset="100%" stop-color="#2b4f78"/>
                </linearGradient>
              </defs>
              <text x="16" y="24" text-anchor="middle" font-family="Georgia,serif" font-size="20" font-weight="bold" fill="white">&#936;</text>
            </svg>
            <span class="nav-brand-name">Psychly</span>
          </div>
          <div class="nav-links">
            <button class="nav-btn" id="nav-journal">📓 Journal</button>
            <button class="nav-btn" id="nav-new-entry">✏️ Nouvelle entrée</button>
            <button class="nav-btn" id="nav-search">🔍 Rechercher</button>
            <button class="nav-btn" id="nav-chat">💬 Chat</button>
            <button class="nav-btn" id="nav-chat-history">📋 Historique</button>
          </div>
          <div id="ollama-status" class="status-indicator status-disconnected">○ Ollama indisponible</div>
        </nav>
        <main id="app-content">
          <journal-list></journal-list>
        </main>
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
  }
}

customElements.define("app-shell", AppShell);
