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
    d="M10.3962 6.83738C10.2636 5.86548 11.0191 5 12 5C12.9809 5 13.7364 5.86548
    13.6038 6.83738L12.5092 14.8649C12.4346 15.4121 12.8177 15.9162 13.3649
    15.9908C13.9121 16.0655 14.4162 15.6823 14.4908 15.1351L15.5855
    7.1076C15.8818 4.93485 14.1929 3 12 3C9.80714 3 8.11823 4.93485 8.41451
    7.1076L9.50917 15.1351C9.5838 15.6823 10.0879 16.0655 10.6351
    15.9908C11.1823 15.9162 11.5655 15.4121 11.4908 14.8649L10.3962 6.83738Z" />
  <path
    d="M12 21.5C13.1046 21.5 14 20.6046 14 19.5C14 18.3954 13.1046 17.5 12
    17.5C10.8954 17.5 10 18.3954 10 19.5C10 20.6046 10.8954 21.5 12 21.5Z" />
</IconWrapper>
