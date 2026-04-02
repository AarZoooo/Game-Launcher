# Media Resolution Flow

The backend owns all media resolution for installed games.

Resolution order:

1. Use cached media already stored on the `games` row.
2. Try a confident IGDB match using normalized title matching.
3. Fall back to images found near the local executable directory.
4. Fall back to generated SVG placeholders.

The frontend only receives final values for:

- `coverVertical`
- `coverHorizontal`
- `banner`
- `icon`
- `accentColor`

Resolved values are cached back into SQLite to avoid repeated API lookups and directory scans.
