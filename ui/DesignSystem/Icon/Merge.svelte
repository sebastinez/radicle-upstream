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
    d="M14.2929 7.70711C14.6834 8.09763 15.3166 8.09763 15.7071 7.70711C16.0976
    7.31658 16.0976 6.68342 15.7071 6.29289L12.7071 3.29289C12.3166 2.90237
    11.6834 2.90237 11.2929 3.29289L8.29289 6.29289C7.90237 6.68342 7.90237
    7.31658 8.29289 7.70711C8.68342 8.09763 9.31658 8.09763 9.70711 7.70711L12
    5.41421L14.2929 7.70711Z" />
  <path
    fill-rule="evenodd"
    clip-rule="evenodd"
    d="M7 14V15.1707C8.16519 15.5825 9 16.6938 9 18C9 19.6569 7.65685 21 6
    21C4.34315 21 3 19.6569 3 18C3 16.6938 3.83481 15.5825 5 15.1707V14C5
    12.3431 6.34315 11 8 11H10C10.5523 11 11 11.4477 11 12C11 12.5523 10.5523 13
    10 13H8C7.44772 13 7 13.4477 7 14ZM5 18C5 17.4477 5.44772 17 6 17C6.55228 17
    7 17.4477 7 18C7 18.5523 6.55228 19 6 19C5.44772 19 5 18.5523 5 18Z" />
  <path
    fill-rule="evenodd"
    clip-rule="evenodd"
    d="M19 14V15.1707C20.1652 15.5825 21 16.6938 21 18C21 19.6569 19.6569 21 18
    21C16.3431 21 15 19.6569 15 18C15 16.6938 15.8348 15.5825 17 15.1707V14C17
    13.4477 16.5523 13 16 13H14C12.3431 13 11 11.6569 11 10V8C11 7.44771 11.4477
    7 12 7C12.5523 7 13 7.44771 13 8V10C13 10.5523 13.4477 11 14 11H16C17.6569
    11 19 12.3431 19 14ZM17 18C17 17.4477 17.4477 17 18 17C18.5523 17 19 17.4477
    19 18C19 18.5523 18.5523 19 18 19C17.4477 19 17 18.5523 17 18Z" />
</IconWrapper>
