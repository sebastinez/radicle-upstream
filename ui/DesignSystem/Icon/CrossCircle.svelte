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
  <path
    fill-rule="evenodd"
    clip-rule="evenodd"
    d="M12 4C7.58172 4 4 7.58172 4 12C4 16.4183 7.58172 20 12 20C16.4183 20 20
    16.4183 20 12C20 7.58172 16.4183 4 12 4ZM2 12C2 6.47715 6.47715 2 12
    2C17.5228 2 22 6.47715 22 12C22 17.5228 17.5228 22 12 22C6.47715 22 2
    17.5228 2 12Z" />
</IconWrapper>
