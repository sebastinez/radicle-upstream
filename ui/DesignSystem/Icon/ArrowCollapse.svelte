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
    d="M19 11C19.5523 11 20 10.5523 20 10C20 9.44772 19.5523 9 19 9H15V5C15
    4.44772 14.5523 4 14 4C13.4477 4 13 4.44772 13 5V10C13 10.5523 13.4477 11 14
    11H19Z" />
  <path
    d="M15.7929 6.79289C15.4024 7.18342 15.4024 7.81658 15.7929 8.20711C16.1834
    8.59763 16.8166 8.59763 17.2071 8.20711L19.7071 5.70711C20.0976 5.31658
    20.0976 4.68342 19.7071 4.29289C19.3166 3.90237 18.6834 3.90237 18.2929
    4.29289L15.7929 6.79289Z" />
  <path
    d="M8.20711 17.2071C8.59763 16.8166 8.59763 16.1834 8.20711 15.7929C7.81658
    15.4024 7.18342 15.4024 6.79289 15.7929L4.29289 18.2929C3.90237 18.6834
    3.90237 19.3166 4.29289 19.7071C4.68342 20.0976 5.31658 20.0976 5.70711
    19.7071L8.20711 17.2071Z" />
  <path
    d="M4 14C4 13.4477 4.44772 13 5 13H10C10.5523 13 11 13.4477 11 14V19C11
    19.5523 10.5523 20 10 20C9.44772 20 9 19.5523 9 19V15H5C4.44772 15 4 14.5523
    4 14Z" />
</IconWrapper>
