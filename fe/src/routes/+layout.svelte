<script lang="ts">
  import { browser } from '$app/environment';
  import Nav from '$lib/components/Nav.svelte';
  import { JWT_KEY } from '$lib/constant';
  import { userStatus } from '$lib/stores/user';

  if (browser) {
    if (localStorage.getItem(JWT_KEY) !== null) {
      userStatus.update((status) => {
        status.isSignedIn = true;
        return status;
      });
    }
  }
</script>

<Nav />
<div class="container mt-3">
  <slot />
</div>
