import { cherryInstance } from './components/CherryMarkdown';
import { listen } from "@tauri-apps/api/event";
import { open } from '@tauri-apps/plugin-dialog';
import { readTextFile } from '@tauri-apps/plugin-fs';

type File = string;
const App = () => {
  const cherryMarkdown = cherryInstance();

  listen('new_file', () => {
    cherryMarkdown.setMarkdown('');
  });

  listen('open_file', async () => {
    const path = await open({
      multiple: false, directory: false, filters: [{
        name: 'markdown',
        extensions: ['md']
      }]
    })

    if (path === null) {
      return;
    }

    const markdown = await readTextFile(path);
     cherryMarkdown.setMarkdown(markdown);
  });

  listen<File>('save', (event) => {
    cherryMarkdown.setMarkdown(event.payload);
  });

  listen<File>('save_as', (event) => {
    cherryMarkdown.setMarkdown(event.payload);
  });
}

export default App;