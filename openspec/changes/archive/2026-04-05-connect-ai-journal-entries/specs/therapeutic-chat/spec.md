## MODIFIED Requirements

### Requirement: Start a therapeutic chat session
The system SHALL allow the user to initiate a chat conversation with the AI therapist from a journal entry or from the main interface.

#### Scenario: User starts chat from a journal entry
- **WHEN** the user selects "Chat" from a journal entry
- **THEN** the system opens a chat session and the backend automatically includes that entry plus other recent and thematically relevant entries in the AI context

#### Scenario: User starts a standalone chat session
- **WHEN** the user opens the chat without selecting a journal entry
- **THEN** the system opens a chat session and the backend automatically includes recent journal entries and thematically relevant entries in the AI context

### Requirement: Conversational exchange with AI therapist
The system SHALL support a turn-by-turn conversational exchange where the user sends messages and the AI responds in a therapeutic manner.

#### Scenario: User sends a message
- **WHEN** the user types a message and sends it
- **THEN** the backend assembles the prompt with automatically retrieved journal entry content, the therapeutic framework, and conversation history to generate a response

#### Scenario: Response streams in real-time
- **WHEN** the AI generates a response
- **THEN** the response text appears progressively (streaming) in the chat interface rather than waiting for the full response

#### Scenario: AI references specific journal content
- **WHEN** the user discusses a topic that relates to content in their journal entries
- **THEN** the AI MAY reference specific details from journal entries in context (e.g., "Tu mentionnais le [date] que...")

### Requirement: Conversation history within a session
The system SHALL maintain the full conversation history within a chat session to provide coherent multi-turn dialogue.

#### Scenario: AI references earlier messages
- **WHEN** the user refers to something mentioned earlier in the conversation
- **THEN** the AI has access to the full session history and responds coherently

#### Scenario: Session context window management
- **WHEN** the conversation exceeds the model's context window
- **THEN** the system summarizes older messages to maintain coherence while respecting the context limit, with the journal content budget preserved separately from the conversation history budget
