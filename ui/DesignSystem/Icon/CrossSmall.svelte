<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import IconWrapper from "./IconWrapper.svelte";

  export let dataCy: string | undefined = undefined;
  export let style: string | undefined = undefined;
</script>

<IconWrapper on:click {dataCy} {style}>
  <path
    fill-rule="evenodd"
    clip-rule="evenodd"
    d="M7.29289 7.29289C7.68342 6.90237 8.31658 6.90237 8.70711 7.29289L12
    10.5858L15.2929 7.29289C15.6834 6.90237 16.3166 6.90237 16.7071
    7.29289C17.0976 7.68342 17.0976 8.31658 16.7071 8.70711L8.70711
    16.7071C8.31658 17.0976 7.68342 17.0976 7.29289 16.7071C6.90237 16.3166
    6.90237 15.6834 7.29289 15.2929L10.5858 12L7.29289 8.70711C6.90237 8.31658
    6.90237 7.68342 7.29289 7.29289ZM13.2929 13.2929C13.6834 12.9024 14.3166
    12.9024 14.7071 13.2929L16.7071 15.2929C17.0976 15.6834 17.0976 16.3166
    16.7071 16.7071C16.3166 17.0976 15.6834 17.0976 15.2929 16.7071L13.2929
    14.7071C12.9024 14.3166 12.9024 13.6834 13.2929 13.2929Z" />
</IconWrapper>
