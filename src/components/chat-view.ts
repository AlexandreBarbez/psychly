import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import {
  startChatSession,
  sendMessage,
  getChatSession,
  type SessionResponse,
  type MessageResponse,
  type ChatStreamEvent,
} from "../api/chat";

export class ChatView extends HTMLElement {
  private session: SessionResponse | null = null;
  private messages: MessageResponse[] = [];
  private isStreaming = false;
  private streamBuffer = "";
  private unlisten: UnlistenFn | null = null;

  static get observedAttributes() {
    return ["entry-id", "session-id"];
  }

  async attributeChangedCallback(name: string, _old: string, value: string) {
    if (name === "entry-id" && value) {
      await this.startFromEntry(value);
    } else if (name === "session-id" && value) {
      await this.loadSession(value);
    }
  }

  async connectedCallback() {
    this.render();
    this.setupStreamListener();

    const entryId = this.getAttribute("entry-id");
    const sessionId = this.getAttribute("session-id");

    if (sessionId) {
      await this.loadSession(sessionId);
    } else if (entryId) {
      await this.startFromEntry(entryId);
    } else {
      await this.startStandalone();
    }
  }

  async disconnectedCallback() {
    if (this.unlisten) {
      this.unlisten();
      this.unlisten = null;
    }
  }

  private async setupStreamListener() {
    this.unlisten = await listen<ChatStreamEvent>("chat-stream", (event) => {
      if (!this.session || event.payload.session_id !== this.session.id) return;

      if (event.payload.done) {
        this.isStreaming = false;
        this.updateInputState();
        return;
      }

      this.streamBuffer += event.payload.chunk;
      this.updateStreamingMessage();
    });
  }

  private async startFromEntry(entryId: string) {
    this.session = await startChatSession(entryId);
    this.messages = [];
    this.renderMessages();
  }

  private async startStandalone() {
    this.session = await startChatSession();
    this.messages = [];
    this.renderMessages();
  }

  private async loadSession(sessionId: string) {
    const [session, messages] = await getChatSession(sessionId);
    this.session = session;
    this.messages = messages;
    this.renderMessages();

    // Read-only if loading an existing session
    const input = this.querySelector<HTMLTextAreaElement>("#chat-input");
    const sendBtn = this.querySelector<HTMLButtonElement>("#send-btn");
    if (input) input.disabled = true;
    if (sendBtn) sendBtn.disabled = true;
  }

  private async handleSend() {
    const input = this.querySelector<HTMLTextAreaElement>("#chat-input");
    if (!input || !this.session || this.isStreaming) return;

    const content = input.value.trim();
    if (!content) return;

    input.value = "";
    this.isStreaming = true;
    this.streamBuffer = "";
    this.updateInputState();

    // Show user message immediately
    this.messages.push({
      id: "",
      session_id: this.session.id,
      role: "user",
      content,
      created_at: new Date().toISOString(),
    });
    this.renderMessages();

    // Add placeholder for streaming response
    this.messages.push({
      id: "",
      session_id: this.session.id,
      role: "assistant",
      content: "",
      created_at: new Date().toISOString(),
    });
    this.renderMessages();

    try {
      const response = await sendMessage(
        this.session.id,
        content
      );
      // Replace placeholder with final response
      this.messages[this.messages.length - 1] = response;
      this.renderMessages();
    } catch (e) {
      // Replace placeholder with error
      this.messages[this.messages.length - 1].content = `Erreur : ${e}`;
      this.renderMessages();
    }

    this.isStreaming = false;
    this.updateInputState();
  }

  private updateStreamingMessage() {
    const thread = this.querySelector("#chat-thread");
    if (!thread) return;

    const lastBubble = thread.querySelector(".chat-bubble:last-child .bubble-content");
    if (lastBubble) {
      lastBubble.textContent = this.streamBuffer;
      thread.scrollTop = thread.scrollHeight;
    }
  }

  private updateInputState() {
    const input = this.querySelector<HTMLTextAreaElement>("#chat-input");
    const sendBtn = this.querySelector<HTMLButtonElement>("#send-btn");
    if (input) input.disabled = this.isStreaming;
    if (sendBtn) sendBtn.disabled = this.isStreaming;
  }

  private render() {
    this.innerHTML = `
      <div class="chat-view">
        <div class="chat-header">
          <button class="btn-back" id="chat-back-btn">← Retour</button>
          <h2>Conversation</h2>
        </div>
        <div class="chat-thread" id="chat-thread"></div>
        <div class="chat-input-area">
          <textarea id="chat-input" placeholder="Écrivez votre message..." rows="2"></textarea>
          <button id="send-btn" class="btn-primary">Envoyer</button>
        </div>
      </div>
    `;

    this.querySelector("#chat-back-btn")?.addEventListener("click", () => {
      this.dispatchEvent(
        new CustomEvent("navigate", { detail: { view: "list" }, bubbles: true })
      );
    });

    this.querySelector("#send-btn")?.addEventListener("click", () => {
      this.handleSend();
    });

    this.querySelector("#chat-input")?.addEventListener("keydown", (e) => {
      const ke = e as KeyboardEvent;
      if (ke.key === "Enter" && !ke.shiftKey) {
        ke.preventDefault();
        this.handleSend();
      }
    });
  }

  private renderMessages() {
    const thread = this.querySelector("#chat-thread");
    if (!thread) return;

    thread.innerHTML = this.messages
      .map((msg) => {
        const isUser = msg.role === "user";
        const roleLabel = isUser ? "Vous" : "Psychly";
        return `
          <div class="chat-bubble ${isUser ? "bubble-user" : "bubble-assistant"}">
            <span class="bubble-role">${this.escapeHtml(roleLabel)}</span>
            <div class="bubble-content">${this.escapeHtml(msg.content)}</div>
          </div>
        `;
      })
      .join("");

    thread.scrollTop = thread.scrollHeight;
  }

  private escapeHtml(text: string): string {
    const div = document.createElement("div");
    div.textContent = text;
    return div.innerHTML.replace(/\n/g, "<br>");
  }
}

customElements.define("chat-view", ChatView);
