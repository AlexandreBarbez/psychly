## ADDED Requirements

### Requirement: Select recent journal entries for chat context
The system SHALL automatically retrieve the most recent journal entries to include their content in the therapeutic chat context.

#### Scenario: Recent entries are available
- **WHEN** the user sends a message in a chat session and journal entries exist
- **THEN** the system retrieves the 5 most recent journal entries sorted by creation date descending

#### Scenario: No journal entries exist
- **WHEN** the user sends a message in a chat session and no journal entries exist
- **THEN** the system proceeds without journal entry content in the context

#### Scenario: Fewer entries than the limit
- **WHEN** the user sends a message and fewer than 5 journal entries exist
- **THEN** the system retrieves all available entries

### Requirement: Select thematically relevant journal entries
The system SHALL complement recent entries with older entries whose analysis themes match the current conversation context.

#### Scenario: Thematic match found
- **WHEN** the system builds the chat context and analyses of older entries share themes with the recent entries' analyses
- **THEN** those older entries are added to the context alongside the recent entries, deduplicated by entry ID

#### Scenario: No thematic match
- **WHEN** no older entries have matching themes
- **THEN** only the recent entries are included in the context

#### Scenario: Theme matching uses existing analyses
- **WHEN** the system performs thematic matching
- **THEN** it SHALL use the themes already extracted and stored by the journal analysis system, without performing new LLM inference

### Requirement: Enforce a token budget for journal content
The system SHALL limit the total token count of journal entry content injected into the chat context to prevent exceeding the LLM context window.

#### Scenario: Total content fits within budget
- **WHEN** the combined content of selected entries fits within the journal content token budget (3 000 tokens)
- **THEN** the full content of all selected entries is included

#### Scenario: Total content exceeds budget
- **WHEN** the combined content of selected entries exceeds the journal content token budget
- **THEN** the system truncates individual entries, prioritizing recent entries over thematic matches, and appends a `[...]` marker to truncated entries

#### Scenario: Single entry exceeds its share of the budget
- **WHEN** one entry's content exceeds its allocated share of the token budget (budget / number of entries)
- **THEN** that entry is truncated to its allocated share, preserving the beginning of the text

### Requirement: Format journal entries for LLM injection
The system SHALL format the selected journal entries into a structured text block suitable for injection into the LLM system prompt.

#### Scenario: Entries formatted with date and content
- **WHEN** the system prepares journal content for the prompt
- **THEN** each entry is formatted with its creation date as a header followed by its (potentially truncated) body text

#### Scenario: Entries ordered chronologically
- **WHEN** multiple entries are included in the context
- **THEN** they are ordered from oldest to most recent so the LLM reads them in chronological order

### Requirement: Retrieve journal entries by ID
The system SHALL support retrieving multiple journal entries by their IDs in a single operation, for use by the thematic selection pass.

#### Scenario: Retrieve entries by list of IDs
- **WHEN** the system requests entries by a list of IDs
- **THEN** it returns all entries matching those IDs in a single database query

#### Scenario: Some IDs do not match
- **WHEN** the system requests entries by IDs and some IDs have no matching entry
- **THEN** it returns only the entries that exist, without error

### Requirement: Retrieve recent journal entries
The system SHALL support retrieving the N most recent journal entries ordered by creation date descending.

#### Scenario: Retrieve recent entries
- **WHEN** the system requests the N most recent entries
- **THEN** it returns up to N entries sorted by creation date descending
