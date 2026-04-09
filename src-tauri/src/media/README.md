# Media Resolution Flow

The backend owns all media resolution for installed games.

Resolution order:

1. Use cached media already stored on the `games` row.
2. Try a confident IGDB match using normalized title matching.
3. Fall back to images found inside the game's local install subtree.
4. Fall back to generated SVG placeholders.

The frontend only receives final values for:

- `coverVertical`
- `coverHorizontal`
- `banner`
- `icon`
- `accentColor`

Resolved values are cached back into SQLite to avoid repeated API lookups and directory scans.

Local file fallback notes:

- Search stays inside the executable directory and technical subfolders above it (for example `Binaries/Win64`) so sibling game folders are not mixed into one candidate pool.
- Generic asset names like `cover`, `poster`, `banner`, `hero`, `icon`, and `logo` are scored by role and image shape, not just title text.
- Supported local formats include `png`, `jpg`, `jpeg`, `webp`, and `bmp`.
- Placeholder-only results are retried on later reads and app launches so recovered API access can still replace them with real media.
