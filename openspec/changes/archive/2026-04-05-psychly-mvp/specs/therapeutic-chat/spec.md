## ADDED Requirements

### Requirement: Start a therapeutic chat session
The system SHALL allow the user to initiate a chat conversation with the AI therapist from a journal entry or from the main interface.

#### Scenario: User starts chat from a journal entry
- **WHEN** the user selects "Chat" from a journal entry
- **THEN** the system opens a chat session with the entry content preloaded as context for the AI

#### Scenario: User starts a standalone chat session
- **WHEN** the user opens the chat without selecting a journal entry
- **THEN** the system opens a chat session using the user's recent journal history as general context

### Requirement: Conversational exchange with AI therapist
The system SHALL support a turn-by-turn conversational exchange where the user sends messages and the AI responds in a therapeutic manner.

#### Scenario: User sends a message
- **WHEN** the user types a message and sends it
- **THEN** the AI generates a response using the therapeutic framework and conversation history

#### Scenario: Response streams in real-time
- **WHEN** the AI generates a response
- **THEN** the response text appears progressively (streaming) in the chat interface rather than waiting for the full response

### Requirement: Therapeutic posture and framework
The AI therapist SHALL adopt a clinical psychology posture grounded in recognized therapeutic frameworks: ACT, CBT, DBT, Schema Therapy, Mindfulness, attachment theory, cognitive distortions, emotional regulation, defense mechanisms, mentalization, and exposure/avoidance concepts.

#### Scenario: AI uses therapeutic techniques
- **WHEN** the user describes an emotional difficulty or thought pattern
- **THEN** the AI responds using relevant therapeutic concepts (e.g., identifying cognitive distortions, proposing acceptance-based exercises, exploring schemas) without being prescriptive or diagnosing

#### Scenario: AI maintains empathetic alliance
- **WHEN** the user shares sensitive or emotionally charged content
- **THEN** the AI responds with empathy, validates the user's experience, and maintains a supportive therapeutic alliance before introducing any analytical framework

#### Scenario: AI maintains empathetic alliance
- **WHEN** the user seems to be needing it
- **THEN** the AI ​​is straightforward and doesn't hesitate to contradict the user if he's wrong. The goal isn't to agree with the user at all costs, but to help them gain perspective, especially if they seem to be mistaken.

### Requirement: Conversation history within a session
The system SHALL maintain the full conversation history within a chat session to provide coherent multi-turn dialogue.

#### Scenario: AI references earlier messages
- **WHEN** the user refers to something mentioned earlier in the conversation
- **THEN** the AI has access to the full session history and responds coherently

#### Scenario: Session context window management
- **WHEN** the conversation exceeds the model's context window
- **THEN** the system summarizes older messages to maintain coherence while respecting the context limit

### Requirement: Persist chat sessions
The system SHALL persist chat sessions locally so the user can review past conversations.

#### Scenario: User views past sessions
- **WHEN** the user opens the chat history
- **THEN** the system displays a list of past sessions with date and a preview of the first exchange

#### Scenario: User reopens a past session
- **WHEN** the user selects a past chat session
- **THEN** the system displays the full conversation in read-only mode

### Requirement: AI responds in French
The AI therapist SHALL respond in French, matching the user's language.

#### Scenario: French language interaction
- **WHEN** the user writes a message in French
- **THEN** the AI responds in fluent, natural French with appropriate therapeutic vocabulary

### Requirement: Safety disclaimers
The system SHALL display a clear disclaimer that the AI is not a substitute for professional mental health care.

#### Scenario: Disclaimer at first use
- **WHEN** the user opens the chat for the first time
- **THEN** the system displays a disclaimer stating the AI is an assistive tool and not a licensed therapist, and recommends consulting a professional for serious concerns

#### Scenario: Crisis detection
- **WHEN** the user expresses content indicating a risk of self-harm or acute crisis
- **THEN** the AI responds with empathy, clearly recommends contacting emergency services or a mental health professional, and provides relevant French helpline numbers (e.g., 3114)
