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
    d="M8 4C8 3.44772 7.55228 3 7 3C6.44772 3 6 3.44772 6 4V9C6 9.55228 6.44772
    10 7 10C7.55228 10 8 9.55228 8 9L8 4Z" />
  <path
    fill-rule="evenodd"
    clip-rule="evenodd"
    d="M17 7C17.5523 7 18 7.44772 18 8V11C18 13.2091 16.2091 15 14
    15H9.82929C9.41746 16.1652 8.30622 17 7 17C5.34315 17 4 15.6569 4 14C4
    12.3431 5.34315 11 7 11C8.30622 11 9.41746 11.8348 9.82929 13H14C15.1046 13
    16 12.1046 16 11V8C16 7.44772 16.4477 7 17 7ZM7 13C7.55228 13 8 13.4477 8
    14C8 14.5523 7.55228 15 7 15C6.44772 15 6 14.5523 6 14C6 13.4477 6.44772 13
    7 13Z" />
  <path
    d="M7 18C7.55228 18 8 18.4477 8 19V20C8 20.5523 7.55228 21 7 21C6.44772 21 6
    20.5523 6 20L6 19C6 18.4477 6.44772 18 7 18Z" />
  <path
    d="M19.2929 7.70711C19.6834 8.09763 20.3166 8.09763 20.7071 7.70711C21.0976
    7.31658 21.0976 6.68342 20.7071 6.29289L17.7071 3.29289C17.3166 2.90237
    16.6834 2.90237 16.2929 3.29289L13.2929 6.29289C12.9024 6.68342 12.9024
    7.31658 13.2929 7.70711C13.6834 8.09763 14.3166 8.09763 14.7071 7.70711L17
    5.41421L19.2929 7.70711Z" />
</IconWrapper>
