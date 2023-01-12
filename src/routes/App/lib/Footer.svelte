<script lang="ts">
  export let footerText: string;
</script>

<p class="footerText">{footerText}</p>

<style>
  .footerText {
    text-align: center;
    font-size: 12px;
    font-weight: 500;
    color: var(--secondary-text-color);
    font-family: Helvetica;
    font-style: normal;
    margin-top: 8px;
  }
</style>
