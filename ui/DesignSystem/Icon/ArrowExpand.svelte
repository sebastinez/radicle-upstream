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
    d="M13 5C13 4.44772 13.4477 4 14 4H19C19.5523 4 20 4.44772 20 5V10C20
    10.5523 19.5523 11 19 11C18.4477 11 18 10.5523 18 10V6H14C13.4477 6 13
    5.55228 13 5Z" />
  <path
    d="M17.2071 8.20711C17.5976 7.81658 17.5976 7.18342 17.2071 6.79289C16.8166
    6.40237 16.1834 6.40237 15.7929 6.79289L13.2929 9.29289C12.9023 9.68342
    12.9023 10.3166 13.2929 10.7071C13.6834 11.0976 14.3166 11.0976 14.7071
    10.7071L17.2071 8.20711Z" />
  <path
    d="M6.79289 15.7929C6.40237 16.1834 6.40237 16.8166 6.79289 17.2071C7.18342
    17.5976 7.81658 17.5976 8.20711 17.2071L10.7071 14.7071C11.0976 14.3166
    11.0976 13.6834 10.7071 13.2929C10.3166 12.9024 9.68342 12.9024 9.29289
    13.2929L6.79289 15.7929Z" />
  <path
    d="M10 20C10.5523 20 11 19.5523 11 19C11 18.4477 10.5523 18 10 18H6V14C6
    13.4477 5.55228 13 5 13C4.44772 13 4 13.4477 4 14V19C4 19.5523 4.44772 20 5
    20H10Z" />
</IconWrapper>
