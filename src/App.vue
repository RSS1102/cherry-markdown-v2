<script setup lang="ts">
import { cherryInstance } from './components/CherryMarkdown';
import { listen } from "@tauri-apps/api/event";
import { open } from '@tauri-apps/plugin-dialog';
import { readTextFile, writeTextFile } from '@tauri-apps/plugin-fs';
import { useFileStore } from './store';

type File = string;

const cherryMarkdown = cherryInstance();
const fileStore = useFileStore();

listen('new_file', () => {
  cherryMarkdown.setMarkdown('');
  fileStore.setCurrentFilePath('');
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
  fileStore.setCurrentFilePath(path);
  const markdown = await readTextFile(path);
  cherryMarkdown.setMarkdown(markdown);
});

listen<File>('save', () => {
  const markdown = cherryMarkdown.getMarkdown();
  writeTextFile(fileStore.currentFilePath, markdown);
});

listen<File>('save_as', (event) => {
  cherryMarkdown.setMarkdown(event.payload);
});
</script>