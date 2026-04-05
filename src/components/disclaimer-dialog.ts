const DISCLAIMER_KEY = "psychly_disclaimer_shown";

export class DisclaimerDialog extends HTMLElement {
  connectedCallback() {
    if (localStorage.getItem(DISCLAIMER_KEY)) {
      this.remove();
      return;
    }
    this.render();
  }

  private render() {
    this.innerHTML = `
      <div class="disclaimer-overlay">
        <div class="disclaimer-dialog">
          <h2>⚠️ Avertissement important</h2>
          <p>
            <strong>Psychly</strong> est un outil d'accompagnement personnel basé sur l'intelligence artificielle.
            Il ne remplace en aucun cas un professionnel de santé mentale agréé (psychologue, psychiatre, psychothérapeute).
          </p>
          <p>
            L'IA peut commettre des erreurs et ses réponses ne constituent pas un avis médical ou thérapeutique.
            Si vous traversez une période difficile, nous vous recommandons vivement de consulter un professionnel.
          </p>
          <p>
            En cas d'urgence, appelez le <strong>3114</strong> (numéro national de prévention du suicide, 24h/24, gratuit et confidentiel).
          </p>
          <button class="btn-primary" id="disclaimer-accept">J'ai compris</button>
        </div>
      </div>
    `;

    this.querySelector("#disclaimer-accept")?.addEventListener("click", () => {
      localStorage.setItem(DISCLAIMER_KEY, "true");
      this.remove();
    });
  }
}

customElements.define("disclaimer-dialog", DisclaimerDialog);
