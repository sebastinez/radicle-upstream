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
    d="M7 3C5.34315 3 4 4.34315 4 6C4 7.65685 5.34315 9 7 9C8.65685 9 10 7.65685
    10 6C10 4.34315 8.65685 3 7 3ZM6 6C6 5.44772 6.44772 5 7 5C7.55228 5 8
    5.44772 8 6C8 6.55228 7.55228 7 7 7C6.44772 7 6 6.55228 6 6Z" />
  <path
    fill-rule="evenodd"
    clip-rule="evenodd"
    d="M17 15.1707V11C17 10.4477 16.5523 10 16 10H15C14.4477 10 14 9.55228 14
    9C14 8.44772 14.4477 8 15 8H16C17.6569 8 19 9.34315 19 11V15.1707C20.1652
    15.5825 21 16.6938 21 18C21 19.6569 19.6569 21 18 21C16.3431 21 15 19.6569
    15 18C15 16.6938 15.8348 15.5825 17 15.1707ZM17 18C17 17.4477 17.4477 17 18
    17C18.5523 17 19 17.4477 19 18C19 18.5523 18.5523 19 18 19C17.4477 19 17
    18.5523 17 18Z" />
  <path
    fill-rule="evenodd"
    clip-rule="evenodd"
    d="M4 18C4 16.3431 5.34315 15 7 15C8.65685 15 10 16.3431 10 18C10 19.6569
    8.65685 21 7 21C5.34315 21 4 19.6569 4 18ZM7 17C6.44772 17 6 17.4477 6 18C6
    18.5523 6.44772 19 7 19C7.55228 19 8 18.5523 8 18C8 17.4477 7.55228 17 7 17Z" />
  <path
    d="M8 11C8 10.4477 7.55228 10 7 10C6.44772 10 6 10.4477 6 11L6 13C6 13.5523
    6.44772 14 7 14C7.55228 14 8 13.5523 8 13V11Z" />
  <path
    d="M14.7071 6.70711L12.4142 9L14.7071 11.2929C15.0976 11.6834 15.0976
    12.3166 14.7071 12.7071C14.3166 13.0976 13.6834 13.0976 13.2929
    12.7071L10.2929 9.70711C9.90237 9.31658 9.90237 8.68342 10.2929
    8.29289L13.2929 5.29289C13.6834 4.90237 14.3166 4.90237 14.7071
    5.29289C15.0976 5.68342 15.0976 6.31658 14.7071 6.70711Z" />
</IconWrapper>
