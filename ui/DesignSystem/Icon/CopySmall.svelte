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
    d="M9 8C9 6.34315 10.3431 5 12 5H16C17.6569 5 19 6.34315 19 8V12C19 13.6569
    17.6569 15 16 15H12C10.3431 15 9 13.6569 9 12V8ZM12 7C11.4477 7 11 7.44772
    11 8V12C11 12.5523 11.4477 13 12 13H16C16.5523 13 17 12.5523 17 12V8C17
    7.44772 16.5523 7 16 7H12ZM7.70711 10.2929C8.09763 10.6834 8.09763 11.3166
    7.70711 11.7071C7.38481 12.0294 7 12.3298 7 12.8284V16C7 16.5523 7.44772 17
    8 17H11.1716C11.6702 17 11.9706 16.6152 12.2929 16.2929C12.6834 15.9024
    13.3166 15.9024 13.7071 16.2929C14.0976 16.6834 14.0976 17.3166 13.7071
    17.7071C12.9936 18.4206 12.2415 19 11.1716 19H8C6.34315 19 5 17.6569 5
    16V12.8284C5 11.7585 5.5794 11.0064 6.29289 10.2929C6.68342 9.90237 7.31658
    9.90237 7.70711 10.2929Z" />
</IconWrapper>
