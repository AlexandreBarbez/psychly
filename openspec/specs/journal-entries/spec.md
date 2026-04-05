## ADDED Requirements

### Requirement: Create journal entry
The system SHALL allow the user to create a new journal entry with a text body and an automatic date timestamp.

#### Scenario: User creates a new entry
- **WHEN** the user writes text in the journal editor and confirms the save
- **THEN** the system persists the entry with the text body, creation date, and a unique identifier

#### Scenario: Entry is timestamped automatically
- **WHEN** a new entry is created
- **THEN** the system assigns the current local date and time as the entry timestamp without requiring manual input

### Requirement: View journal entry list
The system SHALL display a chronological list of all journal entries, showing the date and a preview of each entry.

#### Scenario: User views entry list
- **WHEN** the user opens the journal view
- **THEN** the system displays all entries sorted by date (most recent first), each showing the date and the first lines of text as preview

#### Scenario: Empty journal
- **WHEN** the user opens the journal view with no entries
- **THEN** the system displays a message indicating the journal is empty and invites the user to create a first entry

### Requirement: Read full journal entry
The system SHALL allow the user to open and read the full content of any journal entry.

#### Scenario: User opens an entry
- **WHEN** the user selects an entry from the list
- **THEN** the system displays the full text content along with the entry date

### Requirement: Edit journal entry
The system SHALL allow the user to modify the text of an existing journal entry.

#### Scenario: User edits an entry
- **WHEN** the user modifies the text of an existing entry and confirms the save
- **THEN** the system persists the updated text and records the modification date while preserving the original creation date

### Requirement: Delete journal entry
The system SHALL allow the user to delete a journal entry with confirmation.

#### Scenario: User deletes an entry
- **WHEN** the user requests deletion of an entry and confirms the action
- **THEN** the system permanently removes the entry from storage

#### Scenario: User cancels deletion
- **WHEN** the user requests deletion but cancels the confirmation
- **THEN** the entry remains unchanged

### Requirement: Search journal entries
The system SHALL provide full-text search across all journal entries.

#### Scenario: User searches with matching results
- **WHEN** the user enters a search query
- **THEN** the system returns all entries containing the query text, ranked by relevance, with matching excerpts highlighted

#### Scenario: User searches with no results
- **WHEN** the user enters a search query that matches no entries
- **THEN** the system displays a message indicating no results were found

### Requirement: Persist journal entries locally
The system SHALL store all journal entries in a local SQLite database file with no network dependency.

#### Scenario: Entries survive application restart
- **WHEN** the user closes and reopens the application
- **THEN** all previously created entries are available and unchanged

#### Scenario: Database file is portable
- **WHEN** the data directory is copied to another Mac with the application
- **THEN** all entries are accessible without migration or reconfiguration
