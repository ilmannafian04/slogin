<script lang="ts">
  import api from '$lib/api';

  let isGet = false;
  let key = '';
  let value = '';
  let valueOfKey = '';
  const submitHandler = () => {
    if (isGet) {
      api
        .retrieve(key)
        .then((res) => {
          valueOfKey = res;
        })
        .catch(console.error);
    } else {
      api
        .store(key, value)
        .then(() => {
          isGet = true;
        })
        .catch(console.error);
    }
  };
</script>

<form on:submit|preventDefault={submitHandler}>
  <div class="form-check form-switch">
    <input class="form-check-input" type="checkbox" role="switch" id="isGet" bind:checked={isGet} />
    <label class="form-check-label" for="isGet">{isGet ? 'Get' : 'Set'}</label>
  </div>
  <div class="mb-3">
    <label for="key" class="form-label">Key</label>
    <input type="text" class="form-control" id="key" bind:value={key} />
  </div>
  {#if isGet}
    <div>
      <legend>Value: {valueOfKey}</legend>
    </div>
  {:else}
    <div class="mb-3">
      <label for="value" class="form-label">Value</label>
      <input type="text" class="form-control" id="value" bind:value />
    </div>
  {/if}
  <button type="submit" class="btn btn-primary">Submit</button>
</form>
