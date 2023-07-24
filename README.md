# Tauri Global State Management

A Tauri application with global state management. The local state for individual frontend windows is managed using Zustand, while the global state is managed and synced across all frontend windows using the Rust backend.

## Project Structure

```bash
Project Root
├── scripts
│   └── generate-macros.py
├── src
│   ├── assets
│   ├── constants
│   │   ├── events.ts
│   │   ├── global.ts
│   │   └── index.ts
│   ├── states
│   │   └── global-state.ts
│   ├── types
│   │   └── state.ts
│   ├── App.tsx
│   ├── App.css
│   ├── main.tsx
│   ├── styles.css
│   └── vite-env.d.ts
├── src-tauri
│   ├── src
│   │   ├── autogen
│   │   │   └── constants.rs
│   │   ├── states
│   │   │   └── states.rs
│   │   └── main.rs
│   └── ... other files and folders
└── ... other files and folders
```

## Installation

Clone the repository:

```bash
git clone https://github.com/robosushie/tauri-global-state-management.git
```

Navigate into the project directory:

```bash
cd tauri-global-state-management
```

Install the dependencies:

```bash
pnpm install
```

Generate the Rust constants and enums:

```bash
pnpm run autogen
```

## Usage

Start the Tauri application in development mode:

```bash
pnpm run dev
```

You can interact with the frontend by clicking the increment, decrement, and toggle buttons. The global state should be updated and synced across all frontend windows.

## Future Roadmap

- [ ] Add a STATE_FETCH event to fetch the state when different frontend windows come up at different times. This will also be useful when a webview window comes up after a crash.

- [ ] Convert this into a single plugin with most of the backend state management code written with autogeneration.

## License

This project is licensed under the MIT License.
