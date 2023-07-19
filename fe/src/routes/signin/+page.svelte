<script lang="ts">
  import { goto } from '$app/navigation';

  import api from '$lib/api';
  import { JWT_KEY } from '$lib/constant';
  import type { User } from '../..';

  const formData: User = {
    username: '',
    password: '',
  };
  const submitHandler = () => {
    api
      .signin(formData)
      .then((jwt) => {
        localStorage.setItem(JWT_KEY, jwt);
        goto('/');
      })
      .catch(console.error);
  };
</script>

<form on:submit|preventDefault={submitHandler}>
  <div class="mb-3">
    <label for="username" class="form-label">Username</label>
    <input type="text" class="form-control" id="username" bind:value={formData.username} />
  </div>
  <div class="mb-3">
    <label for="password" class="form-label">Password</label>
    <input type="password" class="form-control" id="password" bind:value={formData.password} />
  </div>
  <button type="submit" class="btn btn-primary">Submit</button>
</form>
