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
    d="M8.8889 6C6.84656 6 5 7.8577 5 10.5031C5 14.2486 9.21846 17.2028 11.6785
    17.9172C11.8896 17.9785 12.1104 17.9785 12.3215 17.9172C14.7815 17.2028 19
    14.2486 19 10.5031C19 7.8577 17.1534 6 15.1111 6C13.1785 6 12 8.11085 12
    8.11085C12 8.11085 10.8215 6 8.8889 6ZM9.99999 11C10.5523 11 11 10.5523 11
    10C11 9.44772 10.5523 9 9.99999 9C9.4477 9 8.99999 9.44772 8.99999
    10C8.99999 10.5523 9.4477 11 9.99999 11ZM14 11C14.5523 11 15 10.5523 15
    10C15 9.44772 14.5523 9 14 9C13.4477 9 13 9.44772 13 10C13 10.5523 13.4477
    11 14 11ZM8.37595 12.584C8.60571 12.2393 9.07137 12.1462 9.41601
    12.376C11.0528 13.4672 12.9637 13.4561 14.584 12.376C14.9286 12.1462 15.3943
    12.2393 15.624 12.584C15.8538 12.9286 15.7607 13.3943 15.416 13.624C13.2834
    15.0457 10.701 15.0354 8.58396 13.624C8.23932 13.3943 8.14619 12.9286
    8.37595 12.584Z" />
</IconWrapper>
