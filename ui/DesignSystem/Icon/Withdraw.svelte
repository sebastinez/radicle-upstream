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
    d="M5 11C5.55228 11 6 11.4477 6 12V17C6 17.5523 6.44772 18 7 18H17C17.5523 18 18 17.5523 18 17V12C18 11.4477 18.4477 11 19 11C19.5523 11 20 11.4477 20 12V17C20 18.6569 18.6569 20 17 20H7C5.34315 20 4 18.6569 4 17V12C4 11.4477 4.44772 11 5 11Z" />
  <path
    fill-rule="evenodd"
    clip-rule="evenodd"
    d="M12 8C12.5523 8 13 8.44772 13 9V14C13 14.5523 12.5523 15 12 15C11.4477 15 11 14.5523 11 14V9C11 8.44772 11.4477 8 12 8Z" />
  <path
    fill-rule="evenodd"
    clip-rule="evenodd"
    d="M7.29289 9.70711C7.68342 10.0976 8.31658 10.0976 8.70711 9.70711L12 6.41421L15.2929 9.70711C15.6834 10.0976 16.3166 10.0976 16.7071 9.70711C17.0976 9.31658 17.0976 8.68342 16.7071 8.29289L12.7071 4.29289C12.3166 3.90237 11.6834 3.90237 11.2929 4.29289L7.29289 8.29289C6.90237 8.68342 6.90237 9.31658 7.29289 9.70711Z" />
</IconWrapper>
