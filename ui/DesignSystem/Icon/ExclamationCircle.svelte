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
    d="M12 8C11.1267 8 10.4859 8.82068 10.6977 9.66789L11.4701 12.7575C11.6041
    13.2933 11.2783 13.8362 10.7425 13.9701C10.2067 14.1041 9.66381 13.7783
    9.52986 13.2425L8.75746 10.153C8.23009 8.04346 9.82558 6 12 6C14.1744 6
    15.7699 8.04346 15.2425 10.153L14.4701 13.2425C14.3362 13.7783 13.7933
    14.1041 13.2575 13.9701C12.7217 13.8362 12.3959 13.2933 12.5299
    12.7575L13.3022 9.66789C13.5141 8.82068 12.8733 8 12 8Z" />
  <path
    d="M13.75 16.25C13.75 17.2165 12.9665 18 12 18C11.0335 18 10.25 17.2165
    10.25 16.25C10.25 15.2835 11.0335 14.5 12 14.5C12.9665 14.5 13.75 15.2835
    13.75 16.25Z" />
  <path
    fill-rule="evenodd"
    clip-rule="evenodd"
    d="M12 2C6.47715 2 2 6.47715 2 12C2 17.5228 6.47715 22 12 22C17.5228 22 22
    17.5228 22 12C22 6.47715 17.5228 2 12 2ZM4 12C4 7.58172 7.58172 4 12
    4C16.4183 4 20 7.58172 20 12C20 16.4183 16.4183 20 12 20C7.58172 20 4
    16.4183 4 12Z" />
</IconWrapper>
