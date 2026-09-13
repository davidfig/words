import { BOARD } from './constants';
import { Board as BoardData } from '../pkg/words';

class Board {
  private element: HTMLElement;
  private data: BoardData;
  private _width!: number;
  private _height!: number;

  constructor(board: HTMLElement) {
    this.element = board;
    const size = this.characterSize();
    this.width = size.width * BOARD.width;
    this.height = size.height * BOARD.height;
    this.data = new BoardData(BOARD.width, BOARD.height);
  }

  // converts from character size to screen size
  private characterSize(): { width: number, height: number } {
    const test = document.createElement('div');
    test.style.margin = '0';
    test.style.padding = '0';
    test.style.width = 'fit-content';
    test.style.height = 'fit-content';
    test.style.fontFamily = "monospace";
    test.innerHTML = 'X';
    document.body.appendChild(test);
    const size = test.getBoundingClientRect();
    test.remove();
    return { width: size.width, height: size.height };
  }

  set width(width: number) {
    this._width = width;
    this.element.style.width = `${width}px`;
  }
  get width(): number { return this._width; }

  set height(height: number) {
    this._height = height;
    this.element.style.height = `${height}px`;
  }
  get height(): number { return this._height; }

  put(x: number, y: number, ch: string) {
    this.data.put(x, y, ch);
    this.element.innerHTML = this.data.print();
  }

  get(x: number, y: number): string {
    return this.data.get(x, y);
  }
}

export function create(board: HTMLElement) {
  const b = new Board(board);

  b.put(5, 5, "A");
  console.log(b.get(5, 5));
}