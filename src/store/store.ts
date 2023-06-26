import { reactive } from 'vue'
import { Log } from '../models/models';
import { invoke } from '@tauri-apps/api'

export const store = reactive({
  active_log: {} as Log,
  all_logs: [] as Log[], 
  set_logs(logs: Log[]) {
    this.all_logs = logs;
  },
  update_content(content: string) {
    console.log("update content")
    this.active_log.content = content;
  },
  select_dailylog(log: Log) {
    invoke('update_daily_log', { dailyLog: store.active_log })
		.then((response) => {
			console.log("Saved on change");
		})
    this.active_log = log;
    console.log(this.active_log);
    store.update_store();
  },
  update_store() {
    invoke('get_log_dates', {})
		.then((response) => {
			store.set_logs(response as Log[]);
			console.log(store.all_logs);
		})
  }
})