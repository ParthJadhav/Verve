<script lang="ts">
  import { getFileName, getTruncatedFilePath } from '../../../utils/path';
  import { getIcon } from '../../../utils/icon';
  export let filePath;
  export let resultType: number;
</script>

<button on:click class="searchResult" id={filePath}>
  <div class="resultContent">
    {#await getIcon(getFileName(filePath).replace(/.app$/, ''))}
      <span class="icon" />
    {:then { icon, fallbackIcon }}
      <img
        class="icon"
        src={icon}
        alt=""
        on:error={event => {
          // @ts-ignore
          event.target.src = fallbackIcon;
        }}
      />
    {/await}
    <p class="fileName">
      {getFileName(filePath).replace(/.app$/, '')}
    </p>
  </div>
  {#if resultType == 2}
    <div class="resultPathDiv">
      <p class="resultPathText">
        {getTruncatedFilePath(filePath)}
      </p>
    </div>
  {/if}
</button>

<style>
  .resultPathDiv {
    position: relative;
    right: 0%;
  }

  .resultPathText {
    font-family: Helvetica;
    font-style: normal;
    font-weight: 500;
    font-size: 12px;
    line-height: 20px;
    margin: 0;
    color: var(--secondary-text-color);
    width: 250px;
    text-align: right;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .searchResult {
    margin-top: 7px;
    margin-left: 12px;
    box-sizing: border-box;
    display: flex;
    flex-direction: row;
    align-items: center;
    padding: 0px 12px;
    width: 726px;
    height: 43px;
    background: transparent;
    border: none;
    border-radius: 8px;
    flex: none;
    order: 1;
    flex-grow: 0;
  }
  .resultContent {
    display: flex;
    flex-direction: row;
    align-items: center;
    flex: none;
    order: 0;
    flex-grow: 0;
    margin-right: auto;
  }

  .icon {
    display: inline-flex;
    width: 24px;
    height: 24px;
    margin-right: 8px;
  }

  .fileName {
    font-family: Helvetica;
    font-style: normal;
    font-weight: 500;
    font-size: 14px;
    line-height: 20px;
    margin: 0;
    color: var(--primary-text-color);
    text-align: left;
    width: 250px;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
</style>
