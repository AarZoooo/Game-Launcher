<script lang="ts">
import { browser } from "$app/environment";
import { page } from "$app/stores";
import Loader from "$lib/components/common/Loader.svelte";
import Sidebar from "$lib/components/layout/Sidebar.svelte";
import { footerColumns, routeAccents } from "$lib/data/navigation";
import { continuePlayingGames, getGameById } from "$lib/stores/libraryStore";
import {
	effectiveUIMode,
	libraryBusy,
	libraryBusyMessage,
	themeMode,
} from "$lib/stores/uiStore";
import { resolveAccentPresentation } from "$lib/utils/accent";

$: accentSource = pageToAccentSource($page.url.pathname, $page.params.id);
$: accentPresentation = resolveAccentPresentation(accentSource);
$: sidebarAccentSource = $continuePlayingGames[0] || accentSource;
$: sidebarAccentPresentation = resolveAccentPresentation(sidebarAccentSource);
$: activeTheme = $page.url.pathname.startsWith("/game/")
	? "dynamic"
	: $themeMode;

$: if (browser) {
	document.body.dataset.theme = activeTheme;
	document.body.classList.toggle("gaming-mode", $effectiveUIMode === "gaming");
	document.body.style.setProperty("--accent-rgb", accentPresentation.rgb);
	document.body.style.setProperty("--accent-contrast", accentPresentation.text);
	document.body.style.setProperty(
		"--sidebar-active-rgb",
		sidebarAccentPresentation.rgb,
	);
	document.body.style.setProperty(
		"--sidebar-active-contrast",
		sidebarAccentPresentation.text,
	);
}

function pageToAccentSource(pathname: string, gameId?: string) {
	if (pathname.startsWith("/game/") && gameId) {
		return getGameById(gameId) || { accent: "silver" as const };
	}

	return { accent: routeAccents[pathname] || "silver" };
}
</script>

<div class="shell" data-mode={$effectiveUIMode}>
  <Sidebar />

  <div class="frame">
    <main>
      <slot />
    </main>

    <footer>
      {#each footerColumns as column}
        <div class="column">
          <span>{column.title}</span>
          {#each column.links as link}
            <a href="/">{link}</a>
          {/each}
        </div>
      {/each}
    </footer>
  </div>
</div>

<Loader loading={$libraryBusy} message={$libraryBusyMessage} size="lg" />

<style>
  .shell {
    --shell-sidebar-width: clamp(13.5rem, 8rem + 12vw, 22rem);
    display: grid;
    grid-template-columns: var(--shell-sidebar-width) minmax(0, 1fr);
    min-height: 100vh;
    background: transparent;
  }

  .shell[data-mode='gaming'] {
    --surface-shadow: var(--shadow-sm);
    --ui-blur: 0px;
    --motion-fast: 90ms;
    --motion-base: 120ms;
  }

  .frame {
    display: flex;
    flex-direction: column;
    min-width: 0;
    background: transparent;
    position: relative;
    z-index: 0;
  }

  main {
    flex: 1;
    padding: var(--page-padding-y) var(--page-padding-x) var(--space-5);
    animation: rise var(--motion-base) ease;
  }

  footer {
    display: grid;
    grid-template-columns: repeat(4, minmax(0, 1fr));
    gap: var(--space-4);
    padding: var(--space-4) var(--page-padding-x) var(--space-6);
    border-top: 1px solid var(--surface-border-soft);
    color: var(--text-muted);
    font-size: var(--font-size-caption);
  }

  .column {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
  }

  .column span {
    color: var(--text-secondary);
  }

  :global(body.gaming-mode *) {
    scrollbar-color: var(--text-muted) transparent;
  }

  @keyframes rise {
    from {
      opacity: 0;
      transform: translateY(var(--space-3));
    }

    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  @media (max-width: 920px) {
    .shell {
      --shell-sidebar-width: 0rem;
      grid-template-columns: 1fr;
    }

    main {
      padding: var(--page-padding-y) var(--page-padding-x) var(--space-4);
    }

    footer {
      grid-template-columns: repeat(2, minmax(0, 1fr));
      padding: var(--space-4);
    }
  }
</style>
