
import child_process from 'child_process'

var url = 'https://www.rust-vue-admin.com'
var cmd = ''
switch (process.platform) {
  case 'win32':
    cmd = 'start'
    child_process.exec(cmd + ' ' + url)
    break

  case 'darwin':
    cmd = 'open'
    child_process.exec(cmd + ' ' + url)
    break
}
