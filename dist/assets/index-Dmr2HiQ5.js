(function(){let e=document.createElement(`link`).relList;if(e&&e.supports&&e.supports(`modulepreload`))return;for(let e of document.querySelectorAll(`link[rel="modulepreload"]`))n(e);new MutationObserver(e=>{for(let t of e)if(t.type===`childList`)for(let e of t.addedNodes)e.tagName===`LINK`&&e.rel===`modulepreload`&&n(e)}).observe(document,{childList:!0,subtree:!0});function t(e){let t={};return e.integrity&&(t.integrity=e.integrity),e.referrerPolicy&&(t.referrerPolicy=e.referrerPolicy),e.crossOrigin===`use-credentials`?t.credentials=`include`:e.crossOrigin===`anonymous`?t.credentials=`omit`:t.credentials=`same-origin`,t}function n(e){if(e.ep)return;e.ep=!0;let n=t(e);fetch(e.href,n)}})();function e(e,t=!1){return window.__TAURI_INTERNALS__.transformCallback(e,t)}async function t(e,t={},n){return window.__TAURI_INTERNALS__.invoke(e,t,n)}var n=class extends HTMLElement{constructor(...e){super(...e),this.currentView=`list`,this.ollamaConnected=!1,this.chatMounted=!1}connectedCallback(){this.render(),this.addEventListener(`navigate`,(e=>{this.navigateTo(e.detail)})),this.checkOllamaStatus(),setInterval(()=>this.checkOllamaStatus(),3e4)}async checkOllamaStatus(){try{this.ollamaConnected=await t(`check_ollama_status`)}catch{this.ollamaConnected=!1}this.updateStatusIndicator()}updateStatusIndicator(){let e=this.querySelector(`#ollama-status`);e&&(e.className=`status-indicator ${this.ollamaConnected?`status-connected`:`status-disconnected`}`,e.textContent=this.ollamaConnected?`● Ollama connecté`:`○ Ollama indisponible`)}navigateTo(e){this.currentView=e.view;let t=this.querySelector(`#app-content`),n=this.querySelector(`#chat-container`);if(!(!t||!n)){if(e.view===`chat`){t.style.display=`none`,n.style.display=`block`,this.chatMounted||=(e.sessionId?n.innerHTML=`<chat-view session-id="${e.sessionId}"></chat-view>`:e.entryId?n.innerHTML=`<chat-view entry-id="${e.entryId}"></chat-view>`:n.innerHTML=`<chat-view></chat-view>`,!0);return}switch(t.style.display=``,n.style.display=`none`,e.view){case`list`:t.innerHTML=`<journal-list></journal-list>`;break;case`editor`:e.id?t.innerHTML=`<journal-editor entry-id="${e.id}"></journal-editor>`:t.innerHTML=`<journal-editor></journal-editor>`;break;case`entry`:t.innerHTML=`<journal-entry-view entry-id="${e.id}"></journal-entry-view>`;break;case`search`:t.innerHTML=`<journal-search></journal-search>`;break;case`chat-history`:t.innerHTML=`<chat-session-list></chat-session-list>`;break}}}render(){this.innerHTML=`
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
            <button class="nav-btn" id="nav-export">💾 Export</button>
          </div>
          <div id="ollama-status" class="status-indicator status-disconnected">○ Ollama indisponible</div>
        </nav>
        <main id="app-content">
          <journal-list></journal-list>
        </main>
        <div id="chat-container"></div>
      </div>
    `,this.querySelector(`#nav-journal`)?.addEventListener(`click`,()=>{this.navigateTo({view:`list`})}),this.querySelector(`#nav-new-entry`)?.addEventListener(`click`,()=>{this.navigateTo({view:`editor`})}),this.querySelector(`#nav-search`)?.addEventListener(`click`,()=>{this.navigateTo({view:`search`})}),this.querySelector(`#nav-chat`)?.addEventListener(`click`,()=>{this.navigateTo({view:`chat`})}),this.querySelector(`#nav-chat-history`)?.addEventListener(`click`,()=>{this.navigateTo({view:`chat-history`})}),this.querySelector(`#nav-export`)?.addEventListener(`click`,()=>{if(document.querySelector(`export-dialog`))return;let e=document.createElement(`export-dialog`);document.body.appendChild(e)}),this.querySelector(`#chat-container`)?.addEventListener(`close-chat`,()=>{let e=this.querySelector(`#chat-container`);e&&(e.innerHTML=``),this.chatMounted=!1,this.navigateTo({view:`list`})})}};customElements.define(`app-shell`,n);async function r(e){return t(`create_entry`,{input:{body:e}})}async function i(e){return t(`get_entry`,{id:e})}async function a(e=0,n=50){return t(`list_entries`,{input:{offset:e,limit:n}})}async function o(e,n){return t(`update_entry`,{input:{id:e,body:n}})}async function s(e){return t(`delete_entry`,{id:e})}async function c(e){return t(`search_entries`,{query:e})}var l=class extends HTMLElement{constructor(...e){super(...e),this.entries=[]}connectedCallback(){this.render(),this.loadEntries()}async loadEntries(){this.entries=await a(),this.render()}render(){if(this.entries.length===0){this.innerHTML=`
        <div class="journal-list-empty">
          <p>Votre journal est vide.</p>
          <p>Commencez par écrire votre première entrée.</p>
          <button class="btn-primary" id="new-entry-btn">Nouvelle entrée</button>
        </div>
      `,this.querySelector(`#new-entry-btn`)?.addEventListener(`click`,()=>{this.dispatchEvent(new CustomEvent(`navigate`,{detail:{view:`editor`},bubbles:!0}))});return}this.innerHTML=`
      <div class="journal-list">
        <div class="journal-list-header">
          <h2>Journal</h2>
          <button class="btn-primary" id="new-entry-btn">Nouvelle entrée</button>
        </div>
        <ul class="entry-list">
          ${this.entries.map(e=>`
            <li class="entry-item" data-id="${e.id}">
              <span class="entry-date">${this.formatDate(e.created_at)}</span>
              <span class="entry-preview">${this.escapeHtml(e.preview)}</span>
              <button class="btn-delete" data-id="${e.id}" title="Supprimer">✕</button>
            </li>
          `).join(``)}
        </ul>
      </div>
    `,this.querySelector(`#new-entry-btn`)?.addEventListener(`click`,()=>{this.dispatchEvent(new CustomEvent(`navigate`,{detail:{view:`editor`},bubbles:!0}))}),this.querySelectorAll(`.entry-item`).forEach(e=>{e.addEventListener(`click`,t=>{if(t.target.classList.contains(`btn-delete`))return;let n=e.dataset.id;this.dispatchEvent(new CustomEvent(`navigate`,{detail:{view:`entry`,id:n},bubbles:!0}))})}),this.querySelectorAll(`.btn-delete`).forEach(e=>{e.addEventListener(`click`,async t=>{t.stopPropagation();let n=e.dataset.id,r=document.createElement(`div`);r.className=`disclaimer-overlay`,r.innerHTML=`
          <div class="disclaimer-dialog">
            <h2 style="margin-top: 0">Suppression</h2>
            <p>Voulez-vous vraiment supprimer cette entrée ?</p>
            <div style="display: flex; gap: 0.5rem; justify-content: flex-end; margin-top: 1.5rem;">
              <button class="btn-secondary" id="confirm-no">Annuler</button>
              <button class="btn-primary" id="confirm-yes" style="background: #ff3b30;">Supprimer</button>
            </div>
          </div>
        `,document.body.appendChild(r);let i=()=>r.remove(),a=e=>{e.key===`Escape`&&i()};document.addEventListener(`keydown`,a,{once:!0}),r.querySelector(`#confirm-yes`)?.addEventListener(`click`,async()=>{document.removeEventListener(`keydown`,a),i();try{await s(n),await this.loadEntries()}catch(e){console.error(`Failed to delete entry:`,e)}}),r.querySelector(`#confirm-no`)?.addEventListener(`click`,()=>{document.removeEventListener(`keydown`,a),i()})})})}formatDate(e){return new Date(e).toLocaleDateString(`fr-FR`,{day:`numeric`,month:`long`,year:`numeric`})}escapeHtml(e){let t=document.createElement(`div`);return t.textContent=e,t.innerHTML}};customElements.define(`journal-list`,l);var u=class extends HTMLElement{constructor(...e){super(...e),this.entryId=null,this.originalBody=``}static get observedAttributes(){return[`entry-id`]}attributeChangedCallback(e,t,n){e===`entry-id`&&(this.entryId=n||null,this.loadEntry())}connectedCallback(){this.entryId=this.getAttribute(`entry-id`)||null,this.render(),this.entryId&&this.loadEntry()}async loadEntry(){if(!this.entryId)return;let e=await i(this.entryId);if(e){this.originalBody=e.body;let t=this.querySelector(`#entry-body`);t&&(t.value=e.body)}}render(){this.innerHTML=`
      <div class="journal-editor">
        <div class="editor-header">
          <button class="btn-back" id="back-btn">← Retour</button>
          <h2>${this.entryId?`Modifier l'entrée`:`Nouvelle entrée`}</h2>
        </div>
        <textarea id="entry-body" placeholder="Écrivez ici..." rows="12">${this.escapeHtml(this.originalBody)}</textarea>
        <div class="editor-actions">
          <button class="btn-primary" id="save-btn">Sauvegarder</button>
        </div>
      </div>
    `,this.querySelector(`#back-btn`)?.addEventListener(`click`,()=>{this.dispatchEvent(new CustomEvent(`navigate`,{detail:{view:`list`},bubbles:!0}))}),this.querySelector(`#save-btn`)?.addEventListener(`click`,()=>this.save())}async save(){let e=this.querySelector(`#entry-body`);if(!e)return;let t=e.value.trim();if(!t){e.classList.add(`error`);return}e.classList.remove(`error`),this.entryId?await o(this.entryId,t):await r(t),this.dispatchEvent(new CustomEvent(`navigate`,{detail:{view:`list`},bubbles:!0}))}escapeHtml(e){let t=document.createElement(`div`);return t.textContent=e,t.innerHTML}};customElements.define(`journal-editor`,u);var d=class extends HTMLElement{constructor(...e){super(...e),this.entry=null}static get observedAttributes(){return[`entry-id`]}attributeChangedCallback(e,t,n){e===`entry-id`&&this.loadEntry(n)}connectedCallback(){let e=this.getAttribute(`entry-id`);e?this.loadEntry(e):this.renderEmpty()}async loadEntry(e){this.entry=await i(e),this.render()}renderEmpty(){this.innerHTML=`<p>Entrée introuvable.</p>`}render(){if(!this.entry){this.renderEmpty();return}this.innerHTML=`
      <div class="journal-entry-view">
        <div class="entry-view-header">
          <button class="btn-back" id="back-btn">← Retour</button>
          <div class="entry-view-actions">
            <button class="btn-secondary" id="edit-btn">Modifier</button>
            <button class="btn-secondary" id="chat-btn">💬 Chat</button>
          </div>
        </div>
        <p class="entry-date">${new Date(this.entry.created_at).toLocaleDateString(`fr-FR`,{day:`numeric`,month:`long`,year:`numeric`,hour:`2-digit`,minute:`2-digit`})}</p>
        <div class="entry-body">${this.escapeHtml(this.entry.body)}</div>
      </div>
    `,this.querySelector(`#back-btn`)?.addEventListener(`click`,()=>{this.dispatchEvent(new CustomEvent(`navigate`,{detail:{view:`list`},bubbles:!0}))}),this.querySelector(`#edit-btn`)?.addEventListener(`click`,()=>{this.dispatchEvent(new CustomEvent(`navigate`,{detail:{view:`editor`,id:this.entry.id},bubbles:!0}))}),this.querySelector(`#chat-btn`)?.addEventListener(`click`,()=>{this.dispatchEvent(new CustomEvent(`navigate`,{detail:{view:`chat`,entryId:this.entry.id},bubbles:!0}))})}escapeHtml(e){let t=document.createElement(`div`);return t.textContent=e,t.innerHTML.replace(/\n/g,`<br>`)}};customElements.define(`journal-entry-view`,d);var f=class extends HTMLElement{constructor(...e){super(...e),this.results=[],this.query=``,this.debounceTimer=null}connectedCallback(){this.render()}render(){this.innerHTML=`
      <div class="journal-search">
        <input type="text" id="search-input" placeholder="Rechercher dans le journal…" value="${this.escapeHtml(this.query)}" />
        <div id="search-results">
          ${this.renderResults()}
        </div>
      </div>
    `,this.querySelector(`#search-input`)?.addEventListener(`input`,e=>{this.query=e.target.value,this.debounceSearch()}),this.querySelectorAll(`.search-result-item`).forEach(e=>{e.addEventListener(`click`,()=>{let t=e.dataset.id;this.dispatchEvent(new CustomEvent(`navigate`,{detail:{view:`entry`,id:t},bubbles:!0}))})})}renderResults(){return this.query?this.results.length===0?`<p class="no-results">Aucun résultat trouvé.</p>`:`
      <ul class="search-results-list">
        ${this.results.map(e=>`
          <li class="search-result-item" data-id="${e.id}">
            <span class="entry-date">${this.formatDate(e.created_at)}</span>
            <span class="entry-preview">${this.highlightMatch(e.preview,this.query)}</span>
          </li>
        `).join(``)}
      </ul>
    `:``}debounceSearch(){this.debounceTimer&&clearTimeout(this.debounceTimer),this.debounceTimer=setTimeout(()=>this.doSearch(),300)}async doSearch(){if(!this.query.trim()){this.results=[],this.updateResults();return}this.results=await c(this.query.trim()),this.updateResults()}updateResults(){let e=this.querySelector(`#search-results`);e&&(e.innerHTML=this.renderResults(),this.querySelectorAll(`.search-result-item`).forEach(e=>{e.addEventListener(`click`,()=>{let t=e.dataset.id;this.dispatchEvent(new CustomEvent(`navigate`,{detail:{view:`entry`,id:t},bubbles:!0}))})}))}highlightMatch(e,t){let n=this.escapeHtml(e),r=RegExp(`(${this.escapeRegex(t)})`,`gi`);return n.replace(r,`<mark>$1</mark>`)}escapeRegex(e){return e.replace(/[.*+?^${}()|[\]\\]/g,`\\$&`)}formatDate(e){return new Date(e).toLocaleDateString(`fr-FR`,{day:`numeric`,month:`long`,year:`numeric`})}escapeHtml(e){let t=document.createElement(`div`);return t.textContent=e,t.innerHTML}};customElements.define(`journal-search`,f);var p;(function(e){e.WINDOW_RESIZED=`tauri://resize`,e.WINDOW_MOVED=`tauri://move`,e.WINDOW_CLOSE_REQUESTED=`tauri://close-requested`,e.WINDOW_DESTROYED=`tauri://destroyed`,e.WINDOW_FOCUS=`tauri://focus`,e.WINDOW_BLUR=`tauri://blur`,e.WINDOW_SCALE_FACTOR_CHANGED=`tauri://scale-change`,e.WINDOW_THEME_CHANGED=`tauri://theme-changed`,e.WINDOW_CREATED=`tauri://window-created`,e.WEBVIEW_CREATED=`tauri://webview-created`,e.DRAG_ENTER=`tauri://drag-enter`,e.DRAG_OVER=`tauri://drag-over`,e.DRAG_DROP=`tauri://drag-drop`,e.DRAG_LEAVE=`tauri://drag-leave`})(p||={});async function m(e,n){window.__TAURI_EVENT_PLUGIN_INTERNALS__.unregisterListener(e,n),await t(`plugin:event|unlisten`,{event:e,eventId:n})}async function h(n,r,i){return t(`plugin:event|listen`,{event:n,target:typeof i?.target==`string`?{kind:`AnyLabel`,label:i.target}:i?.target??{kind:`Any`},handler:e(r)}).then(e=>async()=>m(n,e))}async function g(e){return t(`start_chat_session`,{input:{journal_entry_id:e??null}})}async function _(e,n,r){return t(`send_message`,{input:{session_id:e,content:n,journal_context:r??null}})}async function v(){return t(`list_chat_sessions`)}async function y(e){return t(`get_chat_session`,{sessionId:e})}var b=class extends HTMLElement{constructor(...e){super(...e),this.session=null,this.messages=[],this.isStreaming=!1,this.streamBuffer=``,this.unlisten=null}static get observedAttributes(){return[`entry-id`,`session-id`]}async attributeChangedCallback(e,t,n){e===`entry-id`&&n?await this.startFromEntry(n):e===`session-id`&&n&&await this.loadSession(n)}async connectedCallback(){this.render(),this.setupStreamListener();let e=this.getAttribute(`entry-id`),t=this.getAttribute(`session-id`);t?await this.loadSession(t):e?await this.startFromEntry(e):await this.startStandalone()}async disconnectedCallback(){this.unlisten&&=(this.unlisten(),null)}async setupStreamListener(){this.unlisten=await h(`chat-stream`,e=>{if(!(!this.session||e.payload.session_id!==this.session.id)){if(e.payload.done){this.isStreaming=!1,this.updateInputState();return}this.streamBuffer+=e.payload.chunk,this.updateStreamingMessage()}})}async startFromEntry(e){this.session=await g(e),this.messages=[],this.renderMessages()}async startStandalone(){this.session=await g(),this.messages=[],this.renderMessages()}async loadSession(e){let[t,n]=await y(e);this.session=t,this.messages=n,this.renderMessages();let r=this.querySelector(`#chat-input`),i=this.querySelector(`#send-btn`);r&&(r.disabled=!0),i&&(i.disabled=!0)}async handleSend(){let e=this.querySelector(`#chat-input`);if(!e||!this.session||this.isStreaming)return;let t=e.value.trim();if(t){e.value=``,this.isStreaming=!0,this.streamBuffer=``,this.updateInputState(),this.messages.push({id:``,session_id:this.session.id,role:`user`,content:t,created_at:new Date().toISOString()}),this.renderMessages(),this.messages.push({id:``,session_id:this.session.id,role:`assistant`,content:``,created_at:new Date().toISOString()}),this.renderMessages();try{let e=await _(this.session.id,t);this.messages[this.messages.length-1]=e,this.renderMessages()}catch(e){this.messages[this.messages.length-1].content=`Erreur : ${e}`,this.renderMessages()}this.isStreaming=!1,this.updateInputState()}}updateStreamingMessage(){let e=this.querySelector(`#chat-thread`);if(!e)return;let t=e.querySelector(`.chat-bubble:last-child .bubble-content`);t&&(t.textContent=this.streamBuffer,e.scrollTop=e.scrollHeight)}updateInputState(){let e=this.querySelector(`#chat-input`),t=this.querySelector(`#send-btn`);e&&(e.disabled=this.isStreaming),t&&(t.disabled=this.isStreaming)}render(){this.innerHTML=`
      <div class="chat-view">
        <div class="chat-header">
          <button class="btn-back" id="chat-back-btn">← Retour</button>
          <h2>Conversation</h2>
          <button class="btn-secondary" id="chat-close-btn">✕ Fermer</button>
        </div>
        <div class="chat-thread" id="chat-thread"></div>
        <div class="chat-input-area">
          <textarea id="chat-input" placeholder="Écrivez votre message..." rows="2"></textarea>
          <button id="send-btn" class="btn-primary">Envoyer</button>
        </div>
      </div>
    `,this.querySelector(`#chat-back-btn`)?.addEventListener(`click`,()=>{this.dispatchEvent(new CustomEvent(`navigate`,{detail:{view:`list`},bubbles:!0}))}),this.querySelector(`#chat-close-btn`)?.addEventListener(`click`,()=>{this.dispatchEvent(new CustomEvent(`close-chat`,{bubbles:!0}))}),this.querySelector(`#send-btn`)?.addEventListener(`click`,()=>{this.handleSend()}),this.querySelector(`#chat-input`)?.addEventListener(`keydown`,e=>{let t=e;t.key===`Enter`&&!t.shiftKey&&(t.preventDefault(),this.handleSend())})}renderMessages(){let e=this.querySelector(`#chat-thread`);e&&(e.innerHTML=this.messages.map(e=>{let t=e.role===`user`,n=t?`Vous`:`Psychly`;return`
          <div class="chat-bubble ${t?`bubble-user`:`bubble-assistant`}">
            <span class="bubble-role">${this.escapeHtml(n)}</span>
            <div class="bubble-content">${this.escapeHtml(e.content)}</div>
          </div>
        `}).join(``),e.scrollTop=e.scrollHeight)}escapeHtml(e){let t=document.createElement(`div`);return t.textContent=e,t.innerHTML.replace(/\n/g,`<br>`)}};customElements.define(`chat-view`,b);var x=class extends HTMLElement{constructor(...e){super(...e),this.sessions=[]}async connectedCallback(){await this.loadSessions()}async loadSessions(){this.sessions=await v(),this.render()}render(){if(this.sessions.length===0){this.innerHTML=`
        <div class="chat-session-list">
          <div class="chat-sessions-header">
            <button class="btn-back" id="sessions-back-btn">← Retour</button>
            <h2>Historique des conversations</h2>
          </div>
          <p class="empty-state">Aucune conversation passée.</p>
        </div>
      `,this.querySelector(`#sessions-back-btn`)?.addEventListener(`click`,()=>{this.dispatchEvent(new CustomEvent(`navigate`,{detail:{view:`list`},bubbles:!0}))});return}this.innerHTML=`
      <div class="chat-session-list">
        <div class="chat-sessions-header">
          <button class="btn-back" id="sessions-back-btn">← Retour</button>
          <h2>Historique des conversations</h2>
        </div>
        <ul class="session-list">
          ${this.sessions.map(e=>{let t=new Date(e.created_at).toLocaleDateString(`fr-FR`,{day:`numeric`,month:`long`,year:`numeric`,hour:`2-digit`,minute:`2-digit`}),n=e.journal_entry_id?`Depuis une entrée de journal`:`Conversation libre`;return`
                <li class="session-item" data-id="${e.id}">
                  <span class="session-date">${t}</span>
                  <span class="session-context">${n}</span>
                </li>
              `}).join(``)}
        </ul>
      </div>
    `,this.querySelector(`#sessions-back-btn`)?.addEventListener(`click`,()=>{this.dispatchEvent(new CustomEvent(`navigate`,{detail:{view:`list`},bubbles:!0}))}),this.querySelectorAll(`.session-item`).forEach(e=>{e.addEventListener(`click`,()=>{let t=e.dataset.id;t&&this.dispatchEvent(new CustomEvent(`navigate`,{detail:{view:`chat`,sessionId:t},bubbles:!0}))})})}};customElements.define(`chat-session-list`,x);var S=`psychly_disclaimer_shown`,C=class extends HTMLElement{connectedCallback(){if(localStorage.getItem(S)){this.remove();return}this.render()}render(){this.innerHTML=`
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
    `,this.querySelector(`#disclaimer-accept`)?.addEventListener(`click`,()=>{localStorage.setItem(S,`true`),this.remove()})}};customElements.define(`disclaimer-dialog`,C);async function w(e={}){return typeof e==`object`&&Object.freeze(e),await t(`plugin:dialog|open`,{options:e})}async function T(e={}){return typeof e==`object`&&Object.freeze(e),await t(`plugin:dialog|save`,{options:e})}async function E(e){return t(`export_journal`,{destDir:e})}async function D(e){return t(`import_journal`,{srcDir:e})}async function O(e){return t(`backup_db`,{destPath:e})}async function k(e){return t(`restore_db`,{srcPath:e})}var A=class extends HTMLElement{connectedCallback(){this.render()}render(){this.innerHTML=`
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
    `,this.querySelector(`#btn-close`)?.addEventListener(`click`,()=>this.remove()),this.querySelector(`#btn-export-md`)?.addEventListener(`click`,()=>this.handleExportMd()),this.querySelector(`#btn-import-md`)?.addEventListener(`click`,()=>this.handleImportMd()),this.querySelector(`#btn-backup-db`)?.addEventListener(`click`,()=>this.handleBackupDb()),this.querySelector(`#btn-restore-db`)?.addEventListener(`click`,()=>this.handleRestoreDb())}showFeedback(e,t=!1){let n=this.querySelector(`#export-feedback`);n&&(n.textContent=e,n.className=`export-feedback ${t?`feedback-error`:`feedback-success`}`)}async handleExportMd(){try{let e=await w({directory:!0,multiple:!1}),t=Array.isArray(e)?e[0]:e;if(!t)return;let n=await E(t);this.showFeedback(`${n} entrée(s) exportée(s).`)}catch(e){console.error(`Export Markdown failed:`,e),this.showFeedback(`Erreur : ${e}`,!0)}}async handleImportMd(){try{let e=await w({directory:!0,multiple:!1}),t=Array.isArray(e)?e[0]:e;if(!t)return;let n=await D(t),r=`${n.inserted} importée(s), ${n.skipped} ignorée(s).`;n.errors.length>0&&(r+=` ${n.errors.length} erreur(s).`),this.showFeedback(r,n.errors.length>0),this.refreshJournalList()}catch(e){console.error(`Import Markdown failed:`,e),this.showFeedback(`Erreur : ${e}`,!0)}}async handleBackupDb(){try{let e=await T({defaultPath:`psychly-backup.db`,filters:[{name:`SQLite Database`,extensions:[`db`]}]});if(!e)return;await O(e),this.showFeedback(`Sauvegarde créée.`)}catch(e){console.error(`DB backup failed:`,e),this.showFeedback(`Erreur : ${e}`,!0)}}async handleRestoreDb(){try{let e=await w({multiple:!1,filters:[{name:`SQLite Database`,extensions:[`db`]}]}),t=Array.isArray(e)?e[0]:e;if(!t)return;let n=await k(t),r=`${n.inserted} entrée(s) restaurée(s), ${n.skipped} ignorée(s).`;n.errors.length>0&&(r+=` ${n.errors.length} erreur(s).`),this.showFeedback(r,n.errors.length>0),this.refreshJournalList()}catch(e){console.error(`DB restore failed:`,e),this.showFeedback(`Erreur : ${e}`,!0)}}refreshJournalList(){let e=document.querySelector(`journal-list`);e?.loadEntries&&e.loadEntries()}};customElements.define(`export-dialog`,A);