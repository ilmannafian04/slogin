<script>
  import { JWT_KEY } from '$lib/constant';
  import { userStatus } from '$lib/stores/user';

  const logoutHandler = () => {
    localStorage.removeItem(JWT_KEY);
    userStatus.update((status) => {
      status.isSignedIn = false;
      return status;
    });
  };
</script>

<nav class="navbar navbar-expand-lg navbar-light bg-light">
  <div class="container-fluid">
    <a class="navbar-brand" href="/">SLogin</a>
    <button
      class="navbar-toggler"
      type="button"
      data-bs-toggle="collapse"
      data-bs-target="#navbarSupportedContent"
      aria-controls="navbarSupportedContent"
      aria-expanded="false"
      aria-label="Toggle navigation"
    >
      <span class="navbar-toggler-icon" />
    </button>
    <div class="collapse navbar-collapse" id="navbarSupportedContent">
      <ul class="navbar-nav me-auto mb-2 mb-lg-0">
        {#if !$userStatus.isSignedIn}
          <li class="nav-item">
            <a class="nav-link" href="/signin">Sign In</a>
          </li>
          <li class="nav-item">
            <a class="nav-link" href="/signup">Sign Up</a>
          </li>
        {/if}
      </ul>
    </div>
    {#if $userStatus.isSignedIn}
      <button class="btn btn-outline-danger" type="button" on:click|preventDefault={logoutHandler}
        >Logout</button
      >
    {/if}
  </div>
</nav>
