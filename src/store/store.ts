import { reactive } from 'vue'
import { Log } from '../models/models';

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
    this.active_log = log;
    console.log(this.active_log);
  }
})