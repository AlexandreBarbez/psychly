## ADDED Requirements

### Requirement: Local LLM execution via Ollama
The system SHALL use Ollama as the local LLM runtime, communicating via its HTTP API on localhost.

#### Scenario: LLM responds without internet
- **WHEN** the application sends a prompt to the LLM
- **THEN** the LLM returns a response using the locally stored Qwen 2.5 14B model

#### Scenario: Ollama starts with the application
- **WHEN** the user launches the application
- **THEN** the application ensures Ollama is running locally before accepting chat or analysis requests

#### Scenario: Ollama not available
- **WHEN** the application detects that Ollama is not running and cannot be started
- **THEN** the system displays a clear error message indicating the LLM runtime is unavailable and disables chat and analysis features

### Requirement: No network dependency
The system SHALL operate entirely without network access. No feature SHALL require an active internet connection.

#### Scenario: Full functionality offline
- **WHEN** the machine is completely disconnected from any network
- **THEN** all features (journal CRUD, chat, analysis) function normally

#### Scenario: No outbound requests
- **WHEN** the application is running
- **THEN** the application makes no HTTP requests to any host other than localhost

### Requirement: Local SQLite storage
The system SHALL persist all data (journal entries, chat sessions, analyses) in a local SQLite database with no external database server.

#### Scenario: Single database file
- **WHEN** the application writes data
- **THEN** all data is stored in a single SQLite file within the application's data directory

#### Scenario: No database server required
- **WHEN** the application starts
- **THEN** no external database server or daemon is required — SQLite is embedded

### Requirement: Portable directory structure
The system SHALL be structured as a self-contained directory that can be copied to external media and run on another compatible Mac without installation.

#### Scenario: USB portability
- **WHEN** the user copies the entire application directory to a USB drive and opens it on another Mac with Apple Silicon
- **THEN** the application runs without requiring installation, configuration, or internet access

#### Scenario: Relative paths only
- **WHEN** the application resolves paths to data, models, or configuration
- **THEN** all paths are relative to the application root directory — no hardcoded absolute paths

#### Scenario: Model files travel with the application
- **WHEN** the application directory is copied
- **THEN** the LLM model files (GGUF) are included in the directory and Ollama uses them from the portable location via OLLAMA_MODELS environment variable

### Requirement: Application launcher script
The system SHALL provide a launcher script that starts all required services and the application.

#### Scenario: Single command launch
- **WHEN** the user runs the launcher script (start.sh)
- **THEN** the script starts Ollama with portable configuration, waits for it to be ready, then launches the Tauri application

#### Scenario: Clean shutdown
- **WHEN** the user closes the application
- **THEN** the launcher ensures Ollama is also stopped to free system resources

### Requirement: macOS Apple Silicon compatibility
The system SHALL run natively on macOS with Apple Silicon (M-series) processors, leveraging Metal for LLM inference acceleration.

#### Scenario: Native ARM execution
- **WHEN** the application and Ollama binaries are executed
- **THEN** they run as native ARM64 binaries without Rosetta translation

#### Scenario: Metal GPU acceleration for LLM
- **WHEN** Ollama runs the Qwen 2.5 14B model
- **THEN** inference uses Metal GPU acceleration for optimal performance on Apple Silicon
