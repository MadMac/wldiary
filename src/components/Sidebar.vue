<script setup lang="ts">
import { invoke } from '@tauri-apps/api'

invoke('get_log_dates', {})
	.then((response) => console.log(response))

</script>

<template>
	<div id="sidebar">
		<div class="sidebar-item">
			2023-01-01
		</div>
	</div>
</template>

<style scoped>
#sidebar {
	border-right: 1px solid #242424;
	height: 100%;
	max-width: 300px;
	min-width: 200px;
	flex: 1;
	background-color: #303030;
	-webkit-tap-highlight-color: transparent;
	-webkit-touch-callout: none;
	-webkit-user-select: none;
	-khtml-user-select: none;
	-moz-user-select: none;
	-ms-user-select: none;
	user-select: none;
	overflow-y: scroll;
}

#sidebar:focus {
	outline: none !important;
}

.sidebar-item {
	width: 100%;
	height: 60px;
	border-bottom: 1px solid #242424;
	cursor: pointer;
	padding-top: 17px;
	padding-left: 15px;
}

.sidebar-item:hover {
	background-color: #3f3f3f;
}
</style>
