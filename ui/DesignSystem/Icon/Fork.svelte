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
    d="M3 6C3 7.65685 4.34315 9 6 9C7.65685 9 9 7.65685 9 6C9 4.34315 7.65685 3
    6 3C4.34315 3 3 4.34315 3 6ZM6 7C5.44772 7 5 6.55228 5 6C5 5.44772 5.44772 5
    6 5C6.55228 5 7 5.44772 7 6C7 6.55228 6.55228 7 6 7Z" />
  <path
    fill-rule="evenodd"
    clip-rule="evenodd"
    d="M14.8293 14C14.4175 12.8348 13.3062 12 12 12C10.6938 12 9.58254 12.8348
    9.17071 14H8C7.44772 14 7 13.5523 7 13V11C7 10.4477 6.55229 10 6 10C5.44771
    10 5 10.4477 5 11V13C5 14.6569 6.34315 16 8 16H9.17071C9.58254 17.1652
    10.6938 18 12 18C13.3062 18 14.4175 17.1652 14.8293 16H16C17.6569 16 19
    14.6569 19 13V8C19 7.44772 18.5523 7 18 7C17.4477 7 17 7.44772 17 8V13C17
    13.5523 16.5523 14 16 14H14.8293ZM12 16C11.4477 16 11 15.5523 11 15C11
    14.4477 11.4477 14 12 14C12.5523 14 13 14.4477 13 15C13 15.5523 12.5523 16
    12 16Z" />
  <path
    d="M14.2929 7.70711C14.6834 8.09763 15.3166 8.09763 15.7071 7.70711L18
    5.41421L20.2929 7.70711C20.6834 8.09763 21.3166 8.09763 21.7071
    7.70711C22.0976 7.31658 22.0976 6.68342 21.7071 6.29289L18.7071
    3.29289C18.3166 2.90237 17.6834 2.90237 17.2929 3.29289L14.2929
    6.29289C13.9024 6.68342 13.9024 7.31658 14.2929 7.70711Z" />
  <path
    d="M13 20C13 19.4477 12.5523 19 12 19C11.4477 19 11 19.4477 11 20L11 21C11
    21.5523 11.4477 22 12 22C12.5523 22 13 21.5523 13 21V20Z" />
</IconWrapper>
