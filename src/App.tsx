import { cherryInstance } from './components/CherryMarkdown';

const App = () => {
  const cherryMarkdown = cherryInstance();

  cherryMarkdown.setMarkdown(`+ Hello, CherryMarkdown!`); 

}

export default App;