<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import EmptyState from "ui/App/ScreenLayout/EmptyState.svelte";
</script>

<style>
  .container {
    width: 100vw;
    height: 100vh;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
  }
</style>

<div class="container">
  <EmptyState headerText="Loading..." emoji="🕵️" text="" />
</div>
