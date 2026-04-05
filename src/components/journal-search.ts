import { searchEntries, type EntryResponse } from "../api/journal";

export class JournalSearch extends HTMLElement {
  private results: EntryResponse[] = [];
  private query = "";
  private debounceTimer: ReturnType<typeof setTimeout> | null = null;

  connectedCallback() {
    this.render();
  }

  private render() {
    this.innerHTML = `
      <div class="journal-search">
        <input type="text" id="search-input" placeholder="Rechercher dans le journal…" value="${this.escapeHtml(this.query)}" />
        <div id="search-results">
          ${this.renderResults()}
        </div>
      </div>
    `;

    const input = this.querySelector<HTMLInputElement>("#search-input");
    input?.addEventListener("input", (e) => {
      this.query = (e.target as HTMLInputElement).value;
      this.debounceSearch();
    });

    this.querySelectorAll<HTMLElement>(".search-result-item").forEach((item) => {
      item.addEventListener("click", () => {
        const id = item.dataset.id;
        this.dispatchEvent(
          new CustomEvent("navigate", {
            detail: { view: "entry", id },
            bubbles: true,
          })
        );
      });
    });
  }

  private renderResults(): string {
    if (!this.query) return "";

    if (this.results.length === 0) {
      return `<p class="no-results">Aucun résultat trouvé.</p>`;
    }

    return `
      <ul class="search-results-list">
        ${this.results
          .map(
            (e) => `
          <li class="search-result-item" data-id="${e.id}">
            <span class="entry-date">${this.formatDate(e.created_at)}</span>
            <span class="entry-preview">${this.highlightMatch(e.preview, this.query)}</span>
          </li>
        `
          )
          .join("")}
      </ul>
    `;
  }

  private debounceSearch() {
    if (this.debounceTimer) clearTimeout(this.debounceTimer);
    this.debounceTimer = setTimeout(() => this.doSearch(), 300);
  }

  private async doSearch() {
    if (!this.query.trim()) {
      this.results = [];
      this.updateResults();
      return;
    }
    this.results = await searchEntries(this.query.trim());
    this.updateResults();
  }

  private updateResults() {
    const container = this.querySelector("#search-results");
    if (container) {
      container.innerHTML = this.renderResults();
      this.querySelectorAll<HTMLElement>(".search-result-item").forEach((item) => {
        item.addEventListener("click", () => {
          const id = item.dataset.id;
          this.dispatchEvent(
            new CustomEvent("navigate", {
              detail: { view: "entry", id },
              bubbles: true,
            })
          );
        });
      });
    }
  }

  private highlightMatch(text: string, query: string): string {
    const escaped = this.escapeHtml(text);
    const regex = new RegExp(`(${this.escapeRegex(query)})`, "gi");
    return escaped.replace(regex, "<mark>$1</mark>");
  }

  private escapeRegex(str: string): string {
    return str.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
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

customElements.define("journal-search", JournalSearch);
