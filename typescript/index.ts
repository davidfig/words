import { create } from './board';
import init from "../pkg/words.js";

window.onload = async () => {
  const board = document.querySelector('.board') as HTMLElement;
  if (!board) throw new Error("Could not find base element");

  await init();
  create(board);
};