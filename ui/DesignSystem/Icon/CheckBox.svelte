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
    d="M7 6C6.44772 6 6 6.44772 6 7V17C6 17.5523 6.44772 18 7 18H17C17.5523 18
    18 17.5523 18 17V13C18 12.4477 18.4477 12 19 12C19.5523 12 20 12.4477 20
    13V17C20 18.6569 18.6569 20 17 20H7C5.34315 20 4 18.6569 4 17V7C4 5.34315
    5.34315 4 7 4H15C15.5523 4 16 4.44772 16 5C16 5.55228 15.5523 6 15 6H7ZM19.6
    6.2C20.0418 6.53137 20.1314 7.15817 19.8 7.6L15.1836 13.7552C14.0882 15.2157
    11.9532 15.3674 10.6623 14.0765L8.29289 11.7071C7.90237 11.3166 7.90237
    10.6834 8.29289 10.2929C8.68342 9.90237 9.31658 9.90237 9.70711
    10.2929L12.0765 12.6623C12.5068 13.0926 13.2185 13.042 13.5836 12.5552L18.2
    6.4C18.5314 5.95817 19.1582 5.86863 19.6 6.2Z" />
</IconWrapper>
