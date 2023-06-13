<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import TextEditor from "./components/TextEditor.vue";
import Sidebar from "./components/Sidebar.vue";
</script>

<template>
  <div id="container">
    <Sidebar />
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

#editor {
  height: 100%;
  flex: 5;
  min-width: 500px;
}
</style>
