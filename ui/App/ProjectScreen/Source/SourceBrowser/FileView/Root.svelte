<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import type { CommitHeader } from "ui/src/source";
  import type { Root } from "ui/src/screen/project/source";

  import EmptyState from "ui/App/ScreenLayout/EmptyState.svelte";
  import Readme from "../Readme.svelte";

  import CommitTeaser from "../CommitTeaser.svelte";

  export let commit: CommitHeader;
  export let view: Root;
  export let emptyRepo: boolean;
</script>

<style>
  .commit-header {
    height: 2.5rem;
    margin-bottom: 1rem;
  }
</style>

<div class="commit-header">
  <CommitTeaser {commit} on:select style="height: 100%" />
</div>

{#if view.readme}
  <Readme content={view.readme.content} path={view.readme.path} />
{:else if emptyRepo}
  <EmptyState
    text="This project doesn't have any files yet."
    emoji="👀"
    style="height: 320px;" />
{:else}
  <EmptyState
    text="This project doesn't have a README yet."
    emoji="👀"
    style="height: 320px;" />
{/if}
