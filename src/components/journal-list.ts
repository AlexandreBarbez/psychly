import { listEntries, deleteEntry, type EntryResponse } from "../api/journal";

export class JournalList extends HTMLElement {
  private entries: EntryResponse[] = [];

  connectedCallback() {
    this.render();
    this.loadEntries();
  }

  async loadEntries() {
    this.entries = await listEntries();
    this.render();
  }

  private render() {
    if (this.entries.length === 0) {
      this.innerHTML = `
        <div class="journal-list-empty">
          <p>Votre journal est vide.</p>
          <p>Commencez par écrire votre première entrée.</p>
          <button class="btn-primary" id="new-entry-btn">Nouvelle entrée</button>
        </div>
      `;
      this.querySelector("#new-entry-btn")?.addEventListener("click", () => {
        this.dispatchEvent(
          new CustomEvent("navigate", { detail: { view: "editor" }, bubbles: true })
        );
      });
      return;
    }

    this.innerHTML = `
      <div class="journal-list">
        <div class="journal-list-header">
          <h2>Journal</h2>
          <button class="btn-primary" id="new-entry-btn">Nouvelle entrée</button>
        </div>
        <ul class="entry-list">
          ${this.entries
            .map(
              (e) => `
            <li class="entry-item" data-id="${e.id}">
              <span class="entry-date">${this.formatDate(e.created_at)}</span>
              <span class="entry-preview">${this.escapeHtml(e.preview)}</span>
              <button class="btn-delete" data-id="${e.id}" title="Supprimer">✕</button>
            </li>
          `
            )
            .join("")}
        </ul>
      </div>
    `;

    this.querySelector("#new-entry-btn")?.addEventListener("click", () => {
      this.dispatchEvent(
        new CustomEvent("navigate", { detail: { view: "editor" }, bubbles: true })
      );
    });

    this.querySelectorAll<HTMLElement>(".entry-item").forEach((item) => {
      item.addEventListener("click", (e) => {
        const target = e.target as HTMLElement;
        if (target.classList.contains("btn-delete")) return;
        const id = item.dataset.id;
        this.dispatchEvent(
          new CustomEvent("navigate", {
            detail: { view: "entry", id },
            bubbles: true,
          })
        );
      });
    });

    this.querySelectorAll<HTMLButtonElement>(".btn-delete").forEach((btn) => {
      btn.addEventListener("click", async (e) => {
        e.stopPropagation();
        const id = btn.dataset.id!;
        
        // Custom confirmation dialog
        const dialog = document.createElement("div");
        dialog.className = "disclaimer-overlay";
        dialog.innerHTML = `
          <div class="disclaimer-dialog">
            <h2 style="margin-top: 0">Suppression</h2>
            <p>Voulez-vous vraiment supprimer cette entrée ?</p>
            <div style="display: flex; gap: 0.5rem; justify-content: flex-end; margin-top: 1.5rem;">
              <button class="btn-secondary" id="confirm-no">Annuler</button>
              <button class="btn-primary" id="confirm-yes" style="background: #ff3b30;">Supprimer</button>
            </div>
          </div>
        `;
        
        document.body.appendChild(dialog);

        const cleanup = () => dialog.remove();

        const handleEscape = (e: KeyboardEvent) => {
          if (e.key === "Escape") cleanup();
        };
        document.addEventListener("keydown", handleEscape, { once: true });

        dialog.querySelector("#confirm-yes")?.addEventListener("click", async () => {
          document.removeEventListener("keydown", handleEscape);
          cleanup();
          try {
            await deleteEntry(id);
            await this.loadEntries();
          } catch (error) {
            console.error("Failed to delete entry:", error);
          }
        });

        dialog.querySelector("#confirm-no")?.addEventListener("click", () => {
          document.removeEventListener("keydown", handleEscape);
          cleanup();
        });
      });
    });
  }

  private formatDate(dateStr: string): string {
    const date = new Date(dateStr);
    return date.toLocaleDateString("fr-FR", {
      day: "numeric",
      month: "long",
      year: "numeric",
    });
  }

  private escapeHtml(text: string): string {
    const div = document.createElement("div");
    div.textContent = text;
    return div.innerHTML;
  }
}

customElements.define("journal-list", JournalList);
