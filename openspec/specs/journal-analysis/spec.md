## ADDED Requirements

### Requirement: Automatic journal entry analysis
The system SHALL analyze each journal entry after it is saved to extract emotional themes, cognitive patterns, and relevant therapeutic context.

#### Scenario: Entry is analyzed on save
- **WHEN** the user saves a new or edited journal entry
- **THEN** the system sends the entry to the LLM for analysis and stores the extracted context (emotional tone, key themes, identified patterns) alongside the entry

#### Scenario: Analysis runs locally
- **WHEN** the entry analysis is triggered
- **THEN** the analysis is performed entirely by the local LLM with no network request

### Requirement: Contextual enrichment for chat
The system SHALL use journal analysis results to enrich the therapeutic chat context, allowing the AI to reference patterns observed across entries.

#### Scenario: Chat references journal patterns
- **WHEN** the user starts a chat session
- **THEN** the AI has access to a summary of recent journal analyses (emotional trends, recurring themes, identified cognitive patterns) to personalize its responses

#### Scenario: AI identifies recurring patterns
- **WHEN** journal analyses across multiple entries reveal a recurring emotional pattern or cognitive distortion
- **THEN** the AI can reference this pattern in conversation (e.g., "I notice that over the past few entries, you often describe situations through a catastrophizing lens")

### Requirement: Emotional trend tracking
The system SHALL track emotional tone over time based on journal analyses.

#### Scenario: Emotional trends available to chat
- **WHEN** sufficient entries have been analyzed (at least 3)
- **THEN** the system provides a temporal summary of emotional trends to the chat context (e.g., dominant emotions per week)

### Requirement: Analysis stored per entry
The system SHALL store the analysis result linked to each journal entry in the local database.

#### Scenario: Analysis persists with entry
- **WHEN** the user restarts the application
- **THEN** all previously computed analyses are still available and linked to their respective entries

#### Scenario: Re-analysis on edit
- **WHEN** the user edits an existing journal entry
- **THEN** the system re-runs the analysis on the updated content and replaces the previous analysis result

### Requirement: Analysis does not block user interaction
The system SHALL perform journal analysis asynchronously without blocking the user interface.

#### Scenario: User can continue while analysis runs
- **WHEN** a journal entry is saved and analysis starts
- **THEN** the user can navigate away, create new entries, or open the chat without waiting for the analysis to complete

#### Scenario: Analysis completion notification
- **WHEN** the analysis of an entry completes
- **THEN** the system silently updates the stored analysis without interrupting the user
