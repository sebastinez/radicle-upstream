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
    d="M5.29289 5.29289C5.68342 4.90237 6.31658 4.90237 6.70711 5.29289L12
    10.5858L17.2929 5.29289C17.6834 4.90237 18.3166 4.90237 18.7071
    5.29289C19.0976 5.68342 19.0976 6.31658 18.7071 6.70711L6.70711
    18.7071C6.31658 19.0976 5.68342 19.0976 5.29289 18.7071C4.90237 18.3166
    4.90237 17.6834 5.29289 17.2929L10.5858 12L5.29289 6.70711C4.90237 6.31658
    4.90237 5.68342 5.29289 5.29289ZM13.2929 13.2929C13.6834 12.9024 14.3166
    12.9024 14.7071 13.2929L18.7071 17.2929C19.0976 17.6834 19.0976 18.3166
    18.7071 18.7071C18.3166 19.0976 17.6834 19.0976 17.2929 18.7071L13.2929
    14.7071C12.9024 14.3166 12.9024 13.6834 13.2929 13.2929Z" />
</IconWrapper>
