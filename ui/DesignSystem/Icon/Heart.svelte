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
    d="M7 4C3.67985 4 2 6.99299 2 10C2 13.9227 4.7619 18.7818 10.1286
    20.9285C10.6414 21.1336 11.2234 20.8842 11.4285 20.3714C11.6336 19.8586
    11.3842 19.2766 10.8714 19.0715C6.2381 17.2182 4 13.0773 4 10C4 8.11046
    4.81536 6 7 6C8.78367 6 9.7444 6.95126 10.3113 7.98192C10.6668 8.62832
    10.9142 9.3647 11.0049 10.0994C11.061 10.6002 11.4917 11.0007 12
    11.0001C12.5164 11.0007 12.9448 10.5975 12.9949 10.1007L12.9951
    10.0994C13.0858 9.3647 13.3332 8.62832 13.6887 7.98192C14.2556 6.95126
    15.2163 6 17 6C19.1846 6 20 8.11046 20 10C20 13.0773 17.7619 17.2182 13.1286
    19.0715C12.6158 19.2766 12.3664 19.8586 12.5715 20.3714C12.7766 20.8842
    13.3586 21.1336 13.8714 20.9285C19.2381 18.7818 22 13.9227 22 10C22 6.99299
    20.3201 4 17 4C14.3535 4 12.8243 5.47024 12 6.90478C11.1757 5.47024 9.64655
    4 7 4Z" />
</IconWrapper>
