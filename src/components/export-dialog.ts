import { open, save } from "@tauri-apps/plugin-dialog";
import { exportJournal, importJournal, backupDb, restoreDb } from "../api/export";

export class ExportDialog extends HTMLElement {
  connectedCallback() {
    this.render();
  }

  private render() {
    this.innerHTML = `
      <div class="export-overlay">
        <div class="export-dialog">
          <h2>Export / Import</h2>

          <section class="export-section">
            <h3>📝 Journal (Markdown)</h3>
            <p>Exportez vos entrées vers des fichiers Markdown lisibles, ou importez depuis un dossier existant.</p>
            <div class="export-actions">
              <button class="btn-primary" id="btn-export-md">💾 Exporter</button>
              <button class="btn-secondary" id="btn-import-md">📥 Importer</button>
            </div>
          </section>

          <section class="export-section">
            <h3>🗄️ Base de données complète</h3>
            <p>Sauvegardez ou restaurez l'ensemble de vos données (journal, thérapie, analyses).</p>
            <div class="export-actions">
              <button class="btn-primary" id="btn-backup-db">💾 Sauvegarder</button>
              <button class="btn-secondary" id="btn-restore-db">📥 Restaurer</button>
            </div>
          </section>

          <div id="export-feedback" class="export-feedback"></div>
          <button class="btn-close" id="btn-close">✕ Fermer</button>
        </div>
      </div>
    `;

    this.querySelector("#btn-close")?.addEventListener("click", () => this.remove());
    this.querySelector("#btn-export-md")?.addEventListener("click", () => this.handleExportMd());
    this.querySelector("#btn-import-md")?.addEventListener("click", () => this.handleImportMd());
    this.querySelector("#btn-backup-db")?.addEventListener("click", () => this.handleBackupDb());
    this.querySelector("#btn-restore-db")?.addEventListener("click", () => this.handleRestoreDb());
  }

  private showFeedback(msg: string, isError = false) {
    const el = this.querySelector("#export-feedback");
    if (el) {
      el.textContent = msg;
      el.className = `export-feedback ${isError ? "feedback-error" : "feedback-success"}`;
    }
  }

  private async handleExportMd() {
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

  private async handleImportMd() {
    try {
      const raw = await open({ directory: true, multiple: false });
      const dir = Array.isArray(raw) ? raw[0] : raw;
      if (!dir) return;
      const result = await importJournal(dir);
      let msg = `${result.inserted} importée(s), ${result.skipped} ignorée(s).`;
      if (result.errors.length > 0) msg += ` ${result.errors.length} erreur(s).`;
      this.showFeedback(msg, result.errors.length > 0);
    } catch (e) {
      console.error("Import Markdown failed:", e);
      this.showFeedback(`Erreur : ${e}`, true);
    }
  }

  private async handleBackupDb() {
    try {
      const dest = await save({
        defaultPath: "psychly-backup.db",
        filters: [{ name: "SQLite Database", extensions: ["db"] }],
      });
      if (!dest) return;
      await backupDb(dest);
      this.showFeedback("Sauvegarde créée.");
    } catch (e) {
      console.error("DB backup failed:", e);
      this.showFeedback(`Erreur : ${e}`, true);
    }
  }

  private async handleRestoreDb() {
    try {
      const raw = await open({
        multiple: false,
        filters: [{ name: "SQLite Database", extensions: ["db"] }],
      });
      const src = Array.isArray(raw) ? raw[0] : raw;
      if (!src) return;
      const result = await restoreDb(src);
      let msg = `${result.inserted} entrée(s) restaurée(s), ${result.skipped} ignorée(s).`;
      if (result.errors.length > 0) msg += ` ${result.errors.length} erreur(s).`;
      this.showFeedback(msg, result.errors.length > 0);
    } catch (e) {
      console.error("DB restore failed:", e);
      this.showFeedback(`Erreur : ${e}`, true);
    }
  }
}

customElements.define("export-dialog", ExportDialog);
