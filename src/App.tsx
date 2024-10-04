import { cherryInstance } from './components/CherryMarkdown';
import { listen } from "@tauri-apps/api/event";

type File = string
const App = () => {
  const cherryMarkdown = cherryInstance();

  listen<File>('open_file', (event) => cherryMarkdown.setMarkdown(event.payload));

}

export default App;