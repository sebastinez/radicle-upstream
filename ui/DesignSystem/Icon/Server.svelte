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
    d="M6 3C4.34315 3 3 4.34315 3 6V10C3 10.7684 3.28885 11.4692 3.7639
    12C3.28885 12.5308 3 13.2316 3 14V18C3 19.6569 4.34315 21 6 21H17C17.5523 21
    18 20.5523 18 20C18 19.4477 17.5523 19 17 19H6C5.44772 19 5 18.5523 5
    18V14C5 13.4477 5.44772 13 6 13H18C18.5523 13 19 13.4477 19 14V18C19 18.5523
    19.4477 19 20 19C20.5523 19 21 18.5523 21 18V14C21 12.3431 19.6569 11 18
    11H6C5.44772 11 5 10.5523 5 10V6C5 5.44772 5.44772 5 6 5H18C18.5523 5 19
    5.44772 19 6V9C19 9.55228 19.4477 10 20 10C20.5523 10 21 9.55228 21 9V6C21
    4.34315 19.6569 3 18 3H6Z" />
  <path
    d="M7 8C7 7.44772 7.44772 7 8 7H10C10.5523 7 11 7.44772 11 8C11 8.55228
    10.5523 9 10 9H8C7.44772 9 7 8.55228 7 8Z" />
  <path
    d="M7 16C7 15.4477 7.44772 15 8 15H13C13.5523 15 14 15.4477 14 16C14 16.5523
    13.5523 17 13 17H8C7.44772 17 7 16.5523 7 16Z" />
  <path
    d="M13 9C13.5523 9 14 8.55228 14 8C14 7.44772 13.5523 7 13 7C12.4477 7 12
    7.44772 12 8C12 8.55228 12.4477 9 13 9Z" />
</IconWrapper>
