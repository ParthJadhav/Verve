<script lang="ts">
  export let id: string;
  export let isChecked: boolean;
  export let onToggle: () => void;
</script>

<label class="switch">
  <input type="checkbox" {id} bind:checked={isChecked} on:click={onToggle} />
  <span class="slider round" />
</label>

<style>
  .switch {
    position: relative;
    display: inline-block;
    width: 45px;
    height: 18px;
  }

  .switch input {
    opacity: 0;
    width: 0;
    height: 0;
  }

  .slider {
    position: absolute;
    cursor: pointer;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background-color: var(--secondary-bg-color);
    -webkit-transition: 0.4s;
    transition: 0.4s;
  }

  .slider.round {
    border-radius: 5px;
  }

  .slider:before {
    position: absolute;
    content: '';
    height: 12px;
    width: 12px;
    left: 4px;
    bottom: 3px;
    background-color: var(--primary-accent-color);
    -webkit-transition: 0.4s;
    transition: 0.4s;
  }

  input:checked + .slider {
    background-color: var(--primary-text-color);
  }

  input:checked + .slider:before {
    -webkit-transform: translateX(26px);
    -ms-transform: translateX(26px);
    transform: translateX(26px);
  }

  .slider.round:before {
    border-radius: 50%;
  }
</style>
