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

<IconWrapper on:click on:click {dataCy} {style}>
  <path
    fill-rule="evenodd"
    clip-rule="evenodd"
    d="M8 7C8 5.34315 9.34315 4 11 4H17C18.6569 4 20 5.34315 20 7V13C20 14.6569
    18.6569 16 17 16H11C9.34315 16 8 14.6569 8 13V7ZM11 6C10.4477 6 10 6.44772
    10 7V13C10 13.5523 10.4477 14 11 14H17C17.5523 14 18 13.5523 18 13V7C18
    6.44772 17.5523 6 17 6H11ZM6.70711 9.29289C7.09763 9.68342 7.09763 10.3166
    6.70711 10.7071C6.38481 11.0294 6 11.3298 6 11.8284V17C6 17.5523 6.44772 18
    7 18H12.1716C12.6702 18 12.9706 17.6152 13.2929 17.2929C13.6834 16.9024
    14.3166 16.9024 14.7071 17.2929C15.0976 17.6834 15.0976 18.3166 14.7071
    18.7071C13.9936 19.4206 13.2415 20 12.1716 20H7C5.34315 20 4 18.6569 4
    17V11.8284C4 10.7585 4.5794 10.0064 5.29289 9.29289C5.68342 8.90237 6.31658
    8.90237 6.70711 9.29289Z" />
</IconWrapper>
