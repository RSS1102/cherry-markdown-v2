import { defineStore } from 'pinia';

export const useFileStore = defineStore('file', {
  /**
   * @params currentFilePath 当前的文件路径
   * 
   */
  state: () => ({
    currentFilePath: ''
  }),

  actions: {
    setCurrentFilePath(filePath: string) {
      this.currentFilePath = filePath;
    },
  }
});