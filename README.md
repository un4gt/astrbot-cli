# AstrBot CLI

A command-line interface for managing AstrBot instances with enhanced authentication debugging and persistent credential storage.

## Features

- 🔐 **Persistent Authentication**: Credentials are saved to a config file and persist across command executions
- 🔍 **Comprehensive Debugging**: Detailed authentication status and error diagnostics
- 📢 **Verbose Output Control**: Use `--verbose` flag for detailed logs and comprehensive information
- 🛡️ **Robust JSON Parsing**: Focuses on core fields (name, version, activated) and gracefully handles unexpected or missing fields
- 🌐 **Plugin Management**: List, install, enable/disable plugins
- 📊 **Status Monitoring**: Real-time authentication and connection status
- 🛠️ **Troubleshooting Tools**: Built-in diagnostics for common authentication issues

## Installation

```bash
cargo build --release
```

## Quick Start

1. **Login to your AstrBot instance:**
   ```bash
   cargo run -- login -u your_username -p your_password -s http://your-server:port
   ```

2. **Check authentication status:**
   ```bash
   cargo run -- status
   ```

3. **List installed plugins:**
   ```bash
   # Minimal output (default)
   cargo run -- plugin get
   
   # Verbose output with detailed logs
   cargo run -- --verbose plugin get
   ```

## Verbose Output

All commands support the `--verbose` flag for detailed output and comprehensive logging:

### Normal Mode (Default)
```
Fetching plugin list...
✅ Authentication validated
✅ Found 6 plugin(s):

✅ thinking_filter (1.0.0)
✅ astrbot-reminder (0.0.1)
✅ session_controller (v1.0.1)
```

### Verbose Mode (`--verbose` flag)
```
ℹ️  [INFO] Fetching plugin list...
✅ [SUCCESS] Authentication validated
🔍 [VERBOSE] Server: http://your-server.com
🐛 [DEBUG] Token: eyJhbGci...
🔍 [VERBOSE] Starting plugin retrieval process
🔍 [VERBOSE] JSON parsing successful: 6 plugins found

Name                      Version      Status
--------------------------------------------------
thinking_filter           1.0.0        ✅ Active
astrbot-reminder          0.0.1        ✅ Active
session_controller        v1.0.1       ✅ Active
```

**Benefits of Verbose Mode:**
- 📊 Detailed HTTP request/response information
- 🔍 JSON parsing progress and statistics
- 🐛 Debug information including tokens (truncated for security)
- 📈 Step-by-step process tracking
- 🚨 Enhanced error diagnostics

## Commands

### Authentication Commands

#### `login`
Authenticate with an AstrBot server and save credentials.

```bash
cargo run -- login -u USERNAME -p PASSWORD -s SERVER_URL
```

**Options:**
- `-u, --username`: Your AstrBot username
- `-p, --password`: Your AstrBot password  
- `-s, --server`: Server URL (e.g., `http://localhost:6185`)

**Example:**
```bash
cargo run -- login -u admin -p mypassword -s http://localhost:6185
```

#### `status`
Display comprehensive authentication status and debugging information.

```bash
cargo run -- status
```

This command shows:
- Environment variable status
- Config file location and contents
- Effective credentials being used
- Troubleshooting recommendations

#### `logout`
Clear stored credentials from the config file.

```bash
cargo run -- logout
```

### Plugin Management Commands

#### `plugin get`
List all installed plugins with their status.

```bash
# Minimal output
cargo run -- plugin get

# Detailed output with verbose information
cargo run -- --verbose plugin get
```

**Output Modes:**
- **Normal**: Clean, minimal output showing essential plugin information
- **Verbose**: Comprehensive table with detailed parsing logs and HTTP request information

#### `plugin install` (Coming Soon)
Install plugins from local path or git repository.

```bash
cargo run -- plugin install --from-git https://github.com/user/plugin.git
cargo run -- plugin install --from-local /path/to/plugin
```

#### `plugin on/off` (Coming Soon)
Enable or disable specific plugins.

```bash
cargo run -- plugin on plugin_name
cargo run -- plugin off plugin_name
```

## Robust JSON Parsing

The CLI implements fault-tolerant JSON parsing designed to handle real-world API responses:

### Core Features

- **Focuses on Essential Fields**: Extracts only the three core fields (name, version, activated)
- **Ignores Extra Fields**: Gracefully handles unexpected fields like `online_version`, `handlers`, etc.
- **Provides Fallback Values**: Uses sensible defaults for missing fields
- **Continues on Errors**: Processes remaining plugins even if some fail to parse
- **Supports Multiple Formats**: Handles various JSON structures automatically

### Supported JSON Formats

```json
// Direct array
[{"name": "plugin1", "version": "1.0.0", "activated": true}]

// API wrapper (standard AstrBot response)
{
  "status": "ok",
  "data": [{"name": "plugin1", "version": "1.0.0", "activated": true}]
}

// Single plugin object
{"name": "plugin1", "version": "1.0.0", "activated": true, "extra_field": "ignored"}

// Minimal plugin (with automatic defaults)
{"name": "plugin1"}  // version → "0.0.0", activated → false
```

### Boolean Field Flexibility

The `activated` field supports multiple representations:
- **Boolean**: `true`, `false`
- **String**: `"true"`, `"false"`, `"1"`, `"0"`, `"yes"`, `"no"`, `"on"`, `"off"`, `"enabled"`, `"disabled"`
- **Number**: `1` (true), `0` (false), any non-zero number (true)

### Error Handling

- **Partial Failures**: If some plugins fail to parse, the CLI continues processing others
- **Verbose Logging**: Use `--verbose` to see detailed parsing information
- **Graceful Degradation**: Missing or malformed fields use sensible defaults

## Authentication System

### How It Works

The CLI uses a dual-layer authentication system:

1. **Environment Variables** (temporary, current session only)
   - `ASTRBOT_TOKEN`: Authentication token
   - `ASTRBOT_SERVER_URL`: Server URL

2. **Config File** (persistent across sessions)
   - Location: `~/.astrbot/credentials.json`
   - Contains: token, server URL, username, creation timestamp

### Priority Order

1. Environment variables (if both token and server URL are set)
2. Config file (fallback for persistent storage)

### Why Environment Variables Don't Persist

When you run `cargo run login` and then `cargo run plugin get` as separate commands, they execute as separate processes. Environment variables set in the first process don't carry over to the second process. This is why the CLI now uses a persistent config file.

## Troubleshooting

### Common Issues

#### "No authentication token found"

**Symptoms:**
- Login appears successful but subsequent commands fail
- Error: "No authentication token found"

**Causes:**
- Running commands as separate `cargo run` executions
- Environment variables don't persist between processes

**Solutions:**
1. **Use the config file system** (recommended):
   ```bash
   cargo run -- login -u username -p password -s server_url
   cargo run -- plugin get  # This will now work
   ```

2. **Check authentication status:**
   ```bash
   cargo run -- status
   ```

3. **Clear and re-login if needed:**
   ```bash
   cargo run -- logout
   cargo run -- login -u username -p password -s server_url
   ```

#### "Failed to connect to server"

**Symptoms:**
- Network errors during API calls
- Connection timeouts

**Solutions:**
1. **Verify server URL:**
   ```bash
   cargo run -- status  # Check current server URL
   ```

2. **Test server accessibility:**
   - Open the server URL in a web browser
   - Ensure the server is running
   - Check firewall settings

3. **Re-login with correct URL:**
   ```bash
   cargo run -- login -u username -p password -s http://correct-server:port
   ```

#### "Invalid credentials"

**Symptoms:**
- Login fails with authentication error
- API calls return 401/403 errors

**Solutions:**
1. **Verify credentials:**
   - Double-check username and password
   - Ensure account exists and is active

2. **Clear old credentials:**
   ```bash
   cargo run -- logout
   cargo run -- login -u username -p password -s server_url
   ```

### Debug Information

Use the `status` command to get comprehensive debugging information:

```bash
cargo run -- status
```

This shows:
- ✅/❌ Environment variable status
- 📁 Config file location and contents
- 🎯 Effective credentials being used
- 💡 Specific recommendations for your situation

### Advanced Troubleshooting

#### Manual Config File Inspection

Config file location: `~/.astrbot/credentials.json`

```json
{
  "token": "your_auth_token_here",
  "server_url": "http://your-server:port",
  "username": "your_username",
  "created_at": "2024-01-01T12:00:00Z"
}
```

#### Environment Variable Debugging

Check current environment variables:
```bash
# Windows
set | findstr ASTRBOT

# Linux/Mac
env | grep ASTRBOT
```

## Development

### Project Structure

```
src/
├── main.rs          # Entry point and command routing
├── cli.rs           # Command-line interface definitions (with --verbose flag)
├── config.rs        # Persistent credential storage and status
├── login.rs         # Authentication handling
├── plugin.rs        # Plugin management commands
├── api.rs           # HTTP API client with robust JSON parsing
├── verbose.rs       # Verbose output control system
└── robust_plugin.rs # Fault-tolerant JSON parsing for plugin data
```

### Key Components

- **ConfigManager**: Handles persistent credential storage
- **AuthStatus**: Comprehensive authentication state tracking
- **Verbose Output System**: Categorized logging with different verbosity levels
- **RobustPlugin**: Fault-tolerant JSON deserialization focusing on core fields
- **Enhanced Error Handling**: Detailed diagnostics for common issues
- **Dual Authentication**: Environment variables + config file

### Building

```bash
# Development build
cargo build

# Release build
cargo build --release

# Run tests (including robust JSON parsing tests)
cargo test

# Run with logging
RUST_LOG=debug cargo run -- status

# Test verbose output
cargo run -- --verbose plugin get
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

[Add your license information here]