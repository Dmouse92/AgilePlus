# AgilePlus Frontend

Monorepo for AgilePlus web and desktop applications, built with Turborepo.

**Apps:** `web`, `desktop`, `docs` (VitePress), `storybook`
**Packages:** `ui`, `api-client`, `state`, `types`, `config`, `env-manager`

Uses Bun as the package manager (`bun.lock`).

```bash
bun install          # install dependencies
bun run dev          # start dev servers
bun run build        # build all apps and packages
bun run lint         # lint all workspaces
```

See `turbo.json` for task orchestration and `tsconfig.packages.json` for shared TS config.
