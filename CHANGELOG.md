# Changelog

All changes to this project will be noted in this file.

## Unreleased

### Additions

- Better markdown support
- Account settings
  - Delete all notes (`/delete/notes`)
  - Delete account (`/delete/account`)
- Added `dev/prod` mode

### Fixes

- Fix incorrect HTML generation from Markdown
- Fix cookie removal issues
- Use `SameSite=Lax` to avoid getting logged out when accessing from other sites

### Breaking

- `/createnote` is now `/create/note`

## 0.1.0

This is the initial release of Jotsy

### Additions

- Markdown support
- Authentication
- Multi-user support + Sessions
