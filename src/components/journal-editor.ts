import { createEntry, updateEntry, getEntry } from "../api/journal";

export class JournalEditor extends HTMLElement {
  private entryId: string | null = null;
  private originalBody = "";

  static get observedAttributes() {
    return ["entry-id"];
  }

  attributeChangedCallback(name: string, _old: string, value: string) {
    if (name === "entry-id") {
      this.entryId = value || null;
      this.loadEntry();
    }
  }

  connectedCallback() {
    this.entryId = this.getAttribute("entry-id") || null;
    this.render();
    if (this.entryId) {
      this.loadEntry();
    }
  }

  private async loadEntry() {
    if (!this.entryId) return;
    const entry = await getEntry(this.entryId);
    if (entry) {
      this.originalBody = entry.body;
      const textarea = this.querySelector<HTMLTextAreaElement>("#entry-body");
      if (textarea) textarea.value = entry.body;
    }
  }

  private render() {
    const isEdit = !!this.entryId;
    this.innerHTML = `
      <div class="journal-editor">
        <div class="editor-header">
          <button class="btn-back" id="back-btn">← Retour</button>
          <h2>${isEdit ? "Modifier l'entrée" : "Nouvelle entrée"}</h2>
        </div>
        <textarea id="entry-body" placeholder="Écrivez ici..." rows="12">${this.escapeHtml(this.originalBody)}</textarea>
        <div class="editor-actions">
          <button class="btn-primary" id="save-btn">Sauvegarder</button>
        </div>
      </div>
    `;

    this.querySelector("#back-btn")?.addEventListener("click", () => {
      this.dispatchEvent(
        new CustomEvent("navigate", { detail: { view: "list" }, bubbles: true })
      );
    });

    this.querySelector("#save-btn")?.addEventListener("click", () => this.save());
  }

  private async save() {
    const textarea = this.querySelector<HTMLTextAreaElement>("#entry-body");
    if (!textarea) return;

    const body = textarea.value.trim();
    if (!body) {
      textarea.classList.add("error");
      return;
    }
    textarea.classList.remove("error");

    if (this.entryId) {
      await updateEntry(this.entryId, body);
    } else {
      await createEntry(body);
    }

    this.dispatchEvent(
      new CustomEvent("navigate", { detail: { view: "list" }, bubbles: true })
    );
  }

  private escapeHtml(text: string): string {
    const div = document.createElement("div");
    div.textContent = text;
    return div.innerHTML;
  }
}

customElements.define("journal-editor", JournalEditor);
