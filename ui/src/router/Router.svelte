<script lang="typescript">
  import UserProfileScreen from "ui/Screen/UserProfile.svelte";

  import { state } from "ui/src/router";

</script>

{#if 'component' in $state}
  <svelte:component this={$state.component} {...$state.props} />
{:else if $state.type === 'userProfile'}
  <UserProfileScreen urn={$state.urn} activeTabString={$state.activeTab} />
{/if}
