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
    d="M7 6C7 5.44772 7.44772 5 8 5H12.8847C13.1361 5 13.3783 5.09468 13.563
    5.2652L16.6783 8.14086C16.8834 8.33017 17 8.59656 17 8.87566V17C17 17.5523
    17.4477 18 18 18C18.5523 18 19 17.5523 19 17V8.87566C19 8.03837 18.6501
    7.23917 18.0348 6.67125L14.9195 3.79559C14.3654 3.28405 13.6389 3 12.8847
    3H8C6.34315 3 5 4.34315 5 6V18C5 19.6569 6.34315 21 8 21H15C15.5523 21 16
    20.5523 16 20C16 19.4477 15.5523 19 15 19H8C7.44772 19 7 18.5523 7 18V6Z" />
  <path
    d="M9 6C8.44772 6 8 6.44772 8 7C8 7.55228 8.44772 8 9 8H11C11.5523 8 12
    7.55228 12 7C12 6.44772 11.5523 6 11 6H9Z" />
  <path
    d="M9 10C8.44772 10 8 10.4477 8 11C8 11.5523 8.44772 12 9 12H15C15.5523 12
    16 11.5523 16 11C16 10.4477 15.5523 10 15 10H9Z" />
  <path
    d="M9 13C8.44772 13 8 13.4477 8 14C8 14.5523 8.44772 15 9 15H13C13.5523 15
    14 14.5523 14 14C14 13.4477 13.5523 13 13 13H9Z" />
  <path
    d="M9 16C8.44772 16 8 16.4477 8 17C8 17.5523 8.44772 18 9 18H14C14.5523 18
    15 17.5523 15 17C15 16.4477 14.5523 16 14 16H9Z" />
</IconWrapper>
