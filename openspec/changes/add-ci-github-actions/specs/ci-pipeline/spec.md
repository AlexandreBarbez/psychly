## ADDED Requirements

### Requirement: CI workflow triggers on pull requests and main branch pushes
The system SHALL execute the CI workflow automatically on every pull request (all branches) and on every push to the `main` branch.

#### Scenario: PR opened or updated triggers CI
- **WHEN** a pull request is opened, synchronized, or reopened
- **THEN** the CI workflow runs both the frontend and backend jobs

#### Scenario: Push to main triggers CI
- **WHEN** a commit is pushed to the `main` branch
- **THEN** the CI workflow runs both the frontend and backend jobs

---

### Requirement: Frontend TypeScript compilation is validated
The CI pipeline SHALL compile the TypeScript frontend using `tsc && vite build` and fail if any type errors or build errors are detected.

#### Scenario: TypeScript compiles successfully
- **WHEN** the frontend job runs and there are no type errors or build errors
- **THEN** the job succeeds and reports a passing status

#### Scenario: TypeScript compilation fails
- **WHEN** the frontend job runs and a type error or build error is present
- **THEN** the job fails and blocks the PR from being merged

---

### Requirement: Rust code compiles and tests pass
The CI pipeline SHALL compile the Rust crate in `src-tauri/` and run all unit tests via `cargo test`, failing if compilation or any test fails.

#### Scenario: Rust builds and tests pass
- **WHEN** the backend job runs and all Rust code compiles and all tests pass
- **THEN** the job succeeds and reports a passing status

#### Scenario: Rust compilation fails
- **WHEN** the backend job runs and the Rust code does not compile
- **THEN** the job fails and blocks the PR from being merged

#### Scenario: A Rust unit test fails
- **WHEN** the backend job runs and one or more `cargo test` tests fail
- **THEN** the job fails and blocks the PR from being merged

---

### Requirement: Frontend and backend jobs run in parallel
The CI workflow SHALL execute the frontend and backend jobs concurrently, without sequential dependency between them.

#### Scenario: Both jobs run at the same time
- **WHEN** the CI workflow is triggered
- **THEN** the frontend and backend jobs start simultaneously and do not wait on each other

---

### Requirement: Node.js toolchain uses pnpm
The frontend job SHALL install dependencies using `pnpm`, with the exact version specified via `pnpm/action-setup`.

#### Scenario: Dependencies installed with pnpm
- **WHEN** the frontend job sets up the Node.js environment
- **THEN** `pnpm install` is used to install dependencies (not `npm install` or `yarn install`)

---

### Requirement: Rust toolchain is pinned to the minimum required version
The backend job SHALL use Rust `1.77.2` (the version declared in `Cargo.toml` as `rust-version`).

#### Scenario: Rust version matches project requirement
- **WHEN** the backend job sets up the Rust toolchain
- **THEN** the Rust version used is `1.77.2`

---

### Requirement: Rust build artifacts are cached between runs
The backend job SHALL cache Cargo dependencies and build artifacts to reduce CI execution time on subsequent runs.

#### Scenario: Cache hit reduces build time
- **WHEN** the backend job runs and a Cargo cache exists from a previous run
- **THEN** previously compiled dependencies are restored from cache, skipping recompilation
