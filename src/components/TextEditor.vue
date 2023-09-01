<script setup lang="ts">
import { invoke } from '@tauri-apps/api'
import { store } from '../store/store.js'
import { debounce } from 'lodash-es'
import { ref, nextTick } from 'vue'

const textareaElement = ref<HTMLTextAreaElement | null>(null);

const update = debounce((e) => {
	store.update_content(e.target.value);
}, 100)

const checkKeyDown = async (e: KeyboardEvent) => {
	if (e.key === "Tab") {
		if (textareaElement != null && textareaElement.value != null) {
			e.preventDefault();
			const start = textareaElement.value.selectionStart;
			const end = textareaElement.value.selectionEnd;
			store.active_log.content = store.active_log.content.substring(0, start) + "    " + store.active_log.content.substring(end);
			await nextTick() // Wait a tick so that the caret position restarts before setting it to the correct position
			textareaElement.value.setSelectionRange(start + 4, end + 4);
		}
	}
}

const savecontent = debounce((e) => {
	invoke('update_daily_log', { dailyLog: store.active_log })
		.then((response) => {
			console.log("Done");
		})
}, 1000)

</script>


<template>
	<div class="editor">
		<textarea ref="textareaElement" class="input" v-model="store.active_log.content" @input="update"
			@keydown="checkKeyDown" @keyup="savecontent"></textarea>
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
	tab-size: 2em;
	font-family: 'Ubuntu Mono', monospace;
	font-weight: 400;
	/* Currently makes the caret too tall */
	/* line-height: 2em; */
}
</style>
