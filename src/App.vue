<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import TextEditor from "./components/TextEditor.vue";
</script>

<template>
  <div id="container">
    <div id="sidebar">
      test
    </div>
    <div id="editor">
      <TextEditor />
    </div>
  </div>
</template>

<style scoped>
#container {
  display: flex;
  flex-direction: row;
  margin: 0px;
  height: 100%;
  overflow: hidden;
}

#sidebar {
  border-right: 1px solid black;
  height: 100%;
  min-width: 200px;
  flex: 1;
}

#editor {
  height: 100%;
  flex: 5;
  min-width: 500px;
}
</style>
