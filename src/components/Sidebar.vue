<script setup lang="ts">
import { invoke } from '@tauri-apps/api'
import { store } from '../store/store.js'
import { Log } from '../models/models'
// @ts-ignore  
import { TrashIcon, FolderIcon } from '@heroicons/vue/24/solid'
import { confirm, open } from '@tauri-apps/api/dialog'

store.update_store();

const add_today_date = () => {
	invoke('add_today_date', {})
		.then((response) => {
			console.log(response)
			if (response) {
				store.select_dailylog(response as Log);
			}
		});

	store.update_store();
}

const delete_log = async (daily_log: Log) => {
	const deleteConfirmed = await confirm('Are you sure you want to delete the log?');

	if (deleteConfirmed === true) {
		invoke('delete_daily_log', { dailyLog: daily_log })
			.then((response) => {
				console.log(response)
				store.update_store();
			});
	}
	console.log(daily_log);
}

const open_file_chooser = async () => {
	console.log("Open file chooser");
	const openDatabase = await open({
		filters: [{
			name: 'Database',
			extensions: ['db']
		}]
	});

	// TODO: Make sure the database is valid
	// Do migration if empty?

	console.log(openDatabase);
}

</script>

<template>
	<div id="sidebar">
		<div class="sidebar-item" id="add-item" @click="add_today_date()">
			+
		</div>
		<div class="sidebar-item" v-for="log in store.all_logs" @click="store.select_dailylog(log)"
			:class="{ 'active-item': log.id === store.active_log.id }">
			{{ log.log_date }}
			<div class="trash-icon-container" @click.stop="delete_log(log)">
				<TrashIcon />
			</div>
		</div>
		<div class="load-icon-container" @click="open_file_chooser()">
			<FolderIcon />
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

#add-item {
	text-align: center;
	padding-left: 0px;
	font-size: 1.8em;
}

.load-icon-container {
	width: 25px;
	height: 25px;
	position: absolute;
	bottom: 10px;
	left: 10px;
	padding: 3px;
	border-radius: 10px;
}

.load-icon-container:hover {
	background-color: #8d8d8d;
}

.trash-icon-container {
	width: 25px;
	height: 25px;
	float: right;
	margin-right: 20px;
	padding: 3px;
	vertical-align: middle;
	text-align: center;
	border-radius: 25px;
	z-index: -1;
}

.trash-icon-container:hover {
	background-color: #8d8d8d;
}

.sidebar-item {
	width: 100%;
	height: 60px;
	border-bottom: 1px solid #242424;
	cursor: pointer;
	padding-top: 17px;
	padding-left: 15px;
	overflow: hidden;
}

.sidebar-item:hover {
	background-color: #3f3f3f;
}

.active-item {
	/* background-color: #3a3a3a; */
	background-color: #242424;
	font-weight: 700;
}
</style>
