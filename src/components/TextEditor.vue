<script setup lang="ts">
import { ref } from "vue";
import { store } from '../store/store.js'
import { debounce } from 'lodash-es'

const update = debounce((e) => {
	store.update_content(e.target.value);
}, 100)

</script>


<template>
	<div class="editor">
		<textarea class="input" v-model="store.active_log.content" @input="update"></textarea>
	</div>
</template>

<style scoped>
.editor {
	height: 100%;
	display: flex;
}

.input:focus {
	outline: 0px;
	box-shadow: 0px;
	outline-offset: 0px;
}

.input {
	resize: none;
	border: 0px;
	height: 100%;
	padding-top: 30px;
	padding-bottom: 30px;
	padding-left: 50px;
	padding-right: 50px;
	background-color: #2d2d2d;
	color: #fff;
	flex: 1;
	font-size: 1.1em;
	/* Currently makes the caret too tall */
	/* line-height: 2em; */
}
</style>
