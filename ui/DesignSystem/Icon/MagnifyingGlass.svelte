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
    d="M11 5C7.68629 5 5 7.68629 5 11C5 14.3137 7.68629 17 11 17C14.3137 17 17
    14.3137 17 11C17 7.68629 14.3137 5 11 5ZM7 11C7 8.79086 8.79086 7 11
    7C13.2091 7 15 8.79086 15 11C15 13.2091 13.2091 15 11 15C8.79086 15 7
    13.2091 7 11Z" />
  <path
    d="M19.7071 18.2929L17.7071 16.2929C17.3166 15.9024 16.6834 15.9024 16.2929
    16.2929C15.9024 16.6834 15.9024 17.3166 16.2929 17.7071L18.2929
    19.7071C18.6834 20.0976 19.3166 20.0976 19.7071 19.7071C20.0976 19.3166
    20.0976 18.6834 19.7071 18.2929Z" />
</IconWrapper>
