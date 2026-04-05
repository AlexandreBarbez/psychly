import { getEntry, type EntryResponse } from "../api/journal";

export class JournalEntryView extends HTMLElement {
  private entry: EntryResponse | null = null;

  static get observedAttributes() {
    return ["entry-id"];
  }

  attributeChangedCallback(name: string, _old: string, value: string) {
    if (name === "entry-id") {
      this.loadEntry(value);
    }
  }

  connectedCallback() {
    const id = this.getAttribute("entry-id");
    if (id) {
      this.loadEntry(id);
    } else {
      this.renderEmpty();
    }
  }

  private async loadEntry(id: string) {
    this.entry = await getEntry(id);
    this.render();
  }

  private renderEmpty() {
    this.innerHTML = `<p>Entrée introuvable.</p>`;
  }

  private render() {
    if (!this.entry) {
      this.renderEmpty();
      return;
    }

    const date = new Date(this.entry.created_at).toLocaleDateString("fr-FR", {
      day: "numeric",
      month: "long",
      year: "numeric",
      hour: "2-digit",
      minute: "2-digit",
    });

    this.innerHTML = `
      <div class="journal-entry-view">
        <div class="entry-view-header">
          <button class="btn-back" id="back-btn">← Retour</button>
          <div class="entry-view-actions">
            <button class="btn-secondary" id="edit-btn">Modifier</button>
            <button class="btn-secondary" id="chat-btn">💬 Chat</button>
          </div>
        </div>
        <p class="entry-date">${date}</p>
        <div class="entry-body">${this.escapeHtml(this.entry.body)}</div>
      </div>
    `;

    this.querySelector("#back-btn")?.addEventListener("click", () => {
      this.dispatchEvent(
        new CustomEvent("navigate", { detail: { view: "list" }, bubbles: true })
      );
    });

    this.querySelector("#edit-btn")?.addEventListener("click", () => {
      this.dispatchEvent(
        new CustomEvent("navigate", {
          detail: { view: "editor", id: this.entry!.id },
          bubbles: true,
        })
      );
    });

    this.querySelector("#chat-btn")?.addEventListener("click", () => {
      this.dispatchEvent(
        new CustomEvent("navigate", {
          detail: { view: "chat", entryId: this.entry!.id },
          bubbles: true,
        })
      );
    });
  }

  private escapeHtml(text: string): string {
    const div = document.createElement("div");
    div.textContent = text;
    return div.innerHTML.replace(/\n/g, "<br>");
  }
}

customElements.define("journal-entry-view", JournalEntryView);
