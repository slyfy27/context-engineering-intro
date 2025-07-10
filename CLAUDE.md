### 🔄 Project Awareness & Context
- **Always read `PLANNING.md`** at the start of a new conversation to understand the project's architecture, goals, style, and constraints.
- **Check `TASK.md`** before starting a new task. If the task isn't listed, add it with a brief description and today's date.
- **Use consistent naming conventions, file structure, and architecture patterns** as described in `PLANNING.md`.
- **Run `flutter pub get`** after adding new dependencies to pubspec.yaml.

### 🧱 Code Structure & Modularity
- **Never create a Dart file longer than 500 lines of code.** If a file approaches this limit, refactor by splitting it into separate widgets or service files.
- **Organize code into clearly separated modules**, grouped by feature or responsibility.
  For Flutter apps this looks like:
    - `lib/main.dart` - App entry point and main configuration
    - `lib/screens/` - Screen/page widgets
    - `lib/widgets/` - Reusable UI components
    - `lib/services/` - Business logic and API services
    - `lib/models/` - Data models and DTOs
    - `lib/providers/` - State management (Provider/Riverpod/Bloc)
    - `lib/utils/` - Helper functions and constants
- **Use clear, consistent imports** with package imports first, then relative imports.
- **Use environment variables** through flutter_dotenv or --dart-define for configuration.

### 🧪 Testing & Reliability
- **Always create tests for new features** using Flutter's testing framework.
- **After updating any logic**, check whether existing tests need to be updated. If so, do it.
- **Tests should live in the `/test` folder** mirroring the main lib structure.
  - Include at least:
    - Widget tests for UI components
    - Unit tests for business logic
    - Integration tests for critical user flows
- **Use `flutter test` to run all tests** and ensure they pass before committing.

### ✅ Task Completion
- **Mark completed tasks in `TASK.md`** immediately after finishing them.
- Add new sub-tasks or TODOs discovered during development to `TASK.md` under a "Discovered During Work" section.

### 📎 Style & Conventions
- **Use Dart** as the primary language with Flutter framework.
- **Follow Dart style guide** and use `dart format` for consistent formatting.
- **Use effective Dart conventions** including proper naming for classes, methods, and variables.
- **Use `flutter analyze` to check for lint errors** and fix them before committing.
- Write **documentation comments for public APIs** using Dart's documentation format:
  ```dart
  /// Brief summary of what this function does.
  ///
  /// More detailed description if needed.
  ///
  /// [param1] Description of parameter.
  /// Returns description of return value.
  String exampleFunction(String param1) {
    return 'result';
  }
  ```

### 📱 Flutter-Specific Guidelines
- **Use StatelessWidget when possible**, StatefulWidget only when state management is needed.
- **Prefer Provider/Riverpod/Bloc** for state management over setState for complex state.
- **Use const constructors** wherever possible for better performance.
- **Follow Material Design 3** principles for UI/UX consistency.
- **Test on multiple screen sizes** and ensure responsive design.
- **Use proper asset management** and include different resolutions for images.

### 📚 Documentation & Explainability
- **Update `README.md`** when new features are added, dependencies change, or setup steps are modified.
- **Comment complex Flutter widgets and logic** to ensure everything is understandable to a mid-level Flutter developer.
- When writing complex logic, **add inline comments** explaining the why, not just the what.

### 🧠 AI Behavior Rules
- **Never assume missing context. Ask questions if uncertain.**
- **Never hallucinate Flutter packages or APIs** – only use known, verified pub.dev packages.
- **Always confirm file paths and widget names** exist before referencing them in code or tests.
- **Never delete or overwrite existing code** unless explicitly instructed to or if part of a task from `TASK.md`.
- **Check Flutter version compatibility** for any packages or features used.