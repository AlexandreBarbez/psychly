import { open } from "@tauri-apps/plugin-dialog";
import { exportJournal, importJournal } from "../api/export";

export class ExportDialog extends HTMLElement {
  connectedCallback() {
    this.render();
  }

  private render() {
    this.innerHTML = `
      <div class="export-overlay">
        <div class="export-dialog">
          <h2>Export / Import</h2>
          <p>Exportez vos entrées vers un dossier de fichiers Markdown lisibles, ou importez depuis un export précédent.</p>
          <div class="export-actions">
            <button class="btn-primary" id="btn-export">💾 Exporter vers un dossier</button>
            <button class="btn-secondary" id="btn-import">📥 Importer depuis un dossier</button>
          </div>
          <div id="export-feedback" class="export-feedback"></div>
          <button class="btn-close" id="btn-close">✕ Fermer</button>
        </div>
      </div>
    `;

    this.querySelector("#btn-close")?.addEventListener("click", () => this.remove());
    this.querySelector("#btn-export")?.addEventListener("click", () => this.handleExport());
    this.querySelector("#btn-import")?.addEventListener("click", () => this.handleImport());
  }

  private showFeedback(msg: string, isError = false) {
    const el = this.querySelector("#export-feedback");
    if (el) {
      el.textContent = msg;
      el.className = `export-feedback ${isError ? "feedback-error" : "feedback-success"}`;
    }
  }

  private async handleExport() {
    try {
      const raw = await open({ directory: true, multiple: false });
      const dir = Array.isArray(raw) ? raw[0] : raw;
      if (!dir) return;
      const count = await exportJournal(dir);
      this.showFeedback(`${count} entrée(s) exportée(s).`);
    } catch (e) {
      console.error("Export Markdown failed:", e);
      this.showFeedback(`Erreur : ${e}`, true);
    }
  }

  private async handleImport() {
    try {
      const raw = await open({ directory: true, multiple: false });
      const dir = Array.isArray(raw) ? raw[0] : raw;
      if (!dir) return;
      const result2 = await importJournal(dir);
      let msg = `${result2.inserted} importée(s), ${result2.skipped} ignorée(s).`;
      if (result2.errors.length > 0) {
        msg += ` ${result2.errors.length} erreur(s).`;
      }
      this.showFeedback(msg, result2.errors.length > 0);
    } catch (e) {
      console.error("Import Markdown failed:", e);
      this.showFeedback(`Erreur : ${e}`, true);
    }
  }
}

customElements.define("export-dialog", ExportDialog);
