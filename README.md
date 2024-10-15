# Loop - Directory Command Execution Tool

Loop is a command-line tool that allows you to execute commands across multiple directories simultaneously. It provides various options for customization and control over the execution process.

## Supported Behavior

### Command-Line Interface

Loop uses the following command-line interface:

```
loop [OPTIONS] [COMMAND]...
```

#### Options:

- `-c, --config <FILE>`: Specify a custom configuration file path (default: `.looprc`)
- `-i, --include <DIRECTORIES>`: Specify directories to include (overrides config file)
- `-e, --exclude <DIRECTORIES>`: Specify directories to exclude (adds to config file exclusions)
- `-v, --verbose`: Enable verbose output
- `-s, --silent`: Enable silent mode (suppress all output)
- `--parallel`: Execute commands in parallel
- `--add-aliases-to-global-looprc`: Add shell aliases to the global `.looprc` file

### Configuration

Loop uses a configuration file (default: `.looprc`) in JSON format with the following structure:

```json
{
  "directories": ["dir1", "dir2", ...],
  "ignore": ["dir_to_ignore1", "dir_to_ignore2", ...],
  "verbose": false,
  "silent": false,
  "parallel": false,
  "add_aliases_to_global_looprc": false
}
```

### Functionality

1. **Directory Expansion**: Loop expands the specified directories and their subdirectories, excluding any that match the ignore patterns.

2. **Command Execution**: The specified command is executed in each of the expanded directories.

3. **Parallel Execution**: When the `--parallel` flag is set, commands are executed concurrently across directories.

4. **Alias Support**: Loop supports shell aliases, which can be defined in the global or local `.looprc` file.

5. **Output Control**: 
   - Verbose mode (`-v, --verbose`) provides additional execution details.
   - Silent mode (`-s, --silent`) suppresses all output except for the final summary.

6. **Error Handling**: Loop captures and reports on failed command executions, providing a summary at the end of the run.

7. **Global Alias Management**: The `--add-aliases-to-global-looprc` option allows users to add their shell aliases to the global `.looprc` file.

### Execution Flow

1. Parse command-line arguments
2. Load configuration (from file or defaults)
3. Expand directories based on configuration and CLI options
4. Load aliases from global and local `.looprc` files
5. Execute the specified command in each directory (sequentially or in parallel)
6. Collect and summarize results
7. Display summary and any error details

## Error Handling

Loop provides error handling for various scenarios:
- Configuration file parsing errors
- Directory access errors
- Command execution failures

Errors are reported with context to help users identify and resolve issues.

## Limitations

- Loop currently does not support Windows-specific path separators or commands.
- The tool assumes a Unix-like environment for shell operations and alias handling.
