<script lang="ts">
  import { page } from '$app/stores';
  import Loader from '$lib/components/common/Loader.svelte';
  import Sidebar from '$lib/components/layout/Sidebar.svelte';
  import { libraryBusy, libraryBusyMessage } from '$lib/stores/uiStore';
  import { footerColumns, routeAccents } from '$lib/utils/constants';
  import { getGameById } from '$lib/stores/libraryStore';

  $: accent = pageToAccent($page.url.pathname, $page.params.id);

  function pageToAccent(pathname: string, gameId?: string) {
    if (pathname.startsWith('/game/') && gameId) {
      return getGameById(gameId)?.accent || 'silver';
    }

    return routeAccents[pathname] || 'silver';
  }
</script>

<div class={`shell ${accent}`}>
  <Sidebar {accent} />

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
  :global(body) {
    margin: 0;
    min-height: 100vh;
    background: #404149;
    color: #f3f1f7;
    font-family: 'Segoe UI Variable Text', 'Segoe UI', sans-serif;
  }

  :global(*) {
    box-sizing: border-box;
  }

  :global(a) {
    color: inherit;
    text-decoration: none;
  }

  .shell {
    --page-accent: #d8d9de;
    display: grid;
    grid-template-columns: 13.5rem minmax(0, 1fr);
    min-height: 100vh;
    background:
      radial-gradient(circle at left top, rgba(255, 255, 255, 0.06) 0%, transparent 35%),
      #404149;
  }

  .shell.gold {
    --page-accent: #b69b57;
  }

  .shell.olive {
    --page-accent: #8a9a54;
  }

  .frame {
    display: flex;
    flex-direction: column;
    min-width: 0;
  }

  main {
    flex: 1;
    padding: 1.4rem 1.7rem 1rem;
    animation: rise 360ms ease;
  }

  footer {
    display: grid;
    grid-template-columns: repeat(4, minmax(0, 1fr));
    gap: 1rem;
    padding: 1rem 1.7rem 1.4rem;
    border-top: 1px solid rgba(255, 255, 255, 0.08);
    color: rgba(217, 213, 224, 0.42);
    font-size: 0.7rem;
  }

  .column {
    display: flex;
    flex-direction: column;
    gap: 0.35rem;
  }

  .column span {
    color: rgba(217, 213, 224, 0.34);
  }

  @keyframes rise {
    from {
      opacity: 0;
      transform: translateY(0.8rem);
    }

    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  @media (max-width: 920px) {
    .shell {
      grid-template-columns: 1fr;
    }

    main {
      padding: 1rem;
    }

    footer {
      grid-template-columns: repeat(2, minmax(0, 1fr));
      padding: 1rem;
    }
  }
</style>
