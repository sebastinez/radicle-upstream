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
    d="M11 7.5C11 8.317 11.683 9 12.5 9C13.317 9 14 8.317 14 7.5C14 6.683 13.317
    6 12.5 6C11.683 6 11 6.683 11 7.5Z" />
  <path
    d="M10 10C9.44772 10 9 10.4477 9 11C9 11.5523 9.44772 12 10
    12H10.8195L10.3047 15.089C10.0507 16.6128 11.2258 18 12.7707 18H13C13.5523
    18 14 17.5523 14 17C14 16.4477 13.5523 16 13 16H12.7707C12.4617 16 12.2267
    15.7226 12.2775 15.4178L12.9864 11.1644C13.0347 10.8744 12.953 10.5779
    12.763 10.3536C12.573 10.1293 12.294 10 12 10H10Z" />
</IconWrapper>
