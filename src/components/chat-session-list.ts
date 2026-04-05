import { listChatSessions, type SessionResponse } from "../api/chat";

export class ChatSessionList extends HTMLElement {
  private sessions: SessionResponse[] = [];

  async connectedCallback() {
    await this.loadSessions();
  }

  private async loadSessions() {
    this.sessions = await listChatSessions();
    this.render();
  }

  private render() {
    if (this.sessions.length === 0) {
      this.innerHTML = `
        <div class="chat-session-list">
          <div class="chat-sessions-header">
            <button class="btn-back" id="sessions-back-btn">← Retour</button>
            <h2>Historique des conversations</h2>
          </div>
          <p class="empty-state">Aucune conversation passée.</p>
        </div>
      `;
      this.querySelector("#sessions-back-btn")?.addEventListener("click", () => {
        this.dispatchEvent(
          new CustomEvent("navigate", { detail: { view: "list" }, bubbles: true })
        );
      });
      return;
    }

    this.innerHTML = `
      <div class="chat-session-list">
        <div class="chat-sessions-header">
          <button class="btn-back" id="sessions-back-btn">← Retour</button>
          <h2>Historique des conversations</h2>
        </div>
        <ul class="session-list">
          ${this.sessions
            .map((s) => {
              const date = new Date(s.created_at).toLocaleDateString("fr-FR", {
                day: "numeric",
                month: "long",
                year: "numeric",
                hour: "2-digit",
                minute: "2-digit",
              });
              const context = s.journal_entry_id
                ? "Depuis une entrée de journal"
                : "Conversation libre";
              return `
                <li class="session-item" data-id="${s.id}">
                  <span class="session-date">${date}</span>
                  <span class="session-context">${context}</span>
                </li>
              `;
            })
            .join("")}
        </ul>
      </div>
    `;

    this.querySelector("#sessions-back-btn")?.addEventListener("click", () => {
      this.dispatchEvent(
        new CustomEvent("navigate", { detail: { view: "list" }, bubbles: true })
      );
    });

    this.querySelectorAll(".session-item").forEach((item) => {
      item.addEventListener("click", () => {
        const id = (item as HTMLElement).dataset.id;
        if (id) {
          this.dispatchEvent(
            new CustomEvent("navigate", {
              detail: { view: "chat", sessionId: id },
              bubbles: true,
            })
          );
        }
      });
    });
  }
}

customElements.define("chat-session-list", ChatSessionList);
