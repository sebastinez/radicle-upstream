<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import { createEventDispatcher } from "svelte";

  import { Button, CopyableIdentifier } from "ui/DesignSystem";

  export let peerId: string;

  const dispatch = createEventDispatcher();

  const onKeydown = (event: KeyboardEvent) => {
    if (event.key === "Enter") {
      wallet();
    }
  };

  const profile = () => {
    dispatch("profile");
  };

  const wallet = () => {
    dispatch("wallet");
  };
</script>

<style>
  .container {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100%;
  }

  .content {
    display: flex;
    flex-direction: column;
    align-items: center;
  }

  .buttons {
    display: flex;
    width: 100%;
    gap: 1rem;
    justify-content: center;
  }
</style>

<svelte:window on:keydown={onKeydown} />

<div class="container">
  <div class="content">
    <h1 style="text-align: center; margin-bottom: 1.5rem;">All set!</h1>

    <CopyableIdentifier value={peerId} kind="deviceId" />

    <p
      style="text-align: center; width: 23.13rem; margin: 1.75rem 0 1.75rem 0;
      color: var(--color-foreground-level-6);">
      This is your
      <span class="typo-text-bold">Device ID</span>! It's unique to this device.
      You can find it at any time on your Profile or Settings page. You'll need
      to share it with others to collaborate.
    </p>
    <div class="buttons">
      <Button
        variant="transparent"
        dataCy="go-to-profile-button"
        on:click={profile}>
        Go to profile
      </Button>
      <Button dataCy="go-to-wallet-button" on:click={wallet}>
        Set up your wallet
      </Button>
    </div>
  </div>
</div>
